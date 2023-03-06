#[doc = "Register `LBHGLOBALSTATUS2` reader"]
pub struct R(crate::R<LBHGLOBALSTATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHGLOBALSTATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHGLOBALSTATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHGLOBALSTATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LBHCYCLECOUNTEROVERFLOW` reader - This flag is raised when overflow occurs on the LbhCycleCounter."]
pub type LBHCYCLECOUNTEROVERFLOW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This flag is raised when overflow occurs on the LbhCycleCounter."]
    #[inline(always)]
    pub fn lbhcyclecounteroverflow(&self) -> LBHCYCLECOUNTEROVERFLOW_R {
        LBHCYCLECOUNTEROVERFLOW_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Global status register 2 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhglobalstatus2](index.html) module"]
pub struct LBHGLOBALSTATUS2_SPEC;
impl crate::RegisterSpec for LBHGLOBALSTATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhglobalstatus2::R](R) reader structure"]
impl crate::Readable for LBHGLOBALSTATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LBHGLOBALSTATUS2 to value 0"]
impl crate::Resettable for LBHGLOBALSTATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
