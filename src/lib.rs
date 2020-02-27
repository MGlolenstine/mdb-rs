#[derive(Debug)]
pub struct MDatabase{
    filename: String,
    magic_number: u32,
    file_format_id: String,
    jet_version: u32,
    db_info: Option<DBInfo>,
}

use std::io::Seek;
use std::fs::File;
use std::io::{Read, SeekFrom};
use std::mem::transmute;

impl MDatabase {
    pub fn open_database(filename: &str) -> Result<MDatabase, std::io::Error>{
        let mut file=File::open("test.mdb").unwrap();
        let mut db = read_headers(filename, &mut file).unwrap();
        db = read_db_info(db, &mut file).unwrap();
        Ok(db)
    }
}

fn read_headers(filename: &str, file: &mut File) -> Result<MDatabase, std::io::Error>{
    let magic_number = seek_and_read_u32(0x00, file).unwrap();
    let file_format_id = seek_and_read_string(0x04, file).unwrap();
    let jet_version = seek_and_read_u32(0x14, file).unwrap();
    Ok(MDatabase{
        filename: filename.to_string(),
        magic_number,
        file_format_id: file_format_id,
        jet_version,
        db_info: None
    })
}

fn read_db_info(mut db: MDatabase, file: &mut File) -> Result<MDatabase, std::io::Error>{
    let system_collation = if db.jet_version == 0 {
        seek_and_read_u16(0x22, file).unwrap()
    }else{
        seek_and_read_u16(0x56, file).unwrap()
    };
    let system_code_page = seek_and_read_u16(0x24, file).unwrap();
    let database_key = seek_and_read_u32(0x26, file).unwrap();
    let creation_date = seek_and_read_f64(0x5A, file).unwrap();
    let info = DBInfo{
        system_collation,
        system_code_page,
        database_key,
        database_password: None,
        creation_date,
    };
    db.db_info = Some(info);
    Ok(db)
}

// fn read_headers(filename: &str, file: &mut File) -> Result<MDatabase, std::io::Error>{
//     unsafe{
//         let mut magic_number=[0u8;4];
//         file.read(&mut magic_number).unwrap();
//         let magic_number = transmute::<[u8; 4], u32>(magic_number);
//         let mut file_format_id = [0u8; 16];
//         file.read(&mut file_format_id).unwrap();
//         let file_format_id = String::from_utf8_lossy(&file_format_id);
//         let mut jet_version = [0u8; 4];
//         file.read(&mut jet_version).unwrap();
//         let jet_version = transmute::<[u8; 4], u32>(jet_version);
//         Ok(MDatabase{
//             filename: filename.to_string(),
//             magic_number,
//             file_format_id: file_format_id.to_string(),
//             jet_version
//         })
//     }
// }

// fn read_db_info(db: MDatabase, file: &mut File) -> Result<MDatabase, std::io::Error>{
//     unsafe{
//         let mut system_collation = [0u8; 2];
//         file.seek(SeekFrom::Start(0x22));
//         if(db.jet_version == 0){
//             file.read(&mut system_collation);
//             let system_collation = transmute::<[u8; 2], u16>(system_collation);
//         }
//
//         Ok(db)
//     }
// }

fn seek_and_read_u32(position: u64, file: &mut File) -> Option<u32>{
    unsafe{
        file.seek(SeekFrom::Start(position));
        let mut buf = [0u8; 4];
        file.read(&mut buf);
        let out = transmute::<[u8; 4], u32>(buf);
        return Some(out);
    }
}

fn seek_and_read_u16(position: u64, file: &mut File) -> Option<u16>{
    unsafe{
        file.seek(SeekFrom::Start(position));
        let mut buf = [0u8; 2];
        file.read(&mut buf);
        let out = transmute::<[u8; 2], u16>(buf);
        return Some(out);
    }
}

fn seek_and_read_u64(position: u64, file: &mut File) -> Option<u64>{
    unsafe{
        file.seek(SeekFrom::Start(position));
        let mut buf = [0u8; 8];
        file.read(&mut buf);
        let out = transmute::<[u8; 8], u64>(buf);
        return Some(out);
    }
}

fn seek_and_read_f64(position: u64, file: &mut File) -> Option<f64>{
    unsafe{
        file.seek(SeekFrom::Start(position));
        let mut buf = [0u8; 8];
        file.read(&mut buf);
        let out = transmute::<[u8; 8], f64>(buf);
        return Some(out);
    }
}

fn seek_and_read_string(position: u64, file: &mut File) -> Option<String>{
    file.seek(SeekFrom::Start(position));
    let mut buf = [0u8; 16];
    file.read(&mut buf);
    let out = String::from_utf8_lossy(&buf);
    return Some(out.to_string());
}

#[derive(Debug)]
struct DBInfo{
    system_collation: u16,
    system_code_page: u16,
    database_key: u32, // 0 means not encoded
    database_password: Option<[u8; 20]>, //TODO: Add a working code for Jet4's 40byte array
    creation_date: f64
}
