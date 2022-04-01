use std::io;
use std::thread;
use std::time::Duration;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "115200")]
    baud_rate: u32,

    /// Path to the serial port
    path: String, // should be PathBuf but `serialport` wants a str :-(
}

fn main() -> serialport::Result<()> {
    let args = Args::parse();

    let mut port_read = serialport::new(&args.path, args.baud_rate)
        .timeout(Duration::from_secs(10))
        .open()?;
    let mut port_write = port_read.try_clone()?;

    thread::spawn(move || loop {
        match io::copy(&mut port_read, &mut io::stdout().lock()) {
            Ok(_) => continue,
            Err(err) => match err.kind() {
                io::ErrorKind::TimedOut => continue,
                _ => panic!("I/O error reading from serial port: {}", err),
            }
        }
    });

    io::copy(&mut io::stdin().lock(), &mut port_write).unwrap();

    Ok(())
}
