use colored::Colorize;
use std::{
    fs::{self},
    path::PathBuf,
};

pub fn dir_size(dir: &PathBuf) -> std::io::Result<u64> {
    let meta = fs::symlink_metadata(dir)?;
    let ftype = meta.file_type();
    let mut res = 0;

    if !ftype.is_symlink() && dir.is_dir() {
        res = meta.len();
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                res += dir_size(&path).unwrap_or(0);
            } else {
                let meta = fs::metadata(&path)?;
                res += meta.len();
            }
        }
    }

    Ok(res)
}

pub fn du(dir: &PathBuf, reverse: bool) -> std::io::Result<()> {
    let mut dirs = vec![];

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let sz = dir_size(&path).unwrap_or(0);
                dirs.push((sz, String::from(path.to_str().unwrap_or(""))));
            }
        }
    }

    let prefix = dir
        .to_str()
        .expect("couldnt convert root directory to string");

    dirs.sort_unstable_by_key(|k| k.0);
    if reverse {
        dirs.reverse();
    }
    for (sz, path) in dirs {
        let ss = if sz > 1024 * 1024 * 1024 {
            ((sz / (1024 * 1024 * 1024)).to_string() + "G").red()
        } else if sz > 1024 * 1024 {
            ((sz / (1024 * 1024)).to_string() + "M").blue()
        } else if sz > 1024 {
            ((sz / 1024).to_string() + "K").yellow()
        } else {
            sz.to_string().normal()
        };
        println!("{:<15}  {}", ss, path.strip_prefix(prefix).unwrap());
    }

    Ok(())
}
