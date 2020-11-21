#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod c_types {
    pub enum c_void {}
    pub type c_uchar = u8;
    pub type c_schar = i8;
    pub type c_char = i8;
    pub type c_short = i16;
    pub type c_ushort = u16;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_longlong = i64;
    pub type c_ulonglong = u64;
}

#[derive(Debug)]
pub struct Error(pub c_types::c_uint);

impl Error {
    pub fn from(error: esp_err_t) -> Option<Error> {
        if error == 0 {
            None
        } else {
            Some(Error(error as c_types::c_uint))
        }
    }

    pub fn check_and_return<T>(error: esp_err_t, value: T) -> Result<T, Error> {
        if error == 0 {
            Ok(value)
        } else {
            Err(Error(error as c_types::c_uint))
        }
    }

    pub fn convert(error: esp_err_t) -> Result<(), Error> {
        Error::check_and_return(error, ())
    }

    pub fn check(error: esp_err_t) {
        if error != 0 {
            panic!("Got error code: {}", error);
        }
    }

    pub fn code(self: &Self) -> esp_err_t {
        self.0 as esp_err_t
    }
}

include!("bindings.rs");
