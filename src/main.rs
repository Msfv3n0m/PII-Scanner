use walkdir::WalkDir;
use regex::RegexSet;
use std::fs;
use std::path::Path;
fn main() {
    let file_extensions = RegexSet::new(&[
    r"txt$",
    "csv$",
    "doc$",
    "doc$"]).unwrap();

    let pii = RegexSet::new(&[
    r"name",
    "email",
    "birthday",
    "ssn",
    "credit",
    "card",
    "phone"
    ]).unwrap();
    let folders: [&str; 4] = ["Users", "inetpub", "xampp", "ProgramData"];//
    for folder in folders {
        let folder = "C:\\".to_owned() + folder;
        if Path::new(&folder).is_dir() {
            for file in WalkDir::new(folder).into_iter().filter_map(|file| file.ok()) {
                if file_extensions.is_match(file.path().to_str().unwrap()) {
                    //find pii
                    let file_path = file.path().to_str().unwrap();
                    let contents_string = fs::read_to_string(file_path).unwrap();
                    let contents_str = &contents_string[..];
                    if pii.is_match(contents_str) {
                        println!("{:?}",file.path().display());
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

