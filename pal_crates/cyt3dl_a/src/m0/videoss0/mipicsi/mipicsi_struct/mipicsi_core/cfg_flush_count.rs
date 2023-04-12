#[doc = "Register `CFG_FLUSH_COUNT` reader"]
pub struct R(crate::R<CFG_FLUSH_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_FLUSH_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_FLUSH_COUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_FLUSH_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG_FLUSH_COUNT` writer"]
pub struct W(crate::W<CFG_FLUSH_COUNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_FLUSH_COUNT_SPEC>;
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
impl From<crate::W<CFG_FLUSH_COUNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_FLUSH_COUNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG_FLUSH_COUNT` reader - Sets the number of cycles to wait at the end of flushing out the receive data FIFOs to guarantee that the FIFOs are empty. Data from the PHY is written into the RX controller receive FIFOs at the MIPI receive rate and is read out of the receive FIFOs at the main RX controller clock rate (clk_video). Since the core clock can be much faster than the MIPI receive rate, there is a settling time for FIFO pointers to cross clock boundaries and show that the FIFO is empty. Without this settling time, the very last write could still be in the FIFO, but the RX controller assumes it has not received all the expected data because it sees the FIFO empty and signals an exit_hs_error type error. Default recommend value is 4'd3 for this input to have the RX Controller wait 4 cycles before accepting that the FIFO is empty. If the ratio between MIPI rate and core clock is 20:1 or greater, it is recommended to set this value to 4'd7 to give additional time. Larger values may be used, but if set too high, it is possible for the controller to not have enough time to finish the current packet and be ready to receive the next packet. Example: At the lowest MIPI rate of 80Mbps, the write clock frequency will be 10Mhz (80 bits / 8 bits per byte). If RX Controller main clock (clk_video) is running at 250Mhz, the ratio is 250/10 or 25:1, so CFG_FLUSH_COUNT should be set for 4'd7."]
pub type CFG_FLUSH_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFG_FLUSH_COUNT` writer - Sets the number of cycles to wait at the end of flushing out the receive data FIFOs to guarantee that the FIFOs are empty. Data from the PHY is written into the RX controller receive FIFOs at the MIPI receive rate and is read out of the receive FIFOs at the main RX controller clock rate (clk_video). Since the core clock can be much faster than the MIPI receive rate, there is a settling time for FIFO pointers to cross clock boundaries and show that the FIFO is empty. Without this settling time, the very last write could still be in the FIFO, but the RX controller assumes it has not received all the expected data because it sees the FIFO empty and signals an exit_hs_error type error. Default recommend value is 4'd3 for this input to have the RX Controller wait 4 cycles before accepting that the FIFO is empty. If the ratio between MIPI rate and core clock is 20:1 or greater, it is recommended to set this value to 4'd7 to give additional time. Larger values may be used, but if set too high, it is possible for the controller to not have enough time to finish the current packet and be ready to receive the next packet. Example: At the lowest MIPI rate of 80Mbps, the write clock frequency will be 10Mhz (80 bits / 8 bits per byte). If RX Controller main clock (clk_video) is running at 250Mhz, the ratio is 250/10 or 25:1, so CFG_FLUSH_COUNT should be set for 4'd7."]
pub type CFG_FLUSH_COUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_FLUSH_COUNT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Sets the number of cycles to wait at the end of flushing out the receive data FIFOs to guarantee that the FIFOs are empty. Data from the PHY is written into the RX controller receive FIFOs at the MIPI receive rate and is read out of the receive FIFOs at the main RX controller clock rate (clk_video). Since the core clock can be much faster than the MIPI receive rate, there is a settling time for FIFO pointers to cross clock boundaries and show that the FIFO is empty. Without this settling time, the very last write could still be in the FIFO, but the RX controller assumes it has not received all the expected data because it sees the FIFO empty and signals an exit_hs_error type error. Default recommend value is 4'd3 for this input to have the RX Controller wait 4 cycles before accepting that the FIFO is empty. If the ratio between MIPI rate and core clock is 20:1 or greater, it is recommended to set this value to 4'd7 to give additional time. Larger values may be used, but if set too high, it is possible for the controller to not have enough time to finish the current packet and be ready to receive the next packet. Example: At the lowest MIPI rate of 80Mbps, the write clock frequency will be 10Mhz (80 bits / 8 bits per byte). If RX Controller main clock (clk_video) is running at 250Mhz, the ratio is 250/10 or 25:1, so CFG_FLUSH_COUNT should be set for 4'd7."]
    #[inline(always)]
    pub fn cfg_flush_count(&self) -> CFG_FLUSH_COUNT_R {
        CFG_FLUSH_COUNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sets the number of cycles to wait at the end of flushing out the receive data FIFOs to guarantee that the FIFOs are empty. Data from the PHY is written into the RX controller receive FIFOs at the MIPI receive rate and is read out of the receive FIFOs at the main RX controller clock rate (clk_video). Since the core clock can be much faster than the MIPI receive rate, there is a settling time for FIFO pointers to cross clock boundaries and show that the FIFO is empty. Without this settling time, the very last write could still be in the FIFO, but the RX controller assumes it has not received all the expected data because it sees the FIFO empty and signals an exit_hs_error type error. Default recommend value is 4'd3 for this input to have the RX Controller wait 4 cycles before accepting that the FIFO is empty. If the ratio between MIPI rate and core clock is 20:1 or greater, it is recommended to set this value to 4'd7 to give additional time. Larger values may be used, but if set too high, it is possible for the controller to not have enough time to finish the current packet and be ready to receive the next packet. Example: At the lowest MIPI rate of 80Mbps, the write clock frequency will be 10Mhz (80 bits / 8 bits per byte). If RX Controller main clock (clk_video) is running at 250Mhz, the ratio is 250/10 or 25:1, so CFG_FLUSH_COUNT should be set for 4'd7."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_flush_count(&mut self) -> CFG_FLUSH_COUNT_W<0> {
        CFG_FLUSH_COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CFG_FLUSH_COUNT is a register within the CSI-2 RX Controller Core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg_flush_count](index.html) module"]
pub struct CFG_FLUSH_COUNT_SPEC;
impl crate::RegisterSpec for CFG_FLUSH_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg_flush_count::R](R) reader structure"]
impl crate::Readable for CFG_FLUSH_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg_flush_count::W](W) writer structure"]
impl crate::Writable for CFG_FLUSH_COUNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG_FLUSH_COUNT to value 0x03"]
impl crate::Resettable for CFG_FLUSH_COUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
