#[doc = "Register `TSU_TIMER_INCR_SUB_NSEC` reader"]
pub struct R(crate::R<TSU_TIMER_INCR_SUB_NSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_TIMER_INCR_SUB_NSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_TIMER_INCR_SUB_NSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_TIMER_INCR_SUB_NSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_TIMER_INCR_SUB_NSEC` writer"]
pub struct W(crate::W<TSU_TIMER_INCR_SUB_NSEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_TIMER_INCR_SUB_NSEC_SPEC>;
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
impl From<crate::W<TSU_TIMER_INCR_SUB_NSEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_TIMER_INCR_SUB_NSEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUB_NS_INCR` reader - These are the most significant bits \\[23:8\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle. 24 bits of sub nanosecond precision gives resolution of approximately 5.86E-17 seconds (16 bits gives 15.2 femtoseconds)."]
pub type SUB_NS_INCR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUB_NS_INCR` writer - These are the most significant bits \\[23:8\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle. 24 bits of sub nanosecond precision gives resolution of approximately 5.86E-17 seconds (16 bits gives 15.2 femtoseconds)."]
pub type SUB_NS_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_INCR_SUB_NSEC_SPEC, u16, u16, 16, O>;
#[doc = "Field `SUB_NS_INCR_LSB` reader - These are the least significant bits \\[7:0\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle."]
pub type SUB_NS_INCR_LSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB_NS_INCR_LSB` writer - These are the least significant bits \\[7:0\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle."]
pub type SUB_NS_INCR_LSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_TIMER_INCR_SUB_NSEC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:15 - These are the most significant bits \\[23:8\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle. 24 bits of sub nanosecond precision gives resolution of approximately 5.86E-17 seconds (16 bits gives 15.2 femtoseconds)."]
    #[inline(always)]
    pub fn sub_ns_incr(&self) -> SUB_NS_INCR_R {
        SUB_NS_INCR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - These are the least significant bits \\[7:0\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle."]
    #[inline(always)]
    pub fn sub_ns_incr_lsb(&self) -> SUB_NS_INCR_LSB_R {
        SUB_NS_INCR_LSB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - These are the most significant bits \\[23:8\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle. 24 bits of sub nanosecond precision gives resolution of approximately 5.86E-17 seconds (16 bits gives 15.2 femtoseconds)."]
    #[inline(always)]
    #[must_use]
    pub fn sub_ns_incr(&mut self) -> SUB_NS_INCR_W<0> {
        SUB_NS_INCR_W::new(self)
    }
    #[doc = "Bits 24:31 - These are the least significant bits \\[7:0\\]
of the sub-ns value by which the 1588 timer will be incremented each clock cycle."]
    #[inline(always)]
    #[must_use]
    pub fn sub_ns_incr_lsb(&mut self) -> SUB_NS_INCR_LSB_W<24> {
        SUB_NS_INCR_LSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1588 Timer Increment Register sub nsec\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_timer_incr_sub_nsec](index.html) module"]
pub struct TSU_TIMER_INCR_SUB_NSEC_SPEC;
impl crate::RegisterSpec for TSU_TIMER_INCR_SUB_NSEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_timer_incr_sub_nsec::R](R) reader structure"]
impl crate::Readable for TSU_TIMER_INCR_SUB_NSEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_timer_incr_sub_nsec::W](W) writer structure"]
impl crate::Writable for TSU_TIMER_INCR_SUB_NSEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_TIMER_INCR_SUB_NSEC to value 0"]
impl crate::Resettable for TSU_TIMER_INCR_SUB_NSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
