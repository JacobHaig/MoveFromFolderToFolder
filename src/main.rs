use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

fn main() {
    // Use clap as a CLI manager of sorts.
    // Clap handles all flags to the program.
    let matches = clap::App::new("FileMoveProgram")
        .version("0.2")
        .about("Moves files from one folder to another")
        .arg(
            clap::Arg::with_name("INPUT FOLDER")
                .short("i")
                .long("inputfolder")
                .help("Sets input folder")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("OUTPUT FOLDER")
                .short("o")
                .long("outputfolder")
                .help("Sets output folder")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("DELETE")
                .short("d")
                .long("delete")
                .help("Sets whether to delete files and folder"),
        )
        .get_matches();

    let folder_in = Path::new(matches.value_of("INPUT FOLDER").unwrap());
    let folder_out = Path::new(matches.value_of("OUTPUT FOLDER").unwrap());
    let is_delete = matches.is_present("DELETE");

    loop {
        start(folder_in, folder_out, is_delete);
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    }
}

fn start(folder_in: &Path, folder_out: &Path, is_delete: bool) {
    for path in find_files(folder_in) {
        if !path.is_dir() {
            // Return just the name of the file.
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let contents = read_file(&path.to_path_buf());
            write_file(contents, &pathize!(folder_out, file_name));

            if is_delete {
                delete_file(&path);
            }
        }
    }
}

/// Gets a list of all items in a folder.
/// Takes a Folder as an Argument.
/// Throws Error if path to file is wrong.
/// Returns an iterator of type ReadDir
fn find_files(folder: &Path) -> Vec<PathBuf> {
    fs::read_dir(folder)
        .expect("Folder does not Exist. ")
        .map(|a| a.unwrap().path())
        .collect()
}

/// Read a file in to memory as a Vector of Bytes.
/// Takes a Folder and a File name as Arguments.
/// Throws Error if path to file is wrong or if application
/// doesnt have Permission to access the file.
fn read_file(path: &Path) -> Vec<u8> {
    let mut file = fs::File::open(path).expect("File does not exist or is inaccessable");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Cant retreive contents from file. ");

    contents
}

/// Creates and write contents to a file.
/// Takes File name, Folder, and Vector of Bytes as Arguments.
/// Throws Error if file can not be created.
fn write_file(contents: Vec<u8>, path: &Path) {
    let mut file = fs::File::create(path).expect("Can not create the file. ");

    file.write_all(&contents)
        .expect("Can not write or something. ");
}

/// Removes a file from the filesystem.
/// Takes a Folder and a File name as Arguments.
/// Throws Error if path to file is wrong or if application
/// doesnt have Permission to access the file.
fn delete_file(path: &Path) {
    fs::remove_file(path).expect("Can not remove the files. ");
}

#[macro_export]
macro_rules! pathize {
    ($($args:expr),*) => {{
        std::path::Path::new(".")
        $( .join($args) )*
    }}
}

#[test]
fn move_files_test() {
    let cur_dir = std::env::current_dir().unwrap();
    let folder_in = pathize!(cur_dir.to_str().unwrap(), "tests", "from");
    let folder_out = pathize!(cur_dir.to_str().unwrap(), "tests", "to");
    let is_delete = false;

    start(&folder_in, &folder_out, is_delete);

    // Find the moved file, verify that it is there and correct.
    // Then remove the file.
    for path in find_files(&folder_out) {
        let content = read_file(&path);
        let s = std::str::from_utf8(&content).unwrap();

        if path.file_name().unwrap() == "Something.txt" {
            assert_eq!(s, "Some Text");
        }

        delete_file(&path);
    }
}

#[test]
fn read_rel_file_test() {
    let path = pathize!("tests", "testfile.txt");
    let contents = read_file(&path);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}

#[test]
fn read_abs_file_test() {
    let cur_dir = std::env::current_dir().unwrap();
    let path = pathize!(cur_dir, "tests", "testfile.txt");
    let contents = read_file(&path);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}

#[test]
fn pathize_macro_test() {
    let a: PathBuf = pathize!("tests", "testfile.txt");
    let p: PathBuf = std::path::Path::new(".").join("tests").join("testfile.txt");

    assert_eq!(a, p);
}

#[test]
fn move_file_test() {
    let path = pathize!("tests", "testfile.txt");
    let contents = read_file(&path);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}
