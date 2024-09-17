slint::include_modules!();

use std::path::PathBuf;

use configparser::ini::Ini;
use trauma::downloader::DownloaderBuilder;

use crate::sources::available_distros;

mod sources;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    // slint stuff
    let ui = AppWindow::new()?;

    ui.global::<GDownGlobalProgress>()
        .on_download_progress(|| 0.5);

    // downloader stuff
    let mut config = Ini::new();
    let distro_list = available_distros(&mut config);
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("iso"))
        .build();

    ui.run()
}
