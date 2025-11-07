pub mod cli;
pub mod digit;
pub mod src_code;
pub mod x86_64;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type utf32 = char;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ascii = u8;

#[deprecated(since = "0.1.1", note = "will be renamed to `uoffset8`")]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset8 = u8;
#[expect(deprecated)]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type uoffset8 = offset8;

#[deprecated(since = "0.1.1", note = "will be renamed to `uoffset16`")]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset16 = u16;
#[expect(deprecated)]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type uoffset16 = offset16;

#[deprecated(since = "0.1.1", note = "will be renamed to `uoffset32`")]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset32 = u32;
#[expect(deprecated)]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type uoffset32 = offset32;

#[deprecated(since = "0.1.1", note = "will be renamed to `uoffset64`")]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset64 = u64;
#[expect(deprecated)]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type uoffset64 = offset64;

#[deprecated(since = "0.1.1", note = "will be renamed to `uoffset`")]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset = usize;
#[expect(deprecated)]
#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type uoffset = offset;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ioffset8 = i8;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ioffset16 = i16;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ioffset32 = i32;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ioffset64 = i64;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ioffset = isize;

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
