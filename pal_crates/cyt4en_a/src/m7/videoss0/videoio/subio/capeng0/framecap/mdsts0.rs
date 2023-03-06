#[doc = "Register `MDSTS0` reader"]
pub struct R(crate::R<MDSTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDSTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDSTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVEWIDTH` reader - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
pub type ACTIVEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVEHEIGHT` reader - Frame height of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
pub type ACTIVEHEIGHT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    pub fn activewidth(&self) -> ACTIVEWIDTH_R {
        ACTIVEWIDTH_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Frame height of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    pub fn activeheight(&self) -> ACTIVEHEIGHT_R {
        ACTIVEHEIGHT_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "FrameCap mode detection status register for the active part of the video frames.(bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsts0](index.html) module"]
pub struct MDSTS0_SPEC;
impl crate::RegisterSpec for MDSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdsts0::R](R) reader structure"]
impl crate::Readable for MDSTS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MDSTS0 to value 0"]
impl crate::Resettable for MDSTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
