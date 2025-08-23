pub mod digit;
pub mod src_code;
pub mod x86_64;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type utf32 = char;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type ascii = u8;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset32 = u32;

#[expect(non_camel_case_types, reason = "alias to a primitive type")]
pub type offset64 = u64;

macro_rules! test_assert {
    ($expression:expr, == $pattern:pat $(if $guard:expr)? $(,)?) => {
        const _: () = match $expression {
            $pattern $(if $guard)? => {},
            _ => panic!(),
        };
    };
    ($expression:expr, != $pattern:pat $(if $guard:expr)? $(,)?) => {
        const _: () = match $expression {
            $pattern $(if $guard)? => panic!(),
            _ => {},
        };
    };
}
pub(crate) use test_assert;
