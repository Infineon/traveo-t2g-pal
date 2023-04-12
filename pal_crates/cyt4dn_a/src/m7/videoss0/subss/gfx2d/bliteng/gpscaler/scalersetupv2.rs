#[doc = "Register `SCALERSETUPV2` reader"]
pub struct R(crate::R<SCALERSETUPV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALERSETUPV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALERSETUPV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALERSETUPV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALERSETUPV2` writer"]
pub struct W(crate::W<SCALERSETUPV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALERSETUPV2_SPEC>;
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
impl From<crate::W<SCALERSETUPV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALERSETUPV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE_OFFSET_V1` reader - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top."]
pub type PHASE_OFFSET_V1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHASE_OFFSET_V1` writer - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top."]
pub type PHASE_OFFSET_V1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCALERSETUPV2_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top."]
    #[inline(always)]
    pub fn phase_offset_v1(&self) -> PHASE_OFFSET_V1_R {
        PHASE_OFFSET_V1_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top."]
    #[inline(always)]
    #[must_use]
    pub fn phase_offset_v1(&mut self) -> PHASE_OFFSET_V1_W<0> {
        PHASE_OFFSET_V1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase interpolator setup, selected if input field polarity is 1 and output field polarity is 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalersetupv2](index.html) module"]
pub struct SCALERSETUPV2_SPEC;
impl crate::RegisterSpec for SCALERSETUPV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalersetupv2::R](R) reader structure"]
impl crate::Readable for SCALERSETUPV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalersetupv2::W](W) writer structure"]
impl crate::Writable for SCALERSETUPV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALERSETUPV2 to value 0"]
impl crate::Resettable for SCALERSETUPV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
