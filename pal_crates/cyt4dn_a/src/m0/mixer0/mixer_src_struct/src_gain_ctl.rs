#[doc = "Register `SRC_GAIN_CTL` reader"]
pub struct R(crate::R<SRC_GAIN_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_GAIN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_GAIN_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_GAIN_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_GAIN_CTL` writer"]
pub struct W(crate::W<SRC_GAIN_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_GAIN_CTL_SPEC>;
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
impl From<crate::W<SRC_GAIN_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_GAIN_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Gain code in the range \\[0, 127\\]. Each gain code increment represents 1 dB. Gain code value '0' represents digital mute. Gain code '127' represents a maximum amplification of 12 dB. The gain code is used to derive a multipler value (in the range \\[0, 8191\\]) and shift value (in the range \\[11, 26\\]). '0': digital mute. Multiplier value is '0' and 'shift' is '26'. '1': -114 dB. Multiplier value is '134' and 'shift' is '26'. '2': -113 dB. Multiplier value is '150' and 'shift' is '26'. '3': -112 dB. Multiplier value is '169' and 'shift' is '26'. ... '115': 0 dB. Multiplier value is '4096' and 'shift' is '12'. Note that (4096 * x) >> 12 = x; i.e. the PCM value is unaffected. ... '121': 6 dB. Multiplier value is '8173' and 'shift' is '12'. ... '127': 12 dB. Multiplier value is '8153' and 'shift' is '11'. gc_pcm\\[15:0\\]
= CLIP16 ((multiplier\\[12:0\\]*fir_scaled_pcm\\[15:0\\]) >> shift)"]
pub type CODE_R = crate::FieldReader<u8, CODE_A>;
#[doc = "Gain code in the range \\[0, 127\\]. Each gain code increment represents 1 dB. Gain code value '0' represents digital mute. Gain code '127' represents a maximum amplification of 12 dB. The gain code is used to derive a multipler value (in the range \\[0, 8191\\]) and shift value (in the range \\[11, 26\\]). '0': digital mute. Multiplier value is '0' and 'shift' is '26'. '1': -114 dB. Multiplier value is '134' and 'shift' is '26'. '2': -113 dB. Multiplier value is '150' and 'shift' is '26'. '3': -112 dB. Multiplier value is '169' and 'shift' is '26'. ... '115': 0 dB. Multiplier value is '4096' and 'shift' is '12'. Note that (4096 * x) >> 12 = x; i.e. the PCM value is unaffected. ... '121': 6 dB. Multiplier value is '8173' and 'shift' is '12'. ... '127': 12 dB. Multiplier value is '8153' and 'shift' is '11'. gc_pcm\\[15:0\\]
= CLIP16 ((multiplier\\[12:0\\]*fir_scaled_pcm\\[15:0\\]) >> shift)\n\nValue on reset: 115"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CODE_A {
    #[doc = "0: N/A"]
    CODE_OFF = 0,
    #[doc = "115: N/A"]
    CODE_SAME = 115,
    #[doc = "127: N/A"]
    CODE_MAX = 127,
}
impl From<CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CODE_A) -> Self {
        variant as _
    }
}
impl CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CODE_A> {
        match self.bits {
            0 => Some(CODE_A::CODE_OFF),
            115 => Some(CODE_A::CODE_SAME),
            127 => Some(CODE_A::CODE_MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CODE_OFF`"]
    #[inline(always)]
    pub fn is_code_off(&self) -> bool {
        *self == CODE_A::CODE_OFF
    }
    #[doc = "Checks if the value of the field is `CODE_SAME`"]
    #[inline(always)]
    pub fn is_code_same(&self) -> bool {
        *self == CODE_A::CODE_SAME
    }
    #[doc = "Checks if the value of the field is `CODE_MAX`"]
    #[inline(always)]
    pub fn is_code_max(&self) -> bool {
        *self == CODE_A::CODE_MAX
    }
}
#[doc = "Field `CODE` writer - Gain code in the range \\[0, 127\\]. Each gain code increment represents 1 dB. Gain code value '0' represents digital mute. Gain code '127' represents a maximum amplification of 12 dB. The gain code is used to derive a multipler value (in the range \\[0, 8191\\]) and shift value (in the range \\[11, 26\\]). '0': digital mute. Multiplier value is '0' and 'shift' is '26'. '1': -114 dB. Multiplier value is '134' and 'shift' is '26'. '2': -113 dB. Multiplier value is '150' and 'shift' is '26'. '3': -112 dB. Multiplier value is '169' and 'shift' is '26'. ... '115': 0 dB. Multiplier value is '4096' and 'shift' is '12'. Note that (4096 * x) >> 12 = x; i.e. the PCM value is unaffected. ... '121': 6 dB. Multiplier value is '8173' and 'shift' is '12'. ... '127': 12 dB. Multiplier value is '8153' and 'shift' is '11'. gc_pcm\\[15:0\\]
= CLIP16 ((multiplier\\[12:0\\]*fir_scaled_pcm\\[15:0\\]) >> shift)"]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRC_GAIN_CTL_SPEC, u8, CODE_A, 7, O>;
impl<'a, const O: u8> CODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn code_off(self) -> &'a mut W {
        self.variant(CODE_A::CODE_OFF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn code_same(self) -> &'a mut W {
        self.variant(CODE_A::CODE_SAME)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn code_max(self) -> &'a mut W {
        self.variant(CODE_A::CODE_MAX)
    }
}
impl R {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 127\\]. Each gain code increment represents 1 dB. Gain code value '0' represents digital mute. Gain code '127' represents a maximum amplification of 12 dB. The gain code is used to derive a multipler value (in the range \\[0, 8191\\]) and shift value (in the range \\[11, 26\\]). '0': digital mute. Multiplier value is '0' and 'shift' is '26'. '1': -114 dB. Multiplier value is '134' and 'shift' is '26'. '2': -113 dB. Multiplier value is '150' and 'shift' is '26'. '3': -112 dB. Multiplier value is '169' and 'shift' is '26'. ... '115': 0 dB. Multiplier value is '4096' and 'shift' is '12'. Note that (4096 * x) >> 12 = x; i.e. the PCM value is unaffected. ... '121': 6 dB. Multiplier value is '8173' and 'shift' is '12'. ... '127': 12 dB. Multiplier value is '8153' and 'shift' is '11'. gc_pcm\\[15:0\\]
= CLIP16 ((multiplier\\[12:0\\]*fir_scaled_pcm\\[15:0\\]) >> shift)"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 127\\]. Each gain code increment represents 1 dB. Gain code value '0' represents digital mute. Gain code '127' represents a maximum amplification of 12 dB. The gain code is used to derive a multipler value (in the range \\[0, 8191\\]) and shift value (in the range \\[11, 26\\]). '0': digital mute. Multiplier value is '0' and 'shift' is '26'. '1': -114 dB. Multiplier value is '134' and 'shift' is '26'. '2': -113 dB. Multiplier value is '150' and 'shift' is '26'. '3': -112 dB. Multiplier value is '169' and 'shift' is '26'. ... '115': 0 dB. Multiplier value is '4096' and 'shift' is '12'. Note that (4096 * x) >> 12 = x; i.e. the PCM value is unaffected. ... '121': 6 dB. Multiplier value is '8173' and 'shift' is '12'. ... '127': 12 dB. Multiplier value is '8153' and 'shift' is '11'. gc_pcm\\[15:0\\]
= CLIP16 ((multiplier\\[12:0\\]*fir_scaled_pcm\\[15:0\\]) >> shift)"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source gain control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_gain_ctl](index.html) module"]
pub struct SRC_GAIN_CTL_SPEC;
impl crate::RegisterSpec for SRC_GAIN_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_gain_ctl::R](R) reader structure"]
impl crate::Readable for SRC_GAIN_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_gain_ctl::W](W) writer structure"]
impl crate::Writable for SRC_GAIN_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_GAIN_CTL to value 0x73"]
impl crate::Resettable for SRC_GAIN_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x73;
}
