#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GRP_DONE` reader - Done Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done. Write with '1' to clear bit."]
pub type GRP_DONE_R = crate::BitReader<bool>;
#[doc = "Field `GRP_DONE` writer - Done Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done. Write with '1' to clear bit."]
pub type GRP_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `GRP_CANCELLED` reader - Cancelled Interrupt: hardware sets this interrupt for the last channel of a group if the group scan was preempted and CANCELLED. Note that it is possible that also the GRP_DONE interrupt is set. If that is the case one or more new triggers were detected while the group was already busy, i.e. triggers are too fast. Write with '1' to clear bit."]
pub type GRP_CANCELLED_R = crate::BitReader<bool>;
#[doc = "Field `GRP_CANCELLED` writer - Cancelled Interrupt: hardware sets this interrupt for the last channel of a group if the group scan was preempted and CANCELLED. Note that it is possible that also the GRP_DONE interrupt is set. If that is the case one or more new triggers were detected while the group was already busy, i.e. triggers are too fast. Write with '1' to clear bit."]
pub type GRP_CANCELLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `GRP_OVERFLOW` reader - Overflow Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done and the Done interrupt is already (still) pending. Write with '1' to clear bit."]
pub type GRP_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `GRP_OVERFLOW` writer - Overflow Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done and the Done interrupt is already (still) pending. Write with '1' to clear bit."]
pub type GRP_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CH_RANGE` reader - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. This interrupt is mutual exclusive with Pulse detect interrupt. Write with '1' to clear bit."]
pub type CH_RANGE_R = crate::BitReader<bool>;
#[doc = "Field `CH_RANGE` writer - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. This interrupt is mutual exclusive with Pulse detect interrupt. Write with '1' to clear bit."]
pub type CH_RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CH_PULSE` reader - Pulse detect Interrupt: hardware sets this interrupt for each channel if the positive pulse counter reaches zero. This interrupt is mutual exclusive with Range detect interrupt. Write with '1' to clear bit."]
pub type CH_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `CH_PULSE` writer - Pulse detect Interrupt: hardware sets this interrupt for each channel if the positive pulse counter reaches zero. This interrupt is mutual exclusive with Range detect interrupt. Write with '1' to clear bit."]
pub type CH_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
#[doc = "Field `CH_OVERFLOW` reader - Channel overflow Interrupt: hardware sets this interrupt for each channel if a new Pulse or Range interrupt is detected while the interrupt is still pending or when DW did not acknowledge data pickup. Write with '1' to clear bit."]
pub type CH_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `CH_OVERFLOW` writer - Channel overflow Interrupt: hardware sets this interrupt for each channel if a new Pulse or Range interrupt is detected while the interrupt is still pending or when DW did not acknowledge data pickup. Write with '1' to clear bit."]
pub type CH_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Done Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn grp_done(&self) -> GRP_DONE_R {
        GRP_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancelled Interrupt: hardware sets this interrupt for the last channel of a group if the group scan was preempted and CANCELLED. Note that it is possible that also the GRP_DONE interrupt is set. If that is the case one or more new triggers were detected while the group was already busy, i.e. triggers are too fast. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn grp_cancelled(&self) -> GRP_CANCELLED_R {
        GRP_CANCELLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done and the Done interrupt is already (still) pending. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn grp_overflow(&self) -> GRP_OVERFLOW_R {
        GRP_OVERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. This interrupt is mutual exclusive with Pulse detect interrupt. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn ch_range(&self) -> CH_RANGE_R {
        CH_RANGE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pulse detect Interrupt: hardware sets this interrupt for each channel if the positive pulse counter reaches zero. This interrupt is mutual exclusive with Range detect interrupt. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn ch_pulse(&self) -> CH_PULSE_R {
        CH_PULSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel overflow Interrupt: hardware sets this interrupt for each channel if a new Pulse or Range interrupt is detected while the interrupt is still pending or when DW did not acknowledge data pickup. Write with '1' to clear bit."]
    #[inline(always)]
    pub fn ch_overflow(&self) -> CH_OVERFLOW_R {
        CH_OVERFLOW_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn grp_done(&mut self) -> GRP_DONE_W<0> {
        GRP_DONE_W::new(self)
    }
    #[doc = "Bit 1 - Cancelled Interrupt: hardware sets this interrupt for the last channel of a group if the group scan was preempted and CANCELLED. Note that it is possible that also the GRP_DONE interrupt is set. If that is the case one or more new triggers were detected while the group was already busy, i.e. triggers are too fast. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn grp_cancelled(&mut self) -> GRP_CANCELLED_W<1> {
        GRP_CANCELLED_W::new(self)
    }
    #[doc = "Bit 2 - Overflow Interrupt: hardware sets this interrupt for the last channel of a group if the group scan is done and the Done interrupt is already (still) pending. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn grp_overflow(&mut self) -> GRP_OVERFLOW_W<2> {
        GRP_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Range detect Interrupt: hardware sets this interrupt for each channel if the conversion result (after averaging) of that channel met the condition specified by the SAR_RANGE registers. This interrupt is mutual exclusive with Pulse detect interrupt. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn ch_range(&mut self) -> CH_RANGE_W<8> {
        CH_RANGE_W::new(self)
    }
    #[doc = "Bit 9 - Pulse detect Interrupt: hardware sets this interrupt for each channel if the positive pulse counter reaches zero. This interrupt is mutual exclusive with Range detect interrupt. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn ch_pulse(&mut self) -> CH_PULSE_W<9> {
        CH_PULSE_W::new(self)
    }
    #[doc = "Bit 10 - Channel overflow Interrupt: hardware sets this interrupt for each channel if a new Pulse or Range interrupt is detected while the interrupt is still pending or when DW did not acknowledge data pickup. Write with '1' to clear bit."]
    #[inline(always)]
    #[must_use]
    pub fn ch_overflow(&mut self) -> CH_OVERFLOW_W<10> {
        CH_OVERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt request register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
