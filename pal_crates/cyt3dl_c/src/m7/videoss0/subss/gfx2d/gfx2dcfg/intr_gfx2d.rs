#[doc = "Register `INTR_GFX2D` reader"]
pub struct R(crate::R<INTR_GFX2D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_GFX2D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_GFX2D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_GFX2D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_GFX2D` writer"]
pub struct W(crate::W<INTR_GFX2D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_GFX2D_SPEC>;
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
impl From<crate::W<INTR_GFX2D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_GFX2D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_GFX2D` reader - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_GFX2D_SET is written. By writing 1 the corresponding bit is cleared."]
pub type INTR_GFX2D_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_GFX2D` writer - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_GFX2D_SET is written. By writing 1 the corresponding bit is cleared."]
pub type INTR_GFX2D_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_GFX2D_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_GFX2D_SET is written. By writing 1 the corresponding bit is cleared."]
    #[inline(always)]
    pub fn intr_gfx2d(&self) -> INTR_GFX2D_R {
        INTR_GFX2D_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Interrupt status vector. A bit is set when the related event occurs or the corresponding bit of INTR_GFX2D_SET is written. By writing 1 the corresponding bit is cleared."]
    #[inline(always)]
    #[must_use]
    pub fn intr_gfx2d(&mut self) -> INTR_GFX2D_W<0> {
        INTR_GFX2D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt cause register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_gfx2d](index.html) module"]
pub struct INTR_GFX2D_SPEC;
impl crate::RegisterSpec for INTR_GFX2D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_gfx2d::R](R) reader structure"]
impl crate::Readable for INTR_GFX2D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_gfx2d::W](W) writer structure"]
impl crate::Writable for INTR_GFX2D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_GFX2D to value 0"]
impl crate::Resettable for INTR_GFX2D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
