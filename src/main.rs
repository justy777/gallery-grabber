use bytes::Buf;
use std::fs::File;
use std::{fs, io};

use rayon::prelude::*;

const SOURCE: &str = "https://i2.hentaifox.com/001/700922";
const FOLDER: &str =
    "[Azuma Tesshin] Futari de Dekirumon - You & I can do every lovemaking [English] [Tigoris]";
const NUMBER_OF_PAGES: i32 = 204;

fn main() -> Result<(), anyhow::Error> {
    let mut download_folder = dirs::download_dir().expect("Unable to find download folder");
    download_folder.push(FOLDER);
    fs::create_dir(download_folder.clone())?;

    let client = reqwest::blocking::Client::builder().build()?;

    (1..=NUMBER_OF_PAGES).into_par_iter().for_each(|i| {
        let mut path = download_folder.clone();
        let url = format!("{SOURCE}/{i}.jpg");
        let bytes = client.get(url).send().unwrap().bytes().unwrap();
        let filename = format!("{i:03}.jpg");
        path.push(filename);
        let mut out = File::create(path).expect("failed to create file");
        io::copy(&mut bytes.reader(), &mut out).expect("Failed to copy content");
    });

    Ok(())
}
