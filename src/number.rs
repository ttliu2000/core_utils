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
