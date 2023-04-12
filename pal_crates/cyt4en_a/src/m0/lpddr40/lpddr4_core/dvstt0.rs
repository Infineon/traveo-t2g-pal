#[doc = "Register `DVSTT0` reader"]
pub struct R(crate::R<DVSTT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVSTT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVSTT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVSTT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DEVICE_ID` reader - 6 bits - Version, 5 bits - Reversion, 5 bits - Release"]
pub type DEVICE_ID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - 6 bits - Version, 5 bits - Reversion, 5 bits - Release"]
    #[inline(always)]
    pub fn device_id(&self) -> DEVICE_ID_R {
        DEVICE_ID_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Device ID Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvstt0](index.html) module"]
pub struct DVSTT0_SPEC;
impl crate::RegisterSpec for DVSTT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvstt0::R](R) reader structure"]
impl crate::Readable for DVSTT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DVSTT0 to value 0"]
impl crate::Resettable for DVSTT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
