use std::io::{self, Read, Seek};
 use ebcdic::ebcdic::Ebcdic;
use crate::{
    ebcdic_syg::{BinaryHeader, EbcdicHeader, Trace, TraceHeader},
    errors::SegyError,
    format::SampleFormat,
    utils::{ibm_to_ieee_f32, read_i16_be, read_i32_be, read_u16_be},
    value::{BINARY_HEADER_SIZE, EBCDIC_HEADER_SIZE, TRACE_HEADER_SIZE},
};

pub struct SegyReader<R: Read + Seek> {
  
    reader: R,
}


impl<R: Read + Seek> SegyReader<R> {

 fn is_probably_ascii(buffer: &[u8]) -> bool {
        let mut ascii_count = 0;
        let total = buffer.len();

        for &b in buffer {
            // Check if b is a standard ASCII printable character (0x20..=0x7E),
            // or a few allowed control chars like \n (0x0A), \r (0x0D), tab (0x09).
            if (b >= 0x20 && b <= 0x7E) || b == b'\n' || b == b'\r' || b == b'\t' {
                ascii_count += 1;
            }
        }

        // If more than 80% of the characters appear to be in the valid ASCII range,
        // we assume it is ASCII. Adjust threshold as needed.
        let ratio = ascii_count as f32 / total as f32;
        ratio > 0.80
    }
    /// Creates a new `SegyReader` from any `Read + Seek` source.
    pub fn new(reader: R) -> Self {
        SegyReader { reader }
    }

    /// Reads the 3200-byte EBCDIC textual header.
  
    pub fn read_ebcdic_header(&mut self) -> Result<EbcdicHeader, SegyError> {
        // 1. Read the raw 3200 bytes from the file into `buffer`.
        let mut buffer = vec![0u8; EBCDIC_HEADER_SIZE];
        self.reader.read_exact(&mut buffer)?;

        // 2. Prepare a second buffer for the ASCII output if needed.
        let mut ascii_buffer = vec![0u8; EBCDIC_HEADER_SIZE];

        // 3. Decide if it's likely ASCII or EBCDIC.
        //    If it's ASCII, we just copy the buffer; otherwise, we convert.
        if Self::is_probably_ascii(&buffer) {
            // Likely ASCII: copy the buffer directly.
            ascii_buffer.copy_from_slice(&buffer);
        } else {
            // Likely EBCDIC: convert to ASCII.
            Ebcdic::ebcdic_to_ascii(
                &buffer,             // src
                &mut ascii_buffer,   // dest
                EBCDIC_HEADER_SIZE,  // number of bytes to convert
                true,                // non_printable_to_space
                true,                // nel_to_lf
            );
        }

        // 4. Convert that ASCII buffer into a Rust String (lossy to avoid errors
        //    with unexpected byte values).
        let text = String::from_utf8_lossy(&ascii_buffer).into_owned();

        // 5. Return the EbcdicHeader struct with the final text.
        Ok(EbcdicHeader { text })
    }

    /// Reads the 400-byte binary header.
    /// Only certain fields are parsed as an example; you can parse more if you wish.
    pub(crate) fn read_binary_header(&mut self) -> Result<BinaryHeader, SegyError> {
        let mut buffer = vec![0u8; BINARY_HEADER_SIZE];
        self.reader.read_exact(&mut buffer)?;

       

        let job_id = read_i32_be(&buffer, 0)?; // bytes 3201-3204 -> offset 0
        let line_number = read_i32_be(&buffer, 4)?; // bytes 3205-3208 -> offset 4
        let reel_number = read_i32_be(&buffer, 8)?; // bytes 3209-3212 -> offset 8

        let sample_interval_us = read_u16_be(&buffer, 16)?; // bytes 3217-3218 -> offset 16
        let samples_per_trace = read_u16_be(&buffer, 20)?; // bytes 3221-3222 -> offset 20
        let format_code_raw = read_u16_be(&buffer, 24)?; // bytes 3225-3226 -> offset 24
        let sample_format_code = SampleFormat::from_code(format_code_raw)?;

        Ok(BinaryHeader {
            job_id,
            line_number,
            reel_number,
            sample_format_code,
            samples_per_trace,
            sample_interval_us,
        })
    }

    /// Reads one SEG-Y trace (header + data).
    pub fn read_trace(
        &mut self,
        sample_format: SampleFormat,
        default_samples_per_trace: u16,
    ) -> Result<Trace, SegyError> {
        let header = self.read_trace_header()?;
        let samples_in_trace = if header.trace_sample_count == 0 {
            default_samples_per_trace
        } else {
            header.trace_sample_count
        };

        let data_samples = self.read_trace_data(sample_format, samples_in_trace)?;
        Ok(Trace {
            header,
            data_samples,
        })
    }

    /// Reads the fixed 240-byte trace header, parsing key fields into a `TraceHeader` struct.
    pub fn read_trace_header(&mut self) -> Result<TraceHeader, SegyError> {
        let mut buffer = vec![0u8; TRACE_HEADER_SIZE];
        self.reader.read_exact(&mut buffer)?;

        // Original fields you had:
        let trace_sequence_line = read_i32_be(&buffer, 0)?; // bytes 1-4
        let field_record_number = read_i32_be(&buffer, 8)?; // bytes 9-12
        let trace_number = read_i32_be(&buffer, 12)?; // bytes 13-16
        let source_point_number = read_i32_be(&buffer, 16)?; // bytes 17-20
        let trace_sample_count = read_u16_be(&buffer, 114)?; // bytes 115-116
        let trace_sample_interval_us = read_u16_be(&buffer, 116)?; // bytes 117-118

        // Additional fields:
        let trace_sequence_file = read_i32_be(&buffer, 4)?; // bytes 5-8
        let ensemble_number = read_i32_be(&buffer, 20)?; // bytes 21-24
        let trace_in_ensemble = read_i32_be(&buffer, 24)?; // bytes 25-28
        let offset = read_i32_be(&buffer, 36)?; // bytes 37-40
        let coord_scalar = read_i16_be(&buffer, 70)?; // bytes 71-72
        let cdp_x = read_i32_be(&buffer, 72)?; // bytes 73-76
        let cdp_y = read_i32_be(&buffer, 76)?; // bytes 77-80
        let year_data_recorded = read_u16_be(&buffer, 158)?; // bytes 159-160
        let day_of_year = read_u16_be(&buffer, 160)?; // bytes 161-162
        let hour_of_day = read_u16_be(&buffer, 162)?; // bytes 163-164
        let minute_of_hour = read_u16_be(&buffer, 164)?; // bytes 165-166
        let second_of_minute = read_u16_be(&buffer, 166)?; // bytes 167-168

        Ok(TraceHeader {
            trace_sequence_line,
            field_record_number,
            trace_number,
            source_point_number,
            trace_sample_count,
            trace_sample_interval_us,
            trace_sequence_file,
            ensemble_number,
            trace_in_ensemble,
            offset,
            cdp_x,
            cdp_y,
            coord_scalar,
            year_data_recorded,
            day_of_year,
            hour_of_day,
            minute_of_hour,
            second_of_minute,
        })
    }

    /// Reads the trace sample data, converting to `f32` as needed.
    fn read_trace_data(
        &mut self,
        sample_format: SampleFormat,
        samples_in_trace: u16,
    ) -> Result<Vec<f32>, SegyError> {
        let sample_count = samples_in_trace as usize;
        let sample_size = match sample_format {
            SampleFormat::IbmFloat => 4,
            SampleFormat::Int32 => 4,
            SampleFormat::Int16 => 2,
            SampleFormat::IeeeFloat => 4,
            SampleFormat::Int8 => 1,
        };

        let mut buffer = vec![0u8; sample_count * sample_size];
        self.reader.read_exact(&mut buffer)?;

        let mut data_samples = Vec::with_capacity(sample_count);
        match sample_format {
            SampleFormat::IbmFloat => {
                for chunk in buffer.chunks_exact(4) {
                    let val = ibm_to_ieee_f32(chunk).ok_or(SegyError::IbmFloatConversionError)?;
                    data_samples.push(val);
                }
            }
            SampleFormat::Int32 => {
                for chunk in buffer.chunks_exact(4) {
                    let val = i32::from_be_bytes(chunk.try_into().unwrap()) as f32;
                    data_samples.push(val);
                }
            }
            SampleFormat::Int16 => {
                for chunk in buffer.chunks_exact(2) {
                    let val = i16::from_be_bytes(chunk.try_into().unwrap()) as f32;
                    data_samples.push(val);
                }
            }
            SampleFormat::IeeeFloat => {
                for chunk in buffer.chunks_exact(4) {
                    let val = f32::from_be_bytes(chunk.try_into().unwrap());
                    data_samples.push(val);
                }
            }
            SampleFormat::Int8 => {
                for &byte in buffer.iter() {
                    let val = byte as i8 as f32;
                    data_samples.push(val);
                }
            }
        }
        Ok(data_samples)
    }

    /// Reads all traces from the current file position until EOF.
    pub fn read_all_traces(
        &mut self,
        binary_header: &BinaryHeader,
    ) -> Result<Vec<Trace>, SegyError> {
        let mut traces = Vec::new();
        loop {
            // try to read the next 240-byte trace header.
            let mut header_buffer = [0u8; TRACE_HEADER_SIZE];
            match self.reader.read_exact(&mut header_buffer) {
                Ok(_) => {
                    // We read a full 240 bytes, so parse that.
                    let trace_header = {
                     
                        let trace_sequence_line = read_i32_be(&header_buffer, 0)?;
                        let field_record_number = read_i32_be(&header_buffer, 8)?;
                        let trace_number = read_i32_be(&header_buffer, 12)?;
                        let source_point_number = read_i32_be(&header_buffer, 16)?;
                        let trace_sample_count = read_u16_be(&header_buffer, 114)?;
                        let trace_sample_interval_us = read_u16_be(&header_buffer, 116)?;

                        let trace_sequence_file = read_i32_be(&header_buffer, 4)?;
                        let ensemble_number = read_i32_be(&header_buffer, 20)?;
                        let trace_in_ensemble = read_i32_be(&header_buffer, 24)?;
                        let offset = read_i32_be(&header_buffer, 36)?;
                        let coord_scalar = read_i16_be(&header_buffer, 70)?;
                        let cdp_x = read_i32_be(&header_buffer, 72)?;
                        let cdp_y = read_i32_be(&header_buffer, 76)?;
                        let year_data_recorded = read_u16_be(&header_buffer, 158)?;
                        let day_of_year = read_u16_be(&header_buffer, 160)?;
                        let hour_of_day = read_u16_be(&header_buffer, 162)?;
                        let minute_of_hour = read_u16_be(&header_buffer, 164)?;
                        let second_of_minute = read_u16_be(&header_buffer, 166)?;

                        TraceHeader {
                            trace_sequence_line,
                            field_record_number,
                            trace_number,
                            source_point_number,
                            trace_sample_count,
                            trace_sample_interval_us,
                            trace_sequence_file,
                            ensemble_number,
                            trace_in_ensemble,
                            offset,
                            cdp_x,
                            cdp_y,
                            coord_scalar,
                            year_data_recorded,
                            day_of_year,
                            hour_of_day,
                            minute_of_hour,
                            second_of_minute,
                        }
                    };

                    let samples_in_trace = if trace_header.trace_sample_count == 0 {
                        binary_header.samples_per_trace
                    } else {
                        trace_header.trace_sample_count
                    };
                    let sample_size = binary_header.sample_format_code.sample_size();
                    let mut buffer = vec![0u8; samples_in_trace as usize * sample_size];
                    self.reader.read_exact(&mut buffer)?;

                    // Convert the trace samples to f32, as in read_trace_data.
                    let mut data_samples = Vec::with_capacity(samples_in_trace as usize);
                    match binary_header.sample_format_code {
                        SampleFormat::IbmFloat => {
                            for chunk in buffer.chunks_exact(4) {
                                let val = ibm_to_ieee_f32(chunk)
                                    .ok_or(SegyError::IbmFloatConversionError)?;
                                data_samples.push(val);
                            }
                        }
                        SampleFormat::Int32 => {
                            for chunk in buffer.chunks_exact(4) {
                                let val = i32::from_be_bytes(chunk.try_into().unwrap()) as f32;
                                data_samples.push(val);
                            }
                        }
                        SampleFormat::Int16 => {
                            for chunk in buffer.chunks_exact(2) {
                                let val = i16::from_be_bytes(chunk.try_into().unwrap()) as f32;
                                data_samples.push(val);
                            }
                        }
                        SampleFormat::IeeeFloat => {
                            for chunk in buffer.chunks_exact(4) {
                                let val = f32::from_be_bytes(chunk.try_into().unwrap());
                                data_samples.push(val);
                            }
                        }
                        SampleFormat::Int8 => {
                            for &byte in buffer.iter() {
                                data_samples.push(byte as i8 as f32);
                            }
                        }
                    }

                    traces.push(Trace {
                        header: trace_header,
                        data_samples,
                    });
                }
                Err(e) => {
                    // If we can't read exactly 240 bytes, we assume EOF (or partial file).
                    if e.kind() != io::ErrorKind::UnexpectedEof {
                        return Err(SegyError::IoError(e));
                    }
                    // Graceful EOF
                    break;
                }
            }
        }
        Ok(traces)
    }
}
