#[doc = "Register `CONFIG3` reader"]
pub struct R(crate::R<CONFIG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG3` writer"]
pub struct W(crate::W<CONFIG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG3_SPEC>;
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
impl From<crate::W<CONFIG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSCG_DEPTH` reader - N/A"]
pub type SSCG_DEPTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSCG_DEPTH` writer - N/A"]
pub type SSCG_DEPTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG3_SPEC, u16, u16, 10, O>;
#[doc = "Field `SSCG_RATE` reader - N/A"]
pub type SSCG_RATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSCG_RATE` writer - N/A"]
pub type SSCG_RATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG3_SPEC, u8, u8, 3, O>;
#[doc = "Field `SSCG_DITHER_EN` reader - Enables dithering during spreading. 0: disabled 1: enabled"]
pub type SSCG_DITHER_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_DITHER_EN` writer - Enables dithering during spreading. 0: disabled 1: enabled"]
pub type SSCG_DITHER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, bool, O>;
#[doc = "Field `SSCG_MODE` reader - N/A"]
pub type SSCG_MODE_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_MODE` writer - N/A"]
pub type SSCG_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, bool, O>;
#[doc = "Field `SSCG_EN` reader - Enables spreading mode. When using spreading, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
pub type SSCG_EN_R = crate::BitReader<bool>;
#[doc = "Field `SSCG_EN` writer - Enables spreading mode. When using spreading, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
pub type SSCG_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:9 - N/A"]
    #[inline(always)]
    pub fn sscg_depth(&self) -> SSCG_DEPTH_R {
        SSCG_DEPTH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18 - N/A"]
    #[inline(always)]
    pub fn sscg_rate(&self) -> SSCG_RATE_R {
        SSCG_RATE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Enables dithering during spreading. 0: disabled 1: enabled"]
    #[inline(always)]
    pub fn sscg_dither_en(&self) -> SSCG_DITHER_EN_R {
        SSCG_DITHER_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    pub fn sscg_mode(&self) -> SSCG_MODE_R {
        SSCG_MODE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables spreading mode. When using spreading, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
    #[inline(always)]
    pub fn sscg_en(&self) -> SSCG_EN_R {
        SSCG_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_depth(&mut self) -> SSCG_DEPTH_W<0> {
        SSCG_DEPTH_W::new(self)
    }
    #[doc = "Bits 16:18 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_rate(&mut self) -> SSCG_RATE_W<16> {
        SSCG_RATE_W::new(self)
    }
    #[doc = "Bit 24 - Enables dithering during spreading. 0: disabled 1: enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_dither_en(&mut self) -> SSCG_DITHER_EN_W<24> {
        SSCG_DITHER_EN_W::new(self)
    }
    #[doc = "Bit 28 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sscg_mode(&mut self) -> SSCG_MODE_W<28> {
        SSCG_MODE_W::new(self)
    }
    #[doc = "Bit 31 - Enables spreading mode. When using spreading, see CLK_PLL400M_CONFIG.LOCK_DELAY for an additional configuration requirement."]
    #[inline(always)]
    #[must_use]
    pub fn sscg_en(&mut self) -> SSCG_EN_W<31> {
        SSCG_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "400MHz PLL Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config3](index.html) module"]
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config3::R](R) reader structure"]
impl crate::Readable for CONFIG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config3::W](W) writer structure"]
impl crate::Writable for CONFIG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG3 to value 0"]
impl crate::Resettable for CONFIG3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
