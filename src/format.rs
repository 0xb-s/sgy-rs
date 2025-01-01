use crate::errors::SegyError;

///  SEG-Y sample formats.

#[derive(Debug, Clone, Copy)]
pub enum SampleFormat {
    /// 1 = IBM float (32-bit)
    IbmFloat,
    /// 2 = 32-bit integer
    Int32,
    /// 3 = 16-bit integer
    Int16,
    /// 5 = IEEE float (32-bit)
    IeeeFloat,
    /// 8 = 8-bit integer
    Int8,
}
impl SampleFormat {
  
    pub fn from_code(code: u16) -> Result<Self, SegyError> {
        match code {
            1 => Ok(SampleFormat::IbmFloat),
            2 => Ok(SampleFormat::Int32),
            3 => Ok(SampleFormat::Int16),
            5 => Ok(SampleFormat::IeeeFloat),
            8 => Ok(SampleFormat::Int8),
            other => Err(SegyError::UnsupportedSampleFormat(other)),
        }
    }

    /// Returns the size in bytes of each sample for the given format.
    pub fn sample_size(&self) -> usize {
        match self {
            SampleFormat::IbmFloat => 4,
            SampleFormat::Int32 => 4,
            SampleFormat::Int16 => 2,
            SampleFormat::IeeeFloat => 4,
            SampleFormat::Int8 => 1,
        }
    }
}
