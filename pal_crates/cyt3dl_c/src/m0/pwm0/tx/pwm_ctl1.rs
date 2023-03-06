#[doc = "Register `PWM_CTL1` reader"]
pub struct R(crate::R<PWM_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL1` writer"]
pub struct W(crate::W<PWM_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL1_SPEC>;
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
impl From<crate::W<PWM_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIN` reader - Minimum PWM clip value in the range \\[0, 65535\\]. Note: This field is typically not used for the H-bridge format (field value should be set to '0')."]
pub type MIN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MIN` writer - Minimum PWM clip value in the range \\[0, 65535\\]. Note: This field is typically not used for the H-bridge format (field value should be set to '0')."]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `MAX` reader - Maximum PWM clip value in the range \\[0, 65535\\]. pwm\\[15:0\\]
= (pcm\\[16:0\\]
> MAX) ? MAX : (pcm\\[16:0\\]
&lt; MIN) ? MIN : pcm\\[15:0\\]
Note: The MIN and MAX values are typically in the range \\[0, PWM_CTL2.PERIOD\\]."]
pub type MAX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX` writer - Maximum PWM clip value in the range \\[0, 65535\\]. pwm\\[15:0\\]
= (pcm\\[16:0\\]
> MAX) ? MAX : (pcm\\[16:0\\]
&lt; MIN) ? MIN : pcm\\[15:0\\]
Note: The MIN and MAX values are typically in the range \\[0, PWM_CTL2.PERIOD\\]."]
pub type MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Minimum PWM clip value in the range \\[0, 65535\\]. Note: This field is typically not used for the H-bridge format (field value should be set to '0')."]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Maximum PWM clip value in the range \\[0, 65535\\]. pwm\\[15:0\\]
= (pcm\\[16:0\\]
> MAX) ? MAX : (pcm\\[16:0\\]
&lt; MIN) ? MIN : pcm\\[15:0\\]
Note: The MIN and MAX values are typically in the range \\[0, PWM_CTL2.PERIOD\\]."]
    #[inline(always)]
    pub fn max(&self) -> MAX_R {
        MAX_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum PWM clip value in the range \\[0, 65535\\]. Note: This field is typically not used for the H-bridge format (field value should be set to '0')."]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<0> {
        MIN_W::new(self)
    }
    #[doc = "Bits 16:31 - Maximum PWM clip value in the range \\[0, 65535\\]. pwm\\[15:0\\]
= (pcm\\[16:0\\]
> MAX) ? MAX : (pcm\\[16:0\\]
&lt; MIN) ? MIN : pcm\\[15:0\\]
Note: The MIN and MAX values are typically in the range \\[0, PWM_CTL2.PERIOD\\]."]
    #[inline(always)]
    #[must_use]
    pub fn max(&mut self) -> MAX_W<16> {
        MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl1](index.html) module"]
pub struct PWM_CTL1_SPEC;
impl crate::RegisterSpec for PWM_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl1::R](R) reader structure"]
impl crate::Readable for PWM_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl1::W](W) writer structure"]
impl crate::Writable for PWM_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CTL1 to value 0xffff_0000"]
impl crate::Resettable for PWM_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
