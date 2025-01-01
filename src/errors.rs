/// Errors when reading SEG-Y files.
#[derive(Debug)]
pub enum SegyError {
    IoError(std::io::Error),

    UnsupportedSampleFormat(u16),

    IbmFloatConversionError,

    ParseError(String),
}
impl std::fmt::Display for SegyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SegyError::IoError(e) => write!(f, "I/O error: {}", e),
            SegyError::UnsupportedSampleFormat(code) => {
                write!(f, "Unsupported sample format code: {}", code)
            }
            SegyError::IbmFloatConversionError => write!(f, "Error converting IBM float"),
            SegyError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for SegyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SegyError::IoError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SegyError {
    fn from(err: std::io::Error) -> Self {
        SegyError::IoError(err)
    }
}
