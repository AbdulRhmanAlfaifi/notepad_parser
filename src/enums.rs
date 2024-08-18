use serde::Serialize;

#[derive(Serialize, Debug)]
#[repr(u8)]
pub enum Encoding {
    ANSI = 0x01,
    UTF16LE = 0x02,
    UTF16BE = 0x03,
    UTF8BOM = 0x04,
    UTF8 = 0x05,
    UNKNOWN(u8),
}

impl From<u8> for Encoding {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Encoding::ANSI,
            0x02 => Encoding::UTF16LE,
            0x03 => Encoding::UTF16BE,
            0x04 => Encoding::UTF8BOM,
            0x05 => Encoding::UTF8,
            x => Encoding::UNKNOWN(x),
        }
    }
}

#[derive(Serialize, Debug)]
#[repr(u8)]
pub enum CRType {
    CRLF = 0x1,
    CR = 0x2,
    LF = 0x3,
    UNKNOWN(u8),
}

impl From<u8> for CRType {
    fn from(value: u8) -> Self {
        match value {
            0x01 => CRType::CRLF,
            0x02 => CRType::CR,
            0x03 => CRType::LF,
            x => CRType::UNKNOWN(x),
        }
    }
}
