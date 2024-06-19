use std::path::Path;

pub mod file_handling;

fn main() {
	let path = Path::new("/home/dio/git_repos/just-coding/");
	match file_handling::search_directory(path, "messing-with-apis") {
		Ok(opt) =>  {
			match opt {
				Some(s) => println!("found in: {}", s.to_str().unwrap()),
				None => println!("not found in: {}", path.to_str().unwrap()),
			}
		},

		Err(e) => eprintln!("an error occurred: {:#?}", e),
	};
}
