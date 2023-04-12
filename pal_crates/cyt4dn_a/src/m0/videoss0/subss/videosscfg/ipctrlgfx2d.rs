#[doc = "Register `IPCTRLGFX2D` reader"]
pub struct R(crate::R<IPCTRLGFX2D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCTRLGFX2D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCTRLGFX2D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCTRLGFX2D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCTRLGFX2D` writer"]
pub struct W(crate::W<IPCTRLGFX2D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCTRLGFX2D_SPEC>;
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
impl From<crate::W<IPCTRLGFX2D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCTRLGFX2D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTLENABLED_GFX2D` reader - N/A"]
pub type CTLENABLED_GFX2D_R = crate::BitReader<bool>;
#[doc = "Field `CTLENABLED_GFX2D` writer - N/A"]
pub type CTLENABLED_GFX2D_W<'a, const O: u8> = crate::BitWriter<'a, u32, IPCTRLGFX2D_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn ctlenabled_gfx2d(&self) -> CTLENABLED_GFX2D_R {
        CTLENABLED_GFX2D_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ctlenabled_gfx2d(&mut self) -> CTLENABLED_GFX2D_W<31> {
        CTLENABLED_GFX2D_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Control Register for Graphics 2D Core\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipctrlgfx2d](index.html) module"]
pub struct IPCTRLGFX2D_SPEC;
impl crate::RegisterSpec for IPCTRLGFX2D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipctrlgfx2d::R](R) reader structure"]
impl crate::Readable for IPCTRLGFX2D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipctrlgfx2d::W](W) writer structure"]
impl crate::Writable for IPCTRLGFX2D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCTRLGFX2D to value 0x8000_0000"]
impl crate::Resettable for IPCTRLGFX2D_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
