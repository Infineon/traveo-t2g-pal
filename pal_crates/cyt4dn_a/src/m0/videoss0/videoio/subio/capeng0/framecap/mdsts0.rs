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
#[doc = "Register `MDSTS0` writer"]
pub struct W(crate::W<MDSTS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDSTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MDSTS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDSTS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVEWIDTH` reader - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
pub type ACTIVEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVEWIDTH` writer - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
pub type ACTIVEWIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDSTS0_SPEC, u16, u16, 14, O>;
#[doc = "Field `ACTIVEHEIGHT` reader - Frame height of active part of the video frame detected by the mode detection.The status can be read when MdrCmrDone=1."]
pub type ACTIVEHEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVEHEIGHT` writer - Frame height of active part of the video frame detected by the mode detection.The status can be read when MdrCmrDone=1."]
pub type ACTIVEHEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MDSTS0_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    pub fn activewidth(&self) -> ACTIVEWIDTH_R {
        ACTIVEWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 15:28 - Frame height of active part of the video frame detected by the mode detection.The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    pub fn activeheight(&self) -> ACTIVEHEIGHT_R {
        ACTIVEHEIGHT_R::new(((self.bits >> 15) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame width of active part of the video frame detected by the mode detection. The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    #[must_use]
    pub fn activewidth(&mut self) -> ACTIVEWIDTH_W<0> {
        ACTIVEWIDTH_W::new(self)
    }
    #[doc = "Bits 15:28 - Frame height of active part of the video frame detected by the mode detection.The status can be read when MdrCmrDone=1."]
    #[inline(always)]
    #[must_use]
    pub fn activeheight(&mut self) -> ACTIVEHEIGHT_W<15> {
        ACTIVEHEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameCap mode detection status register for the active part of the video frames.(bit locked when MdrCmrDone=1).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdsts0](index.html) module"]
pub struct MDSTS0_SPEC;
impl crate::RegisterSpec for MDSTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdsts0::R](R) reader structure"]
impl crate::Readable for MDSTS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdsts0::W](W) writer structure"]
impl crate::Writable for MDSTS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDSTS0 to value 0"]
impl crate::Resettable for MDSTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
