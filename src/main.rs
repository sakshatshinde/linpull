use anyhow::Result;
use configparser::ini::Ini;
use sources::load_sources;
use std::path::PathBuf;
use trauma::{download::Download, downloader::DownloaderBuilder};

mod sources;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config = Ini::new();
    let sources = load_sources(&mut config);
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("downloads"))
        .build();

    let iso = sources[0].clone();
    println!("{:?}", iso);

    // let downloads = vec![Download::try_from(iso.mirror_1.as_str()).unwrap()];

    // Starts the actual download -- do not uncomment will spam arch server!
    // downloader.download(&downloads).await;

    Ok(())
}
