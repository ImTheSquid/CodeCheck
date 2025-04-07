use std::{env::args, fs::read_to_string, path::PathBuf, str::FromStr};

use ast::{c::CTree, cpp::CppTree, guess_language_from_path, java::JavaTree, Language, SyntaxTree};
use ndarray::Array1;

fn main() {
    let target = args().nth(1).expect("file as input!");
    let target = PathBuf::from_str(&target).expect("Valid path!");
    let target_text = read_to_string(&target).expect("Valid read");
    let language = guess_language_from_path(&target).expect("Valid language as input");
    let tensor: Array1<f64> = match language {
        Language::C => {
            let tree = CTree::try_from(target_text)
                .expect("Valid C parse")
                .symbol_tree()
                .expect("C tree build");

            tree.first().expect("non-empty C tree").value().into()
        }
        Language::Cpp => {
            let tree = CppTree::try_from(target_text)
                .expect("Valid C++ parse")
                .symbol_tree()
                .expect("C++ tree build");

            tree.first().expect("non-empty c++ tree").value().into()
        }
        Language::Java => {
            let tree = JavaTree::try_from(target_text)
                .expect("Valid Java parse")
                .symbol_tree()
                .expect("Java tree build");

            tree.first().expect("non-empty java tree").value().into()
        }
        Language::Python => {
            todo!();
        }
    };

    println!("Parsed tensor with enum dimensions: {:?}", tensor.dim());
}
