use std::env;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() {
    let mut total_bytes = 0;
    let silent = env::var("PV_SILENT").unwrap_or_default().is_empty();
    loop {
        let mut buffer = vec![0; CHUNK_SIZE];
        let num_read = match io::stdin().read(&mut buffer) {
            Ok(0) => break,
            Ok(num_read) => num_read,
            Err(error) => {
                eprintln!("Error: {}", error);
                break;
            }
        };

        total_bytes += num_read;
        io::stdout().write_all(&buffer[..num_read]).unwrap();
    }
    eprintln!("Read {} bytes", total_bytes);
}
