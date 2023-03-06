#[doc = "Register `TR_CTL0` reader"]
pub struct R(crate::R<TR_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL0` writer"]
pub struct W(crate::W<TR_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL0_SPEC>;
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
impl From<crate::W<TR_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLE_CLOCK_DIV` reader - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
pub type SAMPLE_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAMPLE_CLOCK_DIV` writer - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
pub type SAMPLE_CLOCK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TR_CTL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `RED_CLOCK_DIV` reader - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
pub type RED_CLOCK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RED_CLOCK_DIV` writer - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
pub type RED_CLOCK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_CTL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INIT_DELAY` reader - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
pub type INIT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INIT_DELAY` writer - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
pub type INIT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_CTL0_SPEC, u8, u8, 8, O>;
#[doc = "Field `VON_NEUMANN_CORR` reader - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
pub type VON_NEUMANN_CORR_R = crate::BitReader<bool>;
#[doc = "Field `VON_NEUMANN_CORR` writer - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
pub type VON_NEUMANN_CORR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL0_SPEC, bool, O>;
#[doc = "Field `STOP_ON_AP_DETECT` reader - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type STOP_ON_AP_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `STOP_ON_AP_DETECT` writer - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type STOP_ON_AP_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL0_SPEC, bool, O>;
#[doc = "Field `STOP_ON_RC_DETECT` reader - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type STOP_ON_RC_DETECT_R = crate::BitReader<bool>;
#[doc = "Field `STOP_ON_RC_DETECT` writer - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
pub type STOP_ON_RC_DETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
    #[inline(always)]
    pub fn sample_clock_div(&self) -> SAMPLE_CLOCK_DIV_R {
        SAMPLE_CLOCK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
    #[inline(always)]
    pub fn red_clock_div(&self) -> RED_CLOCK_DIV_R {
        RED_CLOCK_DIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
    #[inline(always)]
    pub fn init_delay(&self) -> INIT_DELAY_R {
        INIT_DELAY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
    #[inline(always)]
    pub fn von_neumann_corr(&self) -> VON_NEUMANN_CORR_R {
        VON_NEUMANN_CORR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    pub fn stop_on_ap_detect(&self) -> STOP_ON_AP_DETECT_R {
        STOP_ON_AP_DETECT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    pub fn stop_on_rc_detect(&self) -> STOP_ON_RC_DETECT_R {
        STOP_ON_RC_DETECT_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Specifies the clock divider that is used to sample oscillator data. This clock divider is wrt. 'clk_sys'. '0': sample clock is 'clk_sys'. '1': sample clock is 'clk_sys'/2. ... '255': sample clock is 'clk_sys'/256."]
    #[inline(always)]
    #[must_use]
    pub fn sample_clock_div(&mut self) -> SAMPLE_CLOCK_DIV_W<0> {
        SAMPLE_CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 8:15 - Specifies the clock divider that is used to produce reduced bits. '0': 1 reduced bit is produced for each sample. '1': 1 reduced bit is produced for each 2 samples. ... '255': 1 reduced bit is produced for each 256 samples. The reduced bits are considered random bits and shifted into TR_RESULT0.DATA32."]
    #[inline(always)]
    #[must_use]
    pub fn red_clock_div(&mut self) -> RED_CLOCK_DIV_W<8> {
        RED_CLOCK_DIV_W::new(self)
    }
    #[doc = "Bits 16:23 - Specifies an initialization delay: number of removed/dropped samples before reduced bits are generated. This field should be programmed in the range \\[1, 255\\]. After starting the oscillators, at least the first 2 samples should be removed/dropped to clear the state of internal synchronizers. In addition, it is advised to drop at least the second 2 samples from the oscillators (to circumvent the semi-predictable oscillator startup behavior). This result in the default field value of '3'. Field encoding is as follows: '0': 1 sample is dropped. '1': 2 samples are dropped. ... '255': 256 samples are dropped. The TR_INITIALIZED interrupt cause is set to '1', when the initialization delay is passed."]
    #[inline(always)]
    #[must_use]
    pub fn init_delay(&mut self) -> INIT_DELAY_W<16> {
        INIT_DELAY_W::new(self)
    }
    #[doc = "Bit 24 - Specifies if the 'von Neumann corrector' is disabled or enabled: '0': disabled. '1': enabled. The 'von Neumann corrector' post-processes the reduced bits to remove a '0' or '1' bias. The corrector operates on reduced bit pairs ('oldest bit, newest bit'): '00': no bit is produced. '01': '0' bit is produced (oldest bit). '10': '1' bit is produced (oldest bit). '11': no bit is produced. Note that the corrector produces bits at a random pace and at a frequency that is 1/4 of the reduced bit frequency (reduced bits are processed in pairs, and half of the pairs do NOT produce a bit)."]
    #[inline(always)]
    #[must_use]
    pub fn von_neumann_corr(&mut self) -> VON_NEUMANN_CORR_W<24> {
        VON_NEUMANN_CORR_W::new(self)
    }
    #[doc = "Bit 28 - Specifies if TRNG functionality is stopped on an adaptive proportion test detection (when HW sets INTR.TR_AP_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_ap_detect(&mut self) -> STOP_ON_AP_DETECT_W<28> {
        STOP_ON_AP_DETECT_W::new(self)
    }
    #[doc = "Bit 29 - Specifies if TRNG functionality is stopped on a repetition count test detection (when HW sets INTR.TR_RC_DETECT to '1'): '0': Functionality is NOT stopped. '1': Functionality is stopped (TR_CTL1 fields are set to '0' by HW)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_on_rc_detect(&mut self) -> STOP_ON_RC_DETECT_W<29> {
        STOP_ON_RC_DETECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl0](index.html) module"]
pub struct TR_CTL0_SPEC;
impl crate::RegisterSpec for TR_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl0::R](R) reader structure"]
impl crate::Readable for TR_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl0::W](W) writer structure"]
impl crate::Writable for TR_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL0 to value 0x0003_0000"]
impl crate::Resettable for TR_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0003_0000;
}
