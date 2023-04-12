#[doc = "Register `PWM_CTL0` reader"]
pub struct R(crate::R<PWM_CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM_CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM_CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM_CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWM_CTL0` writer"]
pub struct W(crate::W<PWM_CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM_CTL0_SPEC>;
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
impl From<crate::W<PWM_CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM_CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - PWM format: '0': 'E-bridge' format. pcm\\[16:0\\]
= gc_pcm\\[15:0\\]
+ PWM_CTL2.OFFSET. Output signals 'line1_p_out'/'line1_n_out' are used. '1': 'H-bridge' format. pcm\\[16:0\\]
= Abs (gc_pcm\\[15:0\\]) and sign = gc_pcm\\[15\\]
(PWM_CTL2.OFFSET is not used). Output signals 'line1_p_out'/'line1_n_out' (when sign is positive/'0') and 'line2_p_out'/'line2_n_out' (when sign is negative/'1') are used. '2': Undefined. '3': Undefined."]
pub type FORMAT_R = crate::FieldReader<u8, FORMAT_A>;
#[doc = "PWM format: '0': 'E-bridge' format. pcm\\[16:0\\]
= gc_pcm\\[15:0\\]
+ PWM_CTL2.OFFSET. Output signals 'line1_p_out'/'line1_n_out' are used. '1': 'H-bridge' format. pcm\\[16:0\\]
= Abs (gc_pcm\\[15:0\\]) and sign = gc_pcm\\[15\\]
(PWM_CTL2.OFFSET is not used). Output signals 'line1_p_out'/'line1_n_out' (when sign is positive/'0') and 'line2_p_out'/'line2_n_out' (when sign is negative/'1') are used. '2': Undefined. '3': Undefined.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FORMAT_A {
    #[doc = "0: N/A"]
    E_BRIDGE = 0,
    #[doc = "1: N/A"]
    H_BRIDGE = 1,
}
impl From<FORMAT_A> for u8 {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as _
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FORMAT_A> {
        match self.bits {
            0 => Some(FORMAT_A::E_BRIDGE),
            1 => Some(FORMAT_A::H_BRIDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `E_BRIDGE`"]
    #[inline(always)]
    pub fn is_e_bridge(&self) -> bool {
        *self == FORMAT_A::E_BRIDGE
    }
    #[doc = "Checks if the value of the field is `H_BRIDGE`"]
    #[inline(always)]
    pub fn is_h_bridge(&self) -> bool {
        *self == FORMAT_A::H_BRIDGE
    }
}
#[doc = "Field `FORMAT` writer - PWM format: '0': 'E-bridge' format. pcm\\[16:0\\]
= gc_pcm\\[15:0\\]
+ PWM_CTL2.OFFSET. Output signals 'line1_p_out'/'line1_n_out' are used. '1': 'H-bridge' format. pcm\\[16:0\\]
= Abs (gc_pcm\\[15:0\\]) and sign = gc_pcm\\[15\\]
(PWM_CTL2.OFFSET is not used). Output signals 'line1_p_out'/'line1_n_out' (when sign is positive/'0') and 'line2_p_out'/'line2_n_out' (when sign is negative/'1') are used. '2': Undefined. '3': Undefined."]
pub type FORMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL0_SPEC, u8, FORMAT_A, 2, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn e_bridge(self) -> &'a mut W {
        self.variant(FORMAT_A::E_BRIDGE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn h_bridge(self) -> &'a mut W {
        self.variant(FORMAT_A::H_BRIDGE)
    }
}
#[doc = "Field `SCALE` reader - Scaling of gain corrected PCM sample. See GAIN_CTL.CODE (scaling is performed as part of gain correction)."]
pub type SCALE_R = crate::FieldReader<u8, SCALE_A>;
#[doc = "Scaling of gain corrected PCM sample. See GAIN_CTL.CODE (scaling is performed as part of gain correction).\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCALE_A {
    #[doc = "0: N/A"]
    SCALE_0 = 0,
    #[doc = "1: N/A"]
    SCALE_1 = 1,
    #[doc = "15: N/A"]
    SCALE_15 = 15,
}
impl From<SCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCALE_A) -> Self {
        variant as _
    }
}
impl SCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCALE_A> {
        match self.bits {
            0 => Some(SCALE_A::SCALE_0),
            1 => Some(SCALE_A::SCALE_1),
            15 => Some(SCALE_A::SCALE_15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCALE_0`"]
    #[inline(always)]
    pub fn is_scale_0(&self) -> bool {
        *self == SCALE_A::SCALE_0
    }
    #[doc = "Checks if the value of the field is `SCALE_1`"]
    #[inline(always)]
    pub fn is_scale_1(&self) -> bool {
        *self == SCALE_A::SCALE_1
    }
    #[doc = "Checks if the value of the field is `SCALE_15`"]
    #[inline(always)]
    pub fn is_scale_15(&self) -> bool {
        *self == SCALE_A::SCALE_15
    }
}
#[doc = "Field `SCALE` writer - Scaling of gain corrected PCM sample. See GAIN_CTL.CODE (scaling is performed as part of gain correction)."]
pub type SCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PWM_CTL0_SPEC, u8, SCALE_A, 4, O>;
impl<'a, const O: u8> SCALE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn scale_0(self) -> &'a mut W {
        self.variant(SCALE_A::SCALE_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn scale_1(self) -> &'a mut W {
        self.variant(SCALE_A::SCALE_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn scale_15(self) -> &'a mut W {
        self.variant(SCALE_A::SCALE_15)
    }
}
impl R {
    #[doc = "Bits 0:1 - PWM format: '0': 'E-bridge' format. pcm\\[16:0\\]
= gc_pcm\\[15:0\\]
+ PWM_CTL2.OFFSET. Output signals 'line1_p_out'/'line1_n_out' are used. '1': 'H-bridge' format. pcm\\[16:0\\]
= Abs (gc_pcm\\[15:0\\]) and sign = gc_pcm\\[15\\]
(PWM_CTL2.OFFSET is not used). Output signals 'line1_p_out'/'line1_n_out' (when sign is positive/'0') and 'line2_p_out'/'line2_n_out' (when sign is negative/'1') are used. '2': Undefined. '3': Undefined."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Scaling of gain corrected PCM sample. See GAIN_CTL.CODE (scaling is performed as part of gain correction)."]
    #[inline(always)]
    pub fn scale(&self) -> SCALE_R {
        SCALE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM format: '0': 'E-bridge' format. pcm\\[16:0\\]
= gc_pcm\\[15:0\\]
+ PWM_CTL2.OFFSET. Output signals 'line1_p_out'/'line1_n_out' are used. '1': 'H-bridge' format. pcm\\[16:0\\]
= Abs (gc_pcm\\[15:0\\]) and sign = gc_pcm\\[15\\]
(PWM_CTL2.OFFSET is not used). Output signals 'line1_p_out'/'line1_n_out' (when sign is positive/'0') and 'line2_p_out'/'line2_n_out' (when sign is negative/'1') are used. '2': Undefined. '3': Undefined."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bits 16:19 - Scaling of gain corrected PCM sample. See GAIN_CTL.CODE (scaling is performed as part of gain correction)."]
    #[inline(always)]
    #[must_use]
    pub fn scale(&mut self) -> SCALE_W<16> {
        SCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm_ctl0](index.html) module"]
pub struct PWM_CTL0_SPEC;
impl crate::RegisterSpec for PWM_CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm_ctl0::R](R) reader structure"]
impl crate::Readable for PWM_CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm_ctl0::W](W) writer structure"]
impl crate::Writable for PWM_CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWM_CTL0 to value 0x0008_0000"]
impl crate::Resettable for PWM_CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
