#[doc = "Register `CAL_CTL` reader"]
pub struct R(crate::R<CAL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL_CTL` writer"]
pub struct W(crate::W<CAL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_CTL_SPEC>;
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
impl From<crate::W<CAL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_RCAL` reader - On-chip termination control bits for manual calibration. This setting has only an effect when NOCAL=1. 2'b00: 20 percent higher than mid-range. Highest impedance setting 2'b01: Mid-range impedance setting. 2'b10: 15 percent lower than mid-range 2'b11: 25 percent lower than mid-range. Lowest setting."]
pub type RX_RCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_RCAL` writer - On-chip termination control bits for manual calibration. This setting has only an effect when NOCAL=1. 2'b00: 20 percent higher than mid-range. Highest impedance setting 2'b01: Mid-range impedance setting. 2'b10: 15 percent lower than mid-range 2'b11: 25 percent lower than mid-range. Lowest setting."]
pub type RX_RCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `NOCAL` reader - To override calibration value set by auto-calibration circuit with RX_RCAL\\[1:0\\]. 1'b0: Auto-calibration operation. 1'b1: Bypass the calibrator. Externally control the HS-RX termination resistance with RX_RCAL\\[1:0\\]"]
pub type NOCAL_R = crate::BitReader<bool>;
#[doc = "Field `NOCAL` writer - To override calibration value set by auto-calibration circuit with RX_RCAL\\[1:0\\]. 1'b0: Auto-calibration operation. 1'b1: Bypass the calibrator. Externally control the HS-RX termination resistance with RX_RCAL\\[1:0\\]"]
pub type NOCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - On-chip termination control bits for manual calibration. This setting has only an effect when NOCAL=1. 2'b00: 20 percent higher than mid-range. Highest impedance setting 2'b01: Mid-range impedance setting. 2'b10: 15 percent lower than mid-range 2'b11: 25 percent lower than mid-range. Lowest setting."]
    #[inline(always)]
    pub fn rx_rcal(&self) -> RX_RCAL_R {
        RX_RCAL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - To override calibration value set by auto-calibration circuit with RX_RCAL\\[1:0\\]. 1'b0: Auto-calibration operation. 1'b1: Bypass the calibrator. Externally control the HS-RX termination resistance with RX_RCAL\\[1:0\\]"]
    #[inline(always)]
    pub fn nocal(&self) -> NOCAL_R {
        NOCAL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - On-chip termination control bits for manual calibration. This setting has only an effect when NOCAL=1. 2'b00: 20 percent higher than mid-range. Highest impedance setting 2'b01: Mid-range impedance setting. 2'b10: 15 percent lower than mid-range 2'b11: 25 percent lower than mid-range. Lowest setting."]
    #[inline(always)]
    #[must_use]
    pub fn rx_rcal(&mut self) -> RX_RCAL_W<0> {
        RX_RCAL_W::new(self)
    }
    #[doc = "Bit 8 - To override calibration value set by auto-calibration circuit with RX_RCAL\\[1:0\\]. 1'b0: Auto-calibration operation. 1'b1: Bypass the calibrator. Externally control the HS-RX termination resistance with RX_RCAL\\[1:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn nocal(&mut self) -> NOCAL_W<8> {
        NOCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal_ctl](index.html) module"]
pub struct CAL_CTL_SPEC;
impl crate::RegisterSpec for CAL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal_ctl::R](R) reader structure"]
impl crate::Readable for CAL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal_ctl::W](W) writer structure"]
impl crate::Writable for CAL_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL_CTL to value 0x01"]
impl crate::Resettable for CAL_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
