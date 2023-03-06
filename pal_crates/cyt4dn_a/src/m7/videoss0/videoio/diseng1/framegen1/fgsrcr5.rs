#[doc = "Register `FGSRCR5` reader"]
pub struct R(crate::R<FGSRCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR5` writer"]
pub struct W(crate::W<FGSRCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR5_SPEC>;
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
impl From<crate::W<FGSRCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCRANGELOW` reader - Sync range of horizontal and vertical skew regulation. Lower value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
pub type SYNCRANGELOW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCRANGELOW` writer - Sync range of horizontal and vertical skew regulation. Lower value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
pub type SYNCRANGELOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FGSRCR5_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Sync range of horizontal and vertical skew regulation. Lower value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
    #[inline(always)]
    pub fn syncrangelow(&self) -> SYNCRANGELOW_R {
        SYNCRANGELOW_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Sync range of horizontal and vertical skew regulation. Lower value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
    #[inline(always)]
    #[must_use]
    pub fn syncrangelow(&mut self) -> SYNCRANGELOW_W<0> {
        SYNCRANGELOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr5](index.html) module"]
pub struct FGSRCR5_SPEC;
impl crate::RegisterSpec for FGSRCR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr5::R](R) reader structure"]
impl crate::Readable for FGSRCR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr5::W](W) writer structure"]
impl crate::Writable for FGSRCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR5 to value 0"]
impl crate::Resettable for FGSRCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
