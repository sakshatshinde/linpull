use configparser::ini::Ini;

#[derive(Debug, Clone)]
pub struct DownloadSource {
    pub mirror_1: String,
    pub mirror_2: String,
    pub mirror_3: String,
    pub checksum: String,
}

impl DownloadSource {
    fn verify_checksum(&self, iso_checksum: chksum::SHA2_256) -> bool {
        self.checksum == iso_checksum.digest().to_string()
    }
}

pub fn load_sources(config: &mut Ini) -> Vec<DownloadSource> {
    config
        .load("config.ini")
        .expect("Failed to load config.ini");

    let src_arch_linux = DownloadSource {
        mirror_1: config.get("ARCH_LINUX", "mirror1").unwrap(),
        mirror_2: config.get("ARCH_LINUX", "mirror2").unwrap(),
        mirror_3: config.get("ARCH_LINUX", "mirror3").unwrap(),
        checksum: config.get("ARCH_LINUX", "checksum").unwrap(),
    };

    let src_ubuntu = DownloadSource {
        mirror_1: config.get("UBUNTU", "mirror1").unwrap(),
        mirror_2: config.get("UBUNTU", "mirror2").unwrap(),
        mirror_3: config.get("UBUNTU", "mirror3").unwrap(),
        checksum: config.get("UBUNTU", "checksum").unwrap(),
    };

    let src_nixos = DownloadSource {
        mirror_1: config.get("NIX_OS", "mirror1").unwrap(),
        mirror_2: config.get("NIX_OS", "mirror2").unwrap(),
        mirror_3: config.get("NIX_OS", "mirror3").unwrap(),
        checksum: config.get("NIX_OS", "checksum").unwrap(),
    };

    let src_zorin = DownloadSource {
        mirror_1: config.get("ZORIN", "mirror1").unwrap(),
        mirror_2: config.get("ZORIN", "mirror2").unwrap(),
        mirror_3: config.get("ZORIN", "mirror3").unwrap(),
        checksum: config.get("ZORIN", "checksum").unwrap(),
    };

    let src_mint = DownloadSource {
        mirror_1: config.get("MINT", "mirror1").unwrap(),
        mirror_2: config.get("MINT", "mirror2").unwrap(),
        mirror_3: config.get("MINT", "mirror3").unwrap(),
        checksum: config.get("MINT", "checksum").unwrap(),
    };

    let src_fedora = DownloadSource {
        mirror_1: config.get("FEDORA", "mirror1").unwrap(),
        mirror_2: config.get("FEDORA", "mirror2").unwrap(),
        mirror_3: config.get("FEDORA", "mirror3").unwrap(),
        checksum: config.get("FEDORA", "mirror3").unwrap(),
    };

    vec![
        src_arch_linux,
        src_mint,
        src_nixos,
        src_ubuntu,
        src_zorin,
        src_fedora,
    ]
}
