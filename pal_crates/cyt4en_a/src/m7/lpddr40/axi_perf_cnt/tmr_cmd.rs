#[doc = "Register `TMR_CMD` reader"]
pub struct R(crate::R<TMR_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMR_CMD` writer"]
pub struct W(crate::W<TMR_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_CMD_SPEC>;
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
impl From<crate::W<TMR_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Writing 1 to this field triggers the start of the measurement window. When this field is written to 1, MU_TMR_STATUS.MEAS in the measurement units are set to 1, indicating that the measurement window starts, and START is automatically cleared to 0. In each measurement unit, all counters are cleared to 0, and the current timer count is set to 1 and is incremented with each cycle of the prescaled clock during the measurment window. START has a higher priority than STOP. Writing 1 to this field during the measurement window restarts the measurement window, clearing the counters and restarting the timer."]
pub type START_R = crate::BitReader<bool>;
#[doc = "Field `START` writer - Writing 1 to this field triggers the start of the measurement window. When this field is written to 1, MU_TMR_STATUS.MEAS in the measurement units are set to 1, indicating that the measurement window starts, and START is automatically cleared to 0. In each measurement unit, all counters are cleared to 0, and the current timer count is set to 1 and is incremented with each cycle of the prescaled clock during the measurment window. START has a higher priority than STOP. Writing 1 to this field during the measurement window restarts the measurement window, clearing the counters and restarting the timer."]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_CMD_SPEC, bool, O>;
#[doc = "Field `STOP` reader - Writing 1 to this field stops the measurement at the next clock edge of the prescaled clock. At this time, the timer count and all counters in the measurement units keep their current values. STOP is automatically cleared to 0 immediately. STOP has a lower priority than START."]
pub type STOP_R = crate::BitReader<bool>;
#[doc = "Field `STOP` writer - Writing 1 to this field stops the measurement at the next clock edge of the prescaled clock. At this time, the timer count and all counters in the measurement units keep their current values. STOP is automatically cleared to 0 immediately. STOP has a lower priority than START."]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TMR_CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Writing 1 to this field triggers the start of the measurement window. When this field is written to 1, MU_TMR_STATUS.MEAS in the measurement units are set to 1, indicating that the measurement window starts, and START is automatically cleared to 0. In each measurement unit, all counters are cleared to 0, and the current timer count is set to 1 and is incremented with each cycle of the prescaled clock during the measurment window. START has a higher priority than STOP. Writing 1 to this field during the measurement window restarts the measurement window, clearing the counters and restarting the timer."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Writing 1 to this field stops the measurement at the next clock edge of the prescaled clock. At this time, the timer count and all counters in the measurement units keep their current values. STOP is automatically cleared to 0 immediately. STOP has a lower priority than START."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this field triggers the start of the measurement window. When this field is written to 1, MU_TMR_STATUS.MEAS in the measurement units are set to 1, indicating that the measurement window starts, and START is automatically cleared to 0. In each measurement unit, all counters are cleared to 0, and the current timer count is set to 1 and is incremented with each cycle of the prescaled clock during the measurment window. START has a higher priority than STOP. Writing 1 to this field during the measurement window restarts the measurement window, clearing the counters and restarting the timer."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 8 - Writing 1 to this field stops the measurement at the next clock edge of the prescaled clock. At this time, the timer count and all counters in the measurement units keep their current values. STOP is automatically cleared to 0 immediately. STOP has a lower priority than START."]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<8> {
        STOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_cmd](index.html) module"]
pub struct TMR_CMD_SPEC;
impl crate::RegisterSpec for TMR_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_cmd::R](R) reader structure"]
impl crate::Readable for TMR_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_cmd::W](W) writer structure"]
impl crate::Writable for TMR_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMR_CMD to value 0"]
impl crate::Resettable for TMR_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
