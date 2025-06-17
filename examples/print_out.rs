use bhlang::LangFile;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    const LANG_PATH: &str =
        "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/languages/language.1.bin";

    let file = File::open(LANG_PATH)?;
    let reader = BufReader::new(file);
    let lang_file = LangFile::read(reader)?;

    for (key, value) in lang_file.iter() {
        println!("{key} = {value}");
    }

    Ok(())
}
