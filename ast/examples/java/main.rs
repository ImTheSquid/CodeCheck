use ast::{java::JavaTree, SyntaxTree};

fn main() {
    let java_file = include_str!("T1.java");
    let t = JavaTree::try_from(java_file.to_string()).unwrap();
    let t = t.symbol_tree().unwrap();
    let mut out = Vec::new();
    syntree::print::print_with_source(&mut out, &t, java_file).unwrap();
    let out = String::from_utf8(out).unwrap();
    println!("{out}");
}
