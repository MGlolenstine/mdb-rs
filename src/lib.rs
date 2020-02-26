#[derive(Debug)]
pub struct MDatabase{
    filename: String,
    magic_number: u32,
    file_format_id: String,
    jet_version: u32,
}

use std::fs::File;
use std::io::{Read, SeekFrom, Seek};
use std::mem::transmute;

impl MDatabase {
    pub fn open_database(filename: &str) -> Result<MDatabase, std::io::Error>{
        unsafe{
            let mut file=File::open("test.mdb").unwrap();
            let mut buf=[0u8;4];
            file.read(&mut buf).unwrap();
            let magic_number = transmute::<[u8; 4], u32>(buf);
            let mut file_format_id = [0u8; 16];
            file.read(&mut file_format_id).unwrap();
            let file_format_id = String::from_utf8_lossy(&file_format_id);
            let mut jet_version = [0u8; 4];
            file.read(&mut jet_version).unwrap();
            let jet_version = transmute::<[u8; 4], u32>(jet_version);
            Ok(MDatabase{
                filename: filename.to_string(),
                magic_number,
                file_format_id: file_format_id.to_string(),
                jet_version
            })
        }
    }
}
