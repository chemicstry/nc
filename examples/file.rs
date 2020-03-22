use std::io::{self, BufRead, BufReader, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct File {
    fd: i32,
    path: PathBuf,
}

impl File {
    pub fn new(path: &Path) -> File {
        File {
            fd: -1,
            path: path.to_path_buf().clone(),
        }
    }

    pub fn open(&mut self) -> Result<(), nc::Errno> {
        match nc::open(self.path.to_str().unwrap(), nc::O_RDONLY, 0) {
            Ok(fd) => {
                self.fd = fd;
                Ok(())
            }
            Err(errno) => Err(errno),
        }
    }
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match nc::read(self.fd, buf) {
            Ok(n_read) => Ok(n_read as usize),
            Err(_errno) => Err(io::Error::new(io::ErrorKind::Other, "errno")),
        }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        if self.fd > -1 {
            println!("closing fd: {}", self.fd);
            let _ = nc::close(self.fd);
            self.fd = -1;
        }
    }
}

fn test_fd() {
    let path = Path::new("/etc/issue");
    let mut fd = File::new(&path);
    let _ = fd.open();
    println!("fd: {:?}", fd);
    let buf = BufReader::new(fd);
    let lines = buf.lines().collect::<io::Result<Vec<String>>>().unwrap();
    print!("lines: {:?}", lines);
}

fn main() {
    test_fd();
}