#[doc = "Register `INTR_VIDEOIO1` reader"]
pub struct R(crate::R<INTR_VIDEOIO1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_VIDEOIO1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_VIDEOIO1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_VIDEOIO1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_VIDEOIO1` writer"]
pub struct W(crate::W<INTR_VIDEOIO1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_VIDEOIO1_SPEC>;
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
impl From<crate::W<INTR_VIDEOIO1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_VIDEOIO1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_VIDEOIO1` reader - N/A"]
pub type INTR_VIDEOIO1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_VIDEOIO1` writer - N/A"]
pub type INTR_VIDEOIO1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_VIDEOIO1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    pub fn intr_videoio1(&self) -> INTR_VIDEOIO1_R {
        INTR_VIDEOIO1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn intr_videoio1(&mut self) -> INTR_VIDEOIO1_W<0> {
        INTR_VIDEOIO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio1](index.html) module"]
pub struct INTR_VIDEOIO1_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_videoio1::R](R) reader structure"]
impl crate::Readable for INTR_VIDEOIO1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_videoio1::W](W) writer structure"]
impl crate::Writable for INTR_VIDEOIO1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_VIDEOIO1 to value 0"]
impl crate::Resettable for INTR_VIDEOIO1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
