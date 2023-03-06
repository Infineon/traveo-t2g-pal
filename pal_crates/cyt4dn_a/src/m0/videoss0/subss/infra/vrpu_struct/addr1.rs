#[doc = "Register `ADDR1` reader"]
pub struct R(crate::R<ADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDR1_SUBREGION_DISABLE` reader - See CPUSS."]
pub type ADDR1_SUBREGION_DISABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR1_ADDR24` reader - See CPUSS."]
pub type ADDR1_ADDR24_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - See CPUSS."]
    #[inline(always)]
    pub fn addr1_subregion_disable(&self) -> ADDR1_SUBREGION_DISABLE_R {
        ADDR1_SUBREGION_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - See CPUSS."]
    #[inline(always)]
    pub fn addr1_addr24(&self) -> ADDR1_ADDR24_R {
        ADDR1_ADDR24_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "ADDR1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr1](index.html) module"]
pub struct ADDR1_SPEC;
impl crate::RegisterSpec for ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr1::R](R) reader structure"]
impl crate::Readable for ADDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for ADDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
