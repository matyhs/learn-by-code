//! Simulating files one step at a time.
use std::fmt;
use std::fmt::{Display};

#[derive(Debug,PartialEq)]
enum State {
    Open,
    Close
}

/// Represents a "file",
/// which probably lives on a file system.
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: State
}

trait Read {
    fn read(
        self: &Self,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String>;
}

impl Read for File {
    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != State::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        Ok(read_length)
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Open => write!(f, "OPEN"),
            State::Close => write!(f, "CLOSE")
        }
    }
}

impl File {
    /// Creates a new 'file'
    /// 
    /// Example:
    /// ```
    /// let file = File::new("f1.txt");
    /// ```
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: State::Close
        }
    }

    fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file.state = State::Close;

        file
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = State::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = State::Close;
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new_with_data("4.txt", &f4_data);

    let mut buffer: Vec<u8> = vec![];

    if f4.read(&mut buffer).is_err() {
        println!("Error checking is working!");
    }

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{}", f4);
    println!("{} is {} bytes long", f4.name, f4_length);
    println!("{}", text);
}
