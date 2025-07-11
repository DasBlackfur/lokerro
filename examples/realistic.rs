use std::{collections::HashMap, fs::File, io::Read};
use lokerro::{ErrorExt, Result};

fn main() -> Result<()> {
    let result = load_settings().loc_msg("Loading settings failed")?;

    println!("{result:?}");

    Ok(())
}

fn load_settings() -> Result<HashMap<u32, String>> {
    let mut config = HashMap::new();
    let config_1 = process_file("this_file_may_not_exist.txt").loc()?;
    let config_2 = process_file("this_file_neither.txt").loc()?;

    config.insert(1, config_1);
    config.insert(2, config_2);

    Ok(config)
}

fn process_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut string = String::new();

    file.read_to_string(&mut string)?;
    Ok(string)
}