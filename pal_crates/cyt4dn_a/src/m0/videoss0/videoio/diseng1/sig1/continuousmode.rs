#[doc = "Register `CONTINUOUSMODE` reader"]
pub struct R(crate::R<CONTINUOUSMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTINUOUSMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTINUOUSMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTINUOUSMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTINUOUSMODE` writer"]
pub struct W(crate::W<CONTINUOUSMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTINUOUSMODE_SPEC>;
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
impl From<crate::W<CONTINUOUSMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTINUOUSMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENCONT` reader - EnCont = 0: disables continuous mode. Signatures will be generated only once and in idle state after completion of measurement. In this 'ONE_TIME' mode the signature must be triggered by 'Kick' field. EnCont = 1: enables continuous mode. Write '1' to this field, signature starts and measures every incoming frame until EnCont is turned to 0. 'Kick' has no effect when 'EnCont' = 1."]
pub type ENCONT_R = crate::BitReader<bool>;
#[doc = "Field `ENCONT` writer - EnCont = 0: disables continuous mode. Signatures will be generated only once and in idle state after completion of measurement. In this 'ONE_TIME' mode the signature must be triggered by 'Kick' field. EnCont = 1: enables continuous mode. Write '1' to this field, signature starts and measures every incoming frame until EnCont is turned to 0. 'Kick' has no effect when 'EnCont' = 1."]
pub type ENCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTINUOUSMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EnCont = 0: disables continuous mode. Signatures will be generated only once and in idle state after completion of measurement. In this 'ONE_TIME' mode the signature must be triggered by 'Kick' field. EnCont = 1: enables continuous mode. Write '1' to this field, signature starts and measures every incoming frame until EnCont is turned to 0. 'Kick' has no effect when 'EnCont' = 1."]
    #[inline(always)]
    pub fn encont(&self) -> ENCONT_R {
        ENCONT_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EnCont = 0: disables continuous mode. Signatures will be generated only once and in idle state after completion of measurement. In this 'ONE_TIME' mode the signature must be triggered by 'Kick' field. EnCont = 1: enables continuous mode. Write '1' to this field, signature starts and measures every incoming frame until EnCont is turned to 0. 'Kick' has no effect when 'EnCont' = 1."]
    #[inline(always)]
    #[must_use]
    pub fn encont(&mut self) -> ENCONT_W<0> {
        ENCONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signature operation mode control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [continuousmode](index.html) module"]
pub struct CONTINUOUSMODE_SPEC;
impl crate::RegisterSpec for CONTINUOUSMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [continuousmode::R](R) reader structure"]
impl crate::Readable for CONTINUOUSMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [continuousmode::W](W) writer structure"]
impl crate::Writable for CONTINUOUSMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTINUOUSMODE to value 0"]
impl crate::Resettable for CONTINUOUSMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
