use error_remark::{Remark, ResultExt};
use std::io;
use std::path::Path;

// This program will crash and print something like:
//      thread 'main' panicked at 'oh no the program blew up: failed to read secret file ("krusty_
//      krab_secret_formula.txt", 4096): Unknown error 666 (os error 666)', examples/demo.rs:24:17
// without using Remark, the output would instead be:
//      thread 'main' panicked at 'oh no the program blew up: Os { code: 666, kind: Other, message:
//      "Unknown error 666" }', examples/demo.rs:24:17
// which doesn't tell you what was happening at all really.

fn main() {
    let result = do_the_thing(&Path::new("krusty_krab_secret_formula.txt"));
    if let Err(ref e) = result {
        // 'e' is a Remark<io::Error>, but Remark is just a wrapper type; you can still inspect
        // fields on the inner error just like normal. In this case, let's do some runtime logic
        // that depends on the I/O error kind:
        match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("user error, file not found");
            }
            _ => {
                panic!("oh no the program blew up: {e}");
            }
        }
    }
}

fn do_the_thing(path: &Path) -> Result<Vec<u8>, Remark<io::Error>> {
    let mut buf = [0u8; 4096];

    // let's do something that might fail
    read_file(path, &mut buf)
        // Add a remark on what's going on if an error happened.
        // This gives users something more meaningful than the generic OS-provided error message.
        // err_remark_vars lets you also include the Debug representation of some variables for
        // context. Extra variables can be of any type (even mixed types) as long as they implement
        // Debug.
        .err_remark_vars("failed to read secret file", &[&path, &buf.len()])?;

    Ok(buf.to_vec())
}

fn read_file(_path: &Path, _buffer: &mut [u8]) -> io::Result<usize> {
    // make up a fake error for demonstration purposes
    Err(io::Error::from_raw_os_error(666))
}
