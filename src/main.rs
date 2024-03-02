use bytes::Buf;
use clap::Parser;
use std::{env, fs::File, io, path::PathBuf};
use url::Url;

use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Base URL for gallery images
    #[arg(short, long)]
    url: Url,

    /// Number of pages to download
    #[arg(short, long)]
    pages: i32,

    /// Write downloaded files to <OUTPUT> instead of the current working directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let source = {
        let mut url = args.url;
        // Ensure Url has trailing slash (see Url impl join)
        url.path_segments_mut().unwrap().pop_if_empty().push("");
        url
    };

    let download_dir = args
        .output
        .unwrap_or_else(|| env::current_dir().expect("Unable to find current working directory"));

    let client = reqwest::blocking::Client::builder().build()?;

    (1..=args.pages).into_par_iter().for_each(|i| {
        let mut path = download_dir.clone();
        let url = source.join(&format!("{i}.jpg")).unwrap();
        let bytes = client.get(url).send().unwrap().bytes().unwrap();
        let filename = format!("{i:03}.jpg");
        path.push(filename);
        let mut out = File::create(path).expect("failed to create file");
        io::copy(&mut bytes.reader(), &mut out).expect("Failed to copy content");
    });

    Ok(())
}
