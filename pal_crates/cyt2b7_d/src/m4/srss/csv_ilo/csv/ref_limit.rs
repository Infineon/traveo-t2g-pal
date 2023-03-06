#[doc = "Register `REF_LIMIT` reader"]
pub struct R(crate::R<REF_LIMIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_LIMIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_LIMIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_LIMIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REF_LIMIT` writer"]
pub struct W(crate::W<REF_LIMIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_LIMIT_SPEC>;
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
impl From<crate::W<REF_LIMIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_LIMIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWER` reader - Cycle time lower limit. Set the lower limit -1, in reference clock cycles, before the next monitored clock event is allowed to happen. If a monitored clock event happens before this limit is reached a CSV error is detected. LOWER must be at least 1 less than UPPER. In case the clocks are asynchronous LOWER must be at least 3 less than UPPER."]
pub type LOWER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOWER` writer - Cycle time lower limit. Set the lower limit -1, in reference clock cycles, before the next monitored clock event is allowed to happen. If a monitored clock event happens before this limit is reached a CSV error is detected. LOWER must be at least 1 less than UPPER. In case the clocks are asynchronous LOWER must be at least 3 less than UPPER."]
pub type LOWER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REF_LIMIT_SPEC, u8, u8, 8, O>;
#[doc = "Field `UPPER` reader - Cycle time upper limit. Set the upper limit -1, in reference clock cycles, before (or same time) the next monitored clock event must happen. If a monitored clock event does not happen before this limit is reached, or does not happen at all (clock loss), a CSV error is detected."]
pub type UPPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPPER` writer - Cycle time upper limit. Set the upper limit -1, in reference clock cycles, before (or same time) the next monitored clock event must happen. If a monitored clock event does not happen before this limit is reached, or does not happen at all (clock loss), a CSV error is detected."]
pub type UPPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REF_LIMIT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Cycle time lower limit. Set the lower limit -1, in reference clock cycles, before the next monitored clock event is allowed to happen. If a monitored clock event happens before this limit is reached a CSV error is detected. LOWER must be at least 1 less than UPPER. In case the clocks are asynchronous LOWER must be at least 3 less than UPPER."]
    #[inline(always)]
    pub fn lower(&self) -> LOWER_R {
        LOWER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Cycle time upper limit. Set the upper limit -1, in reference clock cycles, before (or same time) the next monitored clock event must happen. If a monitored clock event does not happen before this limit is reached, or does not happen at all (clock loss), a CSV error is detected."]
    #[inline(always)]
    pub fn upper(&self) -> UPPER_R {
        UPPER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cycle time lower limit. Set the lower limit -1, in reference clock cycles, before the next monitored clock event is allowed to happen. If a monitored clock event happens before this limit is reached a CSV error is detected. LOWER must be at least 1 less than UPPER. In case the clocks are asynchronous LOWER must be at least 3 less than UPPER."]
    #[inline(always)]
    #[must_use]
    pub fn lower(&mut self) -> LOWER_W<0> {
        LOWER_W::new(self)
    }
    #[doc = "Bits 16:23 - Cycle time upper limit. Set the upper limit -1, in reference clock cycles, before (or same time) the next monitored clock event must happen. If a monitored clock event does not happen before this limit is reached, or does not happen at all (clock loss), a CSV error is detected."]
    #[inline(always)]
    #[must_use]
    pub fn upper(&mut self) -> UPPER_W<16> {
        UPPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Supervision Reference Limits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_limit](index.html) module"]
pub struct REF_LIMIT_SPEC;
impl crate::RegisterSpec for REF_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_limit::R](R) reader structure"]
impl crate::Readable for REF_LIMIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ref_limit::W](W) writer structure"]
impl crate::Writable for REF_LIMIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_LIMIT to value 0"]
impl crate::Resettable for REF_LIMIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
