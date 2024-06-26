use configparser::ini::Ini;

#[derive(Debug, Clone, Default)]
pub struct DownloadSource {
    pub default: String,
    pub minimal: String,
    pub default_torrent: String,
    pub minimal_torrent: String,
    pub checksum: String,
}

impl DownloadSource {
    fn verify_checksum(&self, iso_checksum: chksum::SHA2_256) -> bool {
        self.checksum == iso_checksum.digest().to_string()
    }

    fn from_config(config: &Ini, section: &str) -> Self {
        Self {
            default: config.get(section, "default").unwrap_or_default(),
            minimal: config.get(section, "minimal").unwrap_or_default(),
            default_torrent: config.get(section, "default_torrent").unwrap_or_default(),
            minimal_torrent: config.get(section, "minimal_torrent").unwrap_or_default(),
            checksum: config.get(section, "checksum").unwrap_or_default(),
        }
    }
}
pub fn load_sources(config: &mut Ini) -> Vec<DownloadSource> {
    config
        .load("config.ini")
        .expect("Failed to load config.ini");

    let mut distros = config.sections();

    // sort by alphabetical order.
    distros.sort();

    distros
        .iter()
        .map(|distros| DownloadSource::from_config(config, distros))
        .collect()
}
