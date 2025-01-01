use std::{
    fs::File,
    io::{Seek, SeekFrom},
    path::Path,
};

use errors::SegyError;
use reader::SegyReader;
use sgy::SegyFile;

pub mod ebcdic_syg;
pub mod errors;
pub mod format;
pub mod reader;
pub mod sgy;
pub mod utils;
pub mod value;

pub fn read_segy_from_file<P: AsRef<Path>>(path: P) -> Result<SegyFile, SegyError> {
    let mut file = File::open(path)?;
    let mut cloned_file = file.try_clone()?; 

    let mut reader = SegyReader::new(&mut cloned_file);

    // 1. Read EBCDIC header
    file.seek(SeekFrom::Start(0))?; 
    let ebcdic_header = reader.read_ebcdic_header()?;

    // 2. Read binary header
    let binary_header = reader.read_binary_header()?;

    // 3. Read all traces
    let traces = reader.read_all_traces(&binary_header)?;

    Ok(SegyFile {
        ebcdic_header: ebcdic_header.text,
        binary_header,
        traces,
    })
}
