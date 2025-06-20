use std::{env::args, fs::read_to_string, path::PathBuf, str::FromStr};

use ast::{
    c::CTree, cpp::CppTree, guess_language_from_path, java::JavaTree, prune_tree,
    python::PythonTree, Language, SyntaxTree,
};
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

            for itm in tree.walk() {
                println!("TR: {:?} ({:?})", itm.range(), itm.value());
            }
            tree.first().expect("non-empty java tree").value().into()
        }
        Language::Python => {
            // Add a NEWLINE to the end, just to be safe
            let target_text = format!("{target_text}\n");
            let tt = target_text.clone();
            let mut tree = PythonTree::try_from(target_text)
                .expect("Valid Python parse")
                .symbol_tree()
                .expect("Python tree build");

            // let mut s = Vec::new();
            // syntree::print::print_with_source(&mut s, &tree, &tt).expect("print");
            // println!("TREE:\n{}", String::from_utf8(s).unwrap());
            let old = tree.len();

            prune_tree(&mut tree);
            let mut s = Vec::new();
            syntree::print::print_with_source(&mut s, &tree, &tt).expect("print");
            // println!("PRUNED TREE:\n{}", String::from_utf8(s).unwrap());
            let new = tree.len();
            println!("Old: {old} New: {new} Diff: {diff}", diff = old - new);

            tree.first().expect("non-empty python tree").value().into()
        }
    };

    println!("Parsed tensor with enum dimensions: {:?}", tensor.dim());
}
