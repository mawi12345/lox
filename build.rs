use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

// build script's entry point
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let destination = Path::new(&out_dir).join("tests.rs");
    let mut test_file = File::create(&destination).unwrap();

    // write import
    write!(
        test_file,
        r#"
use crate::Parser;
"#
    )
    .unwrap();

    // write tests ./test/**/*.lox
    let test_categories = read_dir("./test/").expect("test directory to exist and be readable");
    for entry in test_categories {
        let category_directory = entry
            .expect("test category directory to exist and be readable")
            .path();
        if category_directory.is_dir() {
            let demos = category_directory
                .read_dir()
                .expect("test category directory to exist and be readable");
            for file in demos {
                let demo_path = file
                    .expect("demo lox file to to exist and be readable")
                    .path();
                if demo_path.is_file()
                    && matches!(demo_path.extension(), Option::Some(ext) if ext == "lox")
                {
                    write_test(&mut test_file, &demo_path);
                }
            }
        }
    }
}

fn write_test(test_file: &mut File, demo_path: &PathBuf) {
    let directory = demo_path.canonicalize().unwrap();
    let path = directory.display();
    let name = demo_path.file_stem().unwrap().to_str().unwrap();
    let dir = demo_path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let fun = if dir == "expressions" {
        "parse_expression"
    } else {
        "parse"
    };

    write!(
        test_file,
        r#"
#[test]
fn {dir}_{name}() {{
    let parser = Parser::new(include_str!("{path}"));
    parser.{fun}().unwrap();
}}
"#,
    )
    .unwrap();
}
