use std::io::Result;
use std::fs;
use std::path::{Path, PathBuf};



pub fn setup_dir() -> Result<()> {
    let root_dir = "C:/";
    let folio_dir_name = "folio";
    let path_to_dir = Path::new(root_dir).join(folio_dir_name);
    let filename = "tododb.json";

    check_for_dir(path_to_dir.clone())?;
    check_for_file(filename, path_to_dir)?;

    Ok(())
}

pub fn create_dir(pathname: PathBuf) -> Result<()> {
    fs::create_dir(pathname)?;
    return Ok(())
    
}

pub fn check_for_dir (pathname: PathBuf) -> Result<()> {
    if pathname.exists() {
        println!("{} already exists.", pathname.display());
         return Ok(())
    } else {
        create_dir(pathname)?;
    }

    Ok(())
}

pub fn check_for_file(filename: &str, path: PathBuf) -> Result<()> {
    let filepath = path.join(filename);

    if !filepath.exists() {
        println!("File has been created at {:?}", filepath);
        fs::File::create(filepath)?;
    } else {
        println!("File already Exists.");
    }

    Ok(())
}