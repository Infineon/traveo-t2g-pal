#[doc = "Register `SYNCIDSTATUS1` reader"]
pub struct R(crate::R<SYNCIDSTATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE1` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 1."]
pub type SYNCIDCOMPLETE1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 1."]
    #[inline(always)]
    pub fn syncidcomplete1(&self) -> SYNCIDCOMPLETE1_R {
        SYNCIDCOMPLETE1_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus1](index.html) module"]
pub struct SYNCIDSTATUS1_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus1::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS1 to value 0"]
impl crate::Resettable for SYNCIDSTATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
