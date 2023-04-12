#[doc = "Register `TSU_TIMER_INCR` reader"]
pub struct R(crate::R<TSU_TIMER_INCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_TIMER_INCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_TIMER_INCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_TIMER_INCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_TIMER_INCR` writer"]
pub struct W(crate::W<TSU_TIMER_INCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_INCR_SPEC>;
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
impl From<crate::W<TSU_TIMER_INCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_INCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NS_INCREMENT` reader - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle. These are the most significant 8 bits of the 32 bit timer_increment counter. The tsu_timer_incr_sub_nsec register holds the least significant 24 bits of the increment."]
pub type NS_INCREMENT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NS_INCREMENT` writer - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle. These are the most significant 8 bits of the 32 bit timer_increment counter. The tsu_timer_incr_sub_nsec register holds the least significant 24 bits of the increment."]
pub type NS_INCREMENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_INCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `ALT_NS_INCR` reader - Alternative nanoseconds count. Alternative count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
pub type ALT_NS_INCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALT_NS_INCR` writer - Alternative nanoseconds count. Alternative count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
pub type ALT_NS_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_INCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `NUM_INCS` reader - Number of incs before alt inc. The number of increments after which the alternative increment is used."]
pub type NUM_INCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUM_INCS` writer - Number of incs before alt inc. The number of increments after which the alternative increment is used."]
pub type NUM_INCS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_INCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle. These are the most significant 8 bits of the 32 bit timer_increment counter. The tsu_timer_incr_sub_nsec register holds the least significant 24 bits of the increment."]
    #[inline(always)]
    pub fn ns_increment(&self) -> NS_INCREMENT_R {
        NS_INCREMENT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count. Alternative count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
    #[inline(always)]
    pub fn alt_ns_incr(&self) -> ALT_NS_INCR_R {
        ALT_NS_INCR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc. The number of increments after which the alternative increment is used."]
    #[inline(always)]
    pub fn num_incs(&self) -> NUM_INCS_R {
        NUM_INCS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - A count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle. These are the most significant 8 bits of the 32 bit timer_increment counter. The tsu_timer_incr_sub_nsec register holds the least significant 24 bits of the increment."]
    #[inline(always)]
    #[must_use]
    pub fn ns_increment(&mut self) -> NS_INCREMENT_W<0> {
        NS_INCREMENT_W::new(self)
    }
    #[doc = "Bits 8:15 - Alternative nanoseconds count. Alternative count of nanoseconds by which the 1588 timer nanoseconds register will be incremented each clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn alt_ns_incr(&mut self) -> ALT_NS_INCR_W<8> {
        ALT_NS_INCR_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of incs before alt inc. The number of increments after which the alternative increment is used."]
    #[inline(always)]
    #[must_use]
    pub fn num_incs(&mut self) -> NUM_INCS_W<16> {
        NUM_INCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_incr](index.html) module"]
pub struct TSU_TIMER_INCR_SPEC;
impl crate::RegisterSpec for TSU_TIMER_INCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_timer_incr::R](R) reader structure"]
impl crate::Readable for TSU_TIMER_INCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_incr::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_INCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_INCR to value 0"]
impl crate::Resettable for TSU_TIMER_INCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
