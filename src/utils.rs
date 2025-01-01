use crate::errors::SegyError;

pub fn read_i32_be(buffer: &[u8], offset: usize) -> Result<i32, SegyError> {
    let end = offset + 4;
    if end > buffer.len() {
        return Err(SegyError::ParseError(
            "Not enough bytes to read i32".to_string(),
        ));
    }
    Ok(i32::from_be_bytes(buffer[offset..end].try_into().unwrap()))
}

/// Read a big-endian `u16` from the given buffer at the specified offset.
pub fn read_u16_be(buffer: &[u8], offset: usize) -> Result<u16, SegyError> {
    let end = offset + 2;
    if end > buffer.len() {
        return Err(SegyError::ParseError(
            "Not enough bytes to read u16".to_string(),
        ));
    }
    Ok(u16::from_be_bytes(buffer[offset..end].try_into().unwrap()))
}
pub fn read_i16_be(buffer: &[u8], offset: usize) -> Result<i16, SegyError> {
    let end = offset + 2;
    if end > buffer.len() {
        return Err(SegyError::ParseError(
            "Not enough bytes to read i16".to_string(),
        ));
    }
    Ok(i16::from_be_bytes(buffer[offset..end].try_into().unwrap()))
}


pub fn ibm_to_ieee_f32(bytes: &[u8]) -> Option<f32> {
    if bytes.len() != 4 {
        return None;
    }
    let mut fraction: u32 = (bytes[1] as u32) << 16 | (bytes[2] as u32) << 8 | (bytes[3] as u32);

    if fraction == 0 {
        return Some(0.0);
    }

    let mut exponent = (bytes[0] & 0x7F) as i32;
    let sign = (bytes[0] & 0x80) != 0;

    // Adjust exponent from base-16 to base-2:
    exponent -= 64; // top bit used as sign
    exponent *= 4;

    while (fraction & 0x0080_0000) == 0 {
        fraction <<= 1;
        exponent -= 1;
    }

    fraction &= 0x007F_FFFF;
    let sign_bit = (sign as u32) << 31;
    let exponent_bits = ((exponent + 127) as u32) << 23;
    Some(f32::from_bits(sign_bit | exponent_bits | fraction))
}
