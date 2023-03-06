#[doc = "Register `DLLSTTDQ0` reader"]
pub struct R(crate::R<DLLSTTDQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLSTTDQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLSTTDQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLSTTDQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCK` reader - DLL lock status. One bit for each Data Slice."]
pub type LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVFL` reader - Overflow status. This output is set when a quarter of the clock cycle is greater than the largest delay that the delay chain can handle. One bit for each Data Slice."]
pub type OVFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UNFL` reader - Underflow status. This output is set when a quarter of the clock cycle is less than the shortest delay that the delay chain can handle. One bit for each Data Slice."]
pub type UNFL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - DLL lock status. One bit for each Data Slice."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Overflow status. This output is set when a quarter of the clock cycle is greater than the largest delay that the delay chain can handle. One bit for each Data Slice."]
    #[inline(always)]
    pub fn ovfl(&self) -> OVFL_R {
        OVFL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Underflow status. This output is set when a quarter of the clock cycle is less than the shortest delay that the delay chain can handle. One bit for each Data Slice."]
    #[inline(always)]
    pub fn unfl(&self) -> UNFL_R {
        UNFL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[doc = "DLL Status Register for PHY Data Module 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllsttdq0](index.html) module"]
pub struct DLLSTTDQ0_SPEC;
impl crate::RegisterSpec for DLLSTTDQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllsttdq0::R](R) reader structure"]
impl crate::Readable for DLLSTTDQ0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLLSTTDQ0 to value 0"]
impl crate::Resettable for DLLSTTDQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
