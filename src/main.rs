use std::path::PathBuf;

pub struct PathIter{
    buf: PathBuf,
    end: bool,
}

impl PathIter {
    pub fn new(path: PathBuf) -> Self{
        Self{
            buf: path,
            end: false,
        }
    }
    pub fn current() -> std::io::Result<Self> {
        let buf = std::env::current_dir()?;
        Ok(Self{
            buf,
            end: false,
        })
    }
}

impl Iterator for PathIter {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end { return None }
        let ret = self.buf.clone();
        self.end = !self.buf.pop();
        Some(ret)
    }
}

fn main() {
    for path in PathIter::current().unwrap() {
        println!("{:?}", path);
    }
    for path in PathIter::current().unwrap().collect::<Vec<PathBuf>>().iter().rev() {
        println!("{:?}", path);
    }
}
