use crate::NotepadErrors;
use serde::Serialize;
use std::{
    fmt::Display,
    io::{self, Read},
};
use winparsingtools::utils::{bytes_to_hex, read_uleb128, read_utf16_string};

#[derive(Debug, Serialize)]
pub struct UnsavedChunk {
    position: u64,
    num_of_deletion: u64,
    num_of_addition: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
    checksum: String,
}

impl UnsavedChunk {
    pub fn from_reader<R: Read>(reader: &mut R) -> std::result::Result<Self, NotepadErrors> {
        // Read `position`. This is the cursor position where the data will be deleted from or added to
        let position = match read_uleb128(reader) {
            Ok(pos) => pos,
            Err(e) => match e.kind() {
                io::ErrorKind::UnexpectedEof => {
                    return Err(NotepadErrors::EoF);
                }
                _ => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "UnsavedChunk::position".to_string(),
                    ));
                }
            },
        };

        // Read `num_of_deletion`. This is the number of characters to delete.
        let num_of_deletion = match read_uleb128(reader) {
            Ok(num_of_deletion) => num_of_deletion,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "UnsavedChunk::num_of_deletion".to_string(),
                ));
            }
        };

        // Read `num_of_addition`. This is the number of characters to add.
        let num_of_addition = match read_uleb128(reader) {
            Ok(num_of_addition) => num_of_addition,
            Err(e) => {
                return Err(NotepadErrors::ReadError(
                    e.to_string(),
                    "UnsavedChunk::num_of_addition".to_string(),
                ));
            }
        };

        // Read `data` if it is an addition
        let data = match num_of_addition {
            0 => Option::None,
            _ => match read_utf16_string(reader, Option::Some(num_of_addition as usize)) {
                Ok(data) => Option::Some(data),
                Err(e) => {
                    return Err(NotepadErrors::ReadError(
                        e.to_string(),
                        "UnsavedChunk::data".to_string(),
                    ));
                }
            },
        };

        let mut checksum = [0u8; 4];
        if let Err(e) = reader.read_exact(&mut checksum) {
            return Err(NotepadErrors::ReadError(
                e.to_string(),
                "checksum".to_string(),
            ));
        }

        Ok(Self {
            position,
            num_of_deletion,
            num_of_addition,
            data,
            checksum: bytes_to_hex(&checksum.to_vec()),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UnsavedChunks(Vec<UnsavedChunk>);

impl UnsavedChunks {
    pub fn from_reader<R: Read>(reader: &mut R) -> std::result::Result<Self, NotepadErrors> {
        let mut unsaved_chunks: Vec<UnsavedChunk> = vec![];

        loop {
            match UnsavedChunk::from_reader(reader) {
                Ok(chunk) => unsaved_chunks.push(chunk),
                Err(e) => match e {
                    NotepadErrors::EoF => break,
                    e => {
                        return Err(NotepadErrors::Generic(
                            e.to_string(),
                            "UnsavedChunks::from_reader".to_string(),
                            "Error during reading list of UnsavedChunk.".to_string(),
                        ));
                    }
                },
            }
        }

        if unsaved_chunks.len() > 0 {
            Ok(Self(unsaved_chunks))
        } else {
            return Err(NotepadErrors::NA);
        }
    }
}

impl Display for UnsavedChunks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut previous_addition = 0;
        let data = self
            .0
            .iter()
            .map(|x| {
                let mut chunk = String::from("");
                if x.num_of_addition > 0 {
                    if previous_addition == 0 {
                        previous_addition = x.position;
                        chunk.push_str(&format!("[{}]:{}", x.position, &x.data.clone().unwrap()));
                    } else if x.position == (previous_addition + 1) {
                        chunk.push_str(&x.data.clone().unwrap());
                        previous_addition = x.position;
                    } else {
                        chunk.push_str(&format!(",[{}]:{}", x.position, &x.data.clone().unwrap()));
                        previous_addition = x.position;
                    }
                } else {
                    if previous_addition > 0 {
                        previous_addition = previous_addition - 1;
                    }
                    chunk.push_str(&format!("<DEL:{}>", x.position));
                }
                format!("{}", chunk)
            })
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}", data)
    }
}
