#[doc = "Register `DLLSTTCA` reader"]
pub struct R(crate::R<DLLSTTCA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLSTTCA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLSTTCA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLSTTCA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLLSTTCA_LOCK` reader - DLL lock status."]
pub type DLLSTTCA_LOCK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLLSTTCA_OVFL` reader - Overflow status. This output is set when a quarter of the clock cycle is greater than the largest delay that the delay chain can handle."]
pub type DLLSTTCA_OVFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLLSTTCA_UNFL` reader - Underflow status. This output is set when a quarter of the clock cycle is less than the shortest delay that the delay chain can handle."]
pub type DLLSTTCA_UNFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYC_CA0` reader - DLL delay code for Channel A"]
pub type DLYC_CA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYC_CA1` reader - DLL delay code for Channel B"]
pub type DLYC_CA1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - DLL lock status."]
    #[inline(always)]
    pub fn dllsttca_lock(&self) -> DLLSTTCA_LOCK_R {
        DLLSTTCA_LOCK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Overflow status. This output is set when a quarter of the clock cycle is greater than the largest delay that the delay chain can handle."]
    #[inline(always)]
    pub fn dllsttca_ovfl(&self) -> DLLSTTCA_OVFL_R {
        DLLSTTCA_OVFL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Underflow status. This output is set when a quarter of the clock cycle is less than the shortest delay that the delay chain can handle."]
    #[inline(always)]
    pub fn dllsttca_unfl(&self) -> DLLSTTCA_UNFL_R {
        DLLSTTCA_UNFL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:13 - DLL delay code for Channel A"]
    #[inline(always)]
    pub fn dlyc_ca0(&self) -> DLYC_CA0_R {
        DLYC_CA0_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - DLL delay code for Channel B"]
    #[inline(always)]
    pub fn dlyc_ca1(&self) -> DLYC_CA1_R {
        DLYC_CA1_R::new(((self.bits >> 14) & 0xff) as u8)
    }
}
#[doc = "DLL Status Register for PHY Command Module\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllsttca](index.html) module"]
pub struct DLLSTTCA_SPEC;
impl crate::RegisterSpec for DLLSTTCA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllsttca::R](R) reader structure"]
impl crate::Readable for DLLSTTCA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLLSTTCA to value 0"]
impl crate::Resettable for DLLSTTCA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
