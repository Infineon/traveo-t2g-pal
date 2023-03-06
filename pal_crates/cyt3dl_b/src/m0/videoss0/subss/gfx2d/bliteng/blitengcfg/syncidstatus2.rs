#[doc = "Register `SYNCIDSTATUS2` reader"]
pub struct R(crate::R<SYNCIDSTATUS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE2` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 2."]
pub type SYNCIDCOMPLETE2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 2."]
    #[inline(always)]
    pub fn syncidcomplete2(&self) -> SYNCIDCOMPLETE2_R {
        SYNCIDCOMPLETE2_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus2](index.html) module"]
pub struct SYNCIDSTATUS2_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus2::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS2 to value 0"]
impl crate::Resettable for SYNCIDSTATUS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
