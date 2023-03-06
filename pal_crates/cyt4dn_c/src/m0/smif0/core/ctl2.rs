#[doc = "Register `CTL2` reader"]
pub struct R(crate::R<CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL2` writer"]
pub struct W(crate::W<CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL2_SPEC>;
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
impl From<crate::W<CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLL_SPEED_MODE` reader - Set according to the PLL clock frequency that feeds the DLL: 0: 160 to 180 MHz 1: >180 to 266 MHz 2: >266 to 333 MHz 3: >333 to 400 MHz"]
pub type DLL_SPEED_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLL_SPEED_MODE` writer - Set according to the PLL clock frequency that feeds the DLL: 0: 160 to 180 MHz 1: >180 to 266 MHz 2: >266 to 333 MHz 3: >333 to 400 MHz"]
pub type DLL_SPEED_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLKOUT_DIV` reader - The clock out to the memory is a divided version of the MDL clock out: 0: divide by 2 1: divide by 4 2: divide by 8 3: divide by 16"]
pub type CLKOUT_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKOUT_DIV` writer - The clock out to the memory is a divided version of the MDL clock out: 0: divide by 2 1: divide by 4 2: divide by 8 3: divide by 16"]
pub type CLKOUT_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `MDL_TAP_SEL` reader - Determines the relative amount of delay through the MDL for the reference clock in terms of the number of tap delays. There are 16 MDL taps that evenly divide the reference clock period into 16 delay steps. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (which is effectively a full MDL reference clock period, which can be thought of as 'zero delay' since it aligns the posedges of the output to the input reference). When CLKOUT_DIV does not equal zero (not divide by 2), MDL tap selections must be between 2 and 12 in order to meet internal timing. Tap selections of 0,1,13,14, and 15 are illegal. There is an exception in DDR mode, it can support between 2 and 13, while 0,1,14, and 15 are illegal. When CLKOUT_DIV=0 internal timing can be met for all MDL tap selections."]
pub type MDL_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MDL_TAP_SEL` writer - Determines the relative amount of delay through the MDL for the reference clock in terms of the number of tap delays. There are 16 MDL taps that evenly divide the reference clock period into 16 delay steps. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (which is effectively a full MDL reference clock period, which can be thought of as 'zero delay' since it aligns the posedges of the output to the input reference). When CLKOUT_DIV does not equal zero (not divide by 2), MDL tap selections must be between 2 and 12 in order to meet internal timing. Tap selections of 0,1,13,14, and 15 are illegal. There is an exception in DDR mode, it can support between 2 and 13, while 0,1,14, and 15 are illegal. When CLKOUT_DIV=0 internal timing can be met for all MDL tap selections."]
pub type MDL_TAP_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RX_CAPTURE_MODE` reader - N/A"]
pub type RX_CAPTURE_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_CAPTURE_MODE` writer - N/A"]
pub type RX_CAPTURE_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX_CHASE_MARGIN` reader - Determines how long the design should wait after sending the last clock out to the memory interface before 'chase' clocks should be created internally to push remainder data in the interface pipeline into the RX FIFO. Lesser numbers allow for shorter deselect cycles between packets, but if the time is too short then the chase clocks will interfere with the normal capture clock from the last clock cycle from the memory interface (whether it is feedback or RWDS). So a proper balance must be struck. The default value is expected to work for all situations. Other values are there as a backup plan in case something more aggressive can be achieved or whether something even more conservative should be used."]
pub type RX_CHASE_MARGIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_CHASE_MARGIN` writer - Determines how long the design should wait after sending the last clock out to the memory interface before 'chase' clocks should be created internally to push remainder data in the interface pipeline into the RX FIFO. Lesser numbers allow for shorter deselect cycles between packets, but if the time is too short then the chase clocks will interfere with the normal capture clock from the last clock cycle from the memory interface (whether it is feedback or RWDS). So a proper balance must be struck. The default value is expected to work for all situations. Other values are there as a backup plan in case something more aggressive can be achieved or whether something even more conservative should be used."]
pub type RX_CHASE_MARGIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `TX_SDR_EXTRA_SETUP` reader - This field is only applicable in SDR transmit mode. 0: transmit data is launched nominally 1 PLL clock period ahead of the rising edge of the clock out to the memory 1: transmit data is launched nominally 2 PLL clock periods ahead of the rising edge of the clock out to the memory"]
pub type TX_SDR_EXTRA_SETUP_R = crate::BitReader<bool>;
#[doc = "Field `TX_SDR_EXTRA_SETUP` writer - This field is only applicable in SDR transmit mode. 0: transmit data is launched nominally 1 PLL clock period ahead of the rising edge of the clock out to the memory 1: transmit data is launched nominally 2 PLL clock periods ahead of the rising edge of the clock out to the memory"]
pub type TX_SDR_EXTRA_SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Set according to the PLL clock frequency that feeds the DLL: 0: 160 to 180 MHz 1: >180 to 266 MHz 2: >266 to 333 MHz 3: >333 to 400 MHz"]
    #[inline(always)]
    pub fn dll_speed_mode(&self) -> DLL_SPEED_MODE_R {
        DLL_SPEED_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - The clock out to the memory is a divided version of the MDL clock out: 0: divide by 2 1: divide by 4 2: divide by 8 3: divide by 16"]
    #[inline(always)]
    pub fn clkout_div(&self) -> CLKOUT_DIV_R {
        CLKOUT_DIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Determines the relative amount of delay through the MDL for the reference clock in terms of the number of tap delays. There are 16 MDL taps that evenly divide the reference clock period into 16 delay steps. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (which is effectively a full MDL reference clock period, which can be thought of as 'zero delay' since it aligns the posedges of the output to the input reference). When CLKOUT_DIV does not equal zero (not divide by 2), MDL tap selections must be between 2 and 12 in order to meet internal timing. Tap selections of 0,1,13,14, and 15 are illegal. There is an exception in DDR mode, it can support between 2 and 13, while 0,1,14, and 15 are illegal. When CLKOUT_DIV=0 internal timing can be met for all MDL tap selections."]
    #[inline(always)]
    pub fn mdl_tap_sel(&self) -> MDL_TAP_SEL_R {
        MDL_TAP_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - N/A"]
    #[inline(always)]
    pub fn rx_capture_mode(&self) -> RX_CAPTURE_MODE_R {
        RX_CAPTURE_MODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Determines how long the design should wait after sending the last clock out to the memory interface before 'chase' clocks should be created internally to push remainder data in the interface pipeline into the RX FIFO. Lesser numbers allow for shorter deselect cycles between packets, but if the time is too short then the chase clocks will interfere with the normal capture clock from the last clock cycle from the memory interface (whether it is feedback or RWDS). So a proper balance must be struck. The default value is expected to work for all situations. Other values are there as a backup plan in case something more aggressive can be achieved or whether something even more conservative should be used."]
    #[inline(always)]
    pub fn rx_chase_margin(&self) -> RX_CHASE_MARGIN_R {
        RX_CHASE_MARGIN_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 31 - This field is only applicable in SDR transmit mode. 0: transmit data is launched nominally 1 PLL clock period ahead of the rising edge of the clock out to the memory 1: transmit data is launched nominally 2 PLL clock periods ahead of the rising edge of the clock out to the memory"]
    #[inline(always)]
    pub fn tx_sdr_extra_setup(&self) -> TX_SDR_EXTRA_SETUP_R {
        TX_SDR_EXTRA_SETUP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Set according to the PLL clock frequency that feeds the DLL: 0: 160 to 180 MHz 1: >180 to 266 MHz 2: >266 to 333 MHz 3: >333 to 400 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn dll_speed_mode(&mut self) -> DLL_SPEED_MODE_W<0> {
        DLL_SPEED_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - The clock out to the memory is a divided version of the MDL clock out: 0: divide by 2 1: divide by 4 2: divide by 8 3: divide by 16"]
    #[inline(always)]
    #[must_use]
    pub fn clkout_div(&mut self) -> CLKOUT_DIV_W<4> {
        CLKOUT_DIV_W::new(self)
    }
    #[doc = "Bits 8:11 - Determines the relative amount of delay through the MDL for the reference clock in terms of the number of tap delays. There are 16 MDL taps that evenly divide the reference clock period into 16 delay steps. Setting 0 constitutes 1 tap delay, setting 1 constitutes 2 tap delays, etc., setting 15 constitutes 16 tap delays (which is effectively a full MDL reference clock period, which can be thought of as 'zero delay' since it aligns the posedges of the output to the input reference). When CLKOUT_DIV does not equal zero (not divide by 2), MDL tap selections must be between 2 and 12 in order to meet internal timing. Tap selections of 0,1,13,14, and 15 are illegal. There is an exception in DDR mode, it can support between 2 and 13, while 0,1,14, and 15 are illegal. When CLKOUT_DIV=0 internal timing can be met for all MDL tap selections."]
    #[inline(always)]
    #[must_use]
    pub fn mdl_tap_sel(&mut self) -> MDL_TAP_SEL_W<8> {
        MDL_TAP_SEL_W::new(self)
    }
    #[doc = "Bits 20:21 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rx_capture_mode(&mut self) -> RX_CAPTURE_MODE_W<20> {
        RX_CAPTURE_MODE_W::new(self)
    }
    #[doc = "Bits 22:23 - Determines how long the design should wait after sending the last clock out to the memory interface before 'chase' clocks should be created internally to push remainder data in the interface pipeline into the RX FIFO. Lesser numbers allow for shorter deselect cycles between packets, but if the time is too short then the chase clocks will interfere with the normal capture clock from the last clock cycle from the memory interface (whether it is feedback or RWDS). So a proper balance must be struck. The default value is expected to work for all situations. Other values are there as a backup plan in case something more aggressive can be achieved or whether something even more conservative should be used."]
    #[inline(always)]
    #[must_use]
    pub fn rx_chase_margin(&mut self) -> RX_CHASE_MARGIN_W<22> {
        RX_CHASE_MARGIN_W::new(self)
    }
    #[doc = "Bit 31 - This field is only applicable in SDR transmit mode. 0: transmit data is launched nominally 1 PLL clock period ahead of the rising edge of the clock out to the memory 1: transmit data is launched nominally 2 PLL clock periods ahead of the rising edge of the clock out to the memory"]
    #[inline(always)]
    #[must_use]
    pub fn tx_sdr_extra_setup(&mut self) -> TX_SDR_EXTRA_SETUP_W<31> {
        TX_SDR_EXTRA_SETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl2](index.html) module"]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl2::R](R) reader structure"]
impl crate::Readable for CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl2::W](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0x0080_0000"]
impl crate::Resettable for CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0000;
}
