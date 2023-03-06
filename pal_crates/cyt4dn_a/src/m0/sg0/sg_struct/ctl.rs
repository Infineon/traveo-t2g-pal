#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FORMAT` reader - Output format: '0': Separate. The 'sg_ampl_out' (amplitude signal) and 'sg_tone_out' (tone signal) output signals are driven seperately. '1': Combined. The 'sg_tone_out' output signal is the logical AND of the amplitude signal and tone signal."]
pub type FORMAT_R = crate::BitReader<FORMAT_A>;
#[doc = "Output format: '0': Separate. The 'sg_ampl_out' (amplitude signal) and 'sg_tone_out' (tone signal) output signals are driven seperately. '1': Combined. The 'sg_tone_out' output signal is the logical AND of the amplitude signal and tone signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FORMAT_A {
    #[doc = "0: N/A"]
    SEPARATE = 0,
    #[doc = "1: N/A"]
    COMBINED = 1,
}
impl From<FORMAT_A> for bool {
    #[inline(always)]
    fn from(variant: FORMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl FORMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORMAT_A {
        match self.bits {
            false => FORMAT_A::SEPARATE,
            true => FORMAT_A::COMBINED,
        }
    }
    #[doc = "Checks if the value of the field is `SEPARATE`"]
    #[inline(always)]
    pub fn is_separate(&self) -> bool {
        *self == FORMAT_A::SEPARATE
    }
    #[doc = "Checks if the value of the field is `COMBINED`"]
    #[inline(always)]
    pub fn is_combined(&self) -> bool {
        *self == FORMAT_A::COMBINED
    }
}
#[doc = "Field `FORMAT` writer - Output format: '0': Separate. The 'sg_ampl_out' (amplitude signal) and 'sg_tone_out' (tone signal) output signals are driven seperately. '1': Combined. The 'sg_tone_out' output signal is the logical AND of the amplitude signal and tone signal."]
pub type FORMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, FORMAT_A, O>;
impl<'a, const O: u8> FORMAT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn separate(self) -> &'a mut W {
        self.variant(FORMAT_A::SEPARATE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn combined(self) -> &'a mut W {
        self.variant(FORMAT_A::COMBINED)
    }
}
#[doc = "Field `ACTIVE` reader - Activate functionality: '0': Transmitter off. The UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes. Typically, SW sets ACTIVE to '1' after the segment structure and the buffered segment structure are written with valid information."]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - Activate functionality: '0': Transmitter off. The UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes. Typically, SW sets ACTIVE to '1' after the segment structure and the buffered segment structure are written with valid information."]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. TIME_CTL, ..., INTR_TX register) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. TIME_CTL, ..., INTR_TX register) have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Output format: '0': Separate. The 'sg_ampl_out' (amplitude signal) and 'sg_tone_out' (tone signal) output signals are driven seperately. '1': Combined. The 'sg_tone_out' output signal is the logical AND of the amplitude signal and tone signal."]
    #[inline(always)]
    pub fn format(&self) -> FORMAT_R {
        FORMAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Transmitter off. The UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes. Typically, SW sets ACTIVE to '1' after the segment structure and the buffered segment structure are written with valid information."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 31 - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. TIME_CTL, ..., INTR_TX register) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output format: '0': Separate. The 'sg_ampl_out' (amplitude signal) and 'sg_tone_out' (tone signal) output signals are driven seperately. '1': Combined. The 'sg_tone_out' output signal is the logical AND of the amplitude signal and tone signal."]
    #[inline(always)]
    #[must_use]
    pub fn format(&mut self) -> FORMAT_W<0> {
        FORMAT_W::new(self)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Transmitter off. The UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes. Typically, SW sets ACTIVE to '1' after the segment structure and the buffered segment structure are written with valid information."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<18> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 31 - Source enable: '0': Disabled. All non-retained MMIO registers (e.g. TIME_CTL, ..., INTR_TX register) have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
