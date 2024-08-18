/// A Library to parse Windows Notepad `TabState` artifacts
pub mod enums;
pub mod errors;
#[cfg(test)]
mod tests;
pub mod traits;
pub mod unsaved_chunks;

use byteorder::ReadBytesExt;
use enums::{CRType, Encoding};
use errors::NotepadErrors;
use serde::Serialize;
use std::convert::From;
use std::io::Read;
use unsaved_chunks::UnsavedChunks;
use winparsingtools::{
    date_time::FileTime, utils::bytes_to_hex, utils::read_uleb128, utils::read_utf16_string,
};

use std::fs::File;
use traits::ReadBool;

#[derive(Serialize, Debug)]
pub struct ConfigBlock {
    pub word_wrap: bool,
    pub rtl: bool,
    pub show_unicode: bool,
    pub version: u64,
    unknown0: u8,
    unknown1: u8,
}

impl ConfigBlock {
    pub fn from_reader<R: Read>(reader: &mut R) -> std::result::Result<Self, NotepadErrors> {
        // Read `word_wrap` feild
        let word_wrap = match reader.read_bool() {
            Ok(flag) => flag,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::word_wrap".to_string(),
                ))
            }
        };
        let rtl = match reader.read_bool() {
            Ok(flag) => flag,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::rtl".to_string(),
                ))
            }
        };
        let show_unicode = match reader.read_bool() {
            Ok(flag) => flag,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::show_unicode".to_string(),
                ))
            }
        };

        let version = match read_uleb128(reader) {
            Ok(data) => data,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::version".to_string(),
                ))
            }
        };

        let unknown0 = match reader.read_u8() {
            Ok(data) => data,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::unknown0".to_string(),
                ))
            }
        };

        let unknown1 = match reader.read_u8() {
            Ok(data) => data,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "ConfigBlock::unknown1".to_string(),
                ))
            }
        };

        Ok(Self {
            word_wrap,
            rtl,
            show_unicode,
            version,
            unknown0,
            unknown1,
        })
    }
}

impl Default for ConfigBlock {
    fn default() -> Self {
        Self {
            word_wrap: false,
            rtl: false,
            show_unicode: false,
            version: 0,
            unknown0: 0,
            unknown1: 0,
        }
    }
}
/// Represents the structure for `TabState` files
#[derive(Serialize, Debug)]
#[allow(dead_code)]
pub struct NotepadTabStat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tabstate_path: Option<String>,
    #[serde(skip_serializing)]
    pub signature: [u8; 2],
    // #[serde(skip_serializing)]
    pub seq_number: u64,
    pub is_saved_file: bool,
    pub path_size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<Encoding>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cr_type: Option<CRType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_write_time: Option<FileTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_hash: Option<String>,
    #[serde(skip_serializing)]
    pub unknown1: Option<[u8; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_start: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor_end: Option<u64>,
    pub config_block: ConfigBlock,
    pub file_content_size: u64,
    pub file_content: String,
    pub contain_unsaved_data: bool,
    pub checksum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsaved_chunks: Option<UnsavedChunks>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsaved_chunks_str: Option<String>,
}

impl Default for NotepadTabStat {
    fn default() -> Self {
        Self {
            tabstate_path: Option::None,
            signature: [0x4E, 0x50],
            seq_number: 0x00,
            is_saved_file: false,
            path_size: 0x01,
            path: Option::None,
            file_size: Option::None,
            encoding: Option::None,
            cr_type: Option::None,
            last_write_time: Option::None,
            file_hash: Option::None,
            unknown1: Option::None,
            cursor_start: Option::None,
            cursor_end: Option::None,
            config_block: ConfigBlock::default(),
            file_content_size: 0,
            file_content: String::from("Hello :D"),
            contain_unsaved_data: false,
            checksum: String::from("41414141"),
            unsaved_chunks: Option::None,
            unsaved_chunks_str: Option::None,
        }
    }
}

impl NotepadTabStat {
    /// Read the file from `path` and use `from_reader` to parse it
    pub fn from_path(path: &str) -> std::result::Result<Self, NotepadErrors> {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => return Err(NotepadErrors::FileOpen(e.to_string(), format!("{}", path))),
        };

        let mut parsed = match NotepadTabStat::from_reader(&mut file) {
            Ok(data) => data,
            Err(e) => {
                return Err(NotepadErrors::Generic(
                    e.to_string(),
                    "NotepadTabStat::from_path".to_string(),
                    "Error during parsing".to_string(),
                ));
            }
        };

        parsed.tabstate_path = Some(String::from(path));

        Ok(parsed)
    }

    /// Parse data from reader
    pub fn from_reader<R: Read>(reader: &mut R) -> std::result::Result<Self, NotepadErrors> {
        // Read first two bytes as `signature`
        let mut signature = [0u8; 2];
        if let Err(e) = reader.read_exact(&mut signature) {
            return Err(NotepadErrors::ReadError(
                e.to_string(),
                "signature".to_string(),
            ));
        }
        if signature != [0x4E, 0x50] {
            return Err(NotepadErrors::Signature(
                String::from_utf8_lossy(&signature).to_string(),
            ));
        }

        // Read unknown byte
        let seq_number = match read_uleb128(reader) {
            Ok(num) => num,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "unknown0".to_string(),
                ))
            }
        };

        // Read the flag `is_saved_file`
        let is_saved_file = match reader.read_u8() {
            Ok(flag) => match flag {
                0x0 => false,
                0x1 => true,
                x => {
                    return Err(NotepadErrors::UnexpectedValue(
                        "bool <0x1|0x0>".to_string(),
                        format!("{}", x),
                        "is_saved_file".to_string(),
                    ))
                }
            },
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "is_saved_file".to_string(),
                ))
            }
        };

        // Read `path_size`
        let path_size = match read_uleb128(reader) {
            Ok(size) => size,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "path_size".to_string(),
                ))
            }
        };

        // If the TabState file is for a saved file, extract the additinal data
        if is_saved_file {
            // Read the `path`
            let path = match read_utf16_string(reader, Option::Some(path_size as usize)) {
                Ok(path) => path,
                Err(e) => {
                    return Err(NotepadErrors::ReadErrorWithSize(
                        e.to_string(),
                        "path".to_string(),
                        path_size.to_string(),
                    ))
                }
            };

            // Read `file_size`. File size on the disk
            let file_size = match read_uleb128(reader) {
                Ok(size) => size,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "file_size".to_string(),
                    ))
                }
            };

            // Read `encoding`. The encoding used to be used by notepad to view the file
            let encoding = match reader.read_u8() {
                Ok(encoding) => Encoding::from(encoding),
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "encoding".to_string(),
                    ))
                }
            };

            // Read `cr_type` field.
            let cr_type = match reader.read_u8() {
                Ok(cr_type) => CRType::from(cr_type),
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "cr_type".to_string(),
                    ))
                }
            };

            // Read `last_write_time`. This is the last write timestamp for the file
            let last_write_time = match read_uleb128(reader) {
                Ok(timestamp) => FileTime::new(timestamp),
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "last_write_time".to_string(),
                    ));
                }
            };

            // Read `file_hash`. This is the SHA256 hash of the file content on disk
            let mut file_hash = [0u8; 32];
            if let Err(e) = reader.read_exact(&mut file_hash) {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "file_hash".to_string(),
                ));
            }

            // Read `unknown1`
            let mut unknown1 = [0u8; 2];
            if let Err(e) = reader.read_exact(&mut unknown1) {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "unknown1".to_string(),
                ));
            }

            // Read `cursor_start`. This is starting point of the text selection
            let cursor_start = match read_uleb128(reader) {
                Ok(cs) => cs,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "cursor_start".to_string(),
                    ));
                }
            };

            // Read `cursor_end`
            let cursor_end = match read_uleb128(reader) {
                Ok(ce) => ce,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "cursor_end".to_string(),
                    ));
                }
            };

            // Read unknown2
            //TODO: Change to config block
            let config_block = ConfigBlock::from_reader(reader)?;
            // let mut unknown2 = [0u8; 6];
            // if let Err(e) = reader.read_exact(&mut unknown2) {
            //     return Err(NotepadErrors::ReadError(
            //         e.to_string(),
            //         "unknown2".to_string(),
            //     ));
            // }

            // Read `file_content_size`. This is the size of the content in the TabState in chars not bytes
            let file_content_size = match read_uleb128(reader) {
                Ok(size) => size,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "file_content_size".to_string(),
                    ));
                }
            };

            // Read `file_content`. This is the file contant inside the TabState file
            let file_content =
                match read_utf16_string(reader, Option::Some(file_content_size as usize)) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(NotepadErrors::ReadError(
                            e.to_string(),
                            "file_content".to_string(),
                        ));
                    }
                };

            // Read `contain_unsaved_data`
            let contain_unsaved_data = match reader.read_u8() {
                Ok(flag) => match flag {
                    0x0 => false,
                    0x1 => true,
                    x => {
                        return Err(NotepadErrors::UnexpectedValue(
                            "bool <0x0|0x1>".to_string(),
                            x.to_string(),
                            "contain_unsaved_data".to_string(),
                        ));
                    }
                },
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "contain_unsaved_data".to_string(),
                    ));
                }
            };

            // Read `checksum`. CRC32 checksum for the previous data starting from offset 0x3
            let mut checksum = [0u8; 4];
            if let Err(e) = reader.read_exact(&mut checksum) {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "checksum".to_string(),
                ));
            }

            let unsaved_chunks = match UnsavedChunks::from_reader(reader) {
                Ok(data) => Option::Some(data),
                Err(e) => match e {
                    NotepadErrors::NA => Option::None,
                    _ => {
                        return Err(e);
                    }
                },
            };

            let unsaved_chunks_str = match &unsaved_chunks {
                Some(data) => Option::Some(data.to_string()),
                None => Option::None,
            };

            Ok(Self {
                tabstate_path: Option::None,
                signature,
                seq_number,
                is_saved_file,
                path_size,
                path: Option::Some(path),
                file_size: Option::Some(file_size),
                encoding: Option::Some(encoding),
                cr_type: Option::Some(cr_type),
                last_write_time: Option::Some(last_write_time),
                file_hash: Option::Some(bytes_to_hex(&file_hash.to_vec())),
                unknown1: Option::Some(unknown1),
                cursor_start: Option::Some(cursor_start),
                cursor_end: Option::Some(cursor_end),
                config_block,
                file_content_size,
                file_content,
                contain_unsaved_data,
                checksum: bytes_to_hex(&checksum.to_vec()),
                unsaved_chunks,
                unsaved_chunks_str,
            })
        }
        // File isn't saved to file
        else {
            // Read `cursor_start`. This is starting point of the text selection
            let cursor_start = match read_uleb128(reader) {
                Ok(cs) => cs,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "cursor_start".to_string(),
                    ));
                }
            };

            // Read `cursor_end`
            let cursor_end = match read_uleb128(reader) {
                Ok(ce) => ce,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "cursor_end".to_string(),
                    ));
                }
            };
            // Read `unknown3`
            let config_block = ConfigBlock::from_reader(reader)?;

            // Read `file_content_size`. This is the size of the content in the TabState in chars not bytes
            let file_content_size = match read_uleb128(reader) {
                Ok(size) => size,
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "file_content_size".to_string(),
                    ));
                }
            };

            let file_content =
                match read_utf16_string(reader, Option::Some(file_content_size as usize)) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(NotepadErrors::ReadError(
                            e.to_string(),
                            "file_content".to_string(),
                        ));
                    }
                };

            // Read `contain_unsaved_data`
            let contain_unsaved_data = match reader.read_u8() {
                Ok(flag) => match flag {
                    0x0 => false,
                    0x1 => true,
                    x => {
                        return Err(NotepadErrors::UnexpectedValue(
                            "bool <0x0|0x1>".to_string(),
                            x.to_string(),
                            "contain_unsaved_data".to_string(),
                        ));
                    }
                },
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "contain_unsaved_data".to_string(),
                    ));
                }
            };

            // Read `checksum`. CRC32 checksum for the previous data starting from offset 0x3
            let mut checksum = [0u8; 4];
            if let Err(e) = reader.read_exact(&mut checksum) {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "checksum".to_string(),
                ));
            }

            let unsaved_chunks = match UnsavedChunks::from_reader(reader) {
                Ok(data) => Option::Some(data),
                Err(e) => match e {
                    NotepadErrors::NA => Option::None,
                    _ => {
                        return Err(e);
                    }
                },
            };

            let unsaved_chunks_str = match &unsaved_chunks {
                Some(data) => Option::Some(data.to_string()),
                None => Option::None,
            };

            Ok(Self {
                tabstate_path: Option::None,
                signature,
                seq_number,
                is_saved_file,
                path_size,
                path: Option::None,
                file_size: Option::None,
                encoding: Option::None,
                cr_type: Option::None,
                last_write_time: Option::None,
                file_hash: Option::None,
                unknown1: Option::None,
                cursor_start: Some(cursor_start),
                cursor_end: Some(cursor_end),
                file_content_size,
                config_block,
                file_content,
                contain_unsaved_data,
                checksum: bytes_to_hex(&checksum.to_vec()),
                unsaved_chunks,
                unsaved_chunks_str,
            })
        }
    }
}
