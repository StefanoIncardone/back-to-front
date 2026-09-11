pub mod cli;
pub mod digit;
pub mod src_code;
pub mod x86_64;

#[expect(non_camel_case_types, clippy::inline_modules, reason = "alias to primitive types")]
mod __primitives {
    pub type utf32 = char;
    pub type ascii = u8;

    pub type usize8 = u8;
    pub type usize16 = u16;
    pub type usize32 = u32;
    pub type usize64 = u64;

    pub type isize8 = i8;
    pub type isize16 = i16;
    pub type isize32 = i32;
    pub type isize64 = i64;
}

#[expect(clippy::useless_attribute, reason = "false positive")]
#[expect(clippy::pub_use)]
pub use __primitives::*;
