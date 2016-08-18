extern crate user_directories;

fn main() {
    for directory_type in user_directories::DirectoryType::all().into_iter() {
        println!("{:?}", user_directories::find_directory(directory_type));
    }
}