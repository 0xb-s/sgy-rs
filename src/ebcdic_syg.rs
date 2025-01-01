use crate::format::SampleFormat;

/// Should support 40 lines of 80 characters.

#[derive(Debug, Clone)]
pub struct EbcdicHeader {
    /// The raw textual header (3200 characters).
    pub text: String,
}
///information from the SEG-Y binary header.
#[derive(Debug, Clone)]
pub struct BinaryHeader {
    /// Job identification number (bytes 3201-3204).
    pub job_id: i32,
    /// Line number (bytes 3205-3208).
    pub line_number: i32,
    /// Reel number (bytes 3209-3212).
    pub reel_number: i32,
    /// Data sample format code (bytes 3225-3226).
    pub sample_format_code: SampleFormat,
    /// Number of samples per trace (bytes 3221-3222).
    pub samples_per_trace: u16,
    /// Sample interval in microseconds (bytes 3217-3218).
    pub sample_interval_us: u16,
}

/// Trace header.

#[derive(Debug, Clone)]
pub struct TraceHeader {
    /// Trace sequence number within line (bytes 1-4).
    pub trace_sequence_line: i32,
    /// Original field record number (bytes 9-12).
    pub field_record_number: i32,
    /// Trace number within original field record (bytes 13-16).
    pub trace_number: i32,
    /// Energy source point number (bytes 17-20).
    pub source_point_number: i32,
    /// Number of samples in this trace (bytes 115-116).
    pub trace_sample_count: u16,
    /// Sample interval in microseconds (bytes 117-118).
    pub trace_sample_interval_us: u16,
    // TODO 240-byte trace header
    pub trace_sequence_file: i32,
    /// Ensemble (or "CDP") number (bytes 21-24).
    pub ensemble_number: i32,
    /// Trace number within the ensemble (bytes 25-28).
    pub trace_in_ensemble: i32,
    /// Distance from source point (offset) in meters/feet (bytes 37-40).
    pub offset: i32,
    /// CDP X coordinate (bytes 73-76) — usage can vary by convention.
    pub cdp_x: i32,
    /// CDP Y coordinate (bytes 77-80) — usage can vary by convention.
    pub cdp_y: i32,
    /// Coordinate scalar (bytes 71-72) — apply to cdp_x & cdp_y if nonzero.
    pub coord_scalar: i16,
    /// Year data recorded (bytes 159-160) — optional field in SEG-Y.
    pub year_data_recorded: u16,
    /// Day of year (bytes 161-162).
    pub day_of_year: u16,
    /// Hour of day (bytes 163-164).
    pub hour_of_day: u16,
    /// Minute of hour (bytes 165-166).
    pub minute_of_hour: u16,
    /// Second of minute (bytes 167-168).
    pub second_of_minute: u16,
}

///SEG-Y file,

#[derive(Debug, Clone)]
pub struct Trace {
    /// The parsed trace header.
    pub header: TraceHeader,
    /// The parsed data samples, stored as `f32`.
    pub data_samples: Vec<f32>,
}
