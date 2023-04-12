#[doc = "Register `STPW2` reader"]
pub struct R(crate::R<STPW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STPW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STPW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STPW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SSCVA` reader - Stop Watch Captured Slot Counter Value Channel A State of the slot counter for channel A when the stop watch event occurred. Valid values are 0 to 2047."]
pub type SSCVA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSCVB` reader - Stop Watch Captured Slot Counter Value Channel B State of the slot counter for channel B when the stop watch event occurred. Valid values are 0 to 2047."]
pub type SSCVB_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Stop Watch Captured Slot Counter Value Channel A State of the slot counter for channel A when the stop watch event occurred. Valid values are 0 to 2047."]
    #[inline(always)]
    pub fn sscva(&self) -> SSCVA_R {
        SSCVA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Stop Watch Captured Slot Counter Value Channel B State of the slot counter for channel B when the stop watch event occurred. Valid values are 0 to 2047."]
    #[inline(always)]
    pub fn sscvb(&self) -> SSCVB_R {
        SSCVB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Stop Watch Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stpw2](index.html) module"]
pub struct STPW2_SPEC;
impl crate::RegisterSpec for STPW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stpw2::R](R) reader structure"]
impl crate::Readable for STPW2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STPW2 to value 0"]
impl crate::Resettable for STPW2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
