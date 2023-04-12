#[doc = "Register `FGSFIFOMAX` reader"]
pub struct R(crate::R<FGSFIFOMAX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSFIFOMAX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSFIFOMAX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSFIFOMAX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SFIFOMAX` reader - Shows the maximal fill level of the secondary channel pixel FIFO. (For debugging purposes only)."]
pub type SFIFOMAX_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Shows the maximal fill level of the secondary channel pixel FIFO. (For debugging purposes only)."]
    #[inline(always)]
    pub fn sfifomax(&self) -> SFIFOMAX_R {
        SFIFOMAX_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "FrameGen Secondary FIFO Max Fill Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsfifomax](index.html) module"]
pub struct FGSFIFOMAX_SPEC;
impl crate::RegisterSpec for FGSFIFOMAX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsfifomax::R](R) reader structure"]
impl crate::Readable for FGSFIFOMAX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGSFIFOMAX to value 0"]
impl crate::Resettable for FGSFIFOMAX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
