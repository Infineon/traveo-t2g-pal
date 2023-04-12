#[doc = "Register `SRC_FADE_CMD` reader"]
pub struct R(crate::R<SRC_FADE_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRC_FADE_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRC_FADE_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRC_FADE_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRC_FADE_CMD` writer"]
pub struct W(crate::W<SRC_FADE_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRC_FADE_CMD_SPEC>;
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
impl From<crate::W<SRC_FADE_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRC_FADE_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADE_IN` reader - 'Fade in' functionality: '0': Disabled. '1': Enabled: Increments CODE gradually to '115'. Typically, SW sets this field to '1' to start 'fade in' functionality. HW clears this field to '0' to indicate that 'fade in' has completed; i.e. 0 dB amplification is achieved."]
pub type FADE_IN_R = crate::BitReader<bool>;
#[doc = "Field `FADE_IN` writer - 'Fade in' functionality: '0': Disabled. '1': Enabled: Increments CODE gradually to '115'. Typically, SW sets this field to '1' to start 'fade in' functionality. HW clears this field to '0' to indicate that 'fade in' has completed; i.e. 0 dB amplification is achieved."]
pub type FADE_IN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC_FADE_CMD_SPEC, bool, O>;
#[doc = "Field `FADE_OUT` reader - 'Fade out' functionality: '0': Disabled. '1': Enabled: Changes CODE gradually to '0' (digital mute). Typically, SW sets this field to '1' to start 'fade out' functionality. HW clears this field to '0' to indicate that 'fade out' has completed; i.e. 'muting' is achieved. Note: FADE_IN and FADE_OUT should be used mutually exclusively. However, if 'fade in' is desired while 'fading out', SW can set FADE_IN to '1' and clear FADE_OUT to '0'. 'Fade in' will commence from the current CODE value. Similarly, if 'fade out' is desired while 'fading in', SW can set FADE_OUT to '1' and clear FADE_IN to '0'. 'Fade out' will commence from the current CODE value."]
pub type FADE_OUT_R = crate::BitReader<bool>;
#[doc = "Field `FADE_OUT` writer - 'Fade out' functionality: '0': Disabled. '1': Enabled: Changes CODE gradually to '0' (digital mute). Typically, SW sets this field to '1' to start 'fade out' functionality. HW clears this field to '0' to indicate that 'fade out' has completed; i.e. 'muting' is achieved. Note: FADE_IN and FADE_OUT should be used mutually exclusively. However, if 'fade in' is desired while 'fading out', SW can set FADE_IN to '1' and clear FADE_OUT to '0'. 'Fade in' will commence from the current CODE value. Similarly, if 'fade out' is desired while 'fading in', SW can set FADE_OUT to '1' and clear FADE_IN to '0'. 'Fade out' will commence from the current CODE value."]
pub type FADE_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC_FADE_CMD_SPEC, bool, O>;
#[doc = "Field `AUTO_DEACTIVATE` reader - Specifies behavior when 'fade out' has completed: '0': Source does NOT get deactivated. '1': Source gets deactivated; i.e. HW clears SRC_FIFO_CTL.ACTIVE to '0'."]
pub type AUTO_DEACTIVATE_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_DEACTIVATE` writer - Specifies behavior when 'fade out' has completed: '0': Source does NOT get deactivated. '1': Source gets deactivated; i.e. HW clears SRC_FIFO_CTL.ACTIVE to '0'."]
pub type AUTO_DEACTIVATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRC_FADE_CMD_SPEC, bool, O>;
#[doc = "Field `PACE` reader - Specifies the pace/speed of 'fade in' and 'fade out': '0': The same CODE value is used (0+1) times. '1': The same CODE value is used (1+1) times. ... '1023': The same CODE value is used (1023+1) times. A fade in from 'mute' to 0 dB takes 1024*116 = 118,784 PCM samples. At a PCM sample frequency Fs of 48 kHz, this translates into 118,784/48,000 = 2.474 seconds."]
pub type PACE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PACE` writer - Specifies the pace/speed of 'fade in' and 'fade out': '0': The same CODE value is used (0+1) times. '1': The same CODE value is used (1+1) times. ... '1023': The same CODE value is used (1023+1) times. A fade in from 'mute' to 0 dB takes 1024*116 = 118,784 PCM samples. At a PCM sample frequency Fs of 48 kHz, this translates into 118,784/48,000 = 2.474 seconds."]
pub type PACE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRC_FADE_CMD_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - 'Fade in' functionality: '0': Disabled. '1': Enabled: Increments CODE gradually to '115'. Typically, SW sets this field to '1' to start 'fade in' functionality. HW clears this field to '0' to indicate that 'fade in' has completed; i.e. 0 dB amplification is achieved."]
    #[inline(always)]
    pub fn fade_in(&self) -> FADE_IN_R {
        FADE_IN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 'Fade out' functionality: '0': Disabled. '1': Enabled: Changes CODE gradually to '0' (digital mute). Typically, SW sets this field to '1' to start 'fade out' functionality. HW clears this field to '0' to indicate that 'fade out' has completed; i.e. 'muting' is achieved. Note: FADE_IN and FADE_OUT should be used mutually exclusively. However, if 'fade in' is desired while 'fading out', SW can set FADE_IN to '1' and clear FADE_OUT to '0'. 'Fade in' will commence from the current CODE value. Similarly, if 'fade out' is desired while 'fading in', SW can set FADE_OUT to '1' and clear FADE_IN to '0'. 'Fade out' will commence from the current CODE value."]
    #[inline(always)]
    pub fn fade_out(&self) -> FADE_OUT_R {
        FADE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Specifies behavior when 'fade out' has completed: '0': Source does NOT get deactivated. '1': Source gets deactivated; i.e. HW clears SRC_FIFO_CTL.ACTIVE to '0'."]
    #[inline(always)]
    pub fn auto_deactivate(&self) -> AUTO_DEACTIVATE_R {
        AUTO_DEACTIVATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Specifies the pace/speed of 'fade in' and 'fade out': '0': The same CODE value is used (0+1) times. '1': The same CODE value is used (1+1) times. ... '1023': The same CODE value is used (1023+1) times. A fade in from 'mute' to 0 dB takes 1024*116 = 118,784 PCM samples. At a PCM sample frequency Fs of 48 kHz, this translates into 118,784/48,000 = 2.474 seconds."]
    #[inline(always)]
    pub fn pace(&self) -> PACE_R {
        PACE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 'Fade in' functionality: '0': Disabled. '1': Enabled: Increments CODE gradually to '115'. Typically, SW sets this field to '1' to start 'fade in' functionality. HW clears this field to '0' to indicate that 'fade in' has completed; i.e. 0 dB amplification is achieved."]
    #[inline(always)]
    #[must_use]
    pub fn fade_in(&mut self) -> FADE_IN_W<0> {
        FADE_IN_W::new(self)
    }
    #[doc = "Bit 1 - 'Fade out' functionality: '0': Disabled. '1': Enabled: Changes CODE gradually to '0' (digital mute). Typically, SW sets this field to '1' to start 'fade out' functionality. HW clears this field to '0' to indicate that 'fade out' has completed; i.e. 'muting' is achieved. Note: FADE_IN and FADE_OUT should be used mutually exclusively. However, if 'fade in' is desired while 'fading out', SW can set FADE_IN to '1' and clear FADE_OUT to '0'. 'Fade in' will commence from the current CODE value. Similarly, if 'fade out' is desired while 'fading in', SW can set FADE_OUT to '1' and clear FADE_IN to '0'. 'Fade out' will commence from the current CODE value."]
    #[inline(always)]
    #[must_use]
    pub fn fade_out(&mut self) -> FADE_OUT_W<1> {
        FADE_OUT_W::new(self)
    }
    #[doc = "Bit 8 - Specifies behavior when 'fade out' has completed: '0': Source does NOT get deactivated. '1': Source gets deactivated; i.e. HW clears SRC_FIFO_CTL.ACTIVE to '0'."]
    #[inline(always)]
    #[must_use]
    pub fn auto_deactivate(&mut self) -> AUTO_DEACTIVATE_W<8> {
        AUTO_DEACTIVATE_W::new(self)
    }
    #[doc = "Bits 16:25 - Specifies the pace/speed of 'fade in' and 'fade out': '0': The same CODE value is used (0+1) times. '1': The same CODE value is used (1+1) times. ... '1023': The same CODE value is used (1023+1) times. A fade in from 'mute' to 0 dB takes 1024*116 = 118,784 PCM samples. At a PCM sample frequency Fs of 48 kHz, this translates into 118,784/48,000 = 2.474 seconds."]
    #[inline(always)]
    #[must_use]
    pub fn pace(&mut self) -> PACE_W<16> {
        PACE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source fade command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [src_fade_cmd](index.html) module"]
pub struct SRC_FADE_CMD_SPEC;
impl crate::RegisterSpec for SRC_FADE_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [src_fade_cmd::R](R) reader structure"]
impl crate::Readable for SRC_FADE_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [src_fade_cmd::W](W) writer structure"]
impl crate::Writable for SRC_FADE_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRC_FADE_CMD to value 0x01ff_0000"]
impl crate::Resettable for SRC_FADE_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x01ff_0000;
}
