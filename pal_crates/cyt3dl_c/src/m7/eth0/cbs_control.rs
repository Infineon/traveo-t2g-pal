#[doc = "Register `CBS_CONTROL` reader"]
pub struct R(crate::R<CBS_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBS_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBS_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBS_CONTROL` writer"]
pub struct W(crate::W<CBS_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBS_CONTROL_SPEC>;
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
impl From<crate::W<CBS_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBS_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBS_ENABLE_QUEUE_A` reader - Enable Credit-Based Shaping on the highest priority queue (queue A). Write 1 to enable"]
pub type CBS_ENABLE_QUEUE_A_R = crate::BitReader<bool>;
#[doc = "Field `CBS_ENABLE_QUEUE_A` writer - Enable Credit-Based Shaping on the highest priority queue (queue A). Write 1 to enable"]
pub type CBS_ENABLE_QUEUE_A_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CBS_CONTROL_SPEC, bool, O>;
#[doc = "Field `CBS_ENABLE_QUEUE_B` reader - Enable Credit-Based shaping on the 2nd highest priority queue (queue B). Write 1 to enable"]
pub type CBS_ENABLE_QUEUE_B_R = crate::BitReader<bool>;
#[doc = "Field `CBS_ENABLE_QUEUE_B` writer - Enable Credit-Based shaping on the 2nd highest priority queue (queue B). Write 1 to enable"]
pub type CBS_ENABLE_QUEUE_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CBS_CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable Credit-Based Shaping on the highest priority queue (queue A). Write 1 to enable"]
    #[inline(always)]
    pub fn cbs_enable_queue_a(&self) -> CBS_ENABLE_QUEUE_A_R {
        CBS_ENABLE_QUEUE_A_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Credit-Based shaping on the 2nd highest priority queue (queue B). Write 1 to enable"]
    #[inline(always)]
    pub fn cbs_enable_queue_b(&self) -> CBS_ENABLE_QUEUE_B_R {
        CBS_ENABLE_QUEUE_B_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Credit-Based Shaping on the highest priority queue (queue A). Write 1 to enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbs_enable_queue_a(&mut self) -> CBS_ENABLE_QUEUE_A_W<0> {
        CBS_ENABLE_QUEUE_A_W::new(self)
    }
    #[doc = "Bit 1 - Enable Credit-Based shaping on the 2nd highest priority queue (queue B). Write 1 to enable"]
    #[inline(always)]
    #[must_use]
    pub fn cbs_enable_queue_b(&mut self) -> CBS_ENABLE_QUEUE_B_W<1> {
        CBS_ENABLE_QUEUE_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The IdleSlope value is defined as the rate of change of credit when a packet is waiting to be sent. This must not exceed the portTransmitRate which is dependent on the speed of operation, eg, portTranmsitRate. 1Gb/s = 32'h07735940 (125 Mbytes/s), 100Mb/sec = 32'h017D7840 (25 Mnibbles/s), 10Mb/sec = 32'h002625A0 (2.5 Mnibbles/s). If 50 percent of bandwidth was to be allocated to a particular queue in 1Gb/sec mode then the IdleSlope value for that queue would be calculated as 32'h07735940/2. Note that Credit-Based Shaping should be disabled prior to updating the IdleSlope values. As another example, for a 1722 audio packet with a payload of 6 samples per channel, the packet size would be 7 (preamble) + 1 (SFD) + 50 (packet header) + 6x4x2(payload) + 4 (CRC) = 110 bytes. For a rate of 8000 packets per second, the desired rate would 110 x 8000 bytes per second, so the programmed idleSlope value would be 880000 for a 1Gb/s connection, or 1760000 for a 100Mb/s or 10Mbs connection. See Figure 6.3 in the IEEE 1722 standard. In practice, the actual transmission rate will be vary slightly from that calculated. In this case, the idleSlope value should be recalibrated based on the variance between the measured and expected rate, and in this case very accurate transmission rates can be achieved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbs_control](index.html) module"]
pub struct CBS_CONTROL_SPEC;
impl crate::RegisterSpec for CBS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbs_control::R](R) reader structure"]
impl crate::Readable for CBS_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbs_control::W](W) writer structure"]
impl crate::Writable for CBS_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBS_CONTROL to value 0"]
impl crate::Resettable for CBS_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
