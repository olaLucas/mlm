use core::panic;
use std::path::Path;

use file_handling::*;
pub mod file_handling;


fn main() {

	let file_path: &str = "/home/dio/git_repos/psicohits/test_dir/new_file.rs";
	let _file = match create_file(&Path::new(file_path), &Vec::from("Ola, eu sou um arquivo.".as_bytes())) {
		Ok(()) => println!("file {file_path}  created."),
		Err(e) => panic!("{:#?}", e),
	};

	let content = match read_file(&Path::new(file_path)) {
		Ok(opt) => match opt {
			Some(c) => match String::from_utf8(c) {
        Ok(s) => s,
        Err(e) => { 
          eprintln!("{:#?}", e);
          panic!("an error occurred while trying to convert file content content to string");
        },
      },

			None => String::new(),
		},

		Err(e) => panic!("{:#?}", e),
	};

  println!("{content}");
}
