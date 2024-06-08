use anyhow::Result;
use configparser::ini::Ini;
use sources::load_sources;

mod sources;

fn main() -> Result<(), anyhow::Error> {
    let mut config = Ini::new();
    let sources = load_sources(&mut config);

    print!("{:#?}", sources);

    Ok(())
}
