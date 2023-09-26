// Created by h4r on 2023-09-23

use std::fs::File;
use std::io::{self, Read, Write};
use std::env;

const PBM_PREAMBLE: &str = "P1\n# Created by file-2-bitmap\n";
const PBM_WIDTH: usize = 618;
const PBM_HEIGHT: usize = 1000;

fn file_to_binary_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut buffer = [0; 1024];
    let mut bits = String::new();

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        for byte in buffer.iter().take(bytes_read) {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 1;
                bits.push_str(&format!("{}", bit));
            }
        }
    }

    Ok(bits)
}

fn create_pbm(filename: &str, text: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let size = format!("{} {}\n", PBM_WIDTH, PBM_HEIGHT);
    let result = PBM_PREAMBLE.to_string() + &size + text;
    file.write_all(result.trim().as_bytes())?;
    Ok(())
}

fn split_string(s: &str, n: usize) -> (String, String) {
    let (left, right) = s.split_at(n);
    if right.len() < n {
        let n_zeros = n - right.len();
        let mut padded_right = String::new();
        padded_right.push_str(right);
        for _ in 0..n_zeros {
            padded_right.push('0');
        }
        return (left.to_string(), padded_right);
    }
    (left.to_string(), right.to_string())
}

fn convert_file(filename: &str, out_path: &str) -> io::Result<()> {
    let mut binary_string = file_to_binary_string(filename)?;
    let mut i = 0;
    let mut string_to_convert = String::new();

    while binary_string != "0".repeat(PBM_WIDTH * PBM_HEIGHT as usize) {
        let (string_to_convert, new_binary_string) = split_string(&binary_string, PBM_WIDTH * PBM_HEIGHT as usize);
        create_pbm(&format!("{}/out{}.pbm", out_path, i), &string_to_convert)?;
        i += 1;
        binary_string = new_binary_string;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: file2bin <file_to_convert> <output_path");
        return Ok(());
    }

    let filename = &args[1];
    let out_path = &args[2];

    convert_file(filename, out_path)?;
    
    Ok(())
}