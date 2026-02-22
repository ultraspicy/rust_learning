use std::io::Error;
use std::error::Error as stdErr;

struct Trie {
    val: i32,
    child: Option<Box<Trie>>,
}

fn main() -> Result<(), Box<dyn stdErr>> {
    let a: Option<i32> = None;
    let val = a.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "was none"))?;
    Ok(())
}

