use std::path::PathBuf;

use anyhow::Result;
use configparser::ini::Ini;
use trauma::downloader::DownloaderBuilder;

use crate::sources::available_distros;

mod sources;
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut config = Ini::new();
    let distro_list = available_distros(&mut config);
    dbg!(distro_list);

    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("iso"))
        .build();

    // let downloads = vec![Download::try_from(iso.mirror_1.as_str()).unwrap()];

    // Starts the actual download -- do not uncomment will spam arch server!
    // downloader.download(&downloads).await;

    Ok(())
}
