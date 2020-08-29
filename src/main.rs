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
        .arg(
            clap::Arg::with_name("COUNT")
                .short("c")
                .long("count")
                .default_value("-1")
                .takes_value(true)
                .help("Sets the number of files to transfer"),
        )
        .arg(
            clap::Arg::with_name("LOOP")
                .short("l")
                .long("loop")
                .help("Sets whether to loop indefinalitly"),
        )
        .get_matches();

    // Collect the matched values into variables
    let folder_in = Path::new(matches.value_of("INPUT FOLDER").unwrap());
    let folder_out = Path::new(matches.value_of("OUTPUT FOLDER").unwrap());
    let is_delete = matches.is_present("DELETE");
    let is_loop = matches.is_present("LOOP");
    let count: i16 = matches
        .value_of("COUNT")
        .unwrap_or_default()
        .parse()
        .expect("Thats not an interger!");


    // Automatically check and move files when they are
    // found. Runs indef until program is ended
        
    //println!("{}",count);
    start(folder_in, folder_out, is_delete, count);
    while is_loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
        start(folder_in, folder_out, is_delete, count);
    }
}

/// This function move the files in to memory and then into the folder
/// Takes a Folder in, Folder out, and a delete bool as Arguments.
/// The reason this function is separate from the main function is
/// to provide a simple way of easily creating and running tests.
fn start(folder_in: &Path, folder_out: &Path, is_delete: bool, mut count: i16) {
    for path in find_files(folder_in) {
        if !path.is_dir() && (count > 0 || count == -1) {
            // Return just the name of the file.
            let file_name = path.file_name().unwrap().to_str().unwrap();

            let contents = read_file(&path.to_path_buf());
            write_file(contents, &pathize!(folder_out, file_name));

            // Optional delete flag
            if is_delete {
                delete_file(&path);
            }
            count -= 1;
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
/// doesn't have permission to access the file.
fn read_file(path: &Path) -> Vec<u8> {
    let mut file = fs::File::open(path).expect("File does not exist or is inaccessible");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Cant retrieve contents from file. ");

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
/// doesn't have Permission to access the file.
fn delete_file(path: &Path) {
    fs::remove_file(path).expect("Can not remove the files. ");
}

/// pathize is a simple macro that creates a PathBuf
/// given an arbitrary number of strings.
#[macro_export]
macro_rules! pathize {
    ($($args:expr),*) => {{
        std::path::Path::new(".")
        $( .join($args) )*
    }}
}

// This test is a own program test.
#[test]
fn move_file_test() {
    let cur_dir = std::env::current_dir().unwrap();
    let folder_in = pathize!(cur_dir.to_str().unwrap(), "tests", "from");
    let folder_out = pathize!(cur_dir.to_str().unwrap(), "tests", "to");
    let is_delete = false;
    let count = -1;

    start(&folder_in, &folder_out, is_delete, count);

    // Find the moved file, verify that it is there and correct.
    // Then remove the file.

    let mut move_correctly = false;
    for path in find_files(&folder_out) {
        let content = read_file(&path);
        let s = std::str::from_utf8(&content).unwrap();

        // Only check and delete the Something.txt file
        if path.file_name().unwrap() == "Something.txt" {
            if s == "Some Text" {
                move_correctly = true;
            }
            delete_file(&path);
        }
    }
    assert!(move_correctly);
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

/// A test to make sure the pathize! macro works correctly.
#[test]
fn pathize_macro_test() {
    let a: PathBuf = pathize!("tests", "testfile.txt");
    let p: PathBuf = std::path::Path::new(".").join("tests").join("testfile.txt");

    assert_eq!(a, p);
}

/// Simple test to make sure that the contents of the
/// file is being read correctly.
#[test]
fn read_file_test() {
    let path = pathize!("tests", "testfile.txt");
    let contents = read_file(&path);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}
