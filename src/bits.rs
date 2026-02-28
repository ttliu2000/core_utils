use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BitFieldError {
    OutOfBounds { attempted: usize, max: usize },
    InvalidRange { start: usize, end: usize },
    ValueTooLarge { value: u64, max: u64 },
}

impl fmt::Display for BitFieldError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitFieldError::OutOfBounds { attempted, max } => {
                write!(
                    f,
                    "Bit range is out of bounds: attempted {}, max {}",
                    attempted, max
                )
            }
            BitFieldError::InvalidRange { start, end } => {
                write!(
                    f,
                    "Invalid range: start {} must be less than or equal to end {}",
                    start, end
                )
            }
            BitFieldError::ValueTooLarge { value, max } => {
                write!(
                    f,
                    "Provided value {} exceeds the maximum allowed value {}",
                    value, max
                )
            }
        }
    }
}

impl Error for BitFieldError {}

pub struct BitField {
    data: Vec<u8>,
}

impl BitField {
    pub fn new(size_in_bytes: usize) -> Self {
        Self {
            data: vec![0; size_in_bytes],
        }
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn total_bits(&self) -> usize {
        self.data.len() * 8
    }

    pub fn set_bits(
        &mut self,
        start_bit: usize,
        end_bit: usize,
        value: u64,
    ) -> Result<(), BitFieldError> {
        if end_bit < start_bit {
            return Err(BitFieldError::InvalidRange {
                start: start_bit,
                end: end_bit,
            });
        }
        if end_bit >= self.total_bits() {
            return Err(BitFieldError::OutOfBounds {
                attempted: end_bit,
                max: self.total_bits() - 1,
            });
        }

        let num_bits = end_bit - start_bit + 1;
        if num_bits < 64 && value >= (1u64 << num_bits) {
            return Err(BitFieldError::ValueTooLarge {
                value,
                max: (1u64 << num_bits) - 1,
            });
        }

        for i in 0..num_bits {
            let bit_pos = start_bit + i;
            let byte_index = bit_pos / 8;
            let bit_in_byte = bit_pos % 8;

            let bit_to_set = (value >> i) & 1;

            if bit_to_set == 1 {
                self.data[byte_index] |= 1 << bit_in_byte;
            } else {
                self.data[byte_index] &= !(1 << bit_in_byte);
            }
        }

        Ok(())
    }

    pub fn get_bits(&self, start_bit: usize, end_bit: usize) -> Result<u64, BitFieldError> {
        if end_bit < start_bit {
            return Err(BitFieldError::InvalidRange {
                start: start_bit,
                end: end_bit,
            });
        }
        if end_bit >= self.total_bits() {
            return Err(BitFieldError::OutOfBounds {
                attempted: end_bit,
                max: self.total_bits() - 1,
            });
        }

        let num_bits = end_bit - start_bit + 1;
        if num_bits > 64 {
            return Err(BitFieldError::ValueTooLarge { value: 0, max: 0 });
        }

        let mut result: u64 = 0;

        for i in 0..num_bits {
            let bit_pos = start_bit + i;
            let byte_index = bit_pos / 8;
            let bit_in_byte = bit_pos % 8;

            let bit = (self.data[byte_index] >> bit_in_byte) & 1;

            result |= (bit as u64) << i;
        }

        Ok(result)
    }

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_bits() {
        let mut field = BitField::new(4);
        field.set_bits(12, 15, 10).unwrap();

        assert_eq!(field.get_bits(12, 15).unwrap(), 10);
        assert_eq!(field.get_bits(8, 11).unwrap(), 0);
        assert_eq!(field.get_bits(16, 19).unwrap(), 0);

        field.set_bits(4, 11, 202).unwrap();
        assert_eq!(field.get_bits(4, 11).unwrap(), 202);

        let bytes = field.as_bytes();
        assert_eq!(bytes[0], 0b00001100);
        assert_eq!(bytes[1], 0b10101010);
    }

    #[test]
    fn test_error_handling() {
        let mut field = BitField::new(2);

        assert!(matches!(
            field.set_bits(16, 18, 1),
            Err(BitFieldError::OutOfBounds { .. })
        ));
        assert!(matches!(
            field.get_bits(10, 9),
            Err(BitFieldError::InvalidRange { .. })
        ));
        assert!(matches!(
            field.set_bits(0, 3, 16),
            Err(BitFieldError::ValueTooLarge { .. })
        ));
    }
}
