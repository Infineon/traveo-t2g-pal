#[doc = "Register `DLLSTTDQ1` reader"]
pub struct R(crate::R<DLLSTTDQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLLSTTDQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLLSTTDQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLLSTTDQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLYC_SL0` reader - DLL delay code DQ Slice 0"]
pub type DLYC_SL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYC_SL1` reader - DLL delay code DQ Slice 1"]
pub type DLYC_SL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYC_SL2` reader - DLL delay code DQ Slice 2"]
pub type DLYC_SL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYC_SL3` reader - DLL delay code DQ Slice 3"]
pub type DLYC_SL3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DLL delay code DQ Slice 0"]
    #[inline(always)]
    pub fn dlyc_sl0(&self) -> DLYC_SL0_R {
        DLYC_SL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DLL delay code DQ Slice 1"]
    #[inline(always)]
    pub fn dlyc_sl1(&self) -> DLYC_SL1_R {
        DLYC_SL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DLL delay code DQ Slice 2"]
    #[inline(always)]
    pub fn dlyc_sl2(&self) -> DLYC_SL2_R {
        DLYC_SL2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DLL delay code DQ Slice 3"]
    #[inline(always)]
    pub fn dlyc_sl3(&self) -> DLYC_SL3_R {
        DLYC_SL3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DLL Status Register for PHY Data Module 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllsttdq1](index.html) module"]
pub struct DLLSTTDQ1_SPEC;
impl crate::RegisterSpec for DLLSTTDQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dllsttdq1::R](R) reader structure"]
impl crate::Readable for DLLSTTDQ1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLLSTTDQ1 to value 0"]
impl crate::Resettable for DLLSTTDQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
