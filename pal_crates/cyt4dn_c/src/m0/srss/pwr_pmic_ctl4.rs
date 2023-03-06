#[doc = "Register `PWR_PMIC_CTL4` reader"]
pub struct R(crate::R<PWR_PMIC_CTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_PMIC_CTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_PMIC_CTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_PMIC_CTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_PMIC_CTL4` writer"]
pub struct W(crate::W<PWR_PMIC_CTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_PMIC_CTL4_SPEC>;
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
impl From<crate::W<PWR_PMIC_CTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_PMIC_CTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMIC_VADJ_DIS` reader - Disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ."]
pub type PMIC_VADJ_DIS_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_VADJ_DIS` writer - Disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ."]
pub type PMIC_VADJ_DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL4_SPEC, bool, O>;
#[doc = "Field `PMIC_DPSLP` reader - Configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
pub type PMIC_DPSLP_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_DPSLP` writer - Configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
pub type PMIC_DPSLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL4_SPEC, bool, O>;
impl R {
    #[doc = "Bit 30 - Disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ."]
    #[inline(always)]
    pub fn pmic_vadj_dis(&self) -> PMIC_VADJ_DIS_R {
        PMIC_VADJ_DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
    #[inline(always)]
    pub fn pmic_dpslp(&self) -> PMIC_DPSLP_R {
        PMIC_DPSLP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Disables the VADJ circuitry. This can be used to decrease current consumption if the entire feedback network is outside the device. 0: Device generates VADJ when PMIC is enabled. This allows the feedback loop to compensate for voltage drops in the PCB and package. 1: Device does not generate VADJ, and it must not be part of the PMIC feedback loop. This reduces current by turning off the internal resistor divider that generates VADJ."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_vadj_dis(&mut self) -> PMIC_VADJ_DIS_W<30> {
        PMIC_VADJ_DIS_W::new(self)
    }
    #[doc = "Bit 31 - Configures PMIC behavior during DEEPSLEEP. 0: Device operates from internal regulators during DEEPSLEEP. If PMIC is enabled at the beginning of the DEEPSLEEP transition, hardware changes to the internal regulators and disables the PMIC. 1: DEEPSLEEP transition does not change PMIC enable."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_dpslp(&mut self) -> PMIC_DPSLP_W<31> {
        PMIC_DPSLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMIC Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_pmic_ctl4](index.html) module"]
pub struct PWR_PMIC_CTL4_SPEC;
impl crate::RegisterSpec for PWR_PMIC_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_pmic_ctl4::R](R) reader structure"]
impl crate::Readable for PWR_PMIC_CTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_pmic_ctl4::W](W) writer structure"]
impl crate::Writable for PWR_PMIC_CTL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_PMIC_CTL4 to value 0"]
impl crate::Resettable for PWR_PMIC_CTL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
