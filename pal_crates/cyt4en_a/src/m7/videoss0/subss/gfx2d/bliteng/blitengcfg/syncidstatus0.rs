#[doc = "Register `SYNCIDSTATUS0` reader"]
pub struct R(crate::R<SYNCIDSTATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCIDSTATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCIDSTATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCIDSTATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYNCIDCOMPLETE0` reader - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 0."]
pub type SYNCIDCOMPLETE0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latest ID completed as requested by SyncIdRequest and SyncIdTrigger with TaskSelect = 0."]
    #[inline(always)]
    pub fn syncidcomplete0(&self) -> SYNCIDCOMPLETE0_R {
        SYNCIDCOMPLETE0_R::new(self.bits)
    }
}
#[doc = "Sync ID Status register for task 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncidstatus0](index.html) module"]
pub struct SYNCIDSTATUS0_SPEC;
impl crate::RegisterSpec for SYNCIDSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncidstatus0::R](R) reader structure"]
impl crate::Readable for SYNCIDSTATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCIDSTATUS0 to value 0"]
impl crate::Resettable for SYNCIDSTATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
