slint::include_modules!();

use std::path::PathBuf;

use configparser::ini::Ini;
use trauma::downloader::DownloaderBuilder;

use crate::sources::available_distros;

mod sources;

#[tokio::main]
async fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    let mut config = Ini::new();
    let distro_list = available_distros(&mut config);
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("iso"))
        .build();

    ui.run()
}
