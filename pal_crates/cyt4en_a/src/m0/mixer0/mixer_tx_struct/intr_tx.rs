#[doc = "Register `INTR_TX` reader"]
pub struct R(crate::R<INTR_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_TX` writer"]
pub struct W(crate::W<INTR_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_TX_SPEC>;
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
impl From<crate::W<INTR_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_UNDERFLOW` reader - HW sets this field to '1', when reading from an empty destination (TX) FIFO (DST_FIFO_STATUS.USED == 0). This is referred to as an underflow event. Note: HW ensures that either all or none of the frame's channels are transmitted. In a FIFO underflow situation, HW replays previous PCM data or uses a constant PCM data value of '0'."]
pub type FIFO_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_UNDERFLOW` writer - HW sets this field to '1', when reading from an empty destination (TX) FIFO (DST_FIFO_STATUS.USED == 0). This is referred to as an underflow event. Note: HW ensures that either all or none of the frame's channels are transmitted. In a FIFO underflow situation, HW replays previous PCM data or uses a constant PCM data value of '0'."]
pub type FIFO_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_SPEC, bool, O>;
#[doc = "Field `IF_UNDERFLOW` reader - HW sets this field to '1', when PCM samples are not generated in time for the interface logic (interface underflow). This may be an indication that the IP system frequency is too low wrt. the interface frequency (a SW configuration error). The interface underflow is a non-recoverable error and requires SW disabling of the channel (a SW write to INTR_TX.IF_UNDERFLOW does not resolve the interface underflow). Note: This functionality is intended for debug purposes."]
pub type IF_UNDERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `IF_UNDERFLOW` writer - HW sets this field to '1', when PCM samples are not generated in time for the interface logic (interface underflow). This may be an indication that the IP system frequency is too low wrt. the interface frequency (a SW configuration error). The interface underflow is a non-recoverable error and requires SW disabling of the channel (a SW write to INTR_TX.IF_UNDERFLOW does not resolve the interface underflow). Note: This functionality is intended for debug purposes."]
pub type IF_UNDERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_TX_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - HW sets this field to '1', when reading from an empty destination (TX) FIFO (DST_FIFO_STATUS.USED == 0). This is referred to as an underflow event. Note: HW ensures that either all or none of the frame's channels are transmitted. In a FIFO underflow situation, HW replays previous PCM data or uses a constant PCM data value of '0'."]
    #[inline(always)]
    pub fn fifo_underflow(&self) -> FIFO_UNDERFLOW_R {
        FIFO_UNDERFLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when PCM samples are not generated in time for the interface logic (interface underflow). This may be an indication that the IP system frequency is too low wrt. the interface frequency (a SW configuration error). The interface underflow is a non-recoverable error and requires SW disabling of the channel (a SW write to INTR_TX.IF_UNDERFLOW does not resolve the interface underflow). Note: This functionality is intended for debug purposes."]
    #[inline(always)]
    pub fn if_underflow(&self) -> IF_UNDERFLOW_R {
        IF_UNDERFLOW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - HW sets this field to '1', when reading from an empty destination (TX) FIFO (DST_FIFO_STATUS.USED == 0). This is referred to as an underflow event. Note: HW ensures that either all or none of the frame's channels are transmitted. In a FIFO underflow situation, HW replays previous PCM data or uses a constant PCM data value of '0'."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_underflow(&mut self) -> FIFO_UNDERFLOW_W<2> {
        FIFO_UNDERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - HW sets this field to '1', when PCM samples are not generated in time for the interface logic (interface underflow). This may be an indication that the IP system frequency is too low wrt. the interface frequency (a SW configuration error). The interface underflow is a non-recoverable error and requires SW disabling of the channel (a SW write to INTR_TX.IF_UNDERFLOW does not resolve the interface underflow). Note: This functionality is intended for debug purposes."]
    #[inline(always)]
    #[must_use]
    pub fn if_underflow(&mut self) -> IF_UNDERFLOW_W<8> {
        IF_UNDERFLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_tx](index.html) module"]
pub struct INTR_TX_SPEC;
impl crate::RegisterSpec for INTR_TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_tx::R](R) reader structure"]
impl crate::Readable for INTR_TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_tx::W](W) writer structure"]
impl crate::Writable for INTR_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_TX to value 0"]
impl crate::Resettable for INTR_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
