#[doc = "Register `PALETTE[%s]` reader"]
pub struct R(crate::R<PALETTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PALETTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PALETTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PALETTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PALETTE[%s]` writer"]
pub struct W(crate::W<PALETTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PALETTE_SPEC>;
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
impl From<crate::W<PALETTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PALETTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALETTEENTRY` reader - Shared palette entry with 24-bit. Color format can be defined by Fetch configuration."]
pub type PALETTEENTRY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PALETTEENTRY` writer - Shared palette entry with 24-bit. Color format can be defined by Fetch configuration."]
pub type PALETTEENTRY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PALETTE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Shared palette entry with 24-bit. Color format can be defined by Fetch configuration."]
    #[inline(always)]
    pub fn paletteentry(&self) -> PALETTEENTRY_R {
        PALETTEENTRY_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Shared palette entry with 24-bit. Color format can be defined by Fetch configuration."]
    #[inline(always)]
    #[must_use]
    pub fn paletteentry(&mut self) -> PALETTEENTRY_W<0> {
        PALETTEENTRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared Palette Memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [palette](index.html) module"]
pub struct PALETTE_SPEC;
impl crate::RegisterSpec for PALETTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [palette::R](R) reader structure"]
impl crate::Readable for PALETTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [palette::W](W) writer structure"]
impl crate::Writable for PALETTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PALETTE[%s]
to value 0"]
impl crate::Resettable for PALETTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
