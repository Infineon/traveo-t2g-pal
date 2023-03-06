#[doc = "Register `SYNCIDSTATUS5` reader"]
pub struct R(crate::R<SYNCIDSTATUS5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE5` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 5."]
pub type SYNCIDCOMPLETE5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 5."]
    #[inline(always)]
    pub fn syncidcomplete5(&self) -> SYNCIDCOMPLETE5_R {
        SYNCIDCOMPLETE5_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus5](index.html) module"]
pub struct SYNCIDSTATUS5_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus5::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS5 to value 0"]
impl crate::Resettable for SYNCIDSTATUS5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
