#[doc = "Register `STEP_CTL` reader"]
pub struct R(crate::R<STEP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STEP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STEP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STEP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STEP_CTL` writer"]
pub struct W(crate::W<STEP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STEP_CTL_SPEC>;
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
impl From<crate::W<STEP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STEP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STEP` reader - Specifies the amplitude step size of the generated PWM waveform. The value is a two-complements signed number with a legal range of \\[-32768, 32767\\]. The value is added to HIGH after completion of a time period. Clipping is applied if HIGH exceeds its legal range of \\[0, AMPL_CTL.PERIOD\\]."]
pub type STEP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STEP` writer - Specifies the amplitude step size of the generated PWM waveform. The value is a two-complements signed number with a legal range of \\[-32768, 32767\\]. The value is added to HIGH after completion of a time period. Clipping is applied if HIGH exceeds its legal range of \\[0, AMPL_CTL.PERIOD\\]."]
pub type STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STEP_CTL_SPEC, u16, u16, 16, O>;
#[doc = "Field `VALID` reader - Specifies validity/availability of the segment structure (TIME_CTL, NOTE_CTL, AMPL_CTL and STEP_CTL): '0': Invalid. '1': Valid."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Specifies validity/availability of the segment structure (TIME_CTL, NOTE_CTL, AMPL_CTL and STEP_CTL): '0': Invalid. '1': Valid."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, STEP_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Specifies the amplitude step size of the generated PWM waveform. The value is a two-complements signed number with a legal range of \\[-32768, 32767\\]. The value is added to HIGH after completion of a time period. Clipping is applied if HIGH exceeds its legal range of \\[0, AMPL_CTL.PERIOD\\]."]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Specifies validity/availability of the segment structure (TIME_CTL, NOTE_CTL, AMPL_CTL and STEP_CTL): '0': Invalid. '1': Valid."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specifies the amplitude step size of the generated PWM waveform. The value is a two-complements signed number with a legal range of \\[-32768, 32767\\]. The value is added to HIGH after completion of a time period. Clipping is applied if HIGH exceeds its legal range of \\[0, AMPL_CTL.PERIOD\\]."]
    #[inline(always)]
    #[must_use]
    pub fn step(&mut self) -> STEP_W<0> {
        STEP_W::new(self)
    }
    #[doc = "Bit 31 - Specifies validity/availability of the segment structure (TIME_CTL, NOTE_CTL, AMPL_CTL and STEP_CTL): '0': Invalid. '1': Valid."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Step control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [step_ctl](index.html) module"]
pub struct STEP_CTL_SPEC;
impl crate::RegisterSpec for STEP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [step_ctl::R](R) reader structure"]
impl crate::Readable for STEP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [step_ctl::W](W) writer structure"]
impl crate::Writable for STEP_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STEP_CTL to value 0"]
impl crate::Resettable for STEP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
