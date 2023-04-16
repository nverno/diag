use std::{
    fs::{self},
    os::unix::prelude::MetadataExt,
    path::PathBuf,
};

pub fn dir_size(dir: &PathBuf) -> std::io::Result<u64> {
    let meta = fs::metadata(&dir)?;
    let mut res = meta.size();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                res += dir_size(&path).unwrap_or(0);
            } else {
                let meta = fs::metadata(&path)?;
                res += meta.size();
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
        println!("{:<15}  {}", sz, path.strip_prefix(prefix).unwrap());
    }

    Ok(())
}
