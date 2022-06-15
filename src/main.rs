// use::rust-gol::Board{..};

use std::{
    io::{self, Write},
    thread, time,
};

pub(crate) fn main() -> io::Result<()> {
    loop {
        println!("noteon 64 20 1");
        io::stdout().flush()?;
        thread::sleep(time::Duration::from_millis(1000));
    }
}
