#![feature(new_range_api)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{
    ffi::{CStr, CString},
    io::Cursor,
    path::PathBuf,
};

use mimalloc::MiMalloc;
use pyo3::{
    ffi::c_str,
    intern,
    prelude::*,
    types::{PyDict, PyList},
};
use util::Mark;

use crate::data::{rust_data, TmpDirDataset};

// pub mod contrastive;
// pub mod critic;
pub mod data;
// pub mod elu;
// pub mod gat;
// pub mod loss;
// pub mod model;
// pub mod node_process;
// pub mod sequential;

// fn leaky_gain(slope: f64) -> f64 {
//     (2.0 / (1.0 + slope.powi(2))).sqrt()
// }

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const AST_NAME_EMBEDDINGS: &[u8] = include_bytes!("ast_name_embeddings.npz");

mod python_files {
    use std::ffi::CStr;

    use pyo3::ffi::c_str;
    pub const MAIN: &CStr = c_str!(include_str!("../py/main.py"));
    pub const ACTOR: &CStr = c_str!(include_str!("../py/actor.py"));
    pub const CRITIC: &CStr = c_str!(include_str!("../py/critic.py"));
    pub const GRAPHHAM: &CStr = c_str!(include_str!("../py/graphham.py"));
    pub const EMBEDDING: &CStr = c_str!(include_str!("../py/embedding.py"));
}

#[allow(unused)]
fn debug_python_env(py: Python<'_>) {
    let sys = py.import("sys").unwrap();
    let version: String = sys.getattr("version").unwrap().extract().unwrap();
    println!("Python version: {version}");

    let prefix: String = sys.getattr("prefix").unwrap().extract().unwrap();
    println!("Python prefix: {prefix}");

    let executable: String = sys.getattr("executable").unwrap().extract().unwrap();
    println!("Python executable: {executable}");

    let python_path = sys.getattr("path").unwrap();
    let python_path: Vec<String> = python_path.extract().unwrap();
    println!("Python path: {python_path:?}");

    let os = py.import("os").unwrap();
    let path: String = os
        .getattr("getcwd")
        .unwrap()
        .call0()
        .unwrap()
        .extract()
        .unwrap();
    println!("Current directory: {path}");
}

pub fn add_rust_data(py: Python<'_>) -> PyResult<()> {
    let rd = rust_data::_PYO3_DEF.make_module(py, rust_data::__PYO3_GIL_USED)?;
    let sys = PyModule::import(py, "sys")?;
    let py_modules: Bound<'_, PyDict> = sys.getattr("modules")?.downcast_into()?;

    py_modules.set_item("rust_data", rd)?;

    Ok(())
}

pub fn mp_mode(py: Python<'_>) -> PyResult<()> {
    let sys = PyModule::import(py, "sys")?;
    let argv: Bound<'_, PyList> = sys.getattr("argv")?.downcast_into()?;
    argv.append("--multiprocessing-fork")?;

    Ok(())
}

pub fn initialize_python(py: Python<'_>, venv_location: Option<PathBuf>) {
    if let Some(location) = venv_location {
        let location = location
            .canonicalize()
            .expect("valid canoicalization")
            .join("bin/activate_this.py");
        let location = location.to_string_lossy();
        let code = format!(
            r#"
activate_this = "{location}"
exec(open(activate_this).read(), {{'__file__': activate_this}})"#
        );

        py.run(
            CString::new(code).expect("valid cstr").as_c_str(),
            None,
            None,
        )
        .unwrap();
    }

    // debug_python_env(py);

    assert!(
        py.import("math").is_ok(),
        "Sanity check failed: Something is very wrong, math import failed!"
    );

    assert!(
        py.import("torch").is_ok(),
        "Sanity check failed: PyTorch not found! Ensure a virtual environment is present with the necessary packages."
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
        python_files::GRAPHHAM,
        c_str!("graphham.py"),
        c_str!("graphham"),
    )
    .expect("Import graphham");
    PyModule::from_code(
        py,
        python_files::EMBEDDING,
        c_str!("embedding.py"),
        c_str!("embedding"),
    )
    .expect("Import embedding");
    PyModule::from_code(
        py,
        python_files::MAIN,
        c_str!("main.py"),
        c_str!("codecheck"),
    )
    .expect("Import main");
}

#[derive(Debug, Clone)]
pub struct KeyData {
    pub a: usize,
    pub b: usize,
    pub marks: Vec<Mark>,
}

pub fn train(
    py: Python<'_>,
    dataset: TmpDirDataset,
    mode: &str,
    artifact_dir: &str,
    top_k: usize,
    force_cpu: bool,
) -> PyResult<()> {
    let mut ast_embeddings =
        ndarray_npy::NpzReader::new(Cursor::new(AST_NAME_EMBEDDINGS)).expect("valid read");
    let emb: ndarray::Array2<f32> = ast_embeddings
        .by_name("embeddings")
        .expect("has embeddings");

    let codecheck = py.import("codecheck")?;

    let kwargs = PyDict::new(py);
    kwargs.set_item("dataset", dataset)?;
    kwargs.set_item("artifact_dir", artifact_dir)?;
    kwargs.set_item("mode", mode)?;
    kwargs.set_item("ast_embeddings", numpy::PyArray2::from_array(py, &emb))?;
    kwargs.set_item("top_k", top_k)?;
    kwargs.set_item("force_cpu", force_cpu)?;

    codecheck
        .getattr(intern!(py, "rust_train"))
        .expect("codecheck module to contain rust entry point")
        .call((), Some(&kwargs))?;

    Ok(())
}
