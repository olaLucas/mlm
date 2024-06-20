use std::fs::read_dir;
use std::path::{ Path, PathBuf };
use std::io::Error;

// Craw through an directory to find another directory
pub fn search_directory(path_origin: &Path, key_dir: &str) -> Result<Option<PathBuf>, Error> {
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

// Return the files inside an directory
pub fn open_directory(path_to: &Path) -> Result<Option<Vec<PathBuf>>, Error> {  
  let mut dir_content: Vec<PathBuf> = Vec::new();
  for entry in read_dir(path_to)? {
    let entry = entry?;
    dir_content.push(entry.path());
  }

  if dir_content.len() > 0 {
    return Ok(Some(dir_content));
  } else {
    return Ok(None);
  }
} 