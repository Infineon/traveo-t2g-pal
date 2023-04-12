#[doc = "Register `PLL800_CLOCK_CTL` reader"]
pub struct R(crate::R<PLL800_CLOCK_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL800_CLOCK_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL800_CLOCK_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL800_CLOCK_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL800_CLOCK_CTL` writer"]
pub struct W(crate::W<PLL800_CLOCK_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL800_CLOCK_CTL_SPEC>;
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
impl From<crate::W<PLL800_CLOCK_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL800_CLOCK_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for PLL. Certain PLL800_CONFIG and PLL800_CONFIG2 registers are not allowed to be modified when the PLL is enabled (see description of individual configuration registers). 0: Block is disabled. 1: Block is enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for PLL. Certain PLL800_CONFIG and PLL800_CONFIG2 registers are not allowed to be modified when the PLL is enabled (see description of individual configuration registers). 0: Block is disabled. 1: Block is enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL800_CLOCK_CTL_SPEC, bool, O>;
#[doc = "Field `OUTPUT_ENABLE` reader - Control bit for clock gate at PLL output. The output clock must only be enabled after PLL800_STATUS.LOCKED was confirmed to be 1. The output clock must be stopped before changing the OUTPUT_DIV or VCO_CLK_SEL setting. 0: PLL output clock is stopped 1: PLL output clock is enabled"]
pub type OUTPUT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `OUTPUT_ENABLE` writer - Control bit for clock gate at PLL output. The output clock must only be enabled after PLL800_STATUS.LOCKED was confirmed to be 1. The output clock must be stopped before changing the OUTPUT_DIV or VCO_CLK_SEL setting. 0: PLL output clock is stopped 1: PLL output clock is enabled"]
pub type OUTPUT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL800_CLOCK_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Master enable for PLL. Certain PLL800_CONFIG and PLL800_CONFIG2 registers are not allowed to be modified when the PLL is enabled (see description of individual configuration registers). 0: Block is disabled. 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control bit for clock gate at PLL output. The output clock must only be enabled after PLL800_STATUS.LOCKED was confirmed to be 1. The output clock must be stopped before changing the OUTPUT_DIV or VCO_CLK_SEL setting. 0: PLL output clock is stopped 1: PLL output clock is enabled"]
    #[inline(always)]
    pub fn output_enable(&self) -> OUTPUT_ENABLE_R {
        OUTPUT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master enable for PLL. Certain PLL800_CONFIG and PLL800_CONFIG2 registers are not allowed to be modified when the PLL is enabled (see description of individual configuration registers). 0: Block is disabled. 1: Block is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Control bit for clock gate at PLL output. The output clock must only be enabled after PLL800_STATUS.LOCKED was confirmed to be 1. The output clock must be stopped before changing the OUTPUT_DIV or VCO_CLK_SEL setting. 0: PLL output clock is stopped 1: PLL output clock is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn output_enable(&mut self) -> OUTPUT_ENABLE_W<1> {
        OUTPUT_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "800MHz PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll800_clock_ctl](index.html) module"]
pub struct PLL800_CLOCK_CTL_SPEC;
impl crate::RegisterSpec for PLL800_CLOCK_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll800_clock_ctl::R](R) reader structure"]
impl crate::Readable for PLL800_CLOCK_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll800_clock_ctl::W](W) writer structure"]
impl crate::Writable for PLL800_CLOCK_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL800_CLOCK_CTL to value 0"]
impl crate::Resettable for PLL800_CLOCK_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
