use regex::RegexSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let file_extensions = vec!["txt", "csv", "doc"];

    let pii = RegexSet::new(&[
        r"name", "email", "birthday", "ssn", "credit", "card", "phone",
    ])
    .unwrap();

    let root = if cfg!(target_os = "windows") {
        PathBuf::from(r"C:\")
    } else {
        PathBuf::from("/")
    };
    let folders = match std::env::args()
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

    for folder in folders {
        if folder.is_dir() {
            for file in WalkDir::new(folder)
                .into_iter()
                .filter_map(|file| file.ok())
            {
                if file_extensions.iter().any(|ext| file.path().ends_with(ext)) {
                    //find pii
                    let file_path = file.path().to_str().unwrap();
                    let contents_string = fs::read_to_string(file_path).unwrap();
                    let contents_str = &contents_string[..];
                    if pii.is_match(contents_str) {
                        println!("{:?}", file.path().display());
                    }
                }
            }
        }
    }
}

// use walkdir::WalkDir;
// use regex::RegexSet;
// use std::fs;
// fn main() {
//     let file_extensions = RegexSet::new(&[
//     r"txt$",
//     "csv$",
//     "doc$",
//     "doc$"]).unwrap();

//     let pii = RegexSet::new(&[
//     r"name",
//     "email",
//     "birthday",
//     "ssn",
//     "credit",
//     "card",
//     "phone"
//     ]).unwrap();
//             for file in WalkDir::new("C:\\").into_iter().filter_map(|file| file.ok()) {
//                 if file_extensions.is_match(file.path().to_str().unwrap()) {
//                     //find pii
//                     let file_path = file.path().to_str().unwrap();
//                     let contents_string = fs::read_to_string(file_path).unwrap();
//                     let contents_str = &contents_string[..];
//                     if pii.is_match(contents_str) {
//                         println!("{:?}",file.path().display());
//                     }
//                 }
//             }
//         }
