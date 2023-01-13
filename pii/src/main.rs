/***********************
*   Import Libraries   *
***********************/
use regex::RegexSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
/********************
*   Main Function   *
********************/
fn main() {
    let file_extensions = vec!["txt", "csv", "doc"];                                                    // file extensions to scan
    let xml_extensions = vec!["docx", "xlsx"];                                                          // x file extensions to scan

    let pii = RegexSet::new(&[
        r"name", "email", "birthday", "ssn", "credit", "card", "phone",                                 // pii strings to search for in files
    ])
    .unwrap();

    let root = if cfg!(target_os = "windows") {                                                         // if windows, set root dir = C:\
        PathBuf::from(r"C:\")
    } else {                                                                                            // else, set root dir = /         
        PathBuf::from("/")
    };
    let folders = match std::env::args()                                                                // pass folders to scan as arguments
        .skip(1)
        .map(|path| {
            let mut new_path = root.clone();
            new_path.push(Path::new(&path));
            new_path
        })
        .collect::<Vec<_>>() {                                              
        fs if fs.is_empty() => vec![root],
        other => other
    };

    for folder in folders {                                                                             // recursively scan for pii
        if folder.is_dir() {                                                                            // if item is a folder, enter new iteration                            
            for file in WalkDir::new(folder)
                .into_iter()
                .filter_map(|file| file.ok())
            {
                if file_extensions.iter().any(|ext| file.path().ends_with(ext)) {                       // if item is a regular file, scan it for pii
                    //find pii
                    let file_path = file.path().to_str().unwrap();
                    let contents_string = match fs::read_to_string(file_path) {
                        Ok(contents) => contents,
                        Err(e) => {
                            eprintln!("Could not read file {}: {:?}", file_path.display(), e);
                            continue;
                        }
                    };
                    let contents_str = &contents_string[..];
                    if pii.is_match(contents_str) {
                        println!("{:?}", file.path().display());                                        // if pii found, print filepath
                    }
                } else if xml_extensions.iter().any(|ext| file.path().ends_with(ext)) {                 // if item is an x file, scan differently
                    continue; //EDIT THIS HERE STUFF
                }
            }
        }
    }
}
