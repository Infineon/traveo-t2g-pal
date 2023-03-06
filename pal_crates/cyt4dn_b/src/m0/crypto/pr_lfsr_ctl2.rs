#[doc = "Register `PR_LFSR_CTL2` reader"]
pub struct R(crate::R<PR_LFSR_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_LFSR_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_LFSR_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_LFSR_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR_LFSR_CTL2` writer"]
pub struct W(crate::W<PR_LFSR_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_LFSR_CTL2_SPEC>;
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
impl From<crate::W<PR_LFSR_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_LFSR_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LFSR29` reader - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type LFSR29_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LFSR29` writer - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
pub type LFSR29_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PR_LFSR_CTL2_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:28 - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    pub fn lfsr29(&self) -> LFSR29_R {
        LFSR29_R::new(self.bits & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:28 - State of a 29-bit Linear Feedback Shift Registers (LFSR) that is used to generate a pseudo random bit sequence. See PR_LFSR_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn lfsr29(&mut self) -> LFSR29_W<0> {
        LFSR29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pseudo random LFSR control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr_lfsr_ctl2](index.html) module"]
pub struct PR_LFSR_CTL2_SPEC;
impl crate::RegisterSpec for PR_LFSR_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr_lfsr_ctl2::R](R) reader structure"]
impl crate::Readable for PR_LFSR_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr_lfsr_ctl2::W](W) writer structure"]
impl crate::Writable for PR_LFSR_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR_LFSR_CTL2 to value 0x060c_31b7"]
impl crate::Resettable for PR_LFSR_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x060c_31b7;
}
