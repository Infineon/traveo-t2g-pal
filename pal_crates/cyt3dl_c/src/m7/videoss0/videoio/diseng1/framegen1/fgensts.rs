#[doc = "Register `FGENSTS` reader"]
pub struct R(crate::R<FGENSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGENSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGENSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGENSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ENSTS` reader - Indicates the current operating mode of the frame generator. EnSts = 0 means frame generator is disabled. EnSts = 1 means frame generator is enabled. In contrast to the FgEn field of the FgEnable register this bit goes only 0 when all pending frames are processed."]
pub type ENSTS_R = crate::BitReader<bool>;
#[doc = "Field `PANICSTAT` reader - Current status of panic mode (0=normal operation mode, 1=panic mode; not locked)."]
pub type PANICSTAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates the current operating mode of the frame generator. EnSts = 0 means frame generator is disabled. EnSts = 1 means frame generator is enabled. In contrast to the FgEn field of the FgEnable register this bit goes only 0 when all pending frames are processed."]
    #[inline(always)]
    pub fn ensts(&self) -> ENSTS_R {
        ENSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Current status of panic mode (0=normal operation mode, 1=panic mode; not locked)."]
    #[inline(always)]
    pub fn panicstat(&self) -> PANICSTAT_R {
        PANICSTAT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "FrameGen Enable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgensts](index.html) module"]
pub struct FGENSTS_SPEC;
impl crate::RegisterSpec for FGENSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgensts::R](R) reader structure"]
impl crate::Readable for FGENSTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGENSTS to value 0"]
impl crate::Resettable for FGENSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
