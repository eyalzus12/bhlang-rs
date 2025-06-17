use byteorder::{BigEndian as BE, LittleEndian as LE, ReadBytesExt, WriteBytesExt};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::{
    collections::{HashMap, hash_map},
    io::{Read, Write},
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

#[derive(Error, Debug)]
pub enum LangWritingError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Entry count exceeds i32 max: ({entry_count:?})")]
    TooManyEntriesError { entry_count: usize },
    #[error("Key length exceeds u16 max: ({key_length:?})")]
    TooLongKey { key_length: usize },
    #[error("Value length exceeds u16 max: ({value_length:?})")]
    TooLongValue { value_length: usize },
}

impl LangFile {
    pub fn read<R: Read>(mut reader: R) -> Result<Self, LangReadingError> {
        let header = reader.read_u32::<LE>()?;
        let zlib = ZlibDecoder::new(reader);
        Ok(Self {
            header,
            entries: Self::read_entries(zlib)?,
        })
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

    pub fn write<W: Write>(&self, mut writer: W) -> Result<(), LangWritingError> {
        writer.write_u32::<LE>(self.header)?;
        let zlib = ZlibEncoder::new(writer, Compression::best());
        self.write_entries(zlib)?;

        Ok(())
    }

    fn write_entries<W: Write>(&self, mut writer: W) -> Result<(), LangWritingError> {
        let entry_count = self.entries.len();
        let entry_count = match entry_count.try_into() {
            Ok(v) => v,
            Err(_) => return Err(LangWritingError::TooManyEntriesError { entry_count }),
        };
        writer.write_i32::<BE>(entry_count)?;

        for (key, value) in self.entries.iter() {
            let key_length = key.len();
            let key_length = match key_length.try_into() {
                Ok(v) => v,
                Err(_) => return Err(LangWritingError::TooLongKey { key_length }),
            };

            let value_length = value.len();
            let value_length = match value_length.try_into() {
                Ok(v) => v,
                Err(_) => return Err(LangWritingError::TooLongValue { value_length }),
            };

            writer.write_u16::<BE>(key_length)?;
            writer.write_all(key.as_bytes())?;
            writer.write_u16::<BE>(value_length)?;
            writer.write_all(value.as_bytes())?;
        }

        Ok(())
    }
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
