#[doc = "Register `TRANSLATIONVECTORX` reader"]
pub struct R(crate::R<TRANSLATIONVECTORX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANSLATIONVECTORX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANSLATIONVECTORX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANSLATIONVECTORX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANSLATIONVECTORX` writer"]
pub struct W(crate::W<TRANSLATIONVECTORX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANSLATIONVECTORX_SPEC>;
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
impl From<crate::W<TRANSLATIONVECTORX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANSLATIONVECTORX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANSLATIONX` reader - Horizontal translation (X), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the XOffset field of the LayerOffset register for subsequent Blit operation. (format is signed fix-point 16.6)"]
pub type TRANSLATIONX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TRANSLATIONX` writer - Horizontal translation (X), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the XOffset field of the LayerOffset register for subsequent Blit operation. (format is signed fix-point 16.6)"]
pub type TRANSLATIONX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRANSLATIONVECTORX_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:21 - Horizontal translation (X), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the XOffset field of the LayerOffset register for subsequent Blit operation. (format is signed fix-point 16.6)"]
    #[inline(always)]
    pub fn translationx(&self) -> TRANSLATIONX_R {
        TRANSLATIONX_R::new(self.bits & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:21 - Horizontal translation (X), given in signed 16.6 fixed-point format. Fractional part is used for processing. The integer part modifies the XOffset field of the LayerOffset register for subsequent Blit operation. (format is signed fix-point 16.6)"]
    #[inline(always)]
    #[must_use]
    pub fn translationx(&mut self) -> TRANSLATIONX_W<0> {
        TRANSLATIONX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Horizontal translation of the alpha frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [translationvectorx](index.html) module"]
pub struct TRANSLATIONVECTORX_SPEC;
impl crate::RegisterSpec for TRANSLATIONVECTORX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [translationvectorx::R](R) reader structure"]
impl crate::Readable for TRANSLATIONVECTORX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [translationvectorx::W](W) writer structure"]
impl crate::Writable for TRANSLATIONVECTORX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRANSLATIONVECTORX to value 0"]
impl crate::Resettable for TRANSLATIONVECTORX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
