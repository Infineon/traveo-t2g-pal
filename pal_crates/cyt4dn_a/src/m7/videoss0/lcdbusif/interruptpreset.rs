#[doc = "Register `INTERRUPTPRESET` writer"]
pub struct W(crate::W<INTERRUPTPRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPTPRESET_SPEC>;
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
impl From<crate::W<INTERRUPTPRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPTPRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQUENCERERRORINTERRUPTPRESET` writer - Write '1' to this field to set the SequencerError interrupt."]
pub type SEQUENCERERRORINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `SEQUENCERSYNCINTERRUPTPRESET` writer - Write '1' to this field to set the SequencerSync interrupt."]
pub type SEQUENCERSYNCINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `INSTRFIFOINTERRUPTPRESET` writer - Write '1' to this field to set the TxFifo interrupt."]
pub type INSTRFIFOINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `RXFIFOINTERRUPTPRESET` writer - Write '1' to this field to set the RxFifo interrupt."]
pub type RXFIFOINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `TEARINGEFFECTINTERRUPTPRESET` writer - Write '1' to this field to set the TearingEffect interrupt."]
pub type TEARINGEFFECTINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `READCHANNELDONEINTERRUPTPRESET` writer - Write '1' to this field to set the ReadChannelDone interrupt."]
pub type READCHANNELDONEINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
#[doc = "Field `WRITECHANNELDONEINTERRUPTPRESET` writer - Write '1' to this field to set the WriteChannelDone interrupt."]
pub type WRITECHANNELDONEINTERRUPTPRESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTPRESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write '1' to this field to set the SequencerError interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sequencererrorinterruptpreset(&mut self) -> SEQUENCERERRORINTERRUPTPRESET_W<0> {
        SEQUENCERERRORINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to this field to set the SequencerSync interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sequencersyncinterruptpreset(&mut self) -> SEQUENCERSYNCINTERRUPTPRESET_W<1> {
        SEQUENCERSYNCINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to this field to set the TxFifo interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn instrfifointerruptpreset(&mut self) -> INSTRFIFOINTERRUPTPRESET_W<2> {
        INSTRFIFOINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to this field to set the RxFifo interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifointerruptpreset(&mut self) -> RXFIFOINTERRUPTPRESET_W<3> {
        RXFIFOINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to this field to set the TearingEffect interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tearingeffectinterruptpreset(&mut self) -> TEARINGEFFECTINTERRUPTPRESET_W<4> {
        TEARINGEFFECTINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to this field to set the ReadChannelDone interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn readchanneldoneinterruptpreset(&mut self) -> READCHANNELDONEINTERRUPTPRESET_W<5> {
        READCHANNELDONEINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to this field to set the WriteChannelDone interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn writechanneldoneinterruptpreset(&mut self) -> WRITECHANNELDONEINTERRUPTPRESET_W<6> {
        WRITECHANNELDONEINTERRUPTPRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Bus Interface interrupt preset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interruptpreset](index.html) module"]
pub struct INTERRUPTPRESET_SPEC;
impl crate::RegisterSpec for INTERRUPTPRESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [interruptpreset::W](W) writer structure"]
impl crate::Writable for INTERRUPTPRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPTPRESET to value 0"]
impl crate::Resettable for INTERRUPTPRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
