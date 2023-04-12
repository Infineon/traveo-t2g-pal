#[doc = "Register `PWR_PMIC_CTL2` reader"]
pub struct R(crate::R<PWR_PMIC_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_PMIC_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_PMIC_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_PMIC_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_PMIC_CTL2` writer"]
pub struct W(crate::W<PWR_PMIC_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_PMIC_CTL2_SPEC>;
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
impl From<crate::W<PWR_PMIC_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_PMIC_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMIC_STATUS_TIMEOUT` reader - Timeout while waiting for PMIC_STATUS_OK==1 when switching to PMIC. 0: disables timeout. >0: enables timeout of PMIC_STATUS_TIMEOUT*128us (nominal, clocked by IMO). Timeout expiration triggers reset."]
pub type PMIC_STATUS_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMIC_STATUS_TIMEOUT` writer - Timeout while waiting for PMIC_STATUS_OK==1 when switching to PMIC. 0: disables timeout. >0: enables timeout of PMIC_STATUS_TIMEOUT*128us (nominal, clocked by IMO). Timeout expiration triggers reset."]
pub type PMIC_STATUS_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWR_PMIC_CTL2_SPEC, u8, u8, 8, O>;
#[doc = "Field `PMIC_EN` reader - Enable PMIC. This bit will not set if PMIC_CONFIGURED==0. Use PWR_PMIC_STATUS.ENABLED to know the actual status of PMIC. It will differ from this bit in the following cases: A) Do not enter DEEPSLEEP while the sequencer is busy (see PWR_PMIC_STATUS.PMIC_SEQ_BUSY). The hardware sequencer disables PMIC during DEEPSLEEP entry and enables it upon wakeup. B) The debugger requests the chip remain powered up. Hardware prevents PMIC from disabling when this bit is cleared. Hardware does not automatically enable PMIC in response to debugger power up request. If this bit is low when the debugger deasserts the power up request, the hardware sequencer will disable PMIC."]
pub type PMIC_EN_R = crate::BitReader<bool>;
#[doc = "Field `PMIC_EN` writer - Enable PMIC. This bit will not set if PMIC_CONFIGURED==0. Use PWR_PMIC_STATUS.ENABLED to know the actual status of PMIC. It will differ from this bit in the following cases: A) Do not enter DEEPSLEEP while the sequencer is busy (see PWR_PMIC_STATUS.PMIC_SEQ_BUSY). The hardware sequencer disables PMIC during DEEPSLEEP entry and enables it upon wakeup. B) The debugger requests the chip remain powered up. Hardware prevents PMIC from disabling when this bit is cleared. Hardware does not automatically enable PMIC in response to debugger power up request. If this bit is low when the debugger deasserts the power up request, the hardware sequencer will disable PMIC."]
pub type PMIC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_PMIC_CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Timeout while waiting for PMIC_STATUS_OK==1 when switching to PMIC. 0: disables timeout. >0: enables timeout of PMIC_STATUS_TIMEOUT*128us (nominal, clocked by IMO). Timeout expiration triggers reset."]
    #[inline(always)]
    pub fn pmic_status_timeout(&self) -> PMIC_STATUS_TIMEOUT_R {
        PMIC_STATUS_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable PMIC. This bit will not set if PMIC_CONFIGURED==0. Use PWR_PMIC_STATUS.ENABLED to know the actual status of PMIC. It will differ from this bit in the following cases: A) Do not enter DEEPSLEEP while the sequencer is busy (see PWR_PMIC_STATUS.PMIC_SEQ_BUSY). The hardware sequencer disables PMIC during DEEPSLEEP entry and enables it upon wakeup. B) The debugger requests the chip remain powered up. Hardware prevents PMIC from disabling when this bit is cleared. Hardware does not automatically enable PMIC in response to debugger power up request. If this bit is low when the debugger deasserts the power up request, the hardware sequencer will disable PMIC."]
    #[inline(always)]
    pub fn pmic_en(&self) -> PMIC_EN_R {
        PMIC_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout while waiting for PMIC_STATUS_OK==1 when switching to PMIC. 0: disables timeout. >0: enables timeout of PMIC_STATUS_TIMEOUT*128us (nominal, clocked by IMO). Timeout expiration triggers reset."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_status_timeout(&mut self) -> PMIC_STATUS_TIMEOUT_W<0> {
        PMIC_STATUS_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 31 - Enable PMIC. This bit will not set if PMIC_CONFIGURED==0. Use PWR_PMIC_STATUS.ENABLED to know the actual status of PMIC. It will differ from this bit in the following cases: A) Do not enter DEEPSLEEP while the sequencer is busy (see PWR_PMIC_STATUS.PMIC_SEQ_BUSY). The hardware sequencer disables PMIC during DEEPSLEEP entry and enables it upon wakeup. B) The debugger requests the chip remain powered up. Hardware prevents PMIC from disabling when this bit is cleared. Hardware does not automatically enable PMIC in response to debugger power up request. If this bit is low when the debugger deasserts the power up request, the hardware sequencer will disable PMIC."]
    #[inline(always)]
    #[must_use]
    pub fn pmic_en(&mut self) -> PMIC_EN_W<31> {
        PMIC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMIC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_pmic_ctl2](index.html) module"]
pub struct PWR_PMIC_CTL2_SPEC;
impl crate::RegisterSpec for PWR_PMIC_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_pmic_ctl2::R](R) reader structure"]
impl crate::Readable for PWR_PMIC_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_pmic_ctl2::W](W) writer structure"]
impl crate::Writable for PWR_PMIC_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR_PMIC_CTL2 to value 0"]
impl crate::Resettable for PWR_PMIC_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
