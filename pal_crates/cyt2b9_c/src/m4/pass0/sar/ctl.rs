#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRUP_TIME` reader - Number cycles to wait to power up after IDLE_PWRDWN. Check the STATUS.PWRUP_BUSY flag to see if the delay is still in progress. The power up delay is 1 us."]
pub type PWRUP_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PWRUP_TIME` writer - Number cycles to wait to power up after IDLE_PWRDWN. Check the STATUS.PWRUP_BUSY flag to see if the delay is still in progress. The power up delay is 1 us."]
pub type PWRUP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `IDLE_PWRDWN` reader - When idle automatically power down the analog. After an automatic power down a new trigger will power up the analog, however it will take PWRUP_TIME cycles before the first acquisition can be started. Note that re-arbitration happens at that time, i.e. the trigger that caused the power up may not get handled first."]
pub type IDLE_PWRDWN_R = crate::BitReader<bool>;
#[doc = "Field `IDLE_PWRDWN` writer - When idle automatically power down the analog. After an automatic power down a new trigger will power up the analog, however it will take PWRUP_TIME cycles before the first acquisition can be started. Note that re-arbitration happens at that time, i.e. the trigger that caused the power up may not get handled first."]
pub type IDLE_PWRDWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `MSB_STRETCH` reader - When set use 2 cycles for the Most Significant Bit (MSB) - 0: Use 1 clock cycle for MSB - 1: Use 2 clock cycles for MSB"]
pub type MSB_STRETCH_R = crate::BitReader<bool>;
#[doc = "Field `MSB_STRETCH` writer - When set use 2 cycles for the Most Significant Bit (MSB) - 0: Use 1 clock cycle for MSB - 1: Use 2 clock cycles for MSB"]
pub type MSB_STRETCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `HALF_LSB` reader - When set take an extra cycle to convert the half LSB and add it to 12-bit result for Missing Code Recovery This bit should always be set to '1' - 0: disable half LSB conversion (not recommended) - 1: enable half LSB conversion"]
pub type HALF_LSB_R = crate::BitReader<bool>;
#[doc = "Field `HALF_LSB` writer - When set take an extra cycle to convert the half LSB and add it to 12-bit result for Missing Code Recovery This bit should always be set to '1' - 0: disable half LSB conversion (not recommended) - 1: enable half LSB conversion"]
pub type HALF_LSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SARMUX_EN` reader - Enable the SARMUX (only valid if ENABLED=1) - 0: SARMUX disabled (put analog in power down) - 1: SARMUX enabled."]
pub type SARMUX_EN_R = crate::BitReader<bool>;
#[doc = "Field `SARMUX_EN` writer - Enable the SARMUX (only valid if ENABLED=1) - 0: SARMUX disabled (put analog in power down) - 1: SARMUX enabled."]
pub type SARMUX_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ADC_EN` reader - Enable the SAR ADC and SAR sequencer (only valid if ENABLED=1) - 0: SARADC and SARSEQ are disabled (put SARADC analog in power down and stop clocks), also clears all pending triggers. - 1: SAR ADC and SARSEQ are enabled. To enable ADC0 to borrow SARMUX1-3 the corresponding ADC_EN must be set to 0."]
pub type ADC_EN_R = crate::BitReader<bool>;
#[doc = "Field `ADC_EN` writer - Enable the SAR ADC and SAR sequencer (only valid if ENABLED=1) - 0: SARADC and SARSEQ are disabled (put SARADC analog in power down and stop clocks), also clears all pending triggers. - 1: SAR ADC and SARSEQ are enabled. To enable ADC0 to borrow SARMUX1-3 the corresponding ADC_EN must be set to 0."]
pub type ADC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - - 0: SAR IP disabled (put analog in power down and stop clocks), also clears all pending triggers. - 1: SAR IP enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - - 0: SAR IP disabled (put analog in power down and stop clocks), also clears all pending triggers. - 1: SAR IP enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Number cycles to wait to power up after IDLE_PWRDWN. Check the STATUS.PWRUP_BUSY flag to see if the delay is still in progress. The power up delay is 1 us."]
    #[inline(always)]
    pub fn pwrup_time(&self) -> PWRUP_TIME_R {
        PWRUP_TIME_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - When idle automatically power down the analog. After an automatic power down a new trigger will power up the analog, however it will take PWRUP_TIME cycles before the first acquisition can be started. Note that re-arbitration happens at that time, i.e. the trigger that caused the power up may not get handled first."]
    #[inline(always)]
    pub fn idle_pwrdwn(&self) -> IDLE_PWRDWN_R {
        IDLE_PWRDWN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set use 2 cycles for the Most Significant Bit (MSB) - 0: Use 1 clock cycle for MSB - 1: Use 2 clock cycles for MSB"]
    #[inline(always)]
    pub fn msb_stretch(&self) -> MSB_STRETCH_R {
        MSB_STRETCH_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set take an extra cycle to convert the half LSB and add it to 12-bit result for Missing Code Recovery This bit should always be set to '1' - 0: disable half LSB conversion (not recommended) - 1: enable half LSB conversion"]
    #[inline(always)]
    pub fn half_lsb(&self) -> HALF_LSB_R {
        HALF_LSB_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable the SARMUX (only valid if ENABLED=1) - 0: SARMUX disabled (put analog in power down) - 1: SARMUX enabled."]
    #[inline(always)]
    pub fn sarmux_en(&self) -> SARMUX_EN_R {
        SARMUX_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable the SAR ADC and SAR sequencer (only valid if ENABLED=1) - 0: SARADC and SARSEQ are disabled (put SARADC analog in power down and stop clocks), also clears all pending triggers. - 1: SAR ADC and SARSEQ are enabled. To enable ADC0 to borrow SARMUX1-3 the corresponding ADC_EN must be set to 0."]
    #[inline(always)]
    pub fn adc_en(&self) -> ADC_EN_R {
        ADC_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also clears all pending triggers. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number cycles to wait to power up after IDLE_PWRDWN. Check the STATUS.PWRUP_BUSY flag to see if the delay is still in progress. The power up delay is 1 us."]
    #[inline(always)]
    #[must_use]
    pub fn pwrup_time(&mut self) -> PWRUP_TIME_W<0> {
        PWRUP_TIME_W::new(self)
    }
    #[doc = "Bit 8 - When idle automatically power down the analog. After an automatic power down a new trigger will power up the analog, however it will take PWRUP_TIME cycles before the first acquisition can be started. Note that re-arbitration happens at that time, i.e. the trigger that caused the power up may not get handled first."]
    #[inline(always)]
    #[must_use]
    pub fn idle_pwrdwn(&mut self) -> IDLE_PWRDWN_W<8> {
        IDLE_PWRDWN_W::new(self)
    }
    #[doc = "Bit 9 - When set use 2 cycles for the Most Significant Bit (MSB) - 0: Use 1 clock cycle for MSB - 1: Use 2 clock cycles for MSB"]
    #[inline(always)]
    #[must_use]
    pub fn msb_stretch(&mut self) -> MSB_STRETCH_W<9> {
        MSB_STRETCH_W::new(self)
    }
    #[doc = "Bit 10 - When set take an extra cycle to convert the half LSB and add it to 12-bit result for Missing Code Recovery This bit should always be set to '1' - 0: disable half LSB conversion (not recommended) - 1: enable half LSB conversion"]
    #[inline(always)]
    #[must_use]
    pub fn half_lsb(&mut self) -> HALF_LSB_W<10> {
        HALF_LSB_W::new(self)
    }
    #[doc = "Bit 29 - Enable the SARMUX (only valid if ENABLED=1) - 0: SARMUX disabled (put analog in power down) - 1: SARMUX enabled."]
    #[inline(always)]
    #[must_use]
    pub fn sarmux_en(&mut self) -> SARMUX_EN_W<29> {
        SARMUX_EN_W::new(self)
    }
    #[doc = "Bit 30 - Enable the SAR ADC and SAR sequencer (only valid if ENABLED=1) - 0: SARADC and SARSEQ are disabled (put SARADC analog in power down and stop clocks), also clears all pending triggers. - 1: SAR ADC and SARSEQ are enabled. To enable ADC0 to borrow SARMUX1-3 the corresponding ADC_EN must be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_en(&mut self) -> ADC_EN_W<30> {
        ADC_EN_W::new(self)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also clears all pending triggers. - 1: SAR IP enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
