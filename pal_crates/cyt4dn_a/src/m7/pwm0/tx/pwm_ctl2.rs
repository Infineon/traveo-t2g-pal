#[doc = "Register `PWM_CTL2` reader"]
pub struct R(crate::R<PWM_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL2` writer"]
pub struct W(crate::W<PWM_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL2_SPEC>;
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
impl From<crate::W<PWM_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD` reader - Period in PWM interface clock cycles (minus 1)."]
pub type PERIOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERIOD` writer - Period in PWM interface clock cycles (minus 1)."]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL2_SPEC, u16, u16, 16, O>;
#[doc = "Field `OFFSET` reader - Offset in the range \\[0, 0xffff\\]. Note: This field is only used in 'E-bridge' format. Typically, the offset is half of the PWM period: OFFSET = (PERIOD +1) >> 1."]
pub type OFFSET_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFFSET` writer - Offset in the range \\[0, 0xffff\\]. Note: This field is only used in 'E-bridge' format. Typically, the offset is half of the PWM period: OFFSET = (PERIOD +1) >> 1."]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Period in PWM interface clock cycles (minus 1)."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Offset in the range \\[0, 0xffff\\]. Note: This field is only used in 'E-bridge' format. Typically, the offset is half of the PWM period: OFFSET = (PERIOD +1) >> 1."]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period in PWM interface clock cycles (minus 1)."]
    #[inline(always)]
    #[must_use]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Bits 16:31 - Offset in the range \\[0, 0xffff\\]. Note: This field is only used in 'E-bridge' format. Typically, the offset is half of the PWM period: OFFSET = (PERIOD +1) >> 1."]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<16> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl2](index.html) module"]
pub struct PWM_CTL2_SPEC;
impl crate::RegisterSpec for PWM_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl2::R](R) reader structure"]
impl crate::Readable for PWM_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl2::W](W) writer structure"]
impl crate::Writable for PWM_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CTL2 to value 0x8000_ffff"]
impl crate::Resettable for PWM_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_ffff;
}
