use core::panic;
use std::fs::{ read_dir, remove_file, read, File };
use std::path::{ Path, PathBuf };
use std::io::{ Error, ErrorKind, Write };


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


// return an string with the content of the file
pub fn read_file(path: &Path) -> Result<Option<Vec<u8>>, Error> {
  
  if !path.exists() {
    eprintln!("read_file() > Error: src {} does not exist.", path.display());
    return Err(Error::from(ErrorKind::NotFound));
  }

  let content: Vec<u8> = match read(path) {
    Ok(s) => s,
    Err(e) => panic!("read_file() > could not read the file to an string: {:#?}", e ),
  };

  if content.len() > 1 {
    Ok(Some(content))
  } else {
    Ok(None)
  }
}

pub fn create_file(dest: &Path, content: &Vec<u8>) -> Result<(), Error> {

  if dest.exists() {
    eprintln!("create_file > Error: file {} already exists", dest.display());
    return Err(Error::from(ErrorKind::AlreadyExists));
  }

  let mut file: File = match File::create(dest) {
    Ok(f) => f,
    Err(e) => {
      eprintln!("create_file > Error: was not possible to create file {}, error {}", dest.display(), e);
      return Err(e);
    },
  };

  match file.write_all(content) {
    Ok(()) => Ok(()),
    Err(e) => {
      eprintln!("create_file > Error: was not possible to create file {}, error {}", dest.display(), e);
      Err(e)
    },
  }
}


pub fn move_file(src: &Path, dest: &Path) -> Result<(), Error> {
  
  if !src.exists() {
    eprintln!("move_file() > Error: src {} does not exist.", src.display());
    return Err(Error::from(std::io::ErrorKind::NotFound));
  } 
  else if dest.exists() {
    eprintln!("move_file() > {} already exists.", dest.display());
    return Err(Error::from(ErrorKind::AlreadyExists));
  } 

  let src_content = match read_file(src)? {
    Some(s) => s,
    None => { 
      eprintln!("move_file > read_file returned None when trying to read the content of file {} ", src.display()); 
      return Err(Error::from(ErrorKind::WriteZero));
    }
  };

  match create_file(dest, &src_content) {
    Ok(()) => {
      match remove_file(src) {
        Ok(()) => Ok(()),

        Err(e) => {
          eprintln!("move_file > an error occurred while trying to remove {}. Error: {}", src.display(), e);
          Err(e)
        }
      }
    },

    Err(e) => Err(e),
  }
}