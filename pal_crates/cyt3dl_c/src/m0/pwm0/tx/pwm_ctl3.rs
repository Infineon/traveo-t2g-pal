#[doc = "Register `PWM_CTL3` reader"]
pub struct R(crate::R<PWM_CTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL3` writer"]
pub struct W(crate::W<PWM_CTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL3_SPEC>;
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
impl From<crate::W<PWM_CTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DT` reader - Dead time in PWM interface clock cycles. Dead time is implemented by 'delaying' the rising edge of all PWM output signals (before applying LINE_POLARITY). Dead time is used to prevent 'shoot through' (direct connection between power supply and ground)."]
pub type DT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DT` writer - Dead time in PWM interface clock cycles. Dead time is implemented by 'delaying' the rising edge of all PWM output signals (before applying LINE_POLARITY). Dead time is used to prevent 'shoot through' (direct connection between power supply and ground)."]
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Dead time in PWM interface clock cycles. Dead time is implemented by 'delaying' the rising edge of all PWM output signals (before applying LINE_POLARITY). Dead time is used to prevent 'shoot through' (direct connection between power supply and ground)."]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Dead time in PWM interface clock cycles. Dead time is implemented by 'delaying' the rising edge of all PWM output signals (before applying LINE_POLARITY). Dead time is used to prevent 'shoot through' (direct connection between power supply and ground)."]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<0> {
        DT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl3](index.html) module"]
pub struct PWM_CTL3_SPEC;
impl crate::RegisterSpec for PWM_CTL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl3::R](R) reader structure"]
impl crate::Readable for PWM_CTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl3::W](W) writer structure"]
impl crate::Writable for PWM_CTL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CTL3 to value 0"]
impl crate::Resettable for PWM_CTL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
