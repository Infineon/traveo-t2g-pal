#[doc = "Register `INTR_GFX2D_MASKED` reader"]
pub struct R(crate::R<INTR_GFX2D_MASKED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_GFX2D_MASKED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_GFX2D_MASKED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_GFX2D_MASKED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTR_GFX2D_MASKED` reader - Status vector that generates the interrupt to the CPU if at least one bit of this field is set. This field corresponds to the bit-wise AND between INTR_GFX2D and INTR_GFX2D_MASK fields."]
pub type INTR_GFX2D_MASKED_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Status vector that generates the interrupt to the CPU if at least one bit of this field is set. This field corresponds to the bit-wise AND between INTR_GFX2D and INTR_GFX2D_MASK fields."]
    #[inline(always)]
    pub fn intr_gfx2d_masked(&self) -> INTR_GFX2D_MASKED_R {
        INTR_GFX2D_MASKED_R::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Interrupt masked register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_gfx2d_masked](index.html) module"]
pub struct INTR_GFX2D_MASKED_SPEC;
impl crate::RegisterSpec for INTR_GFX2D_MASKED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_gfx2d_masked::R](R) reader structure"]
impl crate::Readable for INTR_GFX2D_MASKED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTR_GFX2D_MASKED to value 0"]
impl crate::Resettable for INTR_GFX2D_MASKED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
