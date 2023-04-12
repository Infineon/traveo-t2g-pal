#[doc = "Register `RX_FIFO_CTL` reader"]
pub struct R(crate::R<RX_FIFO_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX_FIFO_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX_FIFO_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX_FIFO_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RX_FIFO_CTL` writer"]
pub struct W(crate::W<RX_FIFO_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RX_FIFO_CTL_SPEC>;
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
impl From<crate::W<RX_FIFO_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RX_FIFO_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGGER_LEVEL` reader - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_RX.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER_LEVEL` writer - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_RX.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
pub type TRIGGER_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RX_FIFO_CTL_SPEC, u8, u8, 7, O>;
#[doc = "Field `FREEZE` reader - Freeze functionality: '0': HW writes to the RX FIFO and advances the FIFO write pointer. '1': HW writes from the RX FIFO have no effect: freeze will not advance the FIFO write pointer. Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
pub type FREEZE_R = crate::BitReader<bool>;
#[doc = "Field `FREEZE` writer - Freeze functionality: '0': HW writes to the RX FIFO and advances the FIFO write pointer. '1': HW writes from the RX FIFO have no effect: freeze will not advance the FIFO write pointer. Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
pub type FREEZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_FIFO_CTL_SPEC, bool, O>;
#[doc = "Field `ACTIVE` reader - Activate functionality: '0': Receiver off. The FIFO_OVERFLOW interrupt cause will not be activated. '1': Receiver on. The FIFO_OVERFLOW interrupt may be activated (when an overflow event occurs). Note: This functionality is intended for stopping purposes."]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - Activate functionality: '0': Receiver off. The FIFO_OVERFLOW interrupt cause will not be activated. '1': Receiver on. The FIFO_OVERFLOW interrupt may be activated (when an overflow event occurs). Note: This functionality is intended for stopping purposes."]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RX_FIFO_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_RX.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
    #[inline(always)]
    pub fn trigger_level(&self) -> TRIGGER_LEVEL_R {
        TRIGGER_LEVEL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW writes to the RX FIFO and advances the FIFO write pointer. '1': HW writes from the RX FIFO have no effect: freeze will not advance the FIFO write pointer. Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Receiver off. The FIFO_OVERFLOW interrupt cause will not be activated. '1': Receiver on. The FIFO_OVERFLOW interrupt may be activated (when an overflow event occurs). Note: This functionality is intended for stopping purposes."]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Trigger level. When the RX FIFO has more entries than the number of this field, a receiver trigger event is generated: - INTR_RX.FIFO_TRIGGER = (# FIFO entries > TRIGGER_LEVEL)"]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TRIGGER_LEVEL_W<0> {
        TRIGGER_LEVEL_W::new(self)
    }
    #[doc = "Bit 17 - Freeze functionality: '0': HW writes to the RX FIFO and advances the FIFO write pointer. '1': HW writes from the RX FIFO have no effect: freeze will not advance the FIFO write pointer. Note: HW ensures that freeze functionality synchronizes on the first channel of a frame to ensure that PCM data of one channel is not misassigned to another channel. As a result, the freeze functionality can be activated at any time. Note: This functionality is intended for debugging purposes."]
    #[inline(always)]
    #[must_use]
    pub fn freeze(&mut self) -> FREEZE_W<17> {
        FREEZE_W::new(self)
    }
    #[doc = "Bit 18 - Activate functionality: '0': Receiver off. The FIFO_OVERFLOW interrupt cause will not be activated. '1': Receiver on. The FIFO_OVERFLOW interrupt may be activated (when an overflow event occurs). Note: This functionality is intended for stopping purposes."]
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
#[doc = "RX FIFO control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx_fifo_ctl](index.html) module"]
pub struct RX_FIFO_CTL_SPEC;
impl crate::RegisterSpec for RX_FIFO_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx_fifo_ctl::R](R) reader structure"]
impl crate::Readable for RX_FIFO_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rx_fifo_ctl::W](W) writer structure"]
impl crate::Writable for RX_FIFO_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RX_FIFO_CTL to value 0"]
impl crate::Resettable for RX_FIFO_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
