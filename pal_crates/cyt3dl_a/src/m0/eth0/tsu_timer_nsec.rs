#[doc = "Register `TSU_TIMER_NSEC` reader"]
pub struct R(crate::R<TSU_TIMER_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_TIMER_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_TIMER_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_TIMER_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_TIMER_NSEC` writer"]
pub struct W(crate::W<TSU_TIMER_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_NSEC_SPEC>;
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
impl From<crate::W<TSU_TIMER_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_NSEC` reader - Timer count in nanoseconds. This register is writeable. It can also be adjusted by writes to the 1588 timer adjust register. It increments by the value of the 1588 timer increment register each clock cycle (if this register is close to zero and a write to the timer adjust register causes a decrement the seconds register will be decremented if necessary and the nanoseconds register will roll back to 9999999xx(decimal))."]
pub type TIMER_NSEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_NSEC` writer - Timer count in nanoseconds. This register is writeable. It can also be adjusted by writes to the 1588 timer adjust register. It increments by the value of the 1588 timer increment register each clock cycle (if this register is close to zero and a write to the timer adjust register causes a decrement the seconds register will be decremented if necessary and the nanoseconds register will roll back to 9999999xx(decimal))."]
pub type TIMER_NSEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_NSEC_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:29 - Timer count in nanoseconds. This register is writeable. It can also be adjusted by writes to the 1588 timer adjust register. It increments by the value of the 1588 timer increment register each clock cycle (if this register is close to zero and a write to the timer adjust register causes a decrement the seconds register will be decremented if necessary and the nanoseconds register will roll back to 9999999xx(decimal))."]
    #[inline(always)]
    pub fn timer_nsec(&self) -> TIMER_NSEC_R {
        TIMER_NSEC_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer count in nanoseconds. This register is writeable. It can also be adjusted by writes to the 1588 timer adjust register. It increments by the value of the 1588 timer increment register each clock cycle (if this register is close to zero and a write to the timer adjust register causes a decrement the seconds register will be decremented if necessary and the nanoseconds register will roll back to 9999999xx(decimal))."]
    #[inline(always)]
    #[must_use]
    pub fn timer_nsec(&mut self) -> TIMER_NSEC_W<0> {
        TIMER_NSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_nsec](index.html) module"]
pub struct TSU_TIMER_NSEC_SPEC;
impl crate::RegisterSpec for TSU_TIMER_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_timer_nsec::R](R) reader structure"]
impl crate::Readable for TSU_TIMER_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_nsec::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_NSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_NSEC to value 0"]
impl crate::Resettable for TSU_TIMER_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
