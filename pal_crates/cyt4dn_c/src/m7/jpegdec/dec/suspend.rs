#[doc = "Register `SUSPEND` reader"]
pub struct R(crate::R<SUSPEND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSPEND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSPEND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSPEND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSPEND` reader - This bit indicates whether the JPEG decoder core has been suspended due to detection of an APP/COM/unknown marker or the size/sub-sampling information in the JPEG header. 0: Not suspended 1: Suspended due to detection of an APP/COM/unknown marker or the size/sub-sampling"]
pub type SUSPEND_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - This bit indicates whether the JPEG decoder core has been suspended due to detection of an APP/COM/unknown marker or the size/sub-sampling information in the JPEG header. 0: Not suspended 1: Suspended due to detection of an APP/COM/unknown marker or the size/sub-sampling"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Suspend status. Decoding can be resumed from Suspended State by writing '1' to CMD.RESUME.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [suspend](index.html) module"]
pub struct SUSPEND_SPEC;
impl crate::RegisterSpec for SUSPEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [suspend::R](R) reader structure"]
impl crate::Readable for SUSPEND_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SUSPEND to value 0"]
impl crate::Resettable for SUSPEND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
