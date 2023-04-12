#[doc = "Register `IF_CTL` reader"]
pub struct R(crate::R<IF_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF_CTL` writer"]
pub struct W(crate::W<IF_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_CTL_SPEC>;
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
impl From<crate::W<IF_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLOCK_DIV` reader - Interface clock divider (legal range \\[0, 255\\]). The PWM interface clock clk_pwm is defined as clk_if / (CLOCK_DIV + 1). '0': clk_pwm frequency is clk_if frequency (1 clk_pwm cycle consists of 1 clk_if cycles). '1': clk_pwm frequency is 1/2 clk_if frequency (1 clk_pwm cycle consists of 2 clk_if cycles). '2': clk_pwm frequency is 1/3 clk_if frequency. ... '255': clk_pwm frequency is 1/256 clk_if frequency."]
pub type CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_DIV` writer - Interface clock divider (legal range \\[0, 255\\]). The PWM interface clock clk_pwm is defined as clk_if / (CLOCK_DIV + 1). '0': clk_pwm frequency is clk_if frequency (1 clk_pwm cycle consists of 1 clk_if cycles). '1': clk_pwm frequency is 1/2 clk_if frequency (1 clk_pwm cycle consists of 2 clk_if cycles). '2': clk_pwm frequency is 1/3 clk_if frequency. ... '255': clk_pwm frequency is 1/256 clk_if frequency."]
pub type CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IF_CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLOCK_SEL` reader - Interface clock clk_if selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'pwm_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
pub type CLOCK_SEL_R = crate::FieldReader<u8, CLOCK_SEL_A>;
#[doc = "Interface clock clk_if selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'pwm_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLOCK_SEL_A {
    #[doc = "0: N/A"]
    SEL_SRSS_CLOCK0 = 0,
    #[doc = "1: N/A"]
    SEL_SRSS_CLOCK1 = 1,
    #[doc = "2: N/A"]
    SEL_SRSS_CLOCK2 = 2,
    #[doc = "3: N/A"]
    SEL_SRSS_CLOCK3 = 3,
    #[doc = "4: N/A"]
    SEL_PWM_MCK_IN = 4,
}
impl From<CLOCK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLOCK_SEL_A) -> Self {
        variant as _
    }
}
impl CLOCK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCK_SEL_A> {
        match self.bits {
            0 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK0),
            1 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK1),
            2 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK2),
            3 => Some(CLOCK_SEL_A::SEL_SRSS_CLOCK3),
            4 => Some(CLOCK_SEL_A::SEL_PWM_MCK_IN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK0`"]
    #[inline(always)]
    pub fn is_sel_srss_clock0(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK0
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK1`"]
    #[inline(always)]
    pub fn is_sel_srss_clock1(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK1
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK2`"]
    #[inline(always)]
    pub fn is_sel_srss_clock2(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK2
    }
    #[doc = "Checks if the value of the field is `SEL_SRSS_CLOCK3`"]
    #[inline(always)]
    pub fn is_sel_srss_clock3(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_SRSS_CLOCK3
    }
    #[doc = "Checks if the value of the field is `SEL_PWM_MCK_IN`"]
    #[inline(always)]
    pub fn is_sel_pwm_mck_in(&self) -> bool {
        *self == CLOCK_SEL_A::SEL_PWM_MCK_IN
    }
}
#[doc = "Field `CLOCK_SEL` writer - Interface clock clk_if selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'pwm_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
pub type CLOCK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IF_CTL_SPEC, u8, CLOCK_SEL_A, 3, O>;
impl<'a, const O: u8> CLOCK_SEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock0(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock1(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock2(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK2)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_srss_clock3(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_SRSS_CLOCK3)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sel_pwm_mck_in(self) -> &'a mut W {
        self.variant(CLOCK_SEL_A::SEL_PWM_MCK_IN)
    }
}
#[doc = "Field `LINE_POLARITY` reader - Line polarity (one bit for each PWM output signal): '0': Line output signal is used 'as is'. '1': Line output signal inverted. LINE_POLARITY\\[0\\]: 'line1_p_out' output signal. LINE_POLARITY\\[1\\]: 'line1_n_out' output signal. LINE_POLARITY\\[2\\]: 'line2_p_out' output signal. LINE_POLARITY\\[3\\]: 'line2_n_out' output signal. Note: the 'line2_p_out'/'line2_n_out' output signals are not used in 'E bridge' format."]
pub type LINE_POLARITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINE_POLARITY` writer - Line polarity (one bit for each PWM output signal): '0': Line output signal is used 'as is'. '1': Line output signal inverted. LINE_POLARITY\\[0\\]: 'line1_p_out' output signal. LINE_POLARITY\\[1\\]: 'line1_n_out' output signal. LINE_POLARITY\\[2\\]: 'line2_p_out' output signal. LINE_POLARITY\\[3\\]: 'line2_n_out' output signal. Note: the 'line2_p_out'/'line2_n_out' output signals are not used in 'E bridge' format."]
pub type LINE_POLARITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IF_CTL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[0, 255\\]). The PWM interface clock clk_pwm is defined as clk_if / (CLOCK_DIV + 1). '0': clk_pwm frequency is clk_if frequency (1 clk_pwm cycle consists of 1 clk_if cycles). '1': clk_pwm frequency is 1/2 clk_if frequency (1 clk_pwm cycle consists of 2 clk_if cycles). '2': clk_pwm frequency is 1/3 clk_if frequency. ... '255': clk_pwm frequency is 1/256 clk_if frequency."]
    #[inline(always)]
    pub fn clock_div(&self) -> CLOCK_DIV_R {
        CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Interface clock clk_if selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'pwm_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
    #[inline(always)]
    pub fn clock_sel(&self) -> CLOCK_SEL_R {
        CLOCK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Line polarity (one bit for each PWM output signal): '0': Line output signal is used 'as is'. '1': Line output signal inverted. LINE_POLARITY\\[0\\]: 'line1_p_out' output signal. LINE_POLARITY\\[1\\]: 'line1_n_out' output signal. LINE_POLARITY\\[2\\]: 'line2_p_out' output signal. LINE_POLARITY\\[3\\]: 'line2_n_out' output signal. Note: the 'line2_p_out'/'line2_n_out' output signals are not used in 'E bridge' format."]
    #[inline(always)]
    pub fn line_polarity(&self) -> LINE_POLARITY_R {
        LINE_POLARITY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interface clock divider (legal range \\[0, 255\\]). The PWM interface clock clk_pwm is defined as clk_if / (CLOCK_DIV + 1). '0': clk_pwm frequency is clk_if frequency (1 clk_pwm cycle consists of 1 clk_if cycles). '1': clk_pwm frequency is 1/2 clk_if frequency (1 clk_pwm cycle consists of 2 clk_if cycles). '2': clk_pwm frequency is 1/3 clk_if frequency. ... '255': clk_pwm frequency is 1/256 clk_if frequency."]
    #[inline(always)]
    #[must_use]
    pub fn clock_div(&mut self) -> CLOCK_DIV_W<0> {
        CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:10 - Interface clock clk_if selection: '0': SRSS clock clk_if_srss\\[0\\]. '1': SRSS clock clk_if_srss\\[1\\]. '2': SRSS clock clk_if_srss\\[2\\]. '3': SRSS clock clk_if_srss\\[3\\]. '4': Master interface clock 'pwm_mck_in'. '5'-'7': SRSS clock clk_if_srss\\[0\\]. Note: intent was to have '5'-'7' as 'clock off'. Current behavior is as described above. May fix this in future IP release."]
    #[inline(always)]
    #[must_use]
    pub fn clock_sel(&mut self) -> CLOCK_SEL_W<8> {
        CLOCK_SEL_W::new(self)
    }
    #[doc = "Bits 12:15 - Line polarity (one bit for each PWM output signal): '0': Line output signal is used 'as is'. '1': Line output signal inverted. LINE_POLARITY\\[0\\]: 'line1_p_out' output signal. LINE_POLARITY\\[1\\]: 'line1_n_out' output signal. LINE_POLARITY\\[2\\]: 'line2_p_out' output signal. LINE_POLARITY\\[3\\]: 'line2_n_out' output signal. Note: the 'line2_p_out'/'line2_n_out' output signals are not used in 'E bridge' format."]
    #[inline(always)]
    #[must_use]
    pub fn line_polarity(&mut self) -> LINE_POLARITY_W<12> {
        LINE_POLARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_ctl](index.html) module"]
pub struct IF_CTL_SPEC;
impl crate::RegisterSpec for IF_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_ctl::R](R) reader structure"]
impl crate::Readable for IF_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_ctl::W](W) writer structure"]
impl crate::Writable for IF_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF_CTL to value 0x0707"]
impl crate::Resettable for IF_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0707;
}
