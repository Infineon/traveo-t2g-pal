#[doc = "Register `GFX2D_CTL` reader"]
pub struct R(crate::R<GFX2D_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFX2D_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFX2D_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFX2D_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFX2D_CTL` writer"]
pub struct W(crate::W<GFX2D_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFX2D_CTL_SPEC>;
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
impl From<crate::W<GFX2D_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFX2D_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GFX2D_ENABLED` reader - If disabled, the Graphics 2D Core is put on reset and its clock input gated."]
pub type GFX2D_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `GFX2D_ENABLED` writer - If disabled, the Graphics 2D Core is put on reset and its clock input gated."]
pub type GFX2D_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, GFX2D_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - If disabled, the Graphics 2D Core is put on reset and its clock input gated."]
    #[inline(always)]
    pub fn gfx2d_enabled(&self) -> GFX2D_ENABLED_R {
        GFX2D_ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - If disabled, the Graphics 2D Core is put on reset and its clock input gated."]
    #[inline(always)]
    #[must_use]
    pub fn gfx2d_enabled(&mut self) -> GFX2D_ENABLED_W<31> {
        GFX2D_ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control Register for Graphics 2D Core\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfx2d_ctl](index.html) module"]
pub struct GFX2D_CTL_SPEC;
impl crate::RegisterSpec for GFX2D_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfx2d_ctl::R](R) reader structure"]
impl crate::Readable for GFX2D_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfx2d_ctl::W](W) writer structure"]
impl crate::Writable for GFX2D_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GFX2D_CTL to value 0x8000_0000"]
impl crate::Resettable for GFX2D_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
