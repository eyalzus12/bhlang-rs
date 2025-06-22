use bhlang::LangFile;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

const SOURCE_PATH: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/languages/language.1.tsv";
const OUT_PATH: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/languages/language.1.bin";

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(SOURCE_PATH)?;
    let reader = BufReader::new(file);

    let mut lang = LangFile::new(0);
    for str in reader.lines() {
        let str = str?;
        let (key, value) = str.split_once('\t').unwrap();
        let key = key.to_owned();
        let value = value.replace("\\n", "\n").replace("\\r", "\r");
        lang.insert(key, value);
    }

    let out = File::create(OUT_PATH)?;
    let writer = BufWriter::new(out);
    lang.write(writer)?;

    Ok(())
}
