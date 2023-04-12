#[doc = "Register `INTR_VIDEOIO1_MASK` reader"]
pub struct R(crate::R<INTR_VIDEOIO1_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_VIDEOIO1_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_VIDEOIO1_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_VIDEOIO1_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_VIDEOIO1_MASK` writer"]
pub struct W(crate::W<INTR_VIDEOIO1_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_VIDEOIO1_MASK_SPEC>;
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
impl From<crate::W<INTR_VIDEOIO1_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_VIDEOIO1_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_VIDEOIO1_MASK` reader - Interrupt mask vector. When a bit is set in this field, the related INTR_VIDEOIO1 status activates the interrupt signal."]
pub type INTR_VIDEOIO1_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_VIDEOIO1_MASK` writer - Interrupt mask vector. When a bit is set in this field, the related INTR_VIDEOIO1 status activates the interrupt signal."]
pub type INTR_VIDEOIO1_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_VIDEOIO1_MASK_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:27 - Interrupt mask vector. When a bit is set in this field, the related INTR_VIDEOIO1 status activates the interrupt signal."]
    #[inline(always)]
    pub fn intr_videoio1_mask(&self) -> INTR_VIDEOIO1_MASK_R {
        INTR_VIDEOIO1_MASK_R::new(self.bits & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:27 - Interrupt mask vector. When a bit is set in this field, the related INTR_VIDEOIO1 status activates the interrupt signal."]
    #[inline(always)]
    #[must_use]
    pub fn intr_videoio1_mask(&mut self) -> INTR_VIDEOIO1_MASK_W<0> {
        INTR_VIDEOIO1_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_videoio1_mask](index.html) module"]
pub struct INTR_VIDEOIO1_MASK_SPEC;
impl crate::RegisterSpec for INTR_VIDEOIO1_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_videoio1_mask::R](R) reader structure"]
impl crate::Readable for INTR_VIDEOIO1_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_videoio1_mask::W](W) writer structure"]
impl crate::Writable for INTR_VIDEOIO1_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_VIDEOIO1_MASK to value 0"]
impl crate::Resettable for INTR_VIDEOIO1_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
