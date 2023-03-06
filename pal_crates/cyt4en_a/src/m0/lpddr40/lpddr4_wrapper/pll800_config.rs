#[doc = "Register `PLL800_CONFIG` reader"]
pub struct R(crate::R<PLL800_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL800_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL800_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL800_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL800_CONFIG` writer"]
pub struct W(crate::W<PLL800_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL800_CONFIG_SPEC>;
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
impl From<crate::W<PLL800_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL800_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEEDBACK_DIV` reader - Control bits for feedback divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEEDBACK_DIV` writer - Control bits for feedback divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior)"]
pub type FEEDBACK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 8, O>;
#[doc = "Field `REFERENCE_DIV` reader - Control bits for reference divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 18: divide by 18 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFERENCE_DIV` writer - Control bits for reference divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 18: divide by 18 others: illegal (undefined behavior)"]
pub type REFERENCE_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `OUTPUT_DIV` reader - Control bits for Output divider. Set the divide value before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. ... 16: divide by 16. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTPUT_DIV` writer - Control bits for Output divider. Set the divide value before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. ... 16: divide by 16. >16: illegal (undefined behavior)"]
pub type OUTPUT_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `VCO_CLK_SEL` reader - Select source of PLL output clock. Set the register before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: vco_clk divide by 2: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV 1: vco_clk direct output: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV * 2 2,3: Output divider is used: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV / OUTPUT_DIV"]
pub type VCO_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCO_CLK_SEL` writer - Select source of PLL output clock. Set the register before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: vco_clk divide by 2: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV 1: vco_clk direct output: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV * 2 2,3: Output divider is used: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV / OUTPUT_DIV"]
pub type VCO_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `LOCK_DELAY` reader - N/A"]
pub type LOCK_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_DELAY` writer - N/A"]
pub type LOCK_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `WAIT_TIME` reader - Configures the seqeuencer for starting and stopping the PLL. Set the value depending on the external oscillator frequency before enabling the PLL. 0: for 4MHz 1: for >4MHz up to 8MHz 2: for >8MHz up to 16MHz 3: for >16MHz up to 33MHz Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type WAIT_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_TIME` writer - Configures the seqeuencer for starting and stopping the PLL. Set the value depending on the external oscillator frequency before enabling the PLL. 0: for 4MHz 1: for >4MHz up to 8MHz 2: for >8MHz up to 16MHz 3: for >16MHz up to 33MHz Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
pub type WAIT_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PLL800_CONFIG_SPEC, u8, u8, 2, O>;
#[doc = "Field `UNLOCK_ERROR_ENABLE` reader - Control bit for the fault generation based on a PLL UNLOCK_OCCURRED event. 0: No fault event is generated when UNLOCK_OCCURRED is set 1: Setting UNLOCK_OCCURRED generates a fault event"]
pub type UNLOCK_ERROR_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK_ERROR_ENABLE` writer - Control bit for the fault generation based on a PLL UNLOCK_OCCURRED event. 0: No fault event is generated when UNLOCK_OCCURRED is set 1: Setting UNLOCK_OCCURRED generates a fault event"]
pub type UNLOCK_ERROR_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL800_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Control bits for feedback divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn feedback_div(&self) -> FEEDBACK_DIV_R {
        FEEDBACK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 18: divide by 18 others: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn reference_div(&self) -> REFERENCE_DIV_R {
        REFERENCE_DIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. ... 16: divide by 16. >16: illegal (undefined behavior)"]
    #[inline(always)]
    pub fn output_div(&self) -> OUTPUT_DIV_R {
        OUTPUT_DIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 22:23 - Select source of PLL output clock. Set the register before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: vco_clk divide by 2: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV 1: vco_clk direct output: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV * 2 2,3: Output divider is used: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV / OUTPUT_DIV"]
    #[inline(always)]
    pub fn vco_clk_sel(&self) -> VCO_CLK_SEL_R {
        VCO_CLK_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 25:26 - N/A"]
    #[inline(always)]
    pub fn lock_delay(&self) -> LOCK_DELAY_R {
        LOCK_DELAY_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Configures the seqeuencer for starting and stopping the PLL. Set the value depending on the external oscillator frequency before enabling the PLL. 0: for 4MHz 1: for >4MHz up to 8MHz 2: for >8MHz up to 16MHz 3: for >16MHz up to 33MHz Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    pub fn wait_time(&self) -> WAIT_TIME_R {
        WAIT_TIME_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Control bit for the fault generation based on a PLL UNLOCK_OCCURRED event. 0: No fault event is generated when UNLOCK_OCCURRED is set 1: Setting UNLOCK_OCCURRED generates a fault event"]
    #[inline(always)]
    pub fn unlock_error_enable(&self) -> UNLOCK_ERROR_ENABLE_R {
        UNLOCK_ERROR_ENABLE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Control bits for feedback divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0-15: illegal (undefined behavior) 16: divide by 16 ... 200: divide by 200 >200: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn feedback_div(&mut self) -> FEEDBACK_DIV_W<0> {
        FEEDBACK_DIV_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits for reference divider. Set the divide value before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled. 0: illegal (undefined behavior) 1: divide by 1 ... 18: divide by 18 others: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn reference_div(&mut self) -> REFERENCE_DIV_W<8> {
        REFERENCE_DIV_W::new(self)
    }
    #[doc = "Bits 16:20 - Control bits for Output divider. Set the divide value before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: illegal (undefined behavior) 1: illegal (undefined behavior) 2: divide by 2. ... 16: divide by 16. >16: illegal (undefined behavior)"]
    #[inline(always)]
    #[must_use]
    pub fn output_div(&mut self) -> OUTPUT_DIV_W<16> {
        OUTPUT_DIV_W::new(self)
    }
    #[doc = "Bits 22:23 - Select source of PLL output clock. Set the register before enabling the output clock (OUTPUT_ENABLE=1). Do not change the setting when the PLL output clock is enabled. 0: vco_clk divide by 2: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV 1: vco_clk direct output: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV * 2 2,3: Output divider is used: fOUT = (fREF / REFERENCE_DIV) * FEEDBACK_DIV / OUTPUT_DIV"]
    #[inline(always)]
    #[must_use]
    pub fn vco_clk_sel(&mut self) -> VCO_CLK_SEL_W<22> {
        VCO_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 25:26 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn lock_delay(&mut self) -> LOCK_DELAY_W<25> {
        LOCK_DELAY_W::new(self)
    }
    #[doc = "Bits 28:29 - Configures the seqeuencer for starting and stopping the PLL. Set the value depending on the external oscillator frequency before enabling the PLL. 0: for 4MHz 1: for >4MHz up to 8MHz 2: for >8MHz up to 16MHz 3: for >16MHz up to 33MHz Set the register before enabling the PLL (ENABLE=1), and do not change it while PLL is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn wait_time(&mut self) -> WAIT_TIME_W<28> {
        WAIT_TIME_W::new(self)
    }
    #[doc = "Bit 30 - Control bit for the fault generation based on a PLL UNLOCK_OCCURRED event. 0: No fault event is generated when UNLOCK_OCCURRED is set 1: Setting UNLOCK_OCCURRED generates a fault event"]
    #[inline(always)]
    #[must_use]
    pub fn unlock_error_enable(&mut self) -> UNLOCK_ERROR_ENABLE_W<30> {
        UNLOCK_ERROR_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "800MHz PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll800_config](index.html) module"]
pub struct PLL800_CONFIG_SPEC;
impl crate::RegisterSpec for PLL800_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll800_config::R](R) reader structure"]
impl crate::Readable for PLL800_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll800_config::W](W) writer structure"]
impl crate::Writable for PLL800_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL800_CONFIG to value 0x7002_01c8"]
impl crate::Resettable for PLL800_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x7002_01c8;
}
