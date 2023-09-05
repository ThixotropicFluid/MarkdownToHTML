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
#[derive(Debug, Clone)]
enum HTMLComponent {
    Content(String),
    Heading(i32),
    NewLine,
}
impl MDFile {
    fn to_html(&self, config: &Configuration) -> HTMLFile {
        let mut converted_text = config.preamble.to_owned();
        converted_text.push_str(&self.contents[..]);

        let html_components = self.get_html_components().unwrap();
        dbg!(html_components.clone());
        converted_text.push_str(&config.postamble[..]);
        HTMLFile {
            contents: converted_text.clone(),
            path: self.path.clone(),
        }
    }
    fn get_html_components(&self) -> Result<Vec<HTMLComponent>, &str> {
        let mut r = 0; // index of next character to read
        let mut html_components: Vec<HTMLComponent> = vec![];

        let mut content_string: Vec<u8> = vec![];
        while r < self.contents.len() {
            let byte = self.contents.as_bytes().get(r).unwrap();
            r += 1;
            match byte {
                0x23 => {
                    html_components.push(HTMLComponent::Content(
                        String::from_utf8(content_string.clone()).unwrap(),
                    ));
                    content_string = vec![];

                    let mut heading_level: i32 = 1;
                    while r < self.contents.len() {
                        let byte = self.contents.as_bytes().get(r).unwrap();
                        if *byte == 0x23 {
                            r += 1;
                            heading_level += 1;
                        } else {
                            html_components.push(HTMLComponent::Heading(heading_level));
                            break;
                        }
                    }
                }
                0x0a => {
                    html_components.push(HTMLComponent::NewLine);
                }
                0x0d => {}
                _ => content_string.push(*byte),
            }
        }
        html_components.push(HTMLComponent::Content(
            String::from_utf8(content_string.clone()).unwrap(),
        ));

        Ok(html_components)
    }
}
#[derive(Debug)]
#[allow(dead_code)]
struct HTMLFile {
    contents: String,
    path: String,
}
struct Configuration {
    preamble: String,
    postamble: String,
}

struct FormatState {
    heading_level: i32,
}
impl Default for FormatState {
    fn default() -> FormatState {
        FormatState { heading_level: 0 }
    }
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
fn load_html_components() -> Configuration {
    let preamble = fs::read_to_string("preamble.txt").expect("unable to load preamble.txt\n");
    let postamble = fs::read_to_string("postamble.txt").expect("unable to load postamble.txt\n");
    Configuration {
        preamble,
        postamble,
    }
}
//const PREAMBLE: &str = r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><title>title</title><link rel="stylesheet" href="style.css"><script src="script.js"></script></head><body><p>"#;
//const POSTAMBLE: &str = r#"</p></body></html>"#;

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
fn convert_all_files_to_html(files: &Vec<MDFile>, config: &Configuration) -> Vec<HTMLFile> {
    let mut html_files: Vec<HTMLFile> = Vec::new();
    for md_file in files {
        html_files.push(md_file.to_html(&config));
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
    let config = load_html_components();
    populate_file_structure(&mut files);
    let html_files = convert_all_files_to_html(&files, &config);

    save_html_files(&html_files);
    //let test_html = md_to_html(files.get(0).unwrap());
    //print!("\n\n{}\n\n", test_html.contents);
}
