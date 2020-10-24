use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Result;

use regex::Regex;

const TARGET_DIR:&str = "/home/erik/Downloads";

fn get_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn make_directories(extensions:&Vec<String>) -> Result<()> {
    for extension in extensions {

        let mut path:String = TARGET_DIR.to_owned();
        path.push_str("/");
        path.push_str(&extension);

        if extension == "" || Path::new(&path).exists()  {
            continue
        };

        println!("making extension: {}", extension);
        fs::create_dir(path)?;
    }
    Ok(())
}

fn move_files(paths:&Vec<String>) -> Result<()> {
    for path in paths {
        let re = Regex::new(r"[^\\/]*\..*").unwrap();
        if !re.is_match(path) {
            continue;
        }
        let extension = match get_extension(&path) {
            Some(e) => e,
            None => continue,
        };
        let name = re.find(path).unwrap().as_str();
        let location = re.replace(path, "");

        let mut new_name = location.to_string();

        new_name.push_str(extension);
        new_name.push_str("/");
        new_name.push_str(name);

        println!("name: {}", name);
        println!("path: {}", location);
        println!("extension: {}", extension);

        match fs::rename(path, new_name) {
            Ok(_v) => println!("[success] Renamed File"),
            Err(e) => println!("[Fail] {}", e),
        }
    };
    Ok(())
}

fn main() {
    let path_buffers = fs::read_dir(&TARGET_DIR).unwrap();

    let mut extensions: Vec<String> = vec![];
    let mut paths: Vec<String> = vec![];

    for buffer in path_buffers {
        let filename = buffer.unwrap().path().into_os_string().into_string().unwrap(); 

        let extension_option = get_extension(&filename);
        let extension = match extension_option {
            Some(x) => x,
            None => ""
        };

        paths.push(filename.to_string());
        extensions.push(extension.to_string());
    };

    extensions.sort();
    extensions.dedup();

    match make_directories(&extensions) {
        Ok(_v) => println!("[success] Made Directories"),
        Err(e) => println!("[Fail] {}", e)
    };

    match move_files(&paths) {
        Ok(_v) => println!("[success] Moved Files"),
        Err(e) => println!("[Fail] {}", e)
    };

    println!("[Finish]");
}
