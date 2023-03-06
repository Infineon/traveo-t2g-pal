#[doc = "Register `CLK_ECO_CONFIG` reader"]
pub struct R(crate::R<CLK_ECO_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_ECO_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_ECO_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_ECO_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_ECO_CONFIG` writer"]
pub struct W(crate::W<CLK_ECO_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_ECO_CONFIG_SPEC>;
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
impl From<crate::W<CLK_ECO_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_ECO_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AGC_EN` reader - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by CLK_ECO_CONFIG2.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AGC_EN_R = crate::BitReader<bool>;
#[doc = "Field `AGC_EN` writer - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by CLK_ECO_CONFIG2.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
pub type AGC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_ECO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ECO_DIV_DISABLE` reader - ECO prescaler disable command (mutually exclusive with ECO_DIV_ENABLE). SW sets this field to '1' and HW sets this field to '0'. HW sets ECO_DIV_DISABLE field to '0' immediately and HW sets CLK_ECO_PRESCALE.ECO_DIV_EN field to '0' immediately."]
pub type ECO_DIV_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `ECO_DIV_DISABLE` writer - ECO prescaler disable command (mutually exclusive with ECO_DIV_ENABLE). SW sets this field to '1' and HW sets this field to '0'. HW sets ECO_DIV_DISABLE field to '0' immediately and HW sets CLK_ECO_PRESCALE.ECO_DIV_EN field to '0' immediately."]
pub type ECO_DIV_DISABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_ECO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ECO_DIV_ENABLE` reader - ECO prescaler enable command (mutually exclusive with ECO_DIV_DISABLE). ECO Prescaler only works in ACTIVE and SLEEP modes. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the ECO_DIV_DISABLE field. 1: Configure CLK_ECO_PRESCALE registers. 2: Enable the divider using the ECO_DIV_ENABLE field. HW sets the ECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_ECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
pub type ECO_DIV_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ECO_DIV_ENABLE` writer - ECO prescaler enable command (mutually exclusive with ECO_DIV_DISABLE). ECO Prescaler only works in ACTIVE and SLEEP modes. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the ECO_DIV_DISABLE field. 1: Configure CLK_ECO_PRESCALE registers. 2: Enable the divider using the ECO_DIV_ENABLE field. HW sets the ECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_ECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
pub type ECO_DIV_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CLK_ECO_CONFIG_SPEC, bool, O>;
#[doc = "Field `ECO_EN` reader - Master enable for ECO oscillator."]
pub type ECO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ECO_EN` writer - Master enable for ECO oscillator."]
pub type ECO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLK_ECO_CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by CLK_ECO_CONFIG2.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    pub fn agc_en(&self) -> AGC_EN_R {
        AGC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 27 - ECO prescaler disable command (mutually exclusive with ECO_DIV_ENABLE). SW sets this field to '1' and HW sets this field to '0'. HW sets ECO_DIV_DISABLE field to '0' immediately and HW sets CLK_ECO_PRESCALE.ECO_DIV_EN field to '0' immediately."]
    #[inline(always)]
    pub fn eco_div_disable(&self) -> ECO_DIV_DISABLE_R {
        ECO_DIV_DISABLE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ECO prescaler enable command (mutually exclusive with ECO_DIV_DISABLE). ECO Prescaler only works in ACTIVE and SLEEP modes. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the ECO_DIV_DISABLE field. 1: Configure CLK_ECO_PRESCALE registers. 2: Enable the divider using the ECO_DIV_ENABLE field. HW sets the ECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_ECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
    #[inline(always)]
    pub fn eco_div_enable(&self) -> ECO_DIV_ENABLE_R {
        ECO_DIV_ENABLE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    pub fn eco_en(&self) -> ECO_EN_R {
        ECO_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Automatic Gain Control (AGC) enable. When set, the oscillation amplitude is controlled to the level selected by CLK_ECO_CONFIG2.ATRIM. When low, the amplitude is not explicitly controlled and can be as high as the vddd supply. WARNING: use care when disabling AGC because driving a crystal beyond its rated limit can permanently damage the crystal."]
    #[inline(always)]
    #[must_use]
    pub fn agc_en(&mut self) -> AGC_EN_W<1> {
        AGC_EN_W::new(self)
    }
    #[doc = "Bit 27 - ECO prescaler disable command (mutually exclusive with ECO_DIV_ENABLE). SW sets this field to '1' and HW sets this field to '0'. HW sets ECO_DIV_DISABLE field to '0' immediately and HW sets CLK_ECO_PRESCALE.ECO_DIV_EN field to '0' immediately."]
    #[inline(always)]
    #[must_use]
    pub fn eco_div_disable(&mut self) -> ECO_DIV_DISABLE_W<27> {
        ECO_DIV_DISABLE_W::new(self)
    }
    #[doc = "Bit 28 - ECO prescaler enable command (mutually exclusive with ECO_DIV_DISABLE). ECO Prescaler only works in ACTIVE and SLEEP modes. SW sets this field to '1' to enable the divider and HW sets this field to '0' to indicate that divider enabling has completed. When the divider is enabled, its integer and fractional counters are initialized to '0'. If a divider is to be re-enabled using different integer and fractional divider values, the SW should follow these steps: 0: Disable the divider using the ECO_DIV_DISABLE field. 1: Configure CLK_ECO_PRESCALE registers. 2: Enable the divider using the ECO_DIV_ENABLE field. HW sets the ECO_DIV_ENABLE field to '0' when the enabling is performed and HW set CLK_ECO_PRESCALER.ENABLED to '1' when the enabling is performed."]
    #[inline(always)]
    #[must_use]
    pub fn eco_div_enable(&mut self) -> ECO_DIV_ENABLE_W<28> {
        ECO_DIV_ENABLE_W::new(self)
    }
    #[doc = "Bit 31 - Master enable for ECO oscillator."]
    #[inline(always)]
    #[must_use]
    pub fn eco_en(&mut self) -> ECO_EN_W<31> {
        ECO_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_eco_config](index.html) module"]
pub struct CLK_ECO_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_ECO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_eco_config::R](R) reader structure"]
impl crate::Readable for CLK_ECO_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_eco_config::W](W) writer structure"]
impl crate::Writable for CLK_ECO_CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ECO_CONFIG to value 0x02"]
impl crate::Resettable for CLK_ECO_CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
