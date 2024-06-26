use configparser::ini::Ini;

#[derive(Debug, Clone, Default)]
pub struct DownloadSource {
    pub distro: String,
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
            distro: section.to_string(),
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

    distros
        .iter()
        .map(|section| DownloadSource::from_config(config, section))
        .collect()
}

pub fn available_distros(config: &mut Ini) -> Vec<String> {
    let sources = load_sources(config);
    sources.iter().map(|s| s.distro.clone()).collect()
}
