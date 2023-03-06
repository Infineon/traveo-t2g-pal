#[doc = "Register `TSU_TIMER_SEC` reader"]
pub struct R(crate::R<TSU_TIMER_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_TIMER_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_TIMER_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_TIMER_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_TIMER_SEC` writer"]
pub struct W(crate::W<TSU_TIMER_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_SEC_SPEC>;
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
impl From<crate::W<TSU_TIMER_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_SEC` reader - 1588 Timer Seconds Register. TSU timer value (s). Least significant 32 bits of seconds timer count. This register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF)."]
pub type TIMER_SEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_SEC` writer - 1588 Timer Seconds Register. TSU timer value (s). Least significant 32 bits of seconds timer count. This register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF)."]
pub type TIMER_SEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_SEC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register. TSU timer value (s). Least significant 32 bits of seconds timer count. This register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF)."]
    #[inline(always)]
    pub fn timer_sec(&self) -> TIMER_SEC_R {
        TIMER_SEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 1588 Timer Seconds Register. TSU timer value (s). Least significant 32 bits of seconds timer count. This register is writeable. The 48-bit counter increments by one when the 1588 nanoseconds counter counts to one second. It may also be incremented or decremented when the timer adjust register is written (if decremented from zero the 48-bit combined count would roll back to 0xFFFFFFFFFFFF)."]
    #[inline(always)]
    #[must_use]
    pub fn timer_sec(&mut self) -> TIMER_SEC_W<0> {
        TIMER_SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Seconds Register (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_sec](index.html) module"]
pub struct TSU_TIMER_SEC_SPEC;
impl crate::RegisterSpec for TSU_TIMER_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_timer_sec::R](R) reader structure"]
impl crate::Readable for TSU_TIMER_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_sec::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_SEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_SEC to value 0"]
impl crate::Resettable for TSU_TIMER_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
