#[doc = "Register `DELAY_TAP_SEL` reader"]
pub struct R(crate::R<DELAY_TAP_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY_TAP_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY_TAP_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY_TAP_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAY_TAP_SEL` writer"]
pub struct W(crate::W<DELAY_TAP_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY_TAP_SEL_SPEC>;
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
impl From<crate::W<DELAY_TAP_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY_TAP_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL` reader - Delay line tap selection in output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) and RWDS capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]). This is used to shift the strobe signal into the data eye. Note: DELAY_TAP_SEL must not be changed while STATUS.BUSY=1."]
pub type SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEL` writer - Delay line tap selection in output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) and RWDS capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]). This is used to shift the strobe signal into the data eye. Note: DELAY_TAP_SEL must not be changed while STATUS.BUSY=1."]
pub type SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAY_TAP_SEL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Delay line tap selection in output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) and RWDS capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]). This is used to shift the strobe signal into the data eye. Note: DELAY_TAP_SEL must not be changed while STATUS.BUSY=1."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay line tap selection in output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) and RWDS capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]). This is used to shift the strobe signal into the data eye. Note: DELAY_TAP_SEL must not be changed while STATUS.BUSY=1."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SEL_W<0> {
        SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX Clock Delay Tap Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay_tap_sel](index.html) module"]
pub struct DELAY_TAP_SEL_SPEC;
impl crate::RegisterSpec for DELAY_TAP_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay_tap_sel::R](R) reader structure"]
impl crate::Readable for DELAY_TAP_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay_tap_sel::W](W) writer structure"]
impl crate::Writable for DELAY_TAP_SEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DELAY_TAP_SEL to value 0x01"]
impl crate::Resettable for DELAY_TAP_SEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
