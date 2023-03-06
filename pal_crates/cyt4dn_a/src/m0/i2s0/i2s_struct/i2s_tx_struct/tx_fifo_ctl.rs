#[doc = "Register `TX_FIFO_CTL` reader"]
pub struct R(crate::R<TX_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_FIFO_CTL` writer"]
pub struct W(crate::W<TX_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_FIFO_CTL_SPEC>;
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
impl From<crate::W<TX_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: - INTR_TX.FIFO_TRIGGER = (# FIFO entries &lt; TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, TRIGGER_LEVEL_A>;
#[doc = "Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: - INTR_TX.FIFO_TRIGGER = (# FIFO entries &lt; TRIGGER_LEVEL)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGGER_LEVEL_A {
    #[doc = "0: N/A"]
    EMPTY = 0,
    #[doc = "1: N/A"]
    USED_1 = 1,
    #[doc = "63: N/A"]
    USED_63 = 63,
}
impl From<TRIGGER_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGGER_LEVEL_A) -> Self {
        variant as _
    }
}
impl TRIGGER_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGGER_LEVEL_A> {
        match self.bits {
            0 => Some(TRIGGER_LEVEL_A::EMPTY),
            1 => Some(TRIGGER_LEVEL_A::USED_1),
            63 => Some(TRIGGER_LEVEL_A::USED_63),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TRIGGER_LEVEL_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `USED_1`"]
    #[inline(always)]
    pub fn is_used_1(&self) -> bool {
        *self == TRIGGER_LEVEL_A::USED_1
    }
    #[doc = "Checks if the value of the field is `USED_63`"]
    #[inline(always)]
    pub fn is_used_63(&self) -> bool {
        *self == TRIGGER_LEVEL_A::USED_63
    }
}
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: - INTR_TX.FIFO_TRIGGER = (# FIFO entries &lt; TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_FIFO_CTL_SPEC, u8, TRIGGER_LEVEL_A, 6, O>;
impl<'a, const O: u8> TRIGGER_LEVEL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TRIGGER_LEVEL_A::EMPTY)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn used_1(self) -> &'a mut W {
        self.variant(TRIGGER_LEVEL_A::USED_1)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn used_63(self) -> &'a mut W {
        self.variant(TRIGGER_LEVEL_A::USED_63)
    }
}
#[doc = "Field `MUTE` reader - Mute functionality: '0': HW uses TX FIFO data. '1': HW uses a constant PCM data value of '0'. Mute does advance the FIFO read pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
pub type MUTE_R = crate::BitReader<bool>;
#[doc = "Field `MUTE` writer - Mute functionality: '0': HW uses TX FIFO data. '1': HW uses a constant PCM data value of '0'. Mute does advance the FIFO read pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `FREEZE` reader - Freeze functionality: '0': HW uses TX FIFO data and advances the FIFO read pointer. '1': HW uses a constant PCM data value of '0' or the previous channel PCM data is replayed. Freeze does not advance the FIFO read pointer (the FIFO data is not used). Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - Freeze functionality: '0': HW uses TX FIFO data and advances the FIFO read pointer. '1': HW uses a constant PCM data value of '0' or the previous channel PCM data is replayed. Freeze does not advance the FIFO read pointer (the FIFO data is not used). Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - Activate functionality: '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - Activate functionality: '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `REPLAY` reader - Replay functionality (used when FREEZE is '1' or in case of a FIFO underflow event): '0': HW uses a constant PCM data value of '0'. '1': HW uses the previous PCM data value."]
pub type REPLAY_R = crate::BitReader<bool>;
#[doc = "Field `REPLAY` writer - Replay functionality (used when FREEZE is '1' or in case of a FIFO underflow event): '0': HW uses a constant PCM data value of '0'. '1': HW uses the previous PCM data value."]
pub type REPLAY_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_FIFO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: - INTR_TX.FIFO_TRIGGER = (# FIFO entries &lt; TRIGGER_LEVEL)"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Mute functionality: '0': HW uses TX FIFO data. '1': HW uses a constant PCM data value of '0'. Mute does advance the FIFO read pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW uses TX FIFO data and advances the FIFO read pointer. '1': HW uses a constant PCM data value of '0' or the previous channel PCM data is replayed. Freeze does not advance the FIFO read pointer (the FIFO data is not used). Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Replay functionality (used when FREEZE is '1' or in case of a FIFO underflow event): '0': HW uses a constant PCM data value of '0'. '1': HW uses the previous PCM data value."]
    #[inline(always)]
    pub fn replay(&self) -> REPLAY_R {
        REPLAY_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger level. When the TX FIFO has less entries than the number of this field, a transmitter trigger event is generated: - INTR_TX.FIFO_TRIGGER = (# FIFO entries &lt; TRIGGER_LEVEL)"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - Mute functionality: '0': HW uses TX FIFO data. '1': HW uses a constant PCM data value of '0'. Mute does advance the FIFO read pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<16> {
        MUTE_W::new(self)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW uses TX FIFO data and advances the FIFO read pointer. '1': HW uses a constant PCM data value of '0' or the previous channel PCM data is replayed. Freeze does not advance the FIFO read pointer (the FIFO data is not used). Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<17> {
        FREEZE_W::new(self)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Transmitter off. The FIFO_UNDERFLOW interrupt cause will not be activated. '1': Transmitter on. The FIFO_UNDERFLOW interrupt may be activated (when an underflow event occurs). Note: This functionality is intended for startup purposes."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<18> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 19 - Replay functionality (used when FREEZE is '1' or in case of a FIFO underflow event): '0': HW uses a constant PCM data value of '0'. '1': HW uses the previous PCM data value."]
    #[inline(always)]
    #[must_use]
    pub fn replay(&mut self) -> REPLAY_W<19> {
        REPLAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_fifo_ctl](index.html) module"]
pub struct TX_FIFO_CTL_SPEC;
impl crate::RegisterSpec for TX_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_fifo_ctl::R](R) reader structure"]
impl crate::Readable for TX_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_fifo_ctl::W](W) writer structure"]
impl crate::Writable for TX_FIFO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_FIFO_CTL to value 0"]
impl crate::Resettable for TX_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
