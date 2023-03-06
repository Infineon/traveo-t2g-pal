#[doc = "Register `IPCONFIGURATION` reader"]
pub struct R(crate::R<IPCONFIGURATION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCONFIGURATION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCONFIGURATION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCONFIGURATION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FPDLINK1` reader - 2nd FPDLink Video Output Interface available (0 = no, 1 = yes)"]
pub type FPDLINK1_R = crate::BitReader<bool>;
#[doc = "Field `CLKSPEED` reader - Allowed maximum for core and AXI bus clock frequency."]
pub type CLKSPEED_R = crate::FieldReader<u8, CLKSPEED_A>;
#[doc = "Allowed maximum for core and AXI bus clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKSPEED_A {
    #[doc = "0: 200 MHz"]
    MHZ_200 = 0,
    #[doc = "1: 250 MHz"]
    MHZ_250 = 1,
    #[doc = "2: 266 MHz"]
    MHZ_266 = 2,
}
impl From<CLKSPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKSPEED_A) -> Self {
        variant as _
    }
}
impl CLKSPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKSPEED_A> {
        match self.bits {
            0 => Some(CLKSPEED_A::MHZ_200),
            1 => Some(CLKSPEED_A::MHZ_250),
            2 => Some(CLKSPEED_A::MHZ_266),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MHZ_200`"]
    #[inline(always)]
    pub fn is_mhz_200(&self) -> bool {
        *self == CLKSPEED_A::MHZ_200
    }
    #[doc = "Checks if the value of the field is `MHZ_250`"]
    #[inline(always)]
    pub fn is_mhz_250(&self) -> bool {
        *self == CLKSPEED_A::MHZ_250
    }
    #[doc = "Checks if the value of the field is `MHZ_266`"]
    #[inline(always)]
    pub fn is_mhz_266(&self) -> bool {
        *self == CLKSPEED_A::MHZ_266
    }
}
#[doc = "Field `VRAMSIZE` reader - Size of embedded Video RAM in MB. Is 0 if none available."]
pub type VRAMSIZE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 3 - 2nd FPDLink Video Output Interface available (0 = no, 1 = yes)"]
    #[inline(always)]
    pub fn fpdlink1(&self) -> FPDLINK1_R {
        FPDLINK1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Allowed maximum for core and AXI bus clock frequency."]
    #[inline(always)]
    pub fn clkspeed(&self) -> CLKSPEED_R {
        CLKSPEED_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Size of embedded Video RAM in MB. Is 0 if none available."]
    #[inline(always)]
    pub fn vramsize(&self) -> VRAMSIZE_R {
        VRAMSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "IP Design Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipconfiguration](index.html) module"]
pub struct IPCONFIGURATION_SPEC;
impl crate::RegisterSpec for IPCONFIGURATION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipconfiguration::R](R) reader structure"]
impl crate::Readable for IPCONFIGURATION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPCONFIGURATION to value 0"]
impl crate::Resettable for IPCONFIGURATION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
