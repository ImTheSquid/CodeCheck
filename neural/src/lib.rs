#![feature(new_range_api)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use core::panic;
use std::{
    ffi::CStr,
    path::{Path, PathBuf},
};

use ndarray::{arr2, arr3};
use pyo3::{
    ffi::c_str,
    intern,
    prelude::*,
    types::{PyDict, PyList},
};
use util::{Mark, Pair};

pub mod contrastive;
pub mod critic;
pub mod data;
pub mod elu;
pub mod gat;
pub mod loss;
pub mod model;
pub mod node_process;
pub mod sequential;

fn leaky_gain(slope: f64) -> f64 {
    (2.0 / (1.0 + slope.powi(2))).sqrt()
}

mod python_files {
    use std::ffi::CStr;

    use pyo3::ffi::c_str;
    pub const MAIN: &CStr = c_str!(include_str!("../py/main.py"));
    pub const ACTOR: &CStr = c_str!(include_str!("../py/actor.py"));
    pub const CRITIC: &CStr = c_str!(include_str!("../py/critic.py"));
}

fn debug_python_env(py: Python<'_>) {
    let sys = py.import("sys").unwrap();
    let version: String = sys.getattr("version").unwrap().extract().unwrap();
    println!("Python version: {}", version);

    let prefix: String = sys.getattr("prefix").unwrap().extract().unwrap();
    println!("Python prefix: {}", prefix);

    let executable: String = sys.getattr("executable").unwrap().extract().unwrap();
    println!("Python executable: {}", executable);

    let python_path = sys.getattr("path").unwrap();
    let python_path: Vec<String> = python_path.extract().unwrap();
    println!("Python path: {:?}", python_path);

    let os = py.import("os").unwrap();
    let path: String = os
        .getattr("getcwd")
        .unwrap()
        .call0()
        .unwrap()
        .extract()
        .unwrap();
    println!("Current directory: {}", path);
}

pub fn initialize_python(py: Python<'_>, venv_location: Option<PathBuf>) {
    if let Some(location) = venv_location {
        let location = if !location.is_absolute() {
            PathBuf::new().join(".").join(location)
        } else {
            location
        };
        let location = location
            .canonicalize()
            .expect("valid canoicalization")
            .join("bin/activate_this.py");
        let location = location.to_string_lossy();
        println!("Loading venv from {location}");
        let code = format!(
            r#"
activate_this = "{location}"
exec(open(activate_this).read(), {{'__file__': activate_this}})"#
        );

        py.run(
            unsafe { CStr::from_ptr(code.as_ptr() as *const i8) },
            None,
            None,
        )
        .unwrap();
    }

    debug_python_env(py);

    assert!(
        py.import("math").is_ok(),
        "Something is very wrong, math import failed!"
    );

    assert!(
        py.import("torch").is_ok(),
        "PyTorch not found! Ensure a virtual environment is present with the necessary packages."
    );

    PyModule::from_code(py, python_files::ACTOR, c_str!("actor.py"), c_str!("actor"))
        .expect("Import actor");
    PyModule::from_code(
        py,
        python_files::CRITIC,
        c_str!("critic.py"),
        c_str!("critic"),
    )
    .expect("Import critic");
    PyModule::from_code(
        py,
        python_files::MAIN,
        c_str!("main.py"),
        c_str!("codecheck"),
    )
    .expect("Import main");
}

pub struct KeyData {
    pub a: usize,
    pub b: usize,
    pub marks: Vec<Mark>,
}

pub fn train(
    py: Python<'_>,
    features: &[ndarray::Array2<f64>],
    edges: &[ndarray::Array2<usize>],
    keys: &[KeyData],
) -> PyResult<()> {
    let keys = PyList::new(
        py,
        keys.iter().map(|k| {
            let marks = k
                .marks
                .iter()
                .map(|m| {
                    [
                        m.a.start as f64,
                        m.b.start as f64,
                        m.a.end as f64,
                        m.b.end as f64,
                    ]
                })
                .collect::<Vec<_>>();

            let marks = arr2(&marks);

            let marks = numpy::PyArray2::from_array(py, &marks);

            ((k.a, k.b), marks)
        }),
    )?;

    let keys = PyDict::from_sequence(&keys)?;

    let features = PyList::new(
        py,
        features.iter().map(|a| numpy::PyArray2::from_array(py, a)),
    )
    .expect("valid list");

    let edges = PyList::new(py, edges.iter().map(|a| numpy::PyArray2::from_array(py, a)))
        .expect("valid list");

    let codecheck = py.import("codecheck")?;

    codecheck
        .getattr(intern!(py, "rust_train"))
        .expect("codecheck module to contain rust entry point")
        .call1((features, edges, keys))?;

    Ok(())
}
