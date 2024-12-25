use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=config");
    let binding = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let target_dir = binding
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    copy_dir_all("assets", target_dir.join("assets")).unwrap();
}
