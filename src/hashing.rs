use std::{io::{BufReader, Read}, fs::File,
            path::PathBuf};

pub fn  create_hash(files: &Vec<PathBuf>) -> Result<String, std::io::Error>
{
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut buffer: [u8; 2048] = [0; 2048];
    for file in files
    {
        let f: File = File::open(file)?;
        let mut reader = BufReader::new(f);
        
        while let Ok(bytes_read) = reader.read(&mut buffer)
        {
            if bytes_read == 0
            {
                break;
            }
            let window = &buffer[..bytes_read];
            for &byte in window
            {
                hash ^= u64::MAX - byte as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
    }
    Ok(format!("{:x}", hash))
}
