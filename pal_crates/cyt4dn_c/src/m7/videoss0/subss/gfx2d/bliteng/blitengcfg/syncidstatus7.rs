#[doc = "Register `SYNCIDSTATUS7` reader"]
pub struct R(crate::R<SYNCIDSTATUS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE7` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 7."]
pub type SYNCIDCOMPLETE7_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 7."]
    #[inline(always)]
    pub fn syncidcomplete7(&self) -> SYNCIDCOMPLETE7_R {
        SYNCIDCOMPLETE7_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 7.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus7](index.html) module"]
pub struct SYNCIDSTATUS7_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus7::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS7 to value 0"]
impl crate::Resettable for SYNCIDSTATUS7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
