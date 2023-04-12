#[doc = "Register `CFG_BIT_ERR` reader"]
pub struct R(crate::R<CFG_BIT_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_BIT_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_BIT_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_BIT_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIT_ERR` reader - N/A"]
pub type BIT_ERR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:9 - N/A"]
    #[inline(always)]
    pub fn bit_err(&self) -> BIT_ERR_R {
        BIT_ERR_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "CFG_BIT_ERR is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_bit_err](index.html) module"]
pub struct CFG_BIT_ERR_SPEC;
impl crate::RegisterSpec for CFG_BIT_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_bit_err::R](R) reader structure"]
impl crate::Readable for CFG_BIT_ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CFG_BIT_ERR to value 0"]
impl crate::Resettable for CFG_BIT_ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
