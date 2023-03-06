#[doc = "Register `FGSFIFOMIN` reader"]
pub struct R(crate::R<FGSFIFOMIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSFIFOMIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSFIFOMIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSFIFOMIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SFIFOMIN` reader - Shows the minimal fill level of the secondary channel pixel FIFO. (For debugging purposes only)."]
pub type SFIFOMIN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Shows the minimal fill level of the secondary channel pixel FIFO. (For debugging purposes only)."]
    #[inline(always)]
    pub fn sfifomin(&self) -> SFIFOMIN_R {
        SFIFOMIN_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "FrameGen Secondary FIFO Min Fill Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsfifomin](index.html) module"]
pub struct FGSFIFOMIN_SPEC;
impl crate::RegisterSpec for FGSFIFOMIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsfifomin::R](R) reader structure"]
impl crate::Readable for FGSFIFOMIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGSFIFOMIN to value 0"]
impl crate::Resettable for FGSFIFOMIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
