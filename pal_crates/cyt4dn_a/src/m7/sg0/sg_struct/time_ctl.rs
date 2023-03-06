#[doc = "Register `TIME_CTL` reader"]
pub struct R(crate::R<TIME_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME_CTL` writer"]
pub struct W(crate::W<TIME_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_CTL_SPEC>;
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
impl From<crate::W<TIME_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIOD8` reader - Time period in tone periods: a single time period is (PERIOD + 1) tone periods. A tone period is considered a time cycle. HW updates the AMPL_CTL.HIGH value after completion of a time period."]
pub type PERIOD8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIOD8` writer - Time period in tone periods: a single time period is (PERIOD + 1) tone periods. A tone period is considered a time cycle. HW updates the AMPL_CTL.HIGH value after completion of a time period."]
pub type PERIOD8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `NR` reader - Segment in time periods: a single segment is (NR+1) timer periods. HW activates the 'completion' event on completion of a segment descriptor (in registers STEP_CTL, AMPL_CTL, TONE_CTL and TIME_CTL)."]
pub type NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NR` writer - Segment in time periods: a single segment is (NR+1) timer periods. HW activates the 'completion' event on completion of a segment descriptor (in registers STEP_CTL, AMPL_CTL, TONE_CTL and TIME_CTL)."]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_CTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Time period in tone periods: a single time period is (PERIOD + 1) tone periods. A tone period is considered a time cycle. HW updates the AMPL_CTL.HIGH value after completion of a time period."]
    #[inline(always)]
    pub fn period8(&self) -> PERIOD8_R {
        PERIOD8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Segment in time periods: a single segment is (NR+1) timer periods. HW activates the 'completion' event on completion of a segment descriptor (in registers STEP_CTL, AMPL_CTL, TONE_CTL and TIME_CTL)."]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time period in tone periods: a single time period is (PERIOD + 1) tone periods. A tone period is considered a time cycle. HW updates the AMPL_CTL.HIGH value after completion of a time period."]
    #[inline(always)]
    #[must_use]
    pub fn period8(&mut self) -> PERIOD8_W<0> {
        PERIOD8_W::new(self)
    }
    #[doc = "Bits 16:23 - Segment in time periods: a single segment is (NR+1) timer periods. HW activates the 'completion' event on completion of a segment descriptor (in registers STEP_CTL, AMPL_CTL, TONE_CTL and TIME_CTL)."]
    #[inline(always)]
    #[must_use]
    pub fn nr(&mut self) -> NR_W<16> {
        NR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time_ctl](index.html) module"]
pub struct TIME_CTL_SPEC;
impl crate::RegisterSpec for TIME_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time_ctl::R](R) reader structure"]
impl crate::Readable for TIME_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time_ctl::W](W) writer structure"]
impl crate::Writable for TIME_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME_CTL to value 0"]
impl crate::Resettable for TIME_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
