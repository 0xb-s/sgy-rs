use crate::ebcdic_syg::{BinaryHeader, Trace};

/// A structure representing the contents of the entire SEG-Y file.
#[derive(Debug)]
pub struct SegyFile {
    /// The textual EBCDIC header stored in a single string.
    pub ebcdic_header: String,
    /// The parsed binary header containing essential data fields.
    pub binary_header: BinaryHeader,
    /// A list of all traces found in the file, each with a header and data samples.
    pub traces: Vec<Trace>,
}
