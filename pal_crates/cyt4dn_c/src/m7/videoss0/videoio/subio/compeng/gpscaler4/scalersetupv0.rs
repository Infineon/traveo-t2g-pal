#[doc = "Register `SCALERSETUPV0` reader"]
pub struct R(crate::R<SCALERSETUPV0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALERSETUPV0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALERSETUPV0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALERSETUPV0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALERSETUPV0` writer"]
pub struct W(crate::W<SCALERSETUPV0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALERSETUPV0_SPEC>;
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
impl From<crate::W<SCALERSETUPV0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALERSETUPV0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE_FACTOR_V` reader - Scale factor is greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor have to be programmed. (format is unsigned fix-point 1.19)"]
pub type SCALE_FACTOR_V_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCALE_FACTOR_V` writer - Scale factor is greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor have to be programmed. (format is unsigned fix-point 1.19)"]
pub type SCALE_FACTOR_V_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCALERSETUPV0_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Scale factor is greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor have to be programmed. (format is unsigned fix-point 1.19)"]
    #[inline(always)]
    pub fn scale_factor_v(&self) -> SCALE_FACTOR_V_R {
        SCALE_FACTOR_V_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Scale factor is greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor have to be programmed. (format is unsigned fix-point 1.19)"]
    #[inline(always)]
    #[must_use]
    pub fn scale_factor_v(&mut self) -> SCALE_FACTOR_V_W<0> {
        SCALE_FACTOR_V_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase interpolator setup.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalersetupv0](index.html) module"]
pub struct SCALERSETUPV0_SPEC;
impl crate::RegisterSpec for SCALERSETUPV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalersetupv0::R](R) reader structure"]
impl crate::Readable for SCALERSETUPV0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalersetupv0::W](W) writer structure"]
impl crate::Writable for SCALERSETUPV0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALERSETUPV0 to value 0x0008_0000"]
impl crate::Resettable for SCALERSETUPV0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
