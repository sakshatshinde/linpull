use configparser::ini::Ini;

#[derive(Debug, Clone, Default)]
pub struct DownloadSource {
    pub distro: String,
    pub default: String,
    pub minimal: String,
    pub mirror_default: String,
    pub mirror_minimal: String,
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
            mirror_default: config.get(section, "mirror_default").unwrap_or_default(),
            mirror_minimal: config.get(section, "mirror_minimal").unwrap_or_default(),
            checksum: config.get(section, "checksum").unwrap_or_default(),
        }
    }
}
pub fn load_sources(config: &mut Ini) -> Vec<DownloadSource> {
    config
        .load("config.ini")
        .expect("Failed to load config.ini");

    let distros = config.sections();

    distros
        .iter()
        .map(|section| DownloadSource::from_config(config, section))
        .collect()
}

pub fn available_distros(config: &mut Ini) -> Vec<String> {
    let sources = load_sources(config);
    sources.iter().map(|s| s.distro.clone()).collect()
}
