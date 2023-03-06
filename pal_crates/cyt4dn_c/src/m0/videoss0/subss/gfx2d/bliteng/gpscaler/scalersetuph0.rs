#[doc = "Register `SCALERSETUPH0` reader"]
pub struct R(crate::R<SCALERSETUPH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALERSETUPH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALERSETUPH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALERSETUPH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALERSETUPH0` writer"]
pub struct W(crate::W<SCALERSETUPH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALERSETUPH0_SPEC>;
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
impl From<crate::W<SCALERSETUPH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALERSETUPH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE_FACTOR_H` reader - Scale factor has to be greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor has to be programmed. (format is unsigned fix-point 1.19)"]
pub type SCALE_FACTOR_H_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCALE_FACTOR_H` writer - Scale factor has to be greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor has to be programmed. (format is unsigned fix-point 1.19)"]
pub type SCALE_FACTOR_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCALERSETUPH0_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - Scale factor has to be greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor has to be programmed. (format is unsigned fix-point 1.19)"]
    #[inline(always)]
    pub fn scale_factor_h(&self) -> SCALE_FACTOR_H_R {
        SCALE_FACTOR_H_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Scale factor has to be greater than 0.0 and less than or equal to 1.0. If scale_mode is UPSCALE then the inverse scale factor has to be programmed. (format is unsigned fix-point 1.19)"]
    #[inline(always)]
    #[must_use]
    pub fn scale_factor_h(&mut self) -> SCALE_FACTOR_H_W<0> {
        SCALE_FACTOR_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Phase interpolator setup.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalersetuph0](index.html) module"]
pub struct SCALERSETUPH0_SPEC;
impl crate::RegisterSpec for SCALERSETUPH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalersetuph0::R](R) reader structure"]
impl crate::Readable for SCALERSETUPH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalersetuph0::W](W) writer structure"]
impl crate::Writable for SCALERSETUPH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALERSETUPH0 to value 0x0008_0000"]
impl crate::Resettable for SCALERSETUPH0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
