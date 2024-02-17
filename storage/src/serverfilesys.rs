//This program will get path of dir of submission of a student using their Purdue username
//It will gather path of the files that ends with .c, .java, .py, .cpp

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

// Function to recursively list file paths in the given directory path
fn list_file_paths<P: AsRef<Path>>(path: P) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let extensions = ["java", "cpp", "py", "c"];
    let set: HashSet<_> = extensions.iter().cloned().collect();

    // New function to handle recursion
    fn recurse_dir(path: &Path, set: &HashSet<&str>, paths: &mut Vec<PathBuf>) -> io::Result<()> {
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    // Recurse into the directory
                    recurse_dir(&path, set, paths)?;
                } else if path.is_file() {
                    if let Some(extension) = path.extension().and_then(|s| s.to_str()) {
                        if set.contains(extension) {
                            paths.push(path);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // Start the recursive directory traversal
    recurse_dir(path.as_ref(), &set, &mut paths)?;

    Ok(paths)
}

// Function to read the contents of a file
fn read_file_contents<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

fn main() {
    let dir_path = "../lab2"; // Adjust this path as needed

    match list_file_paths(dir_path) {
        Ok(file_paths) => {
            for path in file_paths {
                match read_file_contents(&path) {
                    Ok(contents) => {
                        println!("Contents of {:?}:\n{}", path, contents);
                    },
                    Err(e) => println!("Failed to read {:?}: {}", path, e),
                }
            }
        },
        Err(e) => println!("Failed to list files: {}", e),
    }
}
