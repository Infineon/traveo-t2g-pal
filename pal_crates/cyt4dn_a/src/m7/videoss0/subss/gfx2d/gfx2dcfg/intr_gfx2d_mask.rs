#[doc = "Register `INTR_GFX2D_MASK` reader"]
pub struct R(crate::R<INTR_GFX2D_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_GFX2D_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_GFX2D_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_GFX2D_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_GFX2D_MASK` writer"]
pub struct W(crate::W<INTR_GFX2D_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_GFX2D_MASK_SPEC>;
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
impl From<crate::W<INTR_GFX2D_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_GFX2D_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTR_GFX2D_MASK` reader - Interrupt mask vector. Please note that this enable vector does not affect the INTR0 register field. It only affects INTR_GFX2D_MASKED."]
pub type INTR_GFX2D_MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `INTR_GFX2D_MASK` writer - Interrupt mask vector. Please note that this enable vector does not affect the INTR0 register field. It only affects INTR_GFX2D_MASKED."]
pub type INTR_GFX2D_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INTR_GFX2D_MASK_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Interrupt mask vector. Please note that this enable vector does not affect the INTR0 register field. It only affects INTR_GFX2D_MASKED."]
    #[inline(always)]
    pub fn intr_gfx2d_mask(&self) -> INTR_GFX2D_MASK_R {
        INTR_GFX2D_MASK_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Interrupt mask vector. Please note that this enable vector does not affect the INTR0 register field. It only affects INTR_GFX2D_MASKED."]
    #[inline(always)]
    #[must_use]
    pub fn intr_gfx2d_mask(&mut self) -> INTR_GFX2D_MASK_W<0> {
        INTR_GFX2D_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_gfx2d_mask](index.html) module"]
pub struct INTR_GFX2D_MASK_SPEC;
impl crate::RegisterSpec for INTR_GFX2D_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_gfx2d_mask::R](R) reader structure"]
impl crate::Readable for INTR_GFX2D_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_gfx2d_mask::W](W) writer structure"]
impl crate::Writable for INTR_GFX2D_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_GFX2D_MASK to value 0"]
impl crate::Resettable for INTR_GFX2D_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
