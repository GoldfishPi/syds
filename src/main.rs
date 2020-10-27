
extern crate notify;
extern crate regex;
extern crate structopt;

use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::io::Result;
use std::sync::mpsc::channel;
use std::time::Duration;

use regex::Regex;
use structopt::StructOpt;

use notify::{Watcher, RecursiveMode, watcher};


#[derive(StructOpt)]
#[structopt(name = "syds")]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    #[structopt(short, long)]
    daemon: bool,

    #[structopt(short, long, default_value = "10")]
    update_time:u64
}

fn get_extension(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn make_directories(extensions:&Vec<String>, current_path:&str) -> Result<()> {
    for extension in extensions {

        let mut path:String = current_path.to_owned();
        path.push_str("/");
        path.push_str(&extension);

        fs::create_dir(path)?;
    }
    return Ok(());
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

        fs::rename(path, new_name)?;
    };
    return Ok(());
}

fn org_files(current_dir:&std::path::PathBuf) -> Result<()> {
    let path_buffers = fs::read_dir(&current_dir).unwrap();

    let mut extensions: Vec<String> = vec![];
    let mut paths: Vec<String> = vec![];
    

    for buffer in path_buffers {
        let filename = buffer.unwrap().path().into_os_string().into_string().unwrap(); 

        let extension_option = get_extension(&filename);
        let extension = match extension_option {
            Some(x) => x,
            None => continue
        };

        paths.push(filename.to_string());
        extensions.push(extension.to_string());
    };

    extensions.sort();
    extensions.dedup();

    make_directories(&extensions, &current_dir.display().to_string())?;
    move_files(&paths)?; 

    return Ok(());
}

fn main() -> Result<()> {
    let args = Cli::from_args();

    if args.daemon {
        let (tx, rx) = channel();

        let mut w = watcher(tx, Duration::from_secs(args.update_time)).unwrap();
        w.watch(&args.path, RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(_event) => org_files(&args.path)?,
                Err(err) => panic!("watch err: {}", err)
            }
        }
    } 

    org_files(&args.path)
}
