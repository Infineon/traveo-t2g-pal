#[doc = "Register `TSU_TIMER_ADJUST` writer"]
pub struct W(crate::W<TSU_TIMER_ADJUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_ADJUST_SPEC>;
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
impl From<crate::W<TSU_TIMER_ADJUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_ADJUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INCREMENT_VALUE` writer - Timer increment value. The number of nanoseconds to increment or decrement the 1588 timer nanoseconds register. If necessary the 1588 seconds register will be incremented or decremented."]
pub type INCREMENT_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_ADJUST_SPEC, u32, u32, 30, O>;
#[doc = "Field `ADD_SUBTRACT` writer - Write as one to subtract from the 1588 timer. Write as zero to add to it."]
pub type ADD_SUBTRACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TSU_TIMER_ADJUST_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:29 - Timer increment value. The number of nanoseconds to increment or decrement the 1588 timer nanoseconds register. If necessary the 1588 seconds register will be incremented or decremented."]
    #[inline(always)]
    #[must_use]
    pub fn increment_value(&mut self) -> INCREMENT_VALUE_W<0> {
        INCREMENT_VALUE_W::new(self)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer. Write as zero to add to it."]
    #[inline(always)]
    #[must_use]
    pub fn add_subtract(&mut self) -> ADD_SUBTRACT_W<31> {
        ADD_SUBTRACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to adjust the value of the timer in the TSU. It allows an integral number of nanoseconds to be added or subtracted from the timer in a one-off operation. This register returns all zeroes when read.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_adjust](index.html) module"]
pub struct TSU_TIMER_ADJUST_SPEC;
impl crate::RegisterSpec for TSU_TIMER_ADJUST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_adjust::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_ADJUST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_ADJUST to value 0"]
impl crate::Resettable for TSU_TIMER_ADJUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
