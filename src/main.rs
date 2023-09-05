use std::env;
use std::fs;

#[derive(Debug)]
#[allow(dead_code)]
struct MDFile {
    contents: String,
    path: String,
    html: Option<Box<HTMLFile>>,
}
#[derive(Debug)]
#[allow(dead_code)]
struct HTMLFile {
    contents: String,
    path: String,
    md: Option<Box<MDFile>>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn get_index(dir: String) -> MDFile {
    let index_path = String::from(dir.clone() + "/index.md");
    let index = match fs::read_to_string(index_path.clone()) {
        Ok(x) => x,
        Err(x) => {
            println!("No index.md file found in directory {}", x);
            std::process::exit(2);
        }
    };
    MDFile {
        contents: index.clone(),
        path: index_path.clone(),
        html: None,
    }
}
fn populate_file_structure() {}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(x) => x.clone(),
        None => {
            println!("Usage: markdown_to_html file_path_to_index");
            std::process::exit(1);
        }
    };
    let index = get_index(path.clone());
    let mut files: Vec<MDFile> = vec![index];
    dbg!(files.get(0));
}
