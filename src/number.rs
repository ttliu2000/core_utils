use crate::core_utils_error::CoreUtilsError;

pub fn sign<T: PartialOrd + From<i8>>(x: T) -> T {
    if x < T::from(0) {
        T::from(-1)
    } else if x > T::from(0) {
        T::from(1)
    } else {
        T::from(0)
    }
}

pub fn round_up_to_16(x: usize) -> usize {
    (x + 15) & !15
}

pub fn u8_array_to_u32(data: &[u8]) -> Result<u32, &'static str> {
    if data.len() != 4 {
        return Err("Input slice must have exactly 4 elements");
    }

    let result = ((data[0] as u32) << 24)
    | ((data[1] as u32) << 16)
    | ((data[2] as u32) << 8)
    | (data[3] as u32);

    Ok(result)
}

pub fn u8_array_to_u32_little_endian(data: &[u8]) -> Result<u32, &'static str> {
    if data.len() != 4 {
        return Err("Input slice must have exactly 4 elements");
    }

    let result = ((data[0] as u32) << 0)
    | ((data[1] as u32) << 8)
    | ((data[2] as u32) << 16)
    | ((data[3] as u32) << 24);

    Ok(result)
}

pub fn u32_to_base26(mut number: u32) -> String {
    if number == 0 {
        return "A".to_string();
    }

    let mut result = String::new();

    while number > 0 {
        let remainder = (number % 26) as u8;
        let ch = (b'A' + remainder) as char;
        result.push(ch);
        number /= 26;
    }

    result.chars().rev().collect()
}

pub fn get_u64_from_str(imm:&str) -> Result<u64, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = u64::from_str_radix(&hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u64")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = u64::from_str_radix(&bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u64")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = u64::from_str_radix(&oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u64")))?;
        Ok(i)
    }
    else if imm.starts_with("-"){
        let i = i64::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64 then to u64")))?;
        Ok(i as u64)
    }
    else {
        let i = u64::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u64")))?;
        Ok(i)
    }
}

pub fn get_i64_from_str(imm:&str) -> Result<i64, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = i64::from_str_radix(&hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = i64::from_str_radix(&bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = i64::from_str_radix(&oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64")))?;
        Ok(i)
    }
    else if imm.starts_with("-"){
        let i = i64::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64")))?;
        Ok(i)
    }
    else if imm.starts_with("'") && imm.ends_with("'") && imm.len() == 3 {  // convert char to i64
        let chars: Vec<char> = imm.chars().collect();
        Ok(chars[1] as i64)
    }
    else {
        let i = i64::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i64")))?;
        Ok(i)
    }
}

pub fn get_u32_from_str(imm:&str) -> Result<u32, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = u32::from_str_radix(hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u32")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = u32::from_str_radix(bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u32")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = u32::from_str_radix(oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u32")))?;
        Ok(i)
    }
    else if imm.starts_with("-") {
        let i = i32::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32 then to u32")))?;
        Ok(i as u32)
    }
    else {
        let i = u32::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u32")))?;
        Ok(i)
    }
}

pub fn get_i32_from_str(imm:&str) -> Result<i32, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = i32::from_str_radix(hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = i32::from_str_radix(bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = i32::from_str_radix(oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32")))?;
        Ok(i)
    }
    else if imm.starts_with("-") {
        let i = i32::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32")))?;
        Ok(i)
    }
    else {
        let i = i32::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i32")))?;
        Ok(i)
    }
}

pub fn get_u16_from_str(imm:&str) -> Result<u16, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = u16::from_str_radix(hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u16")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = u16::from_str_radix(bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u16")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = u16::from_str_radix(oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u16")))?;
        Ok(i)
    }
    else if imm.starts_with("-"){
        let i = i16::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i16 then to u16")))?;
        Ok(i as u16)
    }
    else {
        let i = u16::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u16")))?;
        Ok(i)
    }
}

pub fn get_u8_from_str(imm:&str) -> Result<u8, CoreUtilsError> {
    let imm = imm.replace("_", "");
    if imm.starts_with("0x") {
        let hex_trimmed = imm.trim_start_matches("0x");
        let i = u8::from_str_radix(hex_trimmed, 16)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u8")))?;
        Ok(i)
    }
    else if imm.starts_with("0b") {
        let bin_trimmed = imm.trim_start_matches("0b");
        let i = u8::from_str_radix(bin_trimmed, 2)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u8")))?;
        Ok(i)
    }
    else if imm.starts_with("0o") {
        let oct_trimmed = imm.trim_start_matches("0o");
        let i = u8::from_str_radix(oct_trimmed, 8)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u8")))?;
        Ok(i)
    }
    else if imm.starts_with("-"){
        let i = i8::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to i8 then to u8")))?;
        Ok(i as u8)
    }
    else {
        let i = u8::from_str_radix(&imm, 10)
            .map_err(|_| CoreUtilsError::ConversionError(format!("cannot convert imm {imm} to u8")))?;
        Ok(i)
    }
}