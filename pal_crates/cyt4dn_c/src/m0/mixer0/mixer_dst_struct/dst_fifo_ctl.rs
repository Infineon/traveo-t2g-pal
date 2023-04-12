#[doc = "Register `DST_FIFO_CTL` reader"]
pub struct R(crate::R<DST_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST_FIFO_CTL` writer"]
pub struct W(crate::W<DST_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_FIFO_CTL_SPEC>;
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
impl From<crate::W<DST_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the destination FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_DST.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, TRIGGER_LEVEL_A>;
#[doc = "Trigger level. When the destination FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_DST.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)\n\nValue on reset: 0"]
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
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the destination FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_DST.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DST_FIFO_CTL_SPEC, u8, TRIGGER_LEVEL_A, 6, O>;
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
#[doc = "Field `MUTE` reader - Mute functionality: '0': Mixer destination unmuted; i.e. use mixed PCM data. '1': Mixer destination muted; i.e. use PCM data value of '0'. Mute does advance the FIFO write pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
pub type MUTE_R = crate::BitReader<bool>;
#[doc = "Field `MUTE` writer - Mute functionality: '0': Mixer destination unmuted; i.e. use mixed PCM data. '1': Mixer destination muted; i.e. use PCM data value of '0'. Mute does advance the FIFO write pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DST_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - Activate functionality: '0': Mixer destination off; i.e. no PCM data produced. '1': Mixer destination on; i.e. PCM data produced. Note: This functionality is intended for stopping purposes. To start the mixer, SW should first activate the sources(s) and second activate the destination. To stop the mixer, SW should first deactivate the destination and second deactivate the source(s)."]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - Activate functionality: '0': Mixer destination off; i.e. no PCM data produced. '1': Mixer destination on; i.e. PCM data produced. Note: This functionality is intended for stopping purposes. To start the mixer, SW should first activate the sources(s) and second activate the destination. To stop the mixer, SW should first deactivate the destination and second deactivate the source(s)."]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DST_FIFO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Trigger level. When the destination FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_DST.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Mute functionality: '0': Mixer destination unmuted; i.e. use mixed PCM data. '1': Mixer destination muted; i.e. use PCM data value of '0'. Mute does advance the FIFO write pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Mixer destination off; i.e. no PCM data produced. '1': Mixer destination on; i.e. PCM data produced. Note: This functionality is intended for stopping purposes. To start the mixer, SW should first activate the sources(s) and second activate the destination. To stop the mixer, SW should first deactivate the destination and second deactivate the source(s)."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Trigger level. When the destination FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_DST.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 16 - Mute functionality: '0': Mixer destination unmuted; i.e. use mixed PCM data. '1': Mixer destination muted; i.e. use PCM data value of '0'. Mute does advance the FIFO write pointer. Note: HW ensures that mute functionality synchronizes on the first channel of a frame to ensure that either all or none of the frame's channels are muted."]
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<16> {
        MUTE_W::new(self)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Mixer destination off; i.e. no PCM data produced. '1': Mixer destination on; i.e. PCM data produced. Note: This functionality is intended for stopping purposes. To start the mixer, SW should first activate the sources(s) and second activate the destination. To stop the mixer, SW should first deactivate the destination and second deactivate the source(s)."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<18> {
        ACTIVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fifo_ctl](index.html) module"]
pub struct DST_FIFO_CTL_SPEC;
impl crate::RegisterSpec for DST_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fifo_ctl::R](R) reader structure"]
impl crate::Readable for DST_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst_fifo_ctl::W](W) writer structure"]
impl crate::Writable for DST_FIFO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DST_FIFO_CTL to value 0"]
impl crate::Resettable for DST_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
