pub mod cli;
pub mod digit;
pub mod src_code;
pub mod x86_64;

#[expect(non_camel_case_types, reason = "alias to primitive types")]
mod __primitives {
    pub type utf32 = char;
    pub type ascii = u8;

    pub type uoffset8 = u8;
    pub type uoffset16 = u16;
    pub type uoffset32 = u32;
    pub type uoffset64 = u64;
    pub type uoffset = usize;

    #[deprecated(since = "0.1.1", note = "will be renamed to `uoffset8`")]
    pub type offset8 = uoffset8;
    #[deprecated(since = "0.1.1", note = "will be renamed to `uoffset16`")]
    pub type offset16 = uoffset16;
    #[deprecated(since = "0.1.1", note = "will be renamed to `uoffset32`")]
    pub type offset32 = uoffset32;
    #[deprecated(since = "0.1.1", note = "will be renamed to `uoffset64`")]
    pub type offset64 = uoffset64;
    #[deprecated(since = "0.1.1", note = "will be renamed to `uoffset`")]
    pub type offset = uoffset;

    pub type ioffset8 = i8;
    pub type ioffset16 = i16;
    pub type ioffset32 = i32;
    pub type ioffset64 = i64;
    pub type ioffset = isize;
}

#[allow(clippy::useless_attribute, reason = "false positive")]
#[allow(clippy::pub_use)]
pub use __primitives::*;

#[allow(unused_macros)]
macro_rules! test_assert {
    ($expression:expr, == $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => {},
            _ => panic!(),
        }
    };
    ($expression:expr, != $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => panic!(),
            _ => {},
        }
    };
}
#[allow(unused_imports)]
pub(crate) use test_assert;
