#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG` writer"]
pub struct W(crate::W<CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SPEC>;
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
impl From<crate::W<CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEEDBACK_DIV` reader - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 27 to 47, inclusive."]
pub type FEEDBACK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEEDBACK_DIV` writer - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 27 to 47, inclusive."]
pub type FEEDBACK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `REFERENCE_DIV` reader - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 16: divide by 16 others: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 1 to 4, inclusive."]
pub type REFERENCE_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFERENCE_DIV` writer - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 16: divide by 16 others: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 1 to 4, inclusive."]
pub type REFERENCE_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `LOCK_DELAY` reader - N/A"]
pub type LOCK_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_DELAY` writer - N/A"]
pub type LOCK_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `BYPASS_SEL` reader - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running. When changing BYPASS_SEL, do not turn off the reference clock or PLL clock for five cycles (whichever is slower)."]
pub type BYPASS_SEL_R = crate::FieldReader<u8, BYPASS_SEL_A>;
#[doc = "Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running. When changing BYPASS_SEL, do not turn off the reference clock or PLL clock for five cycles (whichever is slower).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYPASS_SEL_A {
    #[doc = "0: Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output. If ENABLE=0, automatically selects PLL reference input."]
    AUTO = 0,
    #[doc = "1: Similar to AUTO, except the clock is gated off when unlocked. This is compatible with clock supervision, because the supervisors allow no clock during startup (until a timeout occurs), and the clock targets the proper frequency whenever it is running. If ENABLE=0, no clock is output."]
    LOCKED_OR_NOTHING = 1,
    #[doc = "2: Select PLL reference input (bypass mode). Ignores lock indicator"]
    PLL_REF = 2,
    #[doc = "3: Select PLL output. Ignores lock indicator. If ENABLE=0, no clock is output."]
    PLL_OUT = 3,
}
impl From<BYPASS_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYPASS_SEL_A) -> Self {
        variant as _
    }
}
impl BYPASS_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_SEL_A {
        match self.bits {
            0 => BYPASS_SEL_A::AUTO,
            1 => BYPASS_SEL_A::LOCKED_OR_NOTHING,
            2 => BYPASS_SEL_A::PLL_REF,
            3 => BYPASS_SEL_A::PLL_OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == BYPASS_SEL_A::AUTO
    }
    #[doc = "Checks if the value of the field is `LOCKED_OR_NOTHING`"]
    #[inline(always)]
    pub fn is_locked_or_nothing(&self) -> bool {
        *self == BYPASS_SEL_A::LOCKED_OR_NOTHING
    }
    #[doc = "Checks if the value of the field is `PLL_REF`"]
    #[inline(always)]
    pub fn is_pll_ref(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_REF
    }
    #[doc = "Checks if the value of the field is `PLL_OUT`"]
    #[inline(always)]
    pub fn is_pll_out(&self) -> bool {
        *self == BYPASS_SEL_A::PLL_OUT
    }
}
#[doc = "Field `BYPASS_SEL` writer - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running. When changing BYPASS_SEL, do not turn off the reference clock or PLL clock for five cycles (whichever is slower)."]
pub type BYPASS_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONFIG_SPEC, u8, BYPASS_SEL_A, 2, O>;
impl<'a, const O: u8> BYPASS_SEL_W<'a, O> {
    #[doc = "Automatic using lock indicator. When unlocked, automatically selects PLL reference input (bypass mode). When locked, automatically selects PLL output. If ENABLE=0, automatically selects PLL reference input."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::AUTO)
    }
    #[doc = "Similar to AUTO, except the clock is gated off when unlocked. This is compatible with clock supervision, because the supervisors allow no clock during startup (until a timeout occurs), and the clock targets the proper frequency whenever it is running. If ENABLE=0, no clock is output."]
    #[inline(always)]
    pub fn locked_or_nothing(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::LOCKED_OR_NOTHING)
    }
    #[doc = "Select PLL reference input (bypass mode). Ignores lock indicator"]
    #[inline(always)]
    pub fn pll_ref(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::PLL_REF)
    }
    #[doc = "Select PLL output. Ignores lock indicator. If ENABLE=0, no clock is output."]
    #[inline(always)]
    pub fn pll_out(self) -> &'a mut W {
        self.variant(BYPASS_SEL_A::PLL_OUT)
    }
}
#[doc = "Field `ENABLE` reader - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. fOUT = (FEEDBACK_DIV + FRAC_EN*FRAC_DIV/2^24) * (fREF / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled. When the PLL disables, hardware controls the bypass mux as described in BYPASS_SEL, before disabling the PLL circuit. 1: Block is enabled"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. fOUT = (FEEDBACK_DIV + FRAC_EN*FRAC_DIV/2^24) * (fREF / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled. When the PLL disables, hardware controls the bypass mux as described in BYPASS_SEL, before disabling the PLL circuit. 1: Block is enabled"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 27 to 47, inclusive."]
    #[inline(always)]
    pub fn feedback_div(&self) -> FEEDBACK_DIV_R {
        FEEDBACK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 16: divide by 16 others: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 1 to 4, inclusive."]
    #[inline(always)]
    pub fn reference_div(&self) -> REFERENCE_DIV_R {
        REFERENCE_DIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&self) -> OUTPUT_DIV_R {
        OUTPUT_DIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 25:26 - N/A"]
    #[inline(always)]
    pub fn lock_delay(&self) -> LOCK_DELAY_R {
        LOCK_DELAY_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running. When changing BYPASS_SEL, do not turn off the reference clock or PLL clock for five cycles (whichever is slower)."]
    #[inline(always)]
    pub fn bypass_sel(&self) -> BYPASS_SEL_R {
        BYPASS_SEL_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. fOUT = (FEEDBACK_DIV + FRAC_EN*FRAC_DIV/2^24) * (fREF / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled. When the PLL disables, hardware controls the bypass mux as described in BYPASS_SEL, before disabling the PLL circuit. 1: Block is enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Control bits for feedback divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 27 to 47, inclusive."]
    #[inline(always)]
    #[must_use]
    pub fn feedback_div(&mut self) -> FEEDBACK_DIV_W<0> {
        FEEDBACK_DIV_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 16: divide by 16 others: illegal (undefined behavior) When using fractional mode, the jitter specs are met over the restricted range of 1 to 4, inclusive."]
    #[inline(always)]
    #[must_use]
    pub fn reference_div(&mut self) -> REFERENCE_DIV_W<8> {
        REFERENCE_DIV_W::new(self)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the PLL, and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. Suitable for direct usage as HFCLK source. ... 16: divide by 16. Suitable for direct usage as HFCLK source. >16: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn output_div(&mut self) -> OUTPUT_DIV_W<16> {
        OUTPUT_DIV_W::new(self)
    }
    #[doc = "Bits 25:26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn lock_delay(&mut self) -> LOCK_DELAY_W<25> {
        LOCK_DELAY_W::new(self)
    }
    #[doc = "Bits 28:29 - Bypass mux located just after PLL output. This selection is glitch-free and can be changed while the PLL is running. When changing BYPASS_SEL, do not turn off the reference clock or PLL clock for five cycles (whichever is slower)."]
    #[inline(always)]
    #[must_use]
    pub fn bypass_sel(&mut self) -> BYPASS_SEL_W<28> {
        BYPASS_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for PLL. Setup FEEDBACK_DIV, REFERENCE_DIV, and OUTPUT_DIV at least one cycle before setting ENABLE=1. fOUT = (FEEDBACK_DIV + FRAC_EN*FRAC_DIV/2^24) * (fREF / REFERENCE_DIV) / (OUTPUT_DIV) 0: Block is disabled. When the PLL disables, hardware controls the bypass mux as described in BYPASS_SEL, before disabling the PLL circuit. 1: Block is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "400MHz PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config::W](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0002_0116"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0116;
}
