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
