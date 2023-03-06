#[doc = "Register `SYNCIDSTATUS6` reader"]
pub struct R(crate::R<SYNCIDSTATUS6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE6` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 6."]
pub type SYNCIDCOMPLETE6_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 6."]
    #[inline(always)]
    pub fn syncidcomplete6(&self) -> SYNCIDCOMPLETE6_R {
        SYNCIDCOMPLETE6_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus6](index.html) module"]
pub struct SYNCIDSTATUS6_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus6::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS6 to value 0"]
impl crate::Resettable for SYNCIDSTATUS6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
