#[doc = "Register `PWR_REGHC_CTL4` reader"]
pub struct R(crate::R<PWR_REGHC_CTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_REGHC_CTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_REGHC_CTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_REGHC_CTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_REGHC_CTL4` writer"]
pub struct W(crate::W<PWR_REGHC_CTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_REGHC_CTL4_SPEC>;
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
impl From<crate::W<PWR_REGHC_CTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_REGHC_CTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGHC_PMIC_VADJ_DIS` reader - When operating in PMIC mode, disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ. This setting is ineffective when REGHC_PMIC_RADJ_EN==1, and VADJ is generated in that case when PMIC is enabled."]
pub type REGHC_PMIC_VADJ_DIS_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_VADJ_DIS` writer - When operating in PMIC mode, disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ. This setting is ineffective when REGHC_PMIC_RADJ_EN==1, and VADJ is generated in that case when PMIC is enabled."]
pub type REGHC_PMIC_VADJ_DIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL4_SPEC, bool, O>;
#[doc = "Field `REGHC_PMIC_DPSLP` reader - When operating in PMIC mode, configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
pub type REGHC_PMIC_DPSLP_R = crate::BitReader<bool>;
#[doc = "Field `REGHC_PMIC_DPSLP` writer - When operating in PMIC mode, configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
pub type REGHC_PMIC_DPSLP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PWR_REGHC_CTL4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - When operating in PMIC mode, disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ. This setting is ineffective when REGHC_PMIC_RADJ_EN==1, and VADJ is generated in that case when PMIC is enabled."]
    #[inline(always)]
    pub fn reghc_pmic_vadj_dis(&self) -> REGHC_PMIC_VADJ_DIS_R {
        REGHC_PMIC_VADJ_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When operating in PMIC mode, configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
    #[inline(always)]
    pub fn reghc_pmic_dpslp(&self) -> REGHC_PMIC_DPSLP_R {
        REGHC_PMIC_DPSLP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - When operating in PMIC mode, disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ. This setting is ineffective when REGHC_PMIC_RADJ_EN==1, and VADJ is generated in that case when PMIC is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_vadj_dis(&mut self) -> REGHC_PMIC_VADJ_DIS_W<30> {
        REGHC_PMIC_VADJ_DIS_W::new(self)
    }
    #[doc = "Bit 31 - When operating in PMIC mode, configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
    #[inline(always)]
    #[must_use]
    pub fn reghc_pmic_dpslp(&mut self) -> REGHC_PMIC_DPSLP_W<31> {
        REGHC_PMIC_DPSLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "REGHC Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_reghc_ctl4](index.html) module"]
pub struct PWR_REGHC_CTL4_SPEC;
impl crate::RegisterSpec for PWR_REGHC_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_reghc_ctl4::R](R) reader structure"]
impl crate::Readable for PWR_REGHC_CTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_reghc_ctl4::W](W) writer structure"]
impl crate::Writable for PWR_REGHC_CTL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_REGHC_CTL4 to value 0"]
impl crate::Resettable for PWR_REGHC_CTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
