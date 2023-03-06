#[doc = "Register `SIGCRCBLUE0` reader"]
pub struct R(crate::R<SIGCRCBLUE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGCRCBLUE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGCRCBLUE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGCRCBLUE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIGCRCBLUE0` reader - CRC values from blue channel."]
pub type SIGCRCBLUE0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC values from blue channel."]
    #[inline(always)]
    pub fn sigcrcblue0(&self) -> SIGCRCBLUE0_R {
        SIGCRCBLUE0_R::new(self.bits)
    }
}
#[doc = "Measured signature of blue channel for evaluation window 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigcrcblue0](index.html) module"]
pub struct SIGCRCBLUE0_SPEC;
impl crate::RegisterSpec for SIGCRCBLUE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigcrcblue0::R](R) reader structure"]
impl crate::Readable for SIGCRCBLUE0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SIGCRCBLUE0 to value 0xffff_ffff"]
impl crate::Resettable for SIGCRCBLUE0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
