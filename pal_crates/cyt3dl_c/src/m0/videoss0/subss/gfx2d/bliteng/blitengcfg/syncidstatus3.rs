#[doc = "Register `SYNCIDSTATUS3` reader"]
pub struct R(crate::R<SYNCIDSTATUS3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE3` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 3."]
pub type SYNCIDCOMPLETE3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 3."]
    #[inline(always)]
    pub fn syncidcomplete3(&self) -> SYNCIDCOMPLETE3_R {
        SYNCIDCOMPLETE3_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus3](index.html) module"]
pub struct SYNCIDSTATUS3_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus3::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS3 to value 0"]
impl crate::Resettable for SYNCIDSTATUS3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
