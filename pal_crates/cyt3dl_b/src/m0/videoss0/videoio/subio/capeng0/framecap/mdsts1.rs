#[doc = "Register `MDSTS1` reader"]
pub struct R(crate::R<MDSTS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDSTS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDSTS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDSTS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `POSITIONX` reader - the current X position of the capture stream."]
pub type POSITIONX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POSITIONY` reader - the current Y position of the capture stream."]
pub type POSITIONY_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - the current X position of the capture stream."]
    #[inline(always)]
    pub fn positionx(&self) -> POSITIONX_R {
        POSITIONX_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - the current Y position of the capture stream."]
    #[inline(always)]
    pub fn positiony(&self) -> POSITIONY_R {
        POSITIONY_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "FrameCap mode detection status register. The current (x,y) position of the capture stream (with respect to the output).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsts1](index.html) module"]
pub struct MDSTS1_SPEC;
impl crate::RegisterSpec for MDSTS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdsts1::R](R) reader structure"]
impl crate::Readable for MDSTS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDSTS1 to value 0"]
impl crate::Resettable for MDSTS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
