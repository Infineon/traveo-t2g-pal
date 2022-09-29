#[doc = "Register `INTR1_CAUSE` reader"]
pub struct R(crate::R<INTR1_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR1_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR1_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR1_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT1` reader - Show pending m_ttcan_int1 of each channel"]
pub type INT1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Show pending m_ttcan_int1 of each channel"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Consolidated interrupt1 cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr1_cause](index.html) module"]
pub struct INTR1_CAUSE_SPEC;
impl crate::RegisterSpec for INTR1_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr1_cause::R](R) reader structure"]
impl crate::Readable for INTR1_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR1_CAUSE to value 0"]
impl crate::Resettable for INTR1_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
