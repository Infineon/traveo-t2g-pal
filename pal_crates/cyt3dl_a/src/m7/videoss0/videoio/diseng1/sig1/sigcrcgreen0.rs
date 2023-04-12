#[doc = "Register `SIGCRCGREEN0` reader"]
pub struct R(crate::R<SIGCRCGREEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCGREEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCGREEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCGREEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCGREEN0` reader - CRC values from green channel."]
pub type SIGCRCGREEN0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC values from green channel."]
    #[inline(always)]
    pub fn sigcrcgreen0(&self) -> SIGCRCGREEN0_R {
        SIGCRCGREEN0_R::new(self.bits)
    }
}
#[doc = "Measured signature of green channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcgreen0](index.html) module"]
pub struct SIGCRCGREEN0_SPEC;
impl crate::RegisterSpec for SIGCRCGREEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcgreen0::R](R) reader structure"]
impl crate::Readable for SIGCRCGREEN0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCGREEN0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCGREEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
