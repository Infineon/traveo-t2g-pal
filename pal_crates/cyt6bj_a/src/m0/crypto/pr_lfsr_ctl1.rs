#[doc = "Register `PR_LFSR_CTL1` reader"]
pub struct R(crate::R<PR_LFSR_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_LFSR_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_LFSR_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_LFSR_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_LFSR_CTL1` writer"]
pub struct W(crate::W<PR_LFSR_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_LFSR_CTL1_SPEC>;
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
impl From<crate::W<PR_LFSR_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_LFSR_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR31` reader - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type LFSR31_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LFSR31` writer - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type LFSR31_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PR_LFSR_CTL1_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    pub fn lfsr31(&self) -> LFSR31_R {
        LFSR31_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - State of a 31-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr31(&mut self) -> LFSR31_W<0> {
        LFSR31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pseudo random LFSR control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_lfsr_ctl1](index.html) module"]
pub struct PR_LFSR_CTL1_SPEC;
impl crate::RegisterSpec for PR_LFSR_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_lfsr_ctl1::R](R) reader structure"]
impl crate::Readable for PR_LFSR_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_lfsr_ctl1::W](W) writer structure"]
impl crate::Writable for PR_LFSR_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL1 to value 0x2bb9_11f8"]
impl crate::Resettable for PR_LFSR_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2bb9_11f8;
}
