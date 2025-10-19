use std::path::PathBuf;
use std::fs;
use anyhow::Result;

// File operations
pub fn copy_file(from: &PathBuf, to: &PathBuf) -> Result<()> {
    if from.is_dir() {
        copy_dir_all(from, to)?;
    } else {
        fs::copy(from, to)?;
    }
    Ok(())
}

pub fn move_file(from: &PathBuf, to: &PathBuf) -> Result<()> {
    fs::rename(from, to)?;
    Ok(())
}

pub fn delete_file(path: &PathBuf) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn rename_file(path: &PathBuf, new_name: &str) -> Result<()> {
    let parent = path.parent().unwrap();
    let new_path = parent.join(new_name);
    fs::rename(path, &new_path)?;
    Ok(())
}

fn copy_dir_all(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}
