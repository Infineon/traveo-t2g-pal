#[doc = "Register `SIGCRCRED0` reader"]
pub struct R(crate::R<SIGCRCRED0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCRED0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCRED0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCRED0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCRED0` reader - CRC values from red channel."]
pub type SIGCRCRED0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC values from red channel."]
    #[inline(always)]
    pub fn sigcrcred0(&self) -> SIGCRCRED0_R {
        SIGCRCRED0_R::new(self.bits)
    }
}
#[doc = "Measured signature of red channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcred0](index.html) module"]
pub struct SIGCRCRED0_SPEC;
impl crate::RegisterSpec for SIGCRCRED0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcred0::R](R) reader structure"]
impl crate::Readable for SIGCRCRED0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCRED0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCRED0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
