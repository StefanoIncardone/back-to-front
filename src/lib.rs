pub mod x86_64;
pub mod digit;
pub mod src_code;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type utf32 = char;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ascii = u8;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset32 = u32;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset64 = u64;
