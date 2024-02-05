use std::path::Path;

#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink as symlink_file;

#[cfg(target_family = "windows")]
use std::os::windows::fs::symlink_file;

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> std::io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();
    println!("src: {:?}", src);
    println!("dst: {:?}", dst);
    if src.is_dir() {
        symlink_file(src, dst)?
    } else {
        symlink_file(src, dst)?
    }
    Ok(())
}
