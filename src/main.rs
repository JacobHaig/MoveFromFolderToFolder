use std::fs;
use std::io::prelude::*;

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
        .get_matches();

    // This should be changed to unwrap() during a release build
    // as it is not acceptible for offical releases.
    // Uncomment top for debugging
    let folder_in = matches.value_of("INPUT FOLDER").unwrap();
    //.unwrap_or(r"C:\Users\jacob\Desktop\Move From");
    let folder_out = matches.value_of("OUTPUT FOLDER").unwrap();
    //.unwrap_or(r"C:\Users\jacob\Desktop\Move To");

    println!("Folder In {}\nFolder Out {}\n", folder_in, folder_out);

    loop {
        for file in find_files(&folder_in) {
            let path = file.unwrap().path();

            if !path.is_dir() {
                // Return just the name of the file.
                let file_name = path.file_name().unwrap().to_str().unwrap();

                println!("File {}", file_name);
                move_file(file_name, &folder_in, &folder_out);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    }
}

/// Gets a list of all items in a folder.
/// Takes a Folder as an Argument.
/// Throws Error if path to file is wrong.
/// Returns an iterator of type ReadDir
fn find_files(folder_in: &str) -> fs::ReadDir {
    fs::read_dir(folder_in).expect("Folder does not Exist. ")
}

/// Simple function to move the file from one folder to another
fn move_file(file_name: &str, folder_in: &str, folder_out: &str) {
    let contents = read_file(folder_in, file_name);
    write_file(contents, folder_out, file_name);
    delete_file(folder_in, file_name);
}

/// Read a file in to memory as a Vector of Bytes.
/// Takes a Folder and a File name as Arguments.
/// Throws Error if path to file is wrong or if application
/// doesnt have Permission to access the file.
fn read_file(folder_in: &str, file_name: &str) -> Vec<u8> {
    let file_result = fs::File::open([folder_in, file_name].join(r"\"));

    let mut file = file_result.expect("File does not exist or is inaccessable");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Cant retreive contents from file. ");

    contents
}

/// Creates and write contents to a file.
/// Takes File name, Folder, and Vector of Bytes as Arguments.
/// Throws Error if file can not be created.
fn write_file(contents: Vec<u8>, folder_out: &str, file_name: &str) {
    let file_result = fs::File::create([folder_out, file_name].join(r"\"));

    let mut file = file_result.expect("Can not create the file. ");

    file.write_all(&contents)
        .expect("Can not write or something. ");
}

/// Removes a file from the filesystem.
/// Takes a Folder and a File name as Arguments.
/// Throws Error if path to file is wrong or if application
/// doesnt have Permission to access the file.
fn delete_file(folder_in: &str, file_name: &str) {
    fs::remove_file([folder_in, file_name].join(r"\")).expect("Can not remove the files. ");
}

#[test]
fn read_file_test() {
    let folder_in = r"tests";
    let file_name = r"testfile.txt";
    let contents = read_file(folder_in, file_name);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}
