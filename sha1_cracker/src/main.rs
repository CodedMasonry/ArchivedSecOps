use sha1::{Digest, Sha1};
use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const SHA1_HASH_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: ");
        println!("sha1_cracker <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let expected_hash = args[2].trim();
    if expected_hash.len() != SHA1_HASH_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist);

    for line in reader.lines() {
        let line = line?.trim().to_string();
        let mut hash = Sha1::new();
        hash.update(line.as_bytes());

        if expected_hash == hex::encode(hash.finalize()) {
            println!("[FOUND] Pass: {}", &line);
            return Ok(());
        }
    }

    println!("No matches found");
    Ok(())
}
