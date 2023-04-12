#[doc = "Register `SYNCIDSTATUS4` reader"]
pub struct R(crate::R<SYNCIDSTATUS4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE4` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 4."]
pub type SYNCIDCOMPLETE4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 4."]
    #[inline(always)]
    pub fn syncidcomplete4(&self) -> SYNCIDCOMPLETE4_R {
        SYNCIDCOMPLETE4_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 4.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus4](index.html) module"]
pub struct SYNCIDSTATUS4_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus4::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS4 to value 0"]
impl crate::Resettable for SYNCIDSTATUS4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
