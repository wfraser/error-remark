use error_remark::ResultExt;
use std::io;
use std::path::Path;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    let path = &Path::new("xyz.file");
    do_something(&path)
        .err_remark_vars("top level", &[&path])?;

    Ok(())
}

/// This function has multiple error conditions, so it returns a boxed error trait object.
fn do_something(path: &Path) -> Result<i64, Error> {
    let mut buf = [0u8; 4096];

    let len = read_file(path, &mut buf)
        .err_remark("failed to read the file")?;

    let s = std::str::from_utf8(&buf[0..len])
        .err_remark_vars("file is not UTF-8", &[&&buf[0..len]])?;

    let n = s.parse().err_remark_vars("not a number", &[&s])?;

    Ok(n)
}

fn read_file(_path: &Path, buffer: &mut [u8]) -> io::Result<usize> {
    let data = b"12345 oops";
    let n = buffer.len().min(data.len());
    buffer[0..n].copy_from_slice(&data[0..n]);
    Ok(n)
}
