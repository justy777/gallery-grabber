use anyhow::Context;
use bytes::Buf;
use clap::Parser;
use reqwest::blocking::Client;
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
    pages: u32,

    /// Write downloaded files to <OUTPUT> instead of the current working directory
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let base_url = {
        let mut url = args.url;
        // Ensure Url has trailing slash (see Url impl join)
        url.path_segments_mut().unwrap().pop_if_empty().push("");
        url
    };

    let download_dir = args
        .output
        .ok_or_else(|| anyhow::anyhow!("No output directory specified"))
        .or_else(|_| env::current_dir())
        .context("Failed to find current working directory")?;

    let client = Client::builder()
        .build()
        .context("Failed to build HTTP Client")?;

    (1..=args.pages)
        .into_par_iter()
        .map(|i| {
            let url = base_url.join(&format!("{i}.webp"))?;
            let bytes = client.get(url).send()?.error_for_status()?.bytes()?;

            let filename = format!("{i:03}.webp");
            let download_path = download_dir.join(&filename);
            let mut output_file = File::create(download_path.as_path())
                .with_context(|| format!("failed to create file {}", download_path.display()))?;
            io::copy(&mut bytes.reader(), &mut output_file).with_context(|| {
                format!("Failed to copy bytes to file {}", download_path.display())
            })?;
            Ok(())
        })
        .collect::<Result<(), anyhow::Error>>()?;

    Ok(())
}
