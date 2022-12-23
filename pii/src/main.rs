//https://github.com/valarauca/xlsx-rs/blob/master/src/lib.rs
use walkdir::WalkDir;
use regex::RegexSet;
use std::fs;
use std::path::Path;

fn main() {
    let pii = RegexSet::new(&[
    r"name",
    r"email",
    r"birthday",
    r"ssn",
    r"credit",
    r"card",
    r"phone"
    ]).unwrap();

    let folders: [&str; 4] = ["Users", "inetpub", "xampp", "ProgramData"];
    for folder in folders {
        let root = "C:\\";
        let folder = root.to_owned() + folder;
        if Path::new(&folder).is_dir() {
            for file in WalkDir::new(folder).into_iter().filter_map(|file| file.ok()) {
                let file_string = file.path().to_str().unwrap();
                if file_string.ends_with(".txt") | file_string.ends_with(".csv") | file_string.ends_with(".doc") | file_string.ends_with(".docx"){
                    let file_path = file.path().to_str().unwrap();
                    let Ok(contents_string) = fs::read_to_string(file_path)
                    else {
                        continue;
                    };
                    let contents_str = &contents_string[..];
                    if pii.is_match(contents_str) {
                        println!("{:?}",file.path().display());
                    }
                }
                else if file_string.ends_with(".xlsx") | file_string.ends_with(".xls"){
                    continue; //EDIT THIS HERE STUFF
                }
            }
        }
    }
}
