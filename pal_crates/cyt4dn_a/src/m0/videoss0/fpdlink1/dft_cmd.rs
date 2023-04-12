#[doc = "Register `DFT_CMD` reader"]
pub struct R(crate::R<DFT_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFT_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFT_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFT_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFT_CMD` writer"]
pub struct W(crate::W<DFT_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFT_CMD_SPEC>;
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
impl From<crate::W<DFT_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFT_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADFT_EN` reader - Enable analog dft (analog test bus)"]
pub type ADFT_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADFT_EN` writer - Enable analog dft (analog test bus)"]
pub type ADFT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFT_CMD_SPEC, bool, O>;
#[doc = "Field `BYPASS_PLL` reader - BYPASS_PLL, use external clock"]
pub type BYPASS_PLL_R = crate::BitReader<BYPASS_PLL_A>;
#[doc = "BYPASS_PLL, use external clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_PLL_A {
    #[doc = "0: CLKOUT is the output of internal PLL"]
    BYPASS_DISABLED = 0,
    #[doc = "1: CLKOUT equals CLKEXT"]
    BYPASS_ENABLED = 1,
}
impl From<BYPASS_PLL_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_PLL_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_PLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_PLL_A {
        match self.bits {
            false => BYPASS_PLL_A::BYPASS_DISABLED,
            true => BYPASS_PLL_A::BYPASS_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS_DISABLED`"]
    #[inline(always)]
    pub fn is_bypass_disabled(&self) -> bool {
        *self == BYPASS_PLL_A::BYPASS_DISABLED
    }
    #[doc = "Checks if the value of the field is `BYPASS_ENABLED`"]
    #[inline(always)]
    pub fn is_bypass_enabled(&self) -> bool {
        *self == BYPASS_PLL_A::BYPASS_ENABLED
    }
}
#[doc = "Field `BYPASS_PLL` writer - BYPASS_PLL, use external clock"]
pub type BYPASS_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFT_CMD_SPEC, BYPASS_PLL_A, O>;
impl<'a, const O: u8> BYPASS_PLL_W<'a, O> {
    #[doc = "CLKOUT is the output of internal PLL"]
    #[inline(always)]
    pub fn bypass_disabled(self) -> &'a mut W {
        self.variant(BYPASS_PLL_A::BYPASS_DISABLED)
    }
    #[doc = "CLKOUT equals CLKEXT"]
    #[inline(always)]
    pub fn bypass_enabled(self) -> &'a mut W {
        self.variant(BYPASS_PLL_A::BYPASS_ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Enable analog dft (analog test bus)"]
    #[inline(always)]
    pub fn adft_en(&self) -> ADFT_EN_R {
        ADFT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - BYPASS_PLL, use external clock"]
    #[inline(always)]
    pub fn bypass_pll(&self) -> BYPASS_PLL_R {
        BYPASS_PLL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable analog dft (analog test bus)"]
    #[inline(always)]
    #[must_use]
    pub fn adft_en(&mut self) -> ADFT_EN_W<0> {
        ADFT_EN_W::new(self)
    }
    #[doc = "Bit 4 - BYPASS_PLL, use external clock"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_pll(&mut self) -> BYPASS_PLL_W<4> {
        BYPASS_PLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFT control register for fpdlink\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dft_cmd](index.html) module"]
pub struct DFT_CMD_SPEC;
impl crate::RegisterSpec for DFT_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dft_cmd::R](R) reader structure"]
impl crate::Readable for DFT_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dft_cmd::W](W) writer structure"]
impl crate::Writable for DFT_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFT_CMD to value 0"]
impl crate::Resettable for DFT_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
