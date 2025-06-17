use byteorder::{BigEndian as BE, LittleEndian as LE, ReadBytesExt};
use flate2::read::ZlibDecoder;
use std::{
    collections::{HashMap, hash_map},
    io::Read,
    string::FromUtf8Error,
};
use thiserror::Error;

type EntryMap = HashMap<String, String>;

pub struct LangFile {
    pub header: u32,
    pub entries: EntryMap,
}

#[derive(Error, Debug)]
pub enum LangReadingError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    FromUtf8Error(#[from] FromUtf8Error),
    #[error("There are a negative number of entries: ({entry_count:?})")]
    NegativeEntryCountError { entry_count: i32 },
}

impl LangFile {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, LangReadingError> {
        let header = reader.read_u32::<LE>()?;
        let zlib = ZlibDecoder::new(reader);
        Ok(Self {
            header,
            entries: read_entries(zlib)?,
        })
    }
}

fn read_entries<R: Read>(mut reader: R) -> Result<EntryMap, LangReadingError> {
    let entry_count = reader.read_i32::<BE>()?;
    if entry_count < 0 {
        return Err(LangReadingError::NegativeEntryCountError { entry_count });
    }

    let mut result = HashMap::with_capacity(entry_count as usize);
    for _ in 0..entry_count {
        let key = read_string(&mut reader)?;
        let value = read_string(&mut reader)?;
        result.insert(key, value);
    }
    Ok(result)
}

fn read_string<R: Read>(mut reader: R) -> Result<String, LangReadingError> {
    let str_len = reader.read_u16::<BE>()?.into();
    let mut str_buf = vec![0u8; str_len];
    reader.read_exact(&mut str_buf)?;
    let str = String::from_utf8(str_buf)?;
    Ok(str)
}

impl LangFile {
    pub fn new(header: u32) -> Self {
        Self {
            header,
            entries: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn insert(&mut self, key: String, value: String) -> Option<String> {
        self.entries.insert(key, value)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    pub fn iter(&self) -> hash_map::Iter<'_, String, String> {
        self.entries.iter()
    }
}
