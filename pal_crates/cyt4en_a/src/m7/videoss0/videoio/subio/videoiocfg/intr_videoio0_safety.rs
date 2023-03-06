#[doc = "Register `INTR_VIDEOIO0_SAFETY` reader"]
pub struct R(crate::R<INTR_VIDEOIO0_SAFETY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_VIDEOIO0_SAFETY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_VIDEOIO0_SAFETY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_VIDEOIO0_SAFETY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_VIDEOIO0_SAFETY` writer"]
pub struct W(crate::W<INTR_VIDEOIO0_SAFETY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_VIDEOIO0_SAFETY_SPEC>;
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
impl From<crate::W<INTR_VIDEOIO0_SAFETY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_VIDEOIO0_SAFETY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_VIDEOIO0_SAFETY` reader - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit mapping is same like in INTR_VIDEOIO0."]
pub type INTR_VIDEOIO0_SAFETY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_VIDEOIO0_SAFETY` writer - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit mapping is same like in INTR_VIDEOIO0."]
pub type INTR_VIDEOIO0_SAFETY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_VIDEOIO0_SAFETY_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit mapping is same like in INTR_VIDEOIO0."]
    #[inline(always)]
    pub fn intr_videoio0_safety(&self) -> INTR_VIDEOIO0_SAFETY_R {
        INTR_VIDEOIO0_SAFETY_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_VIDEOIO0_SET is written. By writing 1 the corresponding bit is cleared. Bit mapping is same like in INTR_VIDEOIO0."]
    #[inline(always)]
    #[must_use]
    pub fn intr_videoio0_safety(&mut self) -> INTR_VIDEOIO0_SAFETY_W<0> {
        INTR_VIDEOIO0_SAFETY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio0_safety](index.html) module"]
pub struct INTR_VIDEOIO0_SAFETY_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO0_SAFETY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_videoio0_safety::R](R) reader structure"]
impl crate::Readable for INTR_VIDEOIO0_SAFETY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_videoio0_safety::W](W) writer structure"]
impl crate::Writable for INTR_VIDEOIO0_SAFETY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_VIDEOIO0_SAFETY to value 0"]
impl crate::Resettable for INTR_VIDEOIO0_SAFETY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
