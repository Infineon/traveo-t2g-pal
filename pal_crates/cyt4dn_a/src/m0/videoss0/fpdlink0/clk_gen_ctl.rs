#[doc = "Register `CLK_GEN_CTL` reader"]
pub struct R(crate::R<CLK_GEN_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GEN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GEN_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GEN_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GEN_CTL` writer"]
pub struct W(crate::W<CLK_GEN_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GEN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CLK_GEN_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GEN_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRANGE` reader - Control Output frequency divider, connects to the port CO on the hardIP"]
pub type FRANGE_R = crate::FieldReader<u8, FRANGE_A>;
#[doc = "Control Output frequency divider, connects to the port CO on the hardIP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRANGE_A {
    #[doc = "0: Internal PLL output divider programmed to 1"]
    DIV1 = 0,
    #[doc = "1: Internal PLL output divider programmed to 2"]
    DIV2 = 1,
    #[doc = "2: Internal PLL output divider programmed to 4"]
    DIV4 = 2,
    #[doc = "3: Internal PLL output divider programmed to 8"]
    DIV8 = 3,
}
impl From<FRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: FRANGE_A) -> Self {
        variant as _
    }
}
impl FRANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRANGE_A {
        match self.bits {
            0 => FRANGE_A::DIV1,
            1 => FRANGE_A::DIV2,
            2 => FRANGE_A::DIV4,
            3 => FRANGE_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == FRANGE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == FRANGE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == FRANGE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == FRANGE_A::DIV8
    }
}
#[doc = "Field `FRANGE` writer - Control Output frequency divider, connects to the port CO on the hardIP"]
pub type FRANGE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_GEN_CTL_SPEC, u8, FRANGE_A, 2, O>;
impl<'a, const O: u8> FRANGE_W<'a, O> {
    #[doc = "Internal PLL output divider programmed to 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(FRANGE_A::DIV1)
    }
    #[doc = "Internal PLL output divider programmed to 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(FRANGE_A::DIV2)
    }
    #[doc = "Internal PLL output divider programmed to 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(FRANGE_A::DIV4)
    }
    #[doc = "Internal PLL output divider programmed to 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(FRANGE_A::DIV8)
    }
}
#[doc = "Field `LFCTRL` reader - Loop filter resistance selection used for low bandwidth feature"]
pub type LFCTRL_R = crate::FieldReader<u8, LFCTRL_A>;
#[doc = "Loop filter resistance selection used for low bandwidth feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LFCTRL_A {
    #[doc = "0: Default"]
    DIV1 = 0,
    #[doc = "1: Lower Bandwidth"]
    DIV2 = 1,
    #[doc = "2: Lower Bandwidth"]
    DIV4 = 2,
    #[doc = "3: Less than 800khz"]
    DIV8 = 3,
}
impl From<LFCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LFCTRL_A) -> Self {
        variant as _
    }
}
impl LFCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFCTRL_A {
        match self.bits {
            0 => LFCTRL_A::DIV1,
            1 => LFCTRL_A::DIV2,
            2 => LFCTRL_A::DIV4,
            3 => LFCTRL_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LFCTRL_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LFCTRL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LFCTRL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LFCTRL_A::DIV8
    }
}
#[doc = "Field `LFCTRL` writer - Loop filter resistance selection used for low bandwidth feature"]
pub type LFCTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CLK_GEN_CTL_SPEC, u8, LFCTRL_A, 2, O>;
impl<'a, const O: u8> LFCTRL_W<'a, O> {
    #[doc = "Default"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LFCTRL_A::DIV1)
    }
    #[doc = "Lower Bandwidth"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LFCTRL_A::DIV2)
    }
    #[doc = "Lower Bandwidth"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LFCTRL_A::DIV4)
    }
    #[doc = "Less than 800khz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LFCTRL_A::DIV8)
    }
}
#[doc = "Field `CA` reader - Driver output current control bits"]
pub type CA_R = crate::FieldReader<u8, CA_A>;
#[doc = "Driver output current control bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CA_A {
    #[doc = "0: Driver output current is 2.4mA"]
    _2P4MA = 0,
    #[doc = "1: Driver output current is 2.72mA"]
    _2P72MA = 1,
    #[doc = "2: Driver output current is 2.88mA"]
    _2P88MA = 2,
    #[doc = "3: Driver output current is 3.2mA"]
    _3P2MA = 3,
    #[doc = "4: Driver output current is 3.36ma, this is the default value to be progammed"]
    _3P36MA = 4,
    #[doc = "5: Driver output current is 3.68mA"]
    _3P68MA = 5,
    #[doc = "6: Driver output current is 3.84mA"]
    _3P84MA = 6,
    #[doc = "7: Driver output current is 4.32mA"]
    _4P32MA = 7,
}
impl From<CA_A> for u8 {
    #[inline(always)]
    fn from(variant: CA_A) -> Self {
        variant as _
    }
}
impl CA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CA_A {
        match self.bits {
            0 => CA_A::_2P4MA,
            1 => CA_A::_2P72MA,
            2 => CA_A::_2P88MA,
            3 => CA_A::_3P2MA,
            4 => CA_A::_3P36MA,
            5 => CA_A::_3P68MA,
            6 => CA_A::_3P84MA,
            7 => CA_A::_4P32MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2P4MA`"]
    #[inline(always)]
    pub fn is_2p4ma(&self) -> bool {
        *self == CA_A::_2P4MA
    }
    #[doc = "Checks if the value of the field is `_2P72MA`"]
    #[inline(always)]
    pub fn is_2p72ma(&self) -> bool {
        *self == CA_A::_2P72MA
    }
    #[doc = "Checks if the value of the field is `_2P88MA`"]
    #[inline(always)]
    pub fn is_2p88ma(&self) -> bool {
        *self == CA_A::_2P88MA
    }
    #[doc = "Checks if the value of the field is `_3P2MA`"]
    #[inline(always)]
    pub fn is_3p2ma(&self) -> bool {
        *self == CA_A::_3P2MA
    }
    #[doc = "Checks if the value of the field is `_3P36MA`"]
    #[inline(always)]
    pub fn is_3p36ma(&self) -> bool {
        *self == CA_A::_3P36MA
    }
    #[doc = "Checks if the value of the field is `_3P68MA`"]
    #[inline(always)]
    pub fn is_3p68ma(&self) -> bool {
        *self == CA_A::_3P68MA
    }
    #[doc = "Checks if the value of the field is `_3P84MA`"]
    #[inline(always)]
    pub fn is_3p84ma(&self) -> bool {
        *self == CA_A::_3P84MA
    }
    #[doc = "Checks if the value of the field is `_4P32MA`"]
    #[inline(always)]
    pub fn is_4p32ma(&self) -> bool {
        *self == CA_A::_4P32MA
    }
}
#[doc = "Field `CA` writer - Driver output current control bits"]
pub type CA_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLK_GEN_CTL_SPEC, u8, CA_A, 3, O>;
impl<'a, const O: u8> CA_W<'a, O> {
    #[doc = "Driver output current is 2.4mA"]
    #[inline(always)]
    pub fn _2p4ma(self) -> &'a mut W {
        self.variant(CA_A::_2P4MA)
    }
    #[doc = "Driver output current is 2.72mA"]
    #[inline(always)]
    pub fn _2p72ma(self) -> &'a mut W {
        self.variant(CA_A::_2P72MA)
    }
    #[doc = "Driver output current is 2.88mA"]
    #[inline(always)]
    pub fn _2p88ma(self) -> &'a mut W {
        self.variant(CA_A::_2P88MA)
    }
    #[doc = "Driver output current is 3.2mA"]
    #[inline(always)]
    pub fn _3p2ma(self) -> &'a mut W {
        self.variant(CA_A::_3P2MA)
    }
    #[doc = "Driver output current is 3.36ma, this is the default value to be progammed"]
    #[inline(always)]
    pub fn _3p36ma(self) -> &'a mut W {
        self.variant(CA_A::_3P36MA)
    }
    #[doc = "Driver output current is 3.68mA"]
    #[inline(always)]
    pub fn _3p68ma(self) -> &'a mut W {
        self.variant(CA_A::_3P68MA)
    }
    #[doc = "Driver output current is 3.84mA"]
    #[inline(always)]
    pub fn _3p84ma(self) -> &'a mut W {
        self.variant(CA_A::_3P84MA)
    }
    #[doc = "Driver output current is 4.32mA"]
    #[inline(always)]
    pub fn _4p32ma(self) -> &'a mut W {
        self.variant(CA_A::_4P32MA)
    }
}
#[doc = "Field `CCM` reader - Common mode voltage control bits."]
pub type CCM_R = crate::FieldReader<u8, CCM_A>;
#[doc = "Common mode voltage control bits.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CCM_A {
    #[doc = "2: Common mode voltage set to 1.15"]
    _1P05 = 2,
    #[doc = "1: Common mode voltage set to 1.10"]
    _1P10 = 1,
    #[doc = "3: Common mode voltage set to 1.20"]
    _1P20 = 3,
    #[doc = "4: Common mode voltage set to 1.25, this is the default value to be progammed"]
    _1P25 = 4,
    #[doc = "5: Common mode voltage set to 1.30"]
    _1P30 = 5,
    #[doc = "6: Common mode voltage set to 1.35"]
    _1P35 = 6,
    #[doc = "7: Common mode voltage set to 1.40"]
    _1P40 = 7,
}
impl From<CCM_A> for u8 {
    #[inline(always)]
    fn from(variant: CCM_A) -> Self {
        variant as _
    }
}
impl CCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCM_A> {
        match self.bits {
            2 => Some(CCM_A::_1P05),
            1 => Some(CCM_A::_1P10),
            3 => Some(CCM_A::_1P20),
            4 => Some(CCM_A::_1P25),
            5 => Some(CCM_A::_1P30),
            6 => Some(CCM_A::_1P35),
            7 => Some(CCM_A::_1P40),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1P05`"]
    #[inline(always)]
    pub fn is_1p05(&self) -> bool {
        *self == CCM_A::_1P05
    }
    #[doc = "Checks if the value of the field is `_1P10`"]
    #[inline(always)]
    pub fn is_1p10(&self) -> bool {
        *self == CCM_A::_1P10
    }
    #[doc = "Checks if the value of the field is `_1P20`"]
    #[inline(always)]
    pub fn is_1p20(&self) -> bool {
        *self == CCM_A::_1P20
    }
    #[doc = "Checks if the value of the field is `_1P25`"]
    #[inline(always)]
    pub fn is_1p25(&self) -> bool {
        *self == CCM_A::_1P25
    }
    #[doc = "Checks if the value of the field is `_1P30`"]
    #[inline(always)]
    pub fn is_1p30(&self) -> bool {
        *self == CCM_A::_1P30
    }
    #[doc = "Checks if the value of the field is `_1P35`"]
    #[inline(always)]
    pub fn is_1p35(&self) -> bool {
        *self == CCM_A::_1P35
    }
    #[doc = "Checks if the value of the field is `_1P40`"]
    #[inline(always)]
    pub fn is_1p40(&self) -> bool {
        *self == CCM_A::_1P40
    }
}
#[doc = "Field `CCM` writer - Common mode voltage control bits."]
pub type CCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_GEN_CTL_SPEC, u8, CCM_A, 3, O>;
impl<'a, const O: u8> CCM_W<'a, O> {
    #[doc = "Common mode voltage set to 1.15"]
    #[inline(always)]
    pub fn _1p05(self) -> &'a mut W {
        self.variant(CCM_A::_1P05)
    }
    #[doc = "Common mode voltage set to 1.10"]
    #[inline(always)]
    pub fn _1p10(self) -> &'a mut W {
        self.variant(CCM_A::_1P10)
    }
    #[doc = "Common mode voltage set to 1.20"]
    #[inline(always)]
    pub fn _1p20(self) -> &'a mut W {
        self.variant(CCM_A::_1P20)
    }
    #[doc = "Common mode voltage set to 1.25, this is the default value to be progammed"]
    #[inline(always)]
    pub fn _1p25(self) -> &'a mut W {
        self.variant(CCM_A::_1P25)
    }
    #[doc = "Common mode voltage set to 1.30"]
    #[inline(always)]
    pub fn _1p30(self) -> &'a mut W {
        self.variant(CCM_A::_1P30)
    }
    #[doc = "Common mode voltage set to 1.35"]
    #[inline(always)]
    pub fn _1p35(self) -> &'a mut W {
        self.variant(CCM_A::_1P35)
    }
    #[doc = "Common mode voltage set to 1.40"]
    #[inline(always)]
    pub fn _1p40(self) -> &'a mut W {
        self.variant(CCM_A::_1P40)
    }
}
#[doc = "Field `CN` reader - Control N divider used for low bandwidth feature"]
pub type CN_R = crate::FieldReader<u8, CN_A>;
#[doc = "Control N divider used for low bandwidth feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CN_A {
    #[doc = "0: Default"]
    DIV1 = 0,
    #[doc = "1: Lower Bandwidth"]
    DIV2 = 1,
    #[doc = "2: Lower Bandwidth"]
    DIV4 = 2,
    #[doc = "3: Less than 800khz"]
    DIV8 = 3,
}
impl From<CN_A> for u8 {
    #[inline(always)]
    fn from(variant: CN_A) -> Self {
        variant as _
    }
}
impl CN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CN_A {
        match self.bits {
            0 => CN_A::DIV1,
            1 => CN_A::DIV2,
            2 => CN_A::DIV4,
            3 => CN_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CN_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CN_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CN_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CN_A::DIV8
    }
}
#[doc = "Field `CN` writer - Control N divider used for low bandwidth feature"]
pub type CN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CLK_GEN_CTL_SPEC, u8, CN_A, 2, O>;
impl<'a, const O: u8> CN_W<'a, O> {
    #[doc = "Default"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CN_A::DIV1)
    }
    #[doc = "Lower Bandwidth"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CN_A::DIV2)
    }
    #[doc = "Lower Bandwidth"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CN_A::DIV4)
    }
    #[doc = "Less than 800khz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CN_A::DIV8)
    }
}
#[doc = "Field `LOCK_SEL` reader - Select internal or external PLL lock"]
pub type LOCK_SEL_R = crate::BitReader<LOCK_SEL_A>;
#[doc = "Select internal or external PLL lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCK_SEL_A {
    #[doc = "0: Select Internal Lock. This is the default value"]
    INT = 0,
    #[doc = "1: Select External Lock"]
    EXT = 1,
}
impl From<LOCK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: LOCK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCK_SEL_A {
        match self.bits {
            false => LOCK_SEL_A::INT,
            true => LOCK_SEL_A::EXT,
        }
    }
    #[doc = "Checks if the value of the field is `INT`"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == LOCK_SEL_A::INT
    }
    #[doc = "Checks if the value of the field is `EXT`"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == LOCK_SEL_A::EXT
    }
}
#[doc = "Field `LOCK_SEL` writer - Select internal or external PLL lock"]
pub type LOCK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GEN_CTL_SPEC, LOCK_SEL_A, O>;
impl<'a, const O: u8> LOCK_SEL_W<'a, O> {
    #[doc = "Select Internal Lock. This is the default value"]
    #[inline(always)]
    pub fn int(self) -> &'a mut W {
        self.variant(LOCK_SEL_A::INT)
    }
    #[doc = "Select External Lock"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut W {
        self.variant(LOCK_SEL_A::EXT)
    }
}
#[doc = "Field `MSB_FIRST` reader - Controls parallel to serial transmission order of input 7-bit word"]
pub type MSB_FIRST_R = crate::BitReader<MSB_FIRST_A>;
#[doc = "Controls parallel to serial transmission order of input 7-bit word\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSB_FIRST_A {
    #[doc = "0: LSB bit is sent first"]
    LSB = 0,
    #[doc = "1: MSB bit is sent first. This is the default value"]
    MSB = 1,
}
impl From<MSB_FIRST_A> for bool {
    #[inline(always)]
    fn from(variant: MSB_FIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl MSB_FIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSB_FIRST_A {
        match self.bits {
            false => MSB_FIRST_A::LSB,
            true => MSB_FIRST_A::MSB,
        }
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == MSB_FIRST_A::LSB
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == MSB_FIRST_A::MSB
    }
}
#[doc = "Field `MSB_FIRST` writer - Controls parallel to serial transmission order of input 7-bit word"]
pub type MSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_GEN_CTL_SPEC, MSB_FIRST_A, O>;
impl<'a, const O: u8> MSB_FIRST_W<'a, O> {
    #[doc = "LSB bit is sent first"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(MSB_FIRST_A::LSB)
    }
    #[doc = "MSB bit is sent first. This is the default value"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(MSB_FIRST_A::MSB)
    }
}
impl R {
    #[doc = "Bits 0:1 - Control Output frequency divider, connects to the port CO on the hardIP"]
    #[inline(always)]
    pub fn frange(&self) -> FRANGE_R {
        FRANGE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Loop filter resistance selection used for low bandwidth feature"]
    #[inline(always)]
    pub fn lfctrl(&self) -> LFCTRL_R {
        LFCTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Driver output current control bits"]
    #[inline(always)]
    pub fn ca(&self) -> CA_R {
        CA_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Common mode voltage control bits."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Control N divider used for low bandwidth feature"]
    #[inline(always)]
    pub fn cn(&self) -> CN_R {
        CN_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Select internal or external PLL lock"]
    #[inline(always)]
    pub fn lock_sel(&self) -> LOCK_SEL_R {
        LOCK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Controls parallel to serial transmission order of input 7-bit word"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control Output frequency divider, connects to the port CO on the hardIP"]
    #[inline(always)]
    #[must_use]
    pub fn frange(&mut self) -> FRANGE_W<0> {
        FRANGE_W::new(self)
    }
    #[doc = "Bits 4:5 - Loop filter resistance selection used for low bandwidth feature"]
    #[inline(always)]
    #[must_use]
    pub fn lfctrl(&mut self) -> LFCTRL_W<4> {
        LFCTRL_W::new(self)
    }
    #[doc = "Bits 8:10 - Driver output current control bits"]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CA_W<8> {
        CA_W::new(self)
    }
    #[doc = "Bits 12:14 - Common mode voltage control bits."]
    #[inline(always)]
    #[must_use]
    pub fn ccm(&mut self) -> CCM_W<12> {
        CCM_W::new(self)
    }
    #[doc = "Bits 16:17 - Control N divider used for low bandwidth feature"]
    #[inline(always)]
    #[must_use]
    pub fn cn(&mut self) -> CN_W<16> {
        CN_W::new(self)
    }
    #[doc = "Bit 20 - Select internal or external PLL lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock_sel(&mut self) -> LOCK_SEL_W<20> {
        LOCK_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Controls parallel to serial transmission order of input 7-bit word"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<24> {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register for CKGEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gen_ctl](index.html) module"]
pub struct CLK_GEN_CTL_SPEC;
impl crate::RegisterSpec for CLK_GEN_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gen_ctl::R](R) reader structure"]
impl crate::Readable for CLK_GEN_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gen_ctl::W](W) writer structure"]
impl crate::Writable for CLK_GEN_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_GEN_CTL to value 0"]
impl crate::Resettable for CLK_GEN_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
