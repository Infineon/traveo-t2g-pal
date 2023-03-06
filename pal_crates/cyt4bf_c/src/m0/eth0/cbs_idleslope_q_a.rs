#[doc = "Register `CBS_IDLESLOPE_Q_A` reader"]
pub struct R(crate::R<CBS_IDLESLOPE_Q_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CBS_IDLESLOPE_Q_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CBS_IDLESLOPE_Q_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CBS_IDLESLOPE_Q_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CBS_IDLESLOPE_Q_A` writer"]
pub struct W(crate::W<CBS_IDLESLOPE_Q_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CBS_IDLESLOPE_Q_A_SPEC>;
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
impl From<crate::W<CBS_IDLESLOPE_Q_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CBS_IDLESLOPE_Q_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLESLOPE_A` reader - IdleSlope value for queue A in bytes/sec for gigabit operation and nibbles/sec for 10/100 operation"]
pub type IDLESLOPE_A_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IDLESLOPE_A` writer - IdleSlope value for queue A in bytes/sec for gigabit operation and nibbles/sec for 10/100 operation"]
pub type IDLESLOPE_A_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CBS_IDLESLOPE_Q_A_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - IdleSlope value for queue A in bytes/sec for gigabit operation and nibbles/sec for 10/100 operation"]
    #[inline(always)]
    pub fn idleslope_a(&self) -> IDLESLOPE_A_R {
        IDLESLOPE_A_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IdleSlope value for queue A in bytes/sec for gigabit operation and nibbles/sec for 10/100 operation"]
    #[inline(always)]
    #[must_use]
    pub fn idleslope_a(&mut self) -> IDLESLOPE_A_W<0> {
        IDLESLOPE_A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "queue A is the highest priority queue. This would be queue 8 in an 8 queue configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cbs_idleslope_q_a](index.html) module"]
pub struct CBS_IDLESLOPE_Q_A_SPEC;
impl crate::RegisterSpec for CBS_IDLESLOPE_Q_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cbs_idleslope_q_a::R](R) reader structure"]
impl crate::Readable for CBS_IDLESLOPE_Q_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cbs_idleslope_q_a::W](W) writer structure"]
impl crate::Writable for CBS_IDLESLOPE_Q_A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CBS_IDLESLOPE_Q_A to value 0"]
impl crate::Resettable for CBS_IDLESLOPE_Q_A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
