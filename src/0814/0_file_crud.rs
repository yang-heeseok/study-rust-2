use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn main() {
    let res: i32 =square(5);
    print!("{res}");

    let _file_path = "my_new_file.txt";
    let _initial_content = "Hello, world!\nThis is a new file.";

    // if let Err(error) = create_file_with_content(file_path, initial_content) {
    //     eprintln!("Error creating file: {}", error);
    // } else {
    //     println!("File created successfully with initial content.");
    // }

    // Read
    // let content = read_file(file_path);
    // println!("File content: {}", content);

    // Write
    // write_to_file(file_path, "This is new content.");

    // Append
    // append_to_file(file_path, "\n and this is appended content.");

    // Delete
    // delete_file(file_path);

}

fn square(num: i32) -> i32 {
    // return num * num
    num * num
}

fn _create_file_with_content(file_path: &str, content: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn _read_file(file_path: &str) -> String {
    let mut file = File::open(file_path).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    content
}

fn _write_to_file(file_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to open file for writing");
    file.write_all(content.as_bytes()).expect("Failed to write to file");
}

fn _append_to_file(file_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(file_path)
        .expect("Failed to open file for appending");
    file.write_all(content.as_bytes()).expect("Failed to append to file");
}

// 파일 삭제 함수 (주의: 신중하게 사용해야 합니다.)
fn _delete_file(file_path: &str) {
    std::fs::remove_file(file_path).expect("Failed to delete file");
}
