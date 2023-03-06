#[doc = "Register `SCALERSETUPV1` reader"]
pub struct R(crate::R<SCALERSETUPV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALERSETUPV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALERSETUPV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALERSETUPV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALERSETUPV1` writer"]
pub struct W(crate::W<SCALERSETUPV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALERSETUPV1_SPEC>;
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
impl From<crate::W<SCALERSETUPV1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALERSETUPV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHASE_OFFSET_V` reader - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top. (format is signed fix-point 2.19)"]
pub type PHASE_OFFSET_V_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PHASE_OFFSET_V` writer - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top. (format is signed fix-point 2.19)"]
pub type PHASE_OFFSET_V_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCALERSETUPV1_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top. (format is signed fix-point 2.19)"]
    #[inline(always)]
    pub fn phase_offset_v(&self) -> PHASE_OFFSET_V_R {
        PHASE_OFFSET_V_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Phase offset has to be larger than -2 and smaller than +2. Depending on scale_mode this value is relative to input (UPSCALE) or output pixel (DOWNSCALE). A negative value shifts the image to the bottom, a positive one to the top. (format is signed fix-point 2.19)"]
    #[inline(always)]
    #[must_use]
    pub fn phase_offset_v(&mut self) -> PHASE_OFFSET_V_W<0> {
        PHASE_OFFSET_V_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase interpolator setup, selected if input and output field polarity is 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalersetupv1](index.html) module"]
pub struct SCALERSETUPV1_SPEC;
impl crate::RegisterSpec for SCALERSETUPV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalersetupv1::R](R) reader structure"]
impl crate::Readable for SCALERSETUPV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalersetupv1::W](W) writer structure"]
impl crate::Writable for SCALERSETUPV1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALERSETUPV1 to value 0"]
impl crate::Resettable for SCALERSETUPV1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
