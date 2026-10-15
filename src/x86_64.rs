use core::fmt::Display;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum Reg64 {
    Rax = 0,
    Rbx = 1,
    Rcx = 2,
    Rdx = 3,

    Rsi = 4,
    Rdi = 5,
    Rbp = 6,
    Rsp = 7,

    R8  = 8,
    R9  = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Reg64 {
    #[must_use] #[inline]
    pub const fn to_str(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Rax => "rax",
            Self::Rbx => "rbx",
            Self::Rcx => "rcx",
            Self::Rdx => "rdx",

            Self::Rsi => "rsi",
            Self::Rdi => "rdi",
            Self::Rbp => "rbp",
            Self::Rsp => "rsp",

            Self::R8  => "r8",
            Self::R9  => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        };
    }
}

impl Display for Reg64 {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum Reg32 {
    Eax  = Reg64::Rax as u8,
    Ebx  = Reg64::Rbx as u8,
    Ecx  = Reg64::Rcx as u8,
    Edx  = Reg64::Rdx as u8,

    Esi  = Reg64::Rsi as u8,
    Edi  = Reg64::Rdi as u8,
    Ebp  = Reg64::Rbp as u8,
    Esp  = Reg64::Rsp as u8,

    R8d  = Reg64::R8 as u8,
    R9d  = Reg64::R9 as u8,
    R10d = Reg64::R10 as u8,
    R11d = Reg64::R11 as u8,
    R12d = Reg64::R12 as u8,
    R13d = Reg64::R13 as u8,
    R14d = Reg64::R14 as u8,
    R15d = Reg64::R15 as u8,
}

impl Reg32 {
    #[must_use] #[inline]
    pub const fn to_str(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Eax  => "eax",
            Self::Ebx  => "ebx",
            Self::Ecx  => "ecx",
            Self::Edx  => "edx",

            Self::Esi  => "esi",
            Self::Edi  => "edi",
            Self::Ebp  => "ebp",
            Self::Esp  => "esp",

            Self::R8d  => "r8d",
            Self::R9d  => "r9d",
            Self::R10d => "r10d",
            Self::R11d => "r11d",
            Self::R12d => "r12d",
            Self::R13d => "r13d",
            Self::R14d => "r14d",
            Self::R15d => "r15d",
        };
    }
}

impl Display for Reg32 {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum Reg16 {
    Ax   = Reg64::Rax as u8,
    Bx   = Reg64::Rbx as u8,
    Cx   = Reg64::Rcx as u8,
    Dx   = Reg64::Rdx as u8,

    Si   = Reg64::Rsi as u8,
    Di   = Reg64::Rdi as u8,
    Bp   = Reg64::Rbp as u8,
    Sp   = Reg64::Rsp as u8,

    R8w  = Reg64::R8 as u8,
    R9w  = Reg64::R9 as u8,
    R10w = Reg64::R10 as u8,
    R11w = Reg64::R11 as u8,
    R12w = Reg64::R12 as u8,
    R13w = Reg64::R13 as u8,
    R14w = Reg64::R14 as u8,
    R15w = Reg64::R15 as u8,
}

impl Reg16 {
    #[must_use] #[inline]
    pub const fn to_str(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Ax   => "ax",
            Self::Bx   => "bx",
            Self::Cx   => "cx",
            Self::Dx   => "dx",

            Self::Si   => "si",
            Self::Di   => "di",
            Self::Bp   => "bp",
            Self::Sp   => "sp",

            Self::R8w  => "r8w",
            Self::R9w  => "r9w",
            Self::R10w => "r10w",
            Self::R11w => "r11w",
            Self::R12w => "r12w",
            Self::R13w => "r13w",
            Self::R14w => "r14w",
            Self::R15w => "r15w",
        };
    }
}

impl Display for Reg16 {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum Reg8l {
    Al   = Reg64::Rax as u8,
    Bl   = Reg64::Rbx as u8,
    Cl   = Reg64::Rcx as u8,
    Dl   = Reg64::Rdx as u8,

    Sil  = Reg64::Rsi as u8,
    Dil  = Reg64::Rdi as u8,
    Bpl  = Reg64::Rbp as u8,
    Spl  = Reg64::Rsp as u8,

    R8b  = Reg64::R8 as u8,
    R9b  = Reg64::R9 as u8,
    R10b = Reg64::R10 as u8,
    R11b = Reg64::R11 as u8,
    R12b = Reg64::R12 as u8,
    R13b = Reg64::R13 as u8,
    R14b = Reg64::R14 as u8,
    R15b = Reg64::R15 as u8,
}

impl Reg8l {
    #[must_use] #[inline]
    pub const fn to_str(self) -> &'static str {
        #[rustfmt::skip]
        return match self {
            Self::Al   => "al",
            Self::Bl   => "bl",
            Self::Cl   => "cl",
            Self::Dl   => "dl",

            Self::Sil  => "sil",
            Self::Dil  => "dil",
            Self::Bpl  => "bpl",
            Self::Spl  => "spl",

            Self::R8b  => "r8b",
            Self::R9b  => "r9b",
            Self::R10b => "r10b",
            Self::R11b => "r11b",
            Self::R12b => "r12b",
            Self::R13b => "r13b",
            Self::R14b => "r14b",
            Self::R15b => "r15b",
        };
    }
}

impl Display for Reg8l {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[must_use] #[repr(u8)]
pub enum Reg8h {
    Ah = Reg64::Rax as u8,
    Bh = Reg64::Rbx as u8,
    Ch = Reg64::Rcx as u8,
    Dh = Reg64::Rdx as u8,
}

impl Reg8h {
    #[must_use] #[inline]
    pub const fn to_str(self) -> &'static str {
        return match self {
            Self::Ah => "ah",
            Self::Bh => "bh",
            Self::Ch => "ch",
            Self::Dh => "dh",
        };
    }
}

impl Display for Reg8h {
    #[inline(always)]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        return f.write_str(self.to_str());
    }
}

impl Reg64 {
    #[inline(always)]
    pub const fn as_Reg32(self) -> Reg32 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg32`]
    #[inline(always)]
    pub const fn as_reg32(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Into<Reg32> for Reg64 {
    #[inline(always)]
    fn into(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Reg64 {
    #[inline(always)]
    pub const fn as_Reg16(self) -> Reg16 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg16`]
    #[inline(always)]
    pub const fn as_reg16(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Into<Reg16> for Reg64 {
    #[inline(always)]
    fn into(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Reg64 {
    #[inline(always)]
    pub const fn as_Reg8l(self) -> Reg8l {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8l`]
    #[inline(always)]
    pub const fn as_reg8l(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Into<Reg8l> for Reg64 {
    #[inline(always)]
    fn into(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Reg64 {
    /// # Safety
    ///
    /// can only safely convert from [`Reg64::Rax`], [`Reg64::Rbx`], [`Reg64::Rcx`] and [`Reg64::Rdx`]
    #[inline(always)]
    pub const unsafe fn as_Reg8h_unsafe(self) -> Reg8h {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h_unsafe`]
    ///
    /// # Safety
    ///
    /// can only safely convert from [`Reg64::Rax`], [`Reg64::Rbx`], [`Reg64::Rcx`] and [`Reg64::Rdx`]
    #[inline(always)]
    pub const unsafe fn as_reg8h_unsafe(self) -> Reg8h {
        return unsafe { self.as_Reg8h_unsafe() };
    }

    #[allow(clippy::enum_glob_use)]
    #[must_use] #[inline]
    pub const fn as_Reg8h(self) -> Option<Reg8h> {
        use Reg64::*;
        return match self {
            Rax | Rbx | Rcx | Rdx                                               => Some(unsafe { self.as_Reg8h_unsafe() }),
            Rsi | Rdi | Rbp | Rsp | R8 | R9 | R10 | R11 | R12 | R13 | R14 | R15 => None,
        };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h`]
    #[must_use] #[inline(always)]
    pub const fn as_reg8h(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Into<Option<Reg8h>> for Reg64 {
    #[inline(always)]
    fn into(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Reg32 {
    #[inline(always)]
    pub const fn as_Reg64(self) -> Reg64 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg64`]
    #[inline(always)]
    pub const fn as_reg64(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Into<Reg64> for Reg32 {
    #[inline(always)]
    fn into(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Reg32 {
    #[inline(always)]
    pub const fn as_Reg16(self) -> Reg16 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg16`]
    #[inline(always)]
    pub const fn as_reg16(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Into<Reg16> for Reg32 {
    #[inline(always)]
    fn into(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Reg32 {
    #[inline(always)]
    pub const fn as_Reg8l(self) -> Reg8l {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8l`]
    #[inline(always)]
    pub const fn as_reg8l(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Into<Reg8l> for Reg32 {
    #[inline(always)]
    fn into(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Reg32 {
    /// # Safety
    ///
    /// can only safely convert from [`Reg32::Eax`], [`Reg32::Ebx`], [`Reg32::Ecx`] and [`Reg32::Edx`]
    #[inline(always)]
    pub const unsafe fn as_Reg8h_unsafe(self) -> Reg8h {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h_unsafe`]
    ///
    /// # Safety
    ///
    /// can only safely convert from [`Reg32::Eax`], [`Reg32::Ebx`], [`Reg32::Ecx`] and [`Reg32::Edx`]
    #[inline(always)]
    pub const unsafe fn as_reg8h_unsafe(self) -> Reg8h {
        return unsafe { self.as_Reg8h_unsafe() };
    }

    #[allow(clippy::enum_glob_use)]
    #[must_use] #[inline]
    pub const fn as_Reg8h(self) -> Option<Reg8h> {
        use Reg32::*;
        return match self {
            Eax | Ebx | Ecx | Edx                                                       => Some(unsafe { self.as_Reg8h_unsafe() }),
            Esi | Edi | Ebp | Esp | R8d | R9d | R10d | R11d | R12d | R13d | R14d | R15d => None,
        };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h`]
    #[must_use] #[inline(always)]
    pub const fn as_reg8h(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Into<Option<Reg8h>> for Reg32 {
    #[inline(always)]
    fn into(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Reg16 {
    #[inline(always)]
    pub const fn as_Reg64(self) -> Reg64 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg64`]
    #[inline(always)]
    pub const fn as_reg64(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Into<Reg64> for Reg16 {
    #[inline(always)]
    fn into(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Reg16 {
    #[inline(always)]
    pub const fn as_Reg32(self) -> Reg32 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg32`]
    #[inline(always)]
    pub const fn as_reg32(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Into<Reg32> for Reg16 {
    #[inline(always)]
    fn into(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Reg16 {
    #[inline(always)]
    pub const fn as_Reg8l(self) -> Reg8l {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8l`]
    #[inline(always)]
    pub const fn as_reg8l(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Into<Reg8l> for Reg16 {
    #[inline(always)]
    fn into(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Reg16 {
    /// # Safety
    ///
    /// can only safely convert from [`Reg16::Ax`], [`Reg16::Bx`], [`Reg16::Cx`] and [`Reg16::Dx`]
    #[inline(always)]
    pub const unsafe fn as_Reg8h_unsafe(self) -> Reg8h {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h_unsafe`]
    ///
    /// # Safety
    ///
    /// can only safely convert from [`Reg16::Ax`], [`Reg16::Bx`], [`Reg16::Cx`] and [`Reg16::Dx`]
    #[inline(always)]
    pub const unsafe fn as_reg8h_unsafe(self) -> Reg8h {
        return unsafe { self.as_Reg8h_unsafe() };
    }

    #[allow(clippy::enum_glob_use)]
    #[must_use] #[inline]
    pub const fn as_Reg8h(self) -> Option<Reg8h> {
        use Reg16::*;
        return match self {
            Ax | Bx | Cx | Dx                                                       => Some(unsafe { self.as_Reg8h_unsafe() }),
            Si | Di | Bp | Sp | R8w | R9w | R10w | R11w | R12w | R13w | R14w | R15w => None,
        };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h`]
    #[must_use] #[inline(always)]
    pub const fn as_reg8h(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Into<Option<Reg8h>> for Reg16 {
    #[inline(always)]
    fn into(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Reg8l {
    #[inline(always)]
    pub const fn as_Reg64(self) -> Reg64 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg64`]
    #[inline(always)]
    pub const fn as_reg64(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Into<Reg64> for Reg8l {
    #[inline(always)]
    fn into(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Reg8l {
    #[inline(always)]
    pub const fn as_Reg32(self) -> Reg32 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg32`]
    #[inline(always)]
    pub const fn as_reg32(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Into<Reg32> for Reg8l {
    #[inline(always)]
    fn into(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Reg8l {
    #[inline(always)]
    pub const fn as_Reg16(self) -> Reg16 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg16`]
    #[inline(always)]
    pub const fn as_reg16(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Into<Reg16> for Reg8l {
    #[inline(always)]
    fn into(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Reg8l {
    /// # Safety
    ///
    /// can only safely convert from [`Reg8l::Al`], [`Reg8l::Bl`], [`Reg8l::Cl`] and [`Reg8l::Dl`]
    #[inline(always)]
    pub const unsafe fn as_Reg8h_unsafe(self) -> Reg8h {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h_unsafe`]
    ///
    /// # Safety
    ///
    /// can only safely convert from [`Reg8l::Al`], [`Reg8l::Bl`], [`Reg8l::Cl`] and [`Reg8l::Dl`]
    #[inline(always)]
    pub const unsafe fn as_reg8h_unsafe(self) -> Reg8h {
        return unsafe { self.as_Reg8h_unsafe() };
    }

    #[allow(clippy::enum_glob_use)]
    #[must_use] #[inline]
    pub const fn as_Reg8h(self) -> Option<Reg8h> {
        use Reg8l::*;
        return match self {
            Al | Bl | Cl | Dl => Some(unsafe { self.as_Reg8h_unsafe() }),
            Sil | Dil | Bpl | Spl | R8b | R9b | R10b | R11b | R12b | R13b | R14b | R15b => None,
        };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8h`]
    #[must_use] #[inline(always)]
    pub const fn as_reg8h(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Into<Option<Reg8h>> for Reg8l {
    #[inline(always)]
    fn into(self) -> Option<Reg8h> {
        return self.as_Reg8h();
    }
}

impl Reg8h {
    #[inline(always)]
    pub const fn as_Reg64(self) -> Reg64 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg64`]
    #[inline(always)]
    pub const fn as_reg64(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Into<Reg64> for Reg8h {
    #[inline(always)]
    fn into(self) -> Reg64 {
        return self.as_Reg64();
    }
}

impl Reg8h {
    #[inline(always)]
    pub const fn as_Reg32(self) -> Reg32 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg32`]
    #[inline(always)]
    pub const fn as_reg32(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Into<Reg32> for Reg8h {
    #[inline(always)]
    fn into(self) -> Reg32 {
        return self.as_Reg32();
    }
}

impl Reg8h {
    #[inline(always)]
    pub const fn as_Reg16(self) -> Reg16 {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg16`]
    #[inline(always)]
    pub const fn as_reg16(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Into<Reg16> for Reg8h {
    #[inline(always)]
    fn into(self) -> Reg16 {
        return self.as_Reg16();
    }
}

impl Reg8h {
    #[inline(always)]
    pub const fn as_Reg8l(self) -> Reg8l {
        return unsafe { core::mem::transmute(self) };
    }

    /// 'Idiomatic' name alias of [`Self::as_Reg8l`]
    #[inline(always)]
    pub const fn as_reg8l(self) -> Reg8l {
        return self.as_Reg8l();
    }
}

impl Into<Reg8l> for Reg8h {
    #[inline(always)]
    fn into(self) -> Reg8l {
        return self.as_Reg8l();
    }
}
