use walkdir::WalkDir;
use std::path::Path;
use std::collections::HashMap;
// use std::env;
use std::fs;
use std::io;
use serde_json;


fn scan_dir_filename(path: &Path) -> Result <Option <Vec<String>>, io::Error> {
  let mut vec: Vec <String> = Vec::new();
  let entries = fs::read_dir(path)?;
  for entry in entries {
    vec.push( match entry {
      Ok(f) => f.path()
        .file_name()
        .map_or_else(String::new, 
          |s| s.to_str()
          .unwrap_or("")
          .to_string()),

      Err(e) => { eprintln!("error: {e:#?}"); String::new() },
    });
  }

  Ok(Some(vec))
}

pub fn scan_dir(path: &Path) -> Result <Option <Vec<String>>, io::Error> {
  let mut vec: Vec <String> = Vec::new();
  let entries = fs::read_dir(path)?;
  for entry in entries {
    vec.push(match entry {
      Ok(f) => f.path()
        .to_str()
        .map_or_else(String::new, String::from),

      Err(e) => { eprintln!("error: {e:#?}"); String::new() },
    });
  }

  Ok(Some(vec))
}

pub fn craw_dir(path: &Path) -> Result< Option <HashMap <String, Vec<String>>>, walkdir::Error> {
  let mut map: HashMap <String, Vec<String>> = HashMap::new();

  if path.exists() {
    for entry in WalkDir::new(path).into_iter().flatten() {
      if entry.path().is_dir() {
        let path: &Path = entry.path();
        let path_str: &str = path
          .to_str()
          .unwrap_or("");

        if !map.contains_key(path_str) {
          map.insert(path_str.to_string(), match scan_dir(Path::new(path_str)) {
            Ok(s) => s.unwrap_or_else(Vec::new),
            Err(e) => { eprintln!("error: {e:#?}"); Vec::new()}, 
          });
          
        } else {
          continue;
        }
      }
    }
  }

  Ok(Some(map))
}

pub fn write_file(file_path: &Path, map: &HashMap<String, Vec<String>>) -> Result<(), serde_json::Error> {

  let content = serde_json::to_string_pretty(&map).unwrap();
  fs::write(file_path, content.as_bytes())
    .expect("Error while trying to write content to output file.");

  Ok(())
}


// return an Vedc<u8> with the content of the file
pub fn read_file(path: &Path) -> Result<Option<Vec<u8>>, io::Error> {
  
  if !path.exists() {
    eprintln!("read_file() > Error: src {} does not exist.", path.display());
    return Err(io::Error::from(io::ErrorKind::NotFound));
  }

  let content: Vec<u8> = match fs::read(path) {
    Ok(s) => s,
    Err(e) => panic!("read_file() > could not read the file to an string: {:#?}", e ),
  };

  if content.len() > 1 {
    Ok(Some(content))
  } else {
    Ok(None)
  }
}

pub fn create_file(dest: &Path, content: &Vec<u8>) -> Result<(), io::Error> {

  if dest.exists() {
    eprintln!("create_file > Error: file {} already exists", dest.display());
    return Err(io::Error::from(io::ErrorKind::AlreadyExists));
  }

  let mut file: fs::File = match fs::File::create(dest) {
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

// i think it is pointless, but i will leave it here. just in case
// pub fn remove_file(src: &Path) -> Result<(), Error> {
//   if !src.exists() {
//     return Err(Error::from(ErrorKind::NotFound));
//   }
// 
//   match remove_file(&src) {
//     Ok(_) => Ok(()),
//     Err(e) = {
//       eprintln("an error occurred while trying to remove file: {}.\nError: {}", src.display(), e);
//       return Err(e);
//     },
//   }
// }


pub fn move_file(src: &Path, dest: &Path) -> Result<(), io::Error> {
  
  if !src.exists() {
    eprintln!("move_file() > Error: src {} does not exist.", src.display());
    return Err(io::Error::from(io::ErrorKind::NotFound));
  } 
  else if dest.exists() {
    eprintln!("move_file() > {} already exists.", dest.display());
    return Err(io::Error::from(io::ErrorKind::AlreadyExists));
  } 

  let src_content = match read_file(src)? {
    Some(s) => s,
    None => { 
      eprintln!("move_file > read_file returned None when trying to read the content of file {} ", src.display()); 
      return Err(io::Error::from(io::ErrorKind::WriteZero));
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
