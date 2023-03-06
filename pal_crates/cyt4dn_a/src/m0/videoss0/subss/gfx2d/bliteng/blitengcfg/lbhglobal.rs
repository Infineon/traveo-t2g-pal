#[doc = "Register `LBHGLOBAL` reader"]
pub struct R(crate::R<LBHGLOBAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHGLOBAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHGLOBAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHGLOBAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHGLOBAL` writer"]
pub struct W(crate::W<LBHGLOBAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHGLOBAL_SPEC>;
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
impl From<crate::W<LBHGLOBAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHGLOBAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHCYCLECOUNTERENABLE` reader - Enable performance counters. \\[Note\\]
these counters should be enabled when the link is already active"]
pub type LBHCYCLECOUNTERENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LBHCYCLECOUNTERENABLE` writer - Enable performance counters. \\[Note\\]
these counters should be enabled when the link is already active"]
pub type LBHCYCLECOUNTERENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LBHGLOBAL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable performance counters. \\[Note\\]
these counters should be enabled when the link is already active"]
    #[inline(always)]
    pub fn lbhcyclecounterenable(&self) -> LBHCYCLECOUNTERENABLE_R {
        LBHCYCLECOUNTERENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable performance counters. \\[Note\\]
these counters should be enabled when the link is already active"]
    #[inline(always)]
    #[must_use]
    pub fn lbhcyclecounterenable(&mut self) -> LBHCYCLECOUNTERENABLE_W<0> {
        LBHCYCLECOUNTERENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Settings common for all line-buffer handshake connections.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhglobal](index.html) module"]
pub struct LBHGLOBAL_SPEC;
impl crate::RegisterSpec for LBHGLOBAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhglobal::R](R) reader structure"]
impl crate::Readable for LBHGLOBAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhglobal::W](W) writer structure"]
impl crate::Writable for LBHGLOBAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHGLOBAL to value 0"]
impl crate::Resettable for LBHGLOBAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
