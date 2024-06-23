use std::fmt::Display;
use std::str::from_utf8;

#[derive(Debug)]
pub enum Entry<'a> {
    KeyData {
        length: u16,
        data: &'a [u8],
    },
    Internal {
        length: u16,
        ty: u8,
        pgno: u32,
        nrecs: u32,
        data: &'a [u8],
    },
    // Overflow {
    //     pgno: u32,
    //     tlen: u32,
    // },
}

impl<'a> Display for Entry<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Self::KeyData { data, .. } = self {
            if let Ok(s) = from_utf8(data) {
                return write!(f, "{s}");
            }
        }
        // TODO: implement Display for Internal and Overflow variants
        write!(f, "...")
    }
}

impl<'a> Entry<'a> {
    pub fn new_keydata(buffer: &'a [u8]) -> Self {
        if buffer.len() < 3 {
            panic!("invalid entry length {}", buffer.len());
        }

        if buffer[2] != 1 {
            panic!("invalid keydata entry type {}", buffer[2]);
        }

        let length = u16::from_le_bytes(buffer[0..2].try_into().unwrap());
        Self::KeyData {
            length,
            data: &buffer[3..length as usize],
        }
    }

    pub fn new_internal(buffer: &'a [u8]) -> Self {
        if buffer.len() < 12 {
            panic!("invalid entry length {}", buffer.len());
        }

        let length = u16::from_le_bytes(buffer[0..2].try_into().unwrap());
        let ty = buffer[2];
        let pgno = u32::from_le_bytes(buffer[4..8].try_into().unwrap());
        let nrecs = u32::from_le_bytes(buffer[8..12].try_into().unwrap());
        let data = &buffer[12..(length + 12) as usize];
        Self::Internal {
            length,
            ty,
            pgno,
            nrecs,
            data,
        }
    }
}
