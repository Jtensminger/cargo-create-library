use clap::{Command, Arg};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = Command::new("cargo-create-library")
        .version("0.1.0")
        .about("Create and import a new library crate in the package")
        .arg(
            Arg::new("name")
                .help("Name of the library crate to create")
                .required(true),
        )
        .get_matches();
        
    let library_name = matches.get_one::<String>("name").unwrap();
    let library_file_name = format!("src/{}.rs", library_name);

    create_library_file(&library_file_name);
    add_module_to_lib_rs(&library_file_name);
}

fn create_library_file(library_file_name: &str) {
    let mut file = File::create(library_file_name).expect("Failed to create library file");

    let file_contents = r#"
pub mod tests {
    use super::*;

    #[test]
    fn ext1() {
        // Add your test code here
    }
}
"#;

    file.write_all(file_contents.as_bytes())
        .expect("Failed to write to library file");
}

fn add_module_to_lib_rs(library_file_name: &str) {
    let module_name = Path::new(library_file_name)
        .file_stem()
        .expect("Failed to get library file stem")
        .to_string_lossy();

    let mut lib_rs = OpenOptions::new()
        .append(true)
        .open("src/lib.rs")
        .expect("Failed to open 'src/lib.rs'");

    writeln!(lib_rs, "\npub mod {};", module_name).expect("Failed to write to 'src/lib.rs'");
    writeln!(lib_rs, "pub use {}::*;", module_name).expect("Failed to write to 'src/lib.rs'");
}
