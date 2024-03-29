#[doc = "Register `RD_DUMMY_CTL` reader"]
pub struct R(crate::R<RD_DUMMY_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_DUMMY_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_DUMMY_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_DUMMY_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_DUMMY_CTL` writer"]
pub struct W(crate::W<RD_DUMMY_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_DUMMY_CTL_SPEC>;
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
impl From<crate::W<RD_DUMMY_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_DUMMY_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE5` reader - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type SIZE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE5` writer - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type SIZE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_DUMMY_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PRESENT2` reader - Presence of dummy cycles: '0': not present '1': present '2': present - Dummy cycles are doubled when RWDS refresh indicator is high during CA cycle. (HyperRAM variable latency mode) Notes: - If CLOCK_IF_TX_SEL = '0' for transmitting command, address and mode fields in SDR mode (driving data between falling edges of spihb_clk_out) and the memory is driving read data in DDR mode (based on memory type and transmitted command), then there must be at least 1 latency / dummy cycle specified (RD_DUMMY_CTL.PRESENT2 > 0') to prevent controller and memory driving data signals at the same time. The same is true for the theoretical (practically irrelevant case) of transmitting at least one of the command, address and mode fields in DDR mode but receiving data from the memory in SDR mode. - When using the RWDS / DQS based capturing scheme (CTL.CLK_IF_RX_SEL=\\[6,7\\]), then there is a minimum number of latency / dummy cycles required (RD_DUMMY_CTL.PRESENT2 > 0). For the Hyperbus protocol (DDR_CTL.SIZE3=7) at least 1 latency / dummy cycle has to be specified, for SPI with DQS capturing at least 2 latency / dummy cycles need to be selected (RD_DUMMY_CTL.SIZE5 > 0, exception see following note). - In case of falling edge RWDS / DQS capturing (CTL.CLOCK_IF_RX_SEL = '6') and SDR mode, SW should reduce the number of dummy cycles by 1 compared to the latency cycles required by the memory device. - For Hyperbus, set RD DUMMY_CTL.SIZE5=initial latency cycles-2"]
pub type PRESENT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESENT2` writer - Presence of dummy cycles: '0': not present '1': present '2': present - Dummy cycles are doubled when RWDS refresh indicator is high during CA cycle. (HyperRAM variable latency mode) Notes: - If CLOCK_IF_TX_SEL = '0' for transmitting command, address and mode fields in SDR mode (driving data between falling edges of spihb_clk_out) and the memory is driving read data in DDR mode (based on memory type and transmitted command), then there must be at least 1 latency / dummy cycle specified (RD_DUMMY_CTL.PRESENT2 > 0') to prevent controller and memory driving data signals at the same time. The same is true for the theoretical (practically irrelevant case) of transmitting at least one of the command, address and mode fields in DDR mode but receiving data from the memory in SDR mode. - When using the RWDS / DQS based capturing scheme (CTL.CLK_IF_RX_SEL=\\[6,7\\]), then there is a minimum number of latency / dummy cycles required (RD_DUMMY_CTL.PRESENT2 > 0). For the Hyperbus protocol (DDR_CTL.SIZE3=7) at least 1 latency / dummy cycle has to be specified, for SPI with DQS capturing at least 2 latency / dummy cycles need to be selected (RD_DUMMY_CTL.SIZE5 > 0, exception see following note). - In case of falling edge RWDS / DQS capturing (CTL.CLOCK_IF_RX_SEL = '6') and SDR mode, SW should reduce the number of dummy cycles by 1 compared to the latency cycles required by the memory device. - For Hyperbus, set RD DUMMY_CTL.SIZE5=initial latency cycles-2"]
pub type PRESENT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_DUMMY_CTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 30:31 - Presence of dummy cycles: '0': not present '1': present '2': present - Dummy cycles are doubled when RWDS refresh indicator is high during CA cycle. (HyperRAM variable latency mode) Notes: - If CLOCK_IF_TX_SEL = '0' for transmitting command, address and mode fields in SDR mode (driving data between falling edges of spihb_clk_out) and the memory is driving read data in DDR mode (based on memory type and transmitted command), then there must be at least 1 latency / dummy cycle specified (RD_DUMMY_CTL.PRESENT2 > 0') to prevent controller and memory driving data signals at the same time. The same is true for the theoretical (practically irrelevant case) of transmitting at least one of the command, address and mode fields in DDR mode but receiving data from the memory in SDR mode. - When using the RWDS / DQS based capturing scheme (CTL.CLK_IF_RX_SEL=\\[6,7\\]), then there is a minimum number of latency / dummy cycles required (RD_DUMMY_CTL.PRESENT2 > 0). For the Hyperbus protocol (DDR_CTL.SIZE3=7) at least 1 latency / dummy cycle has to be specified, for SPI with DQS capturing at least 2 latency / dummy cycles need to be selected (RD_DUMMY_CTL.SIZE5 > 0, exception see following note). - In case of falling edge RWDS / DQS capturing (CTL.CLOCK_IF_RX_SEL = '6') and SDR mode, SW should reduce the number of dummy cycles by 1 compared to the latency cycles required by the memory device. - For Hyperbus, set RD DUMMY_CTL.SIZE5=initial latency cycles-2"]
    #[inline(always)]
    pub fn present2(&self) -> PRESENT2_R {
        PRESENT2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    #[must_use]
    pub fn size5(&mut self) -> SIZE5_W<0> {
        SIZE5_W::new(self)
    }
    #[doc = "Bits 30:31 - Presence of dummy cycles: '0': not present '1': present '2': present - Dummy cycles are doubled when RWDS refresh indicator is high during CA cycle. (HyperRAM variable latency mode) Notes: - If CLOCK_IF_TX_SEL = '0' for transmitting command, address and mode fields in SDR mode (driving data between falling edges of spihb_clk_out) and the memory is driving read data in DDR mode (based on memory type and transmitted command), then there must be at least 1 latency / dummy cycle specified (RD_DUMMY_CTL.PRESENT2 > 0') to prevent controller and memory driving data signals at the same time. The same is true for the theoretical (practically irrelevant case) of transmitting at least one of the command, address and mode fields in DDR mode but receiving data from the memory in SDR mode. - When using the RWDS / DQS based capturing scheme (CTL.CLK_IF_RX_SEL=\\[6,7\\]), then there is a minimum number of latency / dummy cycles required (RD_DUMMY_CTL.PRESENT2 > 0). For the Hyperbus protocol (DDR_CTL.SIZE3=7) at least 1 latency / dummy cycle has to be specified, for SPI with DQS capturing at least 2 latency / dummy cycles need to be selected (RD_DUMMY_CTL.SIZE5 > 0, exception see following note). - In case of falling edge RWDS / DQS capturing (CTL.CLOCK_IF_RX_SEL = '6') and SDR mode, SW should reduce the number of dummy cycles by 1 compared to the latency cycles required by the memory device. - For Hyperbus, set RD DUMMY_CTL.SIZE5=initial latency cycles-2"]
    #[inline(always)]
    #[must_use]
    pub fn present2(&mut self) -> PRESENT2_W<30> {
        PRESENT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read dummy control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_dummy_ctl](index.html) module"]
pub struct RD_DUMMY_CTL_SPEC;
impl crate::RegisterSpec for RD_DUMMY_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_dummy_ctl::R](R) reader structure"]
impl crate::Readable for RD_DUMMY_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_dummy_ctl::W](W) writer structure"]
impl crate::Writable for RD_DUMMY_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_DUMMY_CTL to value 0"]
impl crate::Resettable for RD_DUMMY_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
