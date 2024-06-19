use std::fs::read_dir;
use std::path::{Path, PathBuf};

pub fn search_directory(path_origin: &Path, key_dir: &str) -> Result<Option<PathBuf>, std::io::Error> {
  let mut stack: Vec<PathBuf> = Vec::new();

  stack.push(path_origin.to_path_buf());
  while let Some(current_path) = stack.pop() {
    
    for entry in read_dir(current_path)? {
      let entry: std::fs::DirEntry = entry?;
      let path: PathBuf = entry.path();
      
      if path.is_dir() { 
        if path.file_name().and_then(|name| name.to_str()) == Some(key_dir) {
          return Ok(Some(path));
        } else {
          stack.push(path);
        }
      }
    }
  }

  return Ok(None);
}
