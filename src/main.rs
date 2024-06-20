use core::panic;
use std::path::Path;

use file_handling::*;
pub mod file_handling;


fn main() {
	let path = Path::new("/home/dio/git_repos/just-coding/");
	let path = match search_directory(path, "messing-with-apis") {
		Ok(opt) => {
			match opt {
				Some(s) => s,
				None => panic!("directory not found."),
			}
		},

		Err(e) => panic!("An error occurred during execution: {:#?}", e),
	};

	let path = match path.to_str() {
		Some(s) => s,
		None => panic!("none returned when trying to convert PathBuf to str."),
	};


	let content = match open_directory(&Path::new(path)) {
		Ok(opt) => match opt {
			Some(content_vec) => content_vec,
			None => panic!("Empty folder."),
		},
			
		Err(e) => panic!("An error occurred during execution: {:#?}", e), 
	};

	println!("found in {path}");
	for file in content {
		println!("{}", file.file_name().unwrap().to_str().unwrap());
	}
}
