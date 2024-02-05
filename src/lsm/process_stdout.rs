use std::io::{Read, Write};
// use std::path::Path;
// use std::process::Command;
// use std::process::Stdio;

pub fn process_stdout(child: &mut std::process::Child) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = child.stdout.take().unwrap();
    let mut buf = [0u8; 100];
    let mut do_read = || -> Result<usize, Box<dyn std::error::Error>> {
        let read = stdout.read(&mut buf)?;

        print!("{}", std::str::from_utf8(&buf[0..read])?);
        std::io::stdout().flush()?;
        Ok(read)
    };
    let mut last;
    while child.try_wait()?.is_none() {
        do_read()?;
        // last = do_read()?;
        // println!("\nalive: {}", last);
    }

    println!("try wait: {}", child.try_wait()?.unwrap());
    last = 1;

    while last > 0 {
        last = do_read()?;
        // println!("\ndead: {}", last);
    }

    Ok(())
}
