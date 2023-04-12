#[doc = "Register `TONE_CTL` reader"]
pub struct R(crate::R<TONE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TONE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TONE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TONE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TONE_CTL` writer"]
pub struct W(crate::W<TONE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TONE_CTL_SPEC>;
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
impl From<crate::W<TONE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TONE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD8` reader - Tone period in amplitude periods: a single tone period is 2 * (PERIOD + 1) amplitude periods. A ampolitude period is considered a tone cycle."]
pub type PERIOD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD8` writer - Tone period in amplitude periods: a single tone period is 2 * (PERIOD + 1) amplitude periods. A ampolitude period is considered a tone cycle."]
pub type PERIOD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TONE_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `HIGH8` reader - Specifies the high/'1' time of the generated (right-aligned) tone PWM signal: the 'sg_tone_out' signal is '1' for HIGH+1 PWM periods. Clipping is applied if HIGH exceeds its legal range of \\[0, PERIOD\\]
(the high/'1' time of the period is up to 50 percent) Note: To generate a 50/50 percent duty cycle, HIGH should equal PERIOD."]
pub type HIGH8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGH8` writer - Specifies the high/'1' time of the generated (right-aligned) tone PWM signal: the 'sg_tone_out' signal is '1' for HIGH+1 PWM periods. Clipping is applied if HIGH exceeds its legal range of \\[0, PERIOD\\]
(the high/'1' time of the period is up to 50 percent) Note: To generate a 50/50 percent duty cycle, HIGH should equal PERIOD."]
pub type HIGH8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TONE_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Tone period in amplitude periods: a single tone period is 2 * (PERIOD + 1) amplitude periods. A ampolitude period is considered a tone cycle."]
    #[inline(always)]
    pub fn period8(&self) -> PERIOD8_R {
        PERIOD8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Specifies the high/'1' time of the generated (right-aligned) tone PWM signal: the 'sg_tone_out' signal is '1' for HIGH+1 PWM periods. Clipping is applied if HIGH exceeds its legal range of \\[0, PERIOD\\]
(the high/'1' time of the period is up to 50 percent) Note: To generate a 50/50 percent duty cycle, HIGH should equal PERIOD."]
    #[inline(always)]
    pub fn high8(&self) -> HIGH8_R {
        HIGH8_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Tone period in amplitude periods: a single tone period is 2 * (PERIOD + 1) amplitude periods. A ampolitude period is considered a tone cycle."]
    #[inline(always)]
    #[must_use]
    pub fn period8(&mut self) -> PERIOD8_W<0> {
        PERIOD8_W::new(self)
    }
    #[doc = "Bits 16:23 - Specifies the high/'1' time of the generated (right-aligned) tone PWM signal: the 'sg_tone_out' signal is '1' for HIGH+1 PWM periods. Clipping is applied if HIGH exceeds its legal range of \\[0, PERIOD\\]
(the high/'1' time of the period is up to 50 percent) Note: To generate a 50/50 percent duty cycle, HIGH should equal PERIOD."]
    #[inline(always)]
    #[must_use]
    pub fn high8(&mut self) -> HIGH8_W<16> {
        HIGH8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tone control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tone_ctl](index.html) module"]
pub struct TONE_CTL_SPEC;
impl crate::RegisterSpec for TONE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tone_ctl::R](R) reader structure"]
impl crate::Readable for TONE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tone_ctl::W](W) writer structure"]
impl crate::Writable for TONE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TONE_CTL to value 0"]
impl crate::Resettable for TONE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
