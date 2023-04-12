#[doc = "Register `PR_LFSR_CTL0` reader"]
pub struct R(crate::R<PR_LFSR_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_LFSR_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_LFSR_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_LFSR_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_LFSR_CTL0` writer"]
pub struct W(crate::W<PR_LFSR_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_LFSR_CTL0_SPEC>;
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
impl From<crate::W<PR_LFSR_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_LFSR_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR32` reader - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
pub type LFSR32_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LFSR32` writer - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
pub type LFSR32_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PR_LFSR_CTL0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
    #[inline(always)]
    pub fn lfsr32(&self) -> LFSR32_R {
        LFSR32_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - State of a 32-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. This register needs to be initialized by SW. The initialization value should be different from '0'. The three PR_LFSR_CTL registers represents the state of a 32-bit, 31-bit and 29-bit LFSR. Individually, these LFSRs generate a pseudo random bit sequence that repeats itself after (2^32)-1, (2^31)-1 and (2^29)-1 bits. The numbers (2^32)-1, (2^31)-1 and (2^29)-1 are relatively prime (their greatest common denominator is '1'). The three bit sequence are combined (XOR'd) into a single bitstream to create a pseudo random bit sequence that repeats itself after ((2^32)-1) * ((2^31)-1) * ((2*29)-1) bits. The following polynomials are used: - 32-bit irreducible polynomial: x^32+x^30+x^26+x^25+1. - 31-bit irreducible polynomial: x^31+x^28+1. - 29-bit irreducible polynomial: x^29+x^27+1."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr32(&mut self) -> LFSR32_W<0> {
        LFSR32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pseudo random LFSR control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_lfsr_ctl0](index.html) module"]
pub struct PR_LFSR_CTL0_SPEC;
impl crate::RegisterSpec for PR_LFSR_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_lfsr_ctl0::R](R) reader structure"]
impl crate::Readable for PR_LFSR_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_lfsr_ctl0::W](W) writer structure"]
impl crate::Writable for PR_LFSR_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL0 to value 0xd895_9bc9"]
impl crate::Resettable for PR_LFSR_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0xd895_9bc9;
}
