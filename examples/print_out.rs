use bhlang::LangFile;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

const LANG_PATH: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/languages/language.1.bin";
const OUT_PATH: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/languages/language.1.tsv";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(LANG_PATH)?;
    let reader = BufReader::new(file);
    let lang_file = LangFile::read(reader)?;

    let outfile = File::create(OUT_PATH)?;
    let mut writer = BufWriter::new(outfile);

    let mut entries: Vec<_> = lang_file.iter().collect();
    entries.sort();
    for (key, value) in entries {
        // replace special chars
        let value = value.replace('\n', "\\n").replace('\r', "\\r");

        writer.write(format!("{key}\t{value}\n").as_bytes())?;
    }

    Ok(())
}
