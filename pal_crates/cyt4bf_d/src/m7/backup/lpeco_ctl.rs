#[doc = "Register `LPECO_CTL` reader"]
pub struct R(crate::R<LPECO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPECO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPECO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPECO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPECO_CTL` writer"]
pub struct W(crate::W<LPECO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPECO_CTL_SPEC>;
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
impl From<crate::W<LPECO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPECO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPECO_CRANGE` reader - Specifies the load capacitance of the chosen crystal. 2'b00: load is in range \\[5pF, 10pF\\], 2'b01: load is in range (10pF, 15pF\\], 2'b10: load is in range (15pF, 20pF\\], 2'b11: load is in range (20pF, 25pF\\]"]
pub type LPECO_CRANGE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPECO_CRANGE` writer - Specifies the load capacitance of the chosen crystal. 2'b00: load is in range \\[5pF, 10pF\\], 2'b01: load is in range (10pF, 15pF\\], 2'b10: load is in range (15pF, 20pF\\], 2'b11: load is in range (20pF, 25pF\\]"]
pub type LPECO_CRANGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPECO_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPECO_FRANGE` reader - Specifies the crystal frequency range. 0: Crystal frequency is in range \\[4, 6) MHz 1: Crystal frequency is in range \\[6, 8\\]
MHz. Use this setting for 6 MHz."]
pub type LPECO_FRANGE_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_FRANGE` writer - Specifies the crystal frequency range. 0: Crystal frequency is in range \\[4, 6) MHz 1: Crystal frequency is in range \\[6, 8\\]
MHz. Use this setting for 6 MHz."]
pub type LPECO_FRANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPECO_CTL_SPEC, bool, O>;
#[doc = "Field `LPECO_AMP_SEL` reader - Selects the oscillation amplitude. WARNING: the crystal can be permanently damaged by selecting an amplitude that exceeds the crystal limits. 0: maximum amplitude is 1.35V. This is the lowest power setting. 1: maximum amplitude is 1.8V. This is the lowest jitter setting."]
pub type LPECO_AMP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_AMP_SEL` writer - Selects the oscillation amplitude. WARNING: the crystal can be permanently damaged by selecting an amplitude that exceeds the crystal limits. 0: maximum amplitude is 1.35V. This is the lowest power setting. 1: maximum amplitude is 1.8V. This is the lowest jitter setting."]
pub type LPECO_AMP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPECO_CTL_SPEC, bool, O>;
#[doc = "Field `LPECO_DIV_ENABLE` reader - LPECO prescaler enable. Do not set this to '1' when LPECO_EN==0. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. HW sets the LPECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_LPECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
pub type LPECO_DIV_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_DIV_ENABLE` writer - LPECO prescaler enable. Do not set this to '1' when LPECO_EN==0. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. HW sets the LPECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_LPECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
pub type LPECO_DIV_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPECO_CTL_SPEC, bool, O>;
#[doc = "Field `LPECO_AMPDET_EN` reader - Minimum amplitude detector enable/disable. Ignored when LPECO_EN==0. 0: Initially enabled, and then automatically disabled when amplitude detector detects sufficient amplitude. 1: Keep minimum amplitude detector enabled as long as LPECO is enabled."]
pub type LPECO_AMPDET_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_AMPDET_EN` writer - Minimum amplitude detector enable/disable. Ignored when LPECO_EN==0. 0: Initially enabled, and then automatically disabled when amplitude detector detects sufficient amplitude. 1: Keep minimum amplitude detector enabled as long as LPECO is enabled."]
pub type LPECO_AMPDET_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPECO_CTL_SPEC, bool, O>;
#[doc = "Field `LPECO_EN` reader - Master enable for LPECO oscillator. This also disables the LPECO prescaler."]
pub type LPECO_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPECO_EN` writer - Master enable for LPECO oscillator. This also disables the LPECO prescaler."]
pub type LPECO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPECO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 4:5 - Specifies the load capacitance of the chosen crystal. 2'b00: load is in range \\[5pF, 10pF\\], 2'b01: load is in range (10pF, 15pF\\], 2'b10: load is in range (15pF, 20pF\\], 2'b11: load is in range (20pF, 25pF\\]"]
    #[inline(always)]
    pub fn lpeco_crange(&self) -> LPECO_CRANGE_R {
        LPECO_CRANGE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Specifies the crystal frequency range. 0: Crystal frequency is in range \\[4, 6) MHz 1: Crystal frequency is in range \\[6, 8\\]
MHz. Use this setting for 6 MHz."]
    #[inline(always)]
    pub fn lpeco_frange(&self) -> LPECO_FRANGE_R {
        LPECO_FRANGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the oscillation amplitude. WARNING: the crystal can be permanently damaged by selecting an amplitude that exceeds the crystal limits. 0: maximum amplitude is 1.35V. This is the lowest power setting. 1: maximum amplitude is 1.8V. This is the lowest jitter setting."]
    #[inline(always)]
    pub fn lpeco_amp_sel(&self) -> LPECO_AMP_SEL_R {
        LPECO_AMP_SEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 28 - LPECO prescaler enable. Do not set this to '1' when LPECO_EN==0. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. HW sets the LPECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_LPECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
    #[inline(always)]
    pub fn lpeco_div_enable(&self) -> LPECO_DIV_ENABLE_R {
        LPECO_DIV_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Minimum amplitude detector enable/disable. Ignored when LPECO_EN==0. 0: Initially enabled, and then automatically disabled when amplitude detector detects sufficient amplitude. 1: Keep minimum amplitude detector enabled as long as LPECO is enabled."]
    #[inline(always)]
    pub fn lpeco_ampdet_en(&self) -> LPECO_AMPDET_EN_R {
        LPECO_AMPDET_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for LPECO oscillator. This also disables the LPECO prescaler."]
    #[inline(always)]
    pub fn lpeco_en(&self) -> LPECO_EN_R {
        LPECO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - Specifies the load capacitance of the chosen crystal. 2'b00: load is in range \\[5pF, 10pF\\], 2'b01: load is in range (10pF, 15pF\\], 2'b10: load is in range (15pF, 20pF\\], 2'b11: load is in range (20pF, 25pF\\]"]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_crange(&mut self) -> LPECO_CRANGE_W<4> {
        LPECO_CRANGE_W::new(self)
    }
    #[doc = "Bit 8 - Specifies the crystal frequency range. 0: Crystal frequency is in range \\[4, 6) MHz 1: Crystal frequency is in range \\[6, 8\\]
MHz. Use this setting for 6 MHz."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_frange(&mut self) -> LPECO_FRANGE_W<8> {
        LPECO_FRANGE_W::new(self)
    }
    #[doc = "Bit 12 - Selects the oscillation amplitude. WARNING: the crystal can be permanently damaged by selecting an amplitude that exceeds the crystal limits. 0: maximum amplitude is 1.35V. This is the lowest power setting. 1: maximum amplitude is 1.8V. This is the lowest jitter setting."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_amp_sel(&mut self) -> LPECO_AMP_SEL_W<12> {
        LPECO_AMP_SEL_W::new(self)
    }
    #[doc = "Bit 28 - LPECO prescaler enable. Do not set this to '1' when LPECO_EN==0. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. HW sets the LPECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_LPECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_div_enable(&mut self) -> LPECO_DIV_ENABLE_W<28> {
        LPECO_DIV_ENABLE_W::new(self)
    }
    #[doc = "Bit 30 - Minimum amplitude detector enable/disable. Ignored when LPECO_EN==0. 0: Initially enabled, and then automatically disabled when amplitude detector detects sufficient amplitude. 1: Keep minimum amplitude detector enabled as long as LPECO is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_ampdet_en(&mut self) -> LPECO_AMPDET_EN_W<30> {
        LPECO_AMPDET_EN_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for LPECO oscillator. This also disables the LPECO prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn lpeco_en(&mut self) -> LPECO_EN_W<31> {
        LPECO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low-power external crystal oscillator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpeco_ctl](index.html) module"]
pub struct LPECO_CTL_SPEC;
impl crate::RegisterSpec for LPECO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpeco_ctl::R](R) reader structure"]
impl crate::Readable for LPECO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpeco_ctl::W](W) writer structure"]
impl crate::Writable for LPECO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPECO_CTL to value 0x4000_0000"]
impl crate::Resettable for LPECO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0000;
}
