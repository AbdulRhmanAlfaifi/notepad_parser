use crate::errors::NotepadErrors;

pub trait ReadBool: std::io::Read {
    /// Read a `u8` and return `true` if it is `0x1` or `false` if it is `0x0`, otherwise return Error
    fn read_bool(&mut self) -> std::result::Result<bool, NotepadErrors>;
}

impl<T: std::io::Read> ReadBool for T {
    fn read_bool(&mut self) -> std::result::Result<bool, NotepadErrors> {
        let mut data = [0u8; 1];
        if let Err(e) = self.read_exact(&mut data) {
            return Err(NotepadErrors::ReadError(
                e.to_string(),
                "traits::ReadBool".to_string(),
            ));
        }
        match data[0] {
            0x0 => Ok(false),
            0x1 => Ok(true),
            x => Err(NotepadErrors::UnexpectedValue(
                "bool <0x0|0x1>".to_string(),
                format!("{}", x),
                "traits::ReadBool".to_string(),
            )),
        }
    }
}
