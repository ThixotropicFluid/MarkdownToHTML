use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
struct MDFile {
    contents: String,
    path: String,
}
impl MDFile {
    fn to_html(&self) -> HTMLFile {
        let mut converted_text = String::from(PREAMBLE);
        converted_text.push_str(&self.contents[..]);
        converted_text.push_str(POSTAMBLE);
        HTMLFile {
            contents: converted_text.clone(),
            path: self.path.clone(),
        }
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct HTMLFile {
    contents: String,
    path: String,
}
impl HTMLFile {
    fn save(&self) -> io::Result<()> {
        let input_path = Path::new(&self.path);
        let mut file_path = PathBuf::new();

        // Replace ".md" with ".html" in the file name
        if let Some(file_name) = input_path.file_name() {
            if let Some(name) = file_name.to_str() {
                if name.ends_with(".md") {
                    file_path.push(input_path.parent().unwrap()); // Preserve the parent directory
                    file_path.push(name.trim_end_matches(".md"));
                    file_path.set_extension("html");
                }
            }
        }

        let mut file = File::create(&file_path)?;
        file.write(self.contents.as_bytes())?;
        Ok(())
    }
}

const PREAMBLE: &str = r#"
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>title</title>
    <link rel="stylesheet" href="style.css">
    <script src="script.js"></script>
  </head>
    <body>
        <p> 
        "#;
const POSTAMBLE: &str = r#"
        </p>
    </body>
</html>
"#;

fn populate_file_structure(files: &mut Vec<MDFile>) {
    let paths = fs::read_dir("./").expect("Error reading path");
    for path in paths {
        if let Ok(dir) = path {
            if let Some(ext) = dir.path().extension() {
                if ext == "md" {
                    if let Ok(file_contnets) = fs::read_to_string(dir.path()) {
                        files.push(MDFile {
                            contents: file_contnets,
                            path: dir.path().to_str().unwrap().to_owned(),
                        })
                    }
                }
            }
        }
    }
}
fn convert_all_files_to_html(files: &Vec<MDFile>) -> Vec<HTMLFile> {
    let mut html_files: Vec<HTMLFile> = Vec::new();
    for md_file in files {
        html_files.push(md_file.to_html());
    }
    html_files
}

fn save_html_files(files: &Vec<HTMLFile>) {
    for file in files {
        file.save().unwrap();
    }
}

fn main() {
    let mut files: Vec<MDFile> = vec![];
    populate_file_structure(&mut files);
    let html_files = convert_all_files_to_html(&files);

    save_html_files(&html_files);
    //let test_html = md_to_html(files.get(0).unwrap());
    //print!("\n\n{}\n\n", test_html.contents);
}
