#[doc = "Register `LBHGLOBALSTATUS1` reader"]
pub struct R(crate::R<LBHGLOBALSTATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHGLOBALSTATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHGLOBALSTATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHGLOBALSTATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHREADYCYCLES` reader - This counter increments with every cycle a slice is allowed to be written to any line buffer."]
pub type LBHREADYCYCLES_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - This counter increments with every cycle a slice is allowed to be written to any line buffer."]
    #[inline(always)]
    pub fn lbhreadycycles(&self) -> LBHREADYCYCLES_R {
        LBHREADYCYCLES_R::new(self.bits)
    }
}
#[doc = "Global status register 1 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhglobalstatus1](index.html) module"]
pub struct LBHGLOBALSTATUS1_SPEC;
impl crate::RegisterSpec for LBHGLOBALSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhglobalstatus1::R](R) reader structure"]
impl crate::Readable for LBHGLOBALSTATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHGLOBALSTATUS1 to value 0"]
impl crate::Resettable for LBHGLOBALSTATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
