use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let relevant_args = if args.len() > 1 && args[1] == "problem" {
        &args[2..]
    } else {
        &args[1..]
    };

    if relevant_args.len() != 2 {
        println!("Usage: cargo problem <category> <problem_name>");
        std::process::exit(1);
    }

    let category = &relevant_args[0];
    let problem_name = &relevant_args[1];

    let src_path = PathBuf::from("src");
    let problems_path = src_path.join("problems");
    let category_path = problems_path.join(category);
    let problem_file_path = category_path.join(format!("{}.rs", problem_name));
    let mod_file_path = category_path.join("mod.rs");

    if problem_file_path.exists() {
        println!("Problem file already exists: {:?}", problem_file_path);
        std::process::exit(1);
    }

    fs::create_dir_all(&category_path).expect("Failed to create category directory");

    let mut problem_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&problem_file_path)
        .expect("Failed to create problem file");

    let template = format!(
        "
// Problem
pub fn {}() {{
    // TODO: Implement solution
}}
        ",
        problem_name
    );

    problem_file
        .write_all(template.as_bytes())
        .expect("Failed to write template");

    let mod_declaration = format!("pub mod {};", problem_name);

    if !mod_file_path.exists() {
        let mut mod_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&mod_file_path)
            .expect("Failed to create mod.rs");

        writeln!(mod_file, "{}", mod_declaration).expect("Failed to write to mod.rs");
    } else {
        let mut content = fs::read_to_string(&mod_file_path).expect("Failed to read mod.rs");
        if !content.contains(&mod_declaration) {
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&mod_declaration);
            content.push('\n');
            fs::write(&mod_file_path, content).expect("Failed to update mod.rs");
        }
    }

    println!("Created new problem file: {:?}", problem_file_path);
    println!("Updated mod.rs: {:?}", mod_file_path);
}
