#[doc = "Register `SCHEDULERADDRESS` reader"]
pub struct R(crate::R<SCHEDULERADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCHEDULERADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCHEDULERADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCHEDULERADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SCHEDULERADDRESS` reader - Last fetched address of scheduler"]
pub type SCHEDULERADDRESS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last fetched address of scheduler"]
    #[inline(always)]
    pub fn scheduleraddress(&self) -> SCHEDULERADDRESS_R {
        SCHEDULERADDRESS_R::new(self.bits)
    }
}
#[doc = "Last fetched address of scheduler.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scheduleraddress](index.html) module"]
pub struct SCHEDULERADDRESS_SPEC;
impl crate::RegisterSpec for SCHEDULERADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scheduleraddress::R](R) reader structure"]
impl crate::Readable for SCHEDULERADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SCHEDULERADDRESS to value 0"]
impl crate::Resettable for SCHEDULERADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
