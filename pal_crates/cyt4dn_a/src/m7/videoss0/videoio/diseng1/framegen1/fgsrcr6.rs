#[doc = "Register `FGSRCR6` reader"]
pub struct R(crate::R<FGSRCR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGSRCR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGSRCR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGSRCR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGSRCR6` writer"]
pub struct W(crate::W<FGSRCR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGSRCR6_SPEC>;
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
impl From<crate::W<FGSRCR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGSRCR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCRANGEHIGH` reader - Sync range of horizontal and vertical skew regulation. Upper value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
pub type SYNCRANGEHIGH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SYNCRANGEHIGH` writer - Sync range of horizontal and vertical skew regulation. Upper value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
pub type SYNCRANGEHIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FGSRCR6_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - Sync range of horizontal and vertical skew regulation. Upper value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
    #[inline(always)]
    pub fn syncrangehigh(&self) -> SYNCRANGEHIGH_R {
        SYNCRANGEHIGH_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - Sync range of horizontal and vertical skew regulation. Upper value (signed value). If skew stays within programmed sync range, frame status is assumed synchronized."]
    #[inline(always)]
    #[must_use]
    pub fn syncrangehigh(&mut self) -> SYNCRANGEHIGH_W<0> {
        SYNCRANGEHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Skew Regulation Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgsrcr6](index.html) module"]
pub struct FGSRCR6_SPEC;
impl crate::RegisterSpec for FGSRCR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgsrcr6::R](R) reader structure"]
impl crate::Readable for FGSRCR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgsrcr6::W](W) writer structure"]
impl crate::Writable for FGSRCR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGSRCR6 to value 0x0190"]
impl crate::Resettable for FGSRCR6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0190;
}
