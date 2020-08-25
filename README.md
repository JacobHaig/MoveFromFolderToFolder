# What it do?   
This is a Rust program for moving files from one folder to another automatically. Once set up, movefromto will continuously check the folder's contents to see if there are files in the folder. If there are files, they will be moved to the output folder that you specified.  

## Example  
```bash,norun
$ movefromto.exe --help
Moves files from one folder to another automatically

USAGE:
    movefromto.exe [FLAGS] --inputfolder <INPUT FOLDER> --outputfolder <OUTPUT FOLDER>

FLAGS:
    -d, --delete     Sets whether to delete files and folder
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --inputfolder <INPUT FOLDER>      Sets input folder
    -o, --outputfolder <OUTPUT FOLDER>    Sets output folder
```

