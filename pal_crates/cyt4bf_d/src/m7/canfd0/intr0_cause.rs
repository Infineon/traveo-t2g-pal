#[doc = "Register `INTR0_CAUSE` reader"]
pub struct R(crate::R<INTR0_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR0_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR0_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR0_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INT0` reader - Show pending m_ttcan_int0 of each channel"]
pub type INT0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Show pending m_ttcan_int0 of each channel"]
    #[inline(always)]
    pub fn int0(&self) -> INT0_R {
        INT0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Consolidated interrupt0 cause register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr0_cause](index.html) module"]
pub struct INTR0_CAUSE_SPEC;
impl crate::RegisterSpec for INTR0_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr0_cause::R](R) reader structure"]
impl crate::Readable for INTR0_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR0_CAUSE to value 0"]
impl crate::Resettable for INTR0_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
