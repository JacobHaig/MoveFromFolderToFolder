use std::fs;
use std::fs::File;
use std::io::prelude::*;

use clap;

fn main() {
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

    // This should be changed during a release build
    // as it is not acceptible for offical releases.
    // Uncomment top for debugging
    let mut folder_in = matches.value_of("INPUT FOLDER").unwrap().to_string();
    //.unwrap_or(r"C:\Users\jacob\Desktop\Move From");
    let mut folder_out = matches.value_of("OUTPUT FOLDER").unwrap().to_string();
    //.unwrap_or(r"C:\Users\jacob\Desktop\Move To");

    // Fix cases where folder doesnt end with a "\"
    if !folder_in.ends_with(r"\") {
        folder_in = [folder_in, r"\".to_string()].join("");
    }
    if !folder_out.ends_with(r"\") {
        folder_out = [folder_out, r"\".to_string()].join("");
    }

    println!("Folder In {}\nFolder Out {}\n", folder_in, folder_out);

    loop {
        let list_files = find_files(&folder_in);

        for file in list_files {
            let path = file.unwrap().path();

            if !path.is_dir() {
                let file_name = path.file_name().unwrap().to_str().unwrap();

                println!("File {}", file_name);
                move_file(file_name, &folder_in, &folder_out);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
    }
}

fn find_files(folder_in: &str) -> fs::ReadDir {
    let files = fs::read_dir(folder_in).expect("Error in find_files()");

    files
}

fn move_file(file_name: &str, folder_in: &str, folder_out: &str) {
    let contents = read_file(folder_in, file_name);
    write_file(contents, folder_out, file_name);
    delete_file(folder_in, file_name)
}

fn read_file(folder_in: &str, file_name: &str) -> Vec<u8> {
    let file_result = File::open([folder_in, file_name].join(""));

    let mut file = file_result.expect("File dont exist or something");

    let mut contents = Vec::new();
    file.read_to_end(&mut contents)
        .expect("Cant retreive contents from file");

    contents
}

fn write_file(contents: Vec<u8>, folder_out: &str, file_name: &str) {
    let file_result = File::create([folder_out, file_name].join(""));

    let mut file = file_result.expect("Can not create the file sucker");

    file.write_all(&contents).expect("Cant write or something");
}

fn delete_file(folder_in: &str, file_name: &str) {
    fs::remove_file([folder_in, file_name].join("")).expect("Cant remove the files!");
}

#[test]
fn read_file_test() {
    let folder_in = r"tests\";
    let file_name = r"testfile.txt";
    let contents = read_file(folder_in, file_name);

    assert_eq!(std::str::from_utf8(&contents).unwrap(), "You dumb");
}
