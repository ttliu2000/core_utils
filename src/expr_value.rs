use crate::debug::*;
use crate::core_utils_error::CoreUtilsError;
use rust_macro::*;

pub type CLRObjectComputeResult = Result<ExprValue, CoreUtilsError>;

#[macro_export]
macro_rules! create_clr_obj_return_value_fn {
    ($fn_name:ident, $type_name:ty, $instance:ident) => {
        pub fn $fn_name(&self) -> Result<$type_name, CoreUtilsError> {
            match self {
                Self::$instance(m) => Ok(m.clone()),
                _ => {
                    error_string(format!("cannot get value '{self:?}' in conversion function"));
                    Err(CoreUtilsError::CannotRetrieveValue)
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_from_rust_value_fn {
    ($fname2:ident, $fname:ident, $ty:ident) => {
        pub fn $fname2(v:$ty) -> Self {
            Self::$fname(v)
        }
    }
}

#[macro_export]
macro_rules! create_clr_number_op_fn {
    ($fn_name:ident, $op:tt) => {
        pub fn $fn_name(&self, b:&ExprValue) -> CLRObjectComputeResult {
            if !self.shallow_same_type_as(b) {
                return Err(CoreUtilsError::IncompatibleType);
            }
            else {
                match self {
                    Self::Int8(m) => Ok(Self::Int8(m $op b.i8()?)),
                    Self::Int16(m) => Ok(Self::Int16(m $op b.i16()?)),
                    Self::Int32(m) => Ok(Self::Int32(m $op b.i32()?)),
                    Self::Int64(m) => Ok(Self::Int64(m $op b.i64()?)),
                    Self::Int128(m) => Ok(Self::Int128(m $op b.i128()?)),
                    Self::UInt8(m) => Ok(Self::UInt8(m $op b.u8()?)),
                    Self::UInt16(m) => Ok(Self::UInt16(m $op b.u16()?)),
                    Self::UInt32(m) => Ok(Self::UInt32(m $op b.u32()?)),
                    Self::UInt64(m) => Ok(Self::UInt64(m $op b.u64()?)),
                    Self::UInt128(m) => Ok(Self::UInt128(m $op b.u128()?)),
                    Self::Float32(m) => Ok(Self::Float32(m $op b.f32()?)),
                    Self::Float64(m) => Ok(Self::Float64(m $op b.f64()?)),
                    _ => Err(CoreUtilsError::IncompatibleType)
                }
            }
        }                
    };
}

#[macro_export]
macro_rules! create_clr_convert_op_fn {
    ($fn_name:ident, $ty:ident, $op:tt, $as_only:ident) => {
        pub fn $fn_name(&self) -> CLRObjectComputeResult {
            match self {
                Self::Int8(m) => Ok(Self::$ty(*m as $op)),
                Self::Int16(m) => Ok(Self::$ty(*m as $op)),
                Self::Int32(m) => Ok(Self::$ty(*m as $op)),
                Self::Int64(m) => Ok(Self::$ty(*m as $op)),
                Self::Int128(m) => Ok(Self::$ty(*m as $op)),
                Self::UInt8(m) => Ok(Self::$ty(*m as $op)),
                Self::UInt16(m) => Ok(Self::$ty(*m as $op)),
                Self::UInt32(m) => Ok(Self::$ty(*m as $op)),
                Self::UInt64(m) => Ok(Self::$ty(*m as $op)),
                Self::UInt128(m) => Ok(Self::$ty(*m as $op)),
                Self::Float32(m) => Ok(Self::$ty(if $as_only { *m as $op } else { m.to_bits() as $op } )),
                Self::Float64(m) => Ok(Self::$ty(if $as_only { *m as $op } else { m.to_bits() as $op })),
                _ => Err(CoreUtilsError::IncompatibleType)
            }
        }                      
    };
}

#[macro_export]
macro_rules! create_clr_bit_logic_op_fn {
    ($fn_name:ident, $op:tt) => {
        pub fn $fn_name(&self, b:&ExprValue) -> CLRObjectComputeResult {

            // it used to have a type size check, because we have conv_xxx function, so check type size can be removed
                                   
            match self {
                Self::Int8(m) => Ok(Self::Int8(m $op b.conv_i8()?.i8()?)),
                Self::Int16(m) => Ok(Self::Int16(m $op b.conv_i16()?.i16()?)),
                Self::Int32(m) => Ok(Self::Int32(m $op b.conv_i32()?.i32()?)),
                Self::Int64(m) => Ok(Self::Int64(m $op b.conv_i64()?.i64()?)),
                Self::Int128(m) => Ok(Self::Int128(m $op b.conv_i32()?.i32()? as i128)),
                Self::UInt8(m) => Ok(Self::UInt8(m $op b.conv_u8()?.u8()?)),
                Self::UInt16(m) => Ok(Self::UInt16(m $op b.conv_u16()?.u16()?)),
                Self::UInt32(m) => Ok(Self::UInt32(m $op b.conv_u32()?.u32()?)),
                Self::UInt64(m) => Ok(Self::UInt64(m $op b.conv_u64()?.u64()?)),
                Self::UInt128(m) => Ok(Self::UInt128(m $op b.conv_u32()?.u32()? as u128)),
                Self::Float32(_) => Err(CoreUtilsError::InvalidOperation),
                Self::Float64(_) => Err(CoreUtilsError::InvalidOperation),
                _ => Err(CoreUtilsError::IncompatibleType)
            }            
        }                      
    };
}

#[derive(Debug, GenIsEnumVariant)]
pub enum ExprValue {
    Invalid,
    None,

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),

    NativeInt(isize),
    NativeUInt(usize),

    Float32(f32),
    Float64(f64),

    Boolean(bool), 
}

impl ExprValue {
    create_clr_obj_return_value_fn!(i8, i8, Int8);
    create_clr_obj_return_value_fn!(i16, i16, Int16);
    create_clr_obj_return_value_fn!(i32, i32, Int32);
    create_clr_obj_return_value_fn!(i64, i64, Int64);
    create_clr_obj_return_value_fn!(i128, i128, Int128);

    create_clr_obj_return_value_fn!(u8, u8, UInt8);
    create_clr_obj_return_value_fn!(u16, u16, UInt16);
    create_clr_obj_return_value_fn!(u32, u32, UInt32);
    create_clr_obj_return_value_fn!(u64, u64, UInt64);
    create_clr_obj_return_value_fn!(u128, u128, UInt128);

    create_clr_obj_return_value_fn!(isize, isize, NativeInt);
    create_clr_obj_return_value_fn!(usize, usize, NativeUInt);

    create_clr_obj_return_value_fn!(f32, f32, Float32);
    create_clr_obj_return_value_fn!(f64, f64, Float64);
    create_clr_obj_return_value_fn!(bool, bool, Boolean);

    create_from_rust_value_fn!(from_i8, Int8, i8);
    create_from_rust_value_fn!(from_i16, Int16, i16);
    create_from_rust_value_fn!(from_i32, Int32, i32);
    create_from_rust_value_fn!(from_i64, Int64, i64);
    create_from_rust_value_fn!(from_u8, UInt8, u8);
    create_from_rust_value_fn!(from_u16, UInt16, u16);
    create_from_rust_value_fn!(from_u32, UInt32, u32);
    create_from_rust_value_fn!(from_u64, UInt64, u64);
    create_from_rust_value_fn!(from_f32, Float32, f32);
    create_from_rust_value_fn!(from_f64, Float64, f64); 
    create_from_rust_value_fn!(from_isize, NativeInt, isize);
    create_from_rust_value_fn!(from_usize, NativeUInt, usize);
    create_from_rust_value_fn!(from_bool, Boolean, bool);

    create_clr_number_op_fn!(add, +);
    create_clr_number_op_fn!(sub, -);
    create_clr_number_op_fn!(mul, *);
    create_clr_number_op_fn!(div, /);
    create_clr_number_op_fn!(rem, %);

    create_clr_convert_op_fn!(conv_i8, Int8, i8, false);
    create_clr_convert_op_fn!(conv_u8, UInt8, u8, false);
    create_clr_convert_op_fn!(conv_i16, Int16, i16, false);
    create_clr_convert_op_fn!(conv_u16, UInt16, u16, false);
    create_clr_convert_op_fn!(conv_i32, Int32, i32, false);
    create_clr_convert_op_fn!(conv_u32, UInt32, u32, false);
    create_clr_convert_op_fn!(conv_i64, Int64, i64, false);
    create_clr_convert_op_fn!(conv_u64, UInt64, u64, false);
    create_clr_convert_op_fn!(conv_i128, Int128, i128, false);
    create_clr_convert_op_fn!(conv_u128, UInt128, u128, false);
    create_clr_convert_op_fn!(conv_f32, Float32, f32, true);
    create_clr_convert_op_fn!(conv_f64, Float64, f64, true);

    create_clr_bit_logic_op_fn!(and, &);
    create_clr_bit_logic_op_fn!(or, |);
    create_clr_bit_logic_op_fn!(xor, ^);
    create_clr_bit_logic_op_fn!(shl, <<);
    create_clr_bit_logic_op_fn!(shr, >>);

    pub fn get_usize(&self) -> Result<usize, CoreUtilsError> {
        match self {
            Self::Int8(n) => Ok(*n as usize),
            Self::Int16(n) => Ok(*n as usize),
            Self::Int32(n) => Ok(*n as usize),
            Self::Int64(n) => Ok(*n as usize),
            Self::Int128(n) => Ok(*n as usize),
            Self::UInt8(n) => Ok(*n as usize),
            Self::UInt16(n) => Ok(*n as usize),
            Self::UInt32(n) => Ok(*n as usize),
            Self::UInt64(n) => Ok(*n as usize),
            Self::UInt128(n) => Ok(*n as usize),
            Self::NativeInt(n) => Ok(*n as usize),
            Self::NativeUInt(n) => Ok(*n),
            _ => Err(CoreUtilsError::InvalidOperation),
        }
    }

    pub fn get_isize(&self) -> Result<isize, CoreUtilsError> {
        match self {
            Self::Int8(n) => Ok(*n as isize),
            Self::Int16(n) => Ok(*n as isize),
            Self::Int32(n) => Ok(*n as isize),
            Self::Int64(n) => Ok(*n as isize),
            Self::Int128(n) => Ok(*n as isize),
            Self::UInt8(n) => Ok(*n as isize),
            Self::UInt16(n) => Ok(*n as isize),
            Self::UInt32(n) => Ok(*n as isize),
            Self::UInt64(n) => Ok(*n as isize),
            Self::UInt128(n) => Ok(*n as isize),
            Self::NativeInt(n) => Ok(*n),
            Self::NativeUInt(n) => Ok(*n as isize),
            _ => Err(CoreUtilsError::InvalidOperation),
        }
    }

    pub fn bool_from_string(v:&str) -> Result<Self, CoreUtilsError> {
        let r = crate::string::string_to_bool(v);
        Ok(Self::Boolean(r))
    }

    pub fn bool_and(&self, b:&Self) -> CLRObjectComputeResult {
        let m = self.bool()?;
        let n = b.bool()?;
        Ok(Self::from_bool(m && n))
    }

    pub fn bool_or(&self, b:&Self) -> CLRObjectComputeResult {
        let m = self.bool()?;
        let n = b.bool()?;
        Ok(Self::from_bool(m || n))
    }

    pub fn shallow_same_type_as(&self, b:&Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(b)
    }

    pub fn pow(&self, obj:&Self) -> CLRObjectComputeResult {
        match (self, obj) {
            (Self::Int8(a), _) => Ok(Self::Int8(a.pow(obj.conv_u32()?.u32()?))),
            (Self::Int16(a), _) => Ok(Self::Int16(a.pow(obj.conv_u32()?.u32()?))),
            (Self::Int32(a), _) => Ok(Self::Int32(a.pow(obj.conv_u32()?.u32()?))),
            (Self::Int64(a), _) => Ok(Self::Int64(a.pow(obj.conv_u32()?.u32()?))),
            (Self::UInt8(a), _) => Ok(Self::UInt8(a.pow(obj.conv_u32()?.u32()?))),
            (Self::UInt16(a), _) => Ok(Self::UInt16(a.pow(obj.conv_u32()?.u32()?))),
            (Self::UInt32(a), _) => Ok(Self::UInt32(a.pow(obj.u32()?))),
            (Self::UInt64(a), _) => Ok(Self::UInt64(a.pow(obj.conv_u32()?.u32()?))),
            (Self::Float32(a), _) => Ok(Self::Float32(a.powf(obj.conv_f32()?.f32()?))),
            (Self::Float64(a), _) => Ok(Self::Float64(a.powf(obj.conv_f64()?.f64()?))),
            _ => Err(CoreUtilsError::IncompatibleType)
        }
    }

    pub fn not(&self) -> CLRObjectComputeResult {
        match self {
            Self::Int8(m) => Ok(Self::Int8(!m)),
            Self::Int16(m) => Ok(Self::Int16(!m)),
            Self::Int32(m) => Ok(Self::Int32(!m)),
            Self::Int64(m) => Ok(Self::Int64(!m)),
            Self::Int128(m) => Ok(Self::Int128(!m)),
            Self::UInt8(m) => Ok(Self::UInt8(!m)),
            Self::UInt16(m) => Ok(Self::UInt16(!m)),
            Self::UInt32(m) => Ok(Self::UInt32(!m)),
            Self::UInt64(m) => Ok(Self::UInt64(!m)),
            Self::UInt128(m) => Ok(Self::UInt128(!m)),
            Self::NativeInt(m) => Ok(Self::NativeInt(!m)),
            Self::NativeUInt(m) => Ok(Self::NativeUInt(!m)),
            Self::Float32(_) => Err(CoreUtilsError::InvalidOperation),
            Self::Float64(_) => Err(CoreUtilsError::InvalidOperation),
            _ => Err(CoreUtilsError::IncompatibleType)
        }
    }

    pub fn neg(&self) -> CLRObjectComputeResult {
        match self {
            Self::Int8(m) => Ok(Self::Int8(-m)),
            Self::Int16(m) => Ok(Self::Int16(-m)),
            Self::Int32(m) => Ok(Self::Int32(-m)),
            Self::Int64(m) => Ok(Self::Int64(-m)),
            Self::Int128(m) => Ok(Self::Int128(-m)),
            Self::NativeInt(m) => Ok(Self::NativeInt(-m)),
            Self::UInt8(_) => Err(CoreUtilsError::InvalidOperation),
            Self::UInt16(_) => Err(CoreUtilsError::InvalidOperation),
            Self::UInt32(_) => Err(CoreUtilsError::InvalidOperation),
            Self::UInt64(_) => Err(CoreUtilsError::InvalidOperation),
            Self::UInt128(_) => Err(CoreUtilsError::InvalidOperation),
            Self::NativeUInt(_) => Err(CoreUtilsError::InvalidOperation),
            Self::Float32(m) => Ok(Self::Float32(-m)),
            Self::Float64(m) => Ok(Self::Float64(-m)),
            _ => Err(CoreUtilsError::IncompatibleType)
        }
    }
}