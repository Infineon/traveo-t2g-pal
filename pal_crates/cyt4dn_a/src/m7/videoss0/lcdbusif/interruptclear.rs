#[doc = "Register `INTERRUPTCLEAR` writer"]
pub struct W(crate::W<INTERRUPTCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPTCLEAR_SPEC>;
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
impl From<crate::W<INTERRUPTCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPTCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQUENCERERRORINTERRUPTCLEAR` writer - Write '1' to this field to clear the SequencerError interrupt."]
pub type SEQUENCERERRORINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `SEQUENCERSYNCINTERRUPTCLEAR` writer - Write '1' to this field to clear the SequencerSync interrupt."]
pub type SEQUENCERSYNCINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `INSTRFIFOINTERRUPTCLEAR` writer - Write '1' to this field to clear the TxFifo interrupt."]
pub type INSTRFIFOINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `RXFIFOINTERRUPTCLEAR` writer - Write '1' to this field to clear the RxFifo interrupt."]
pub type RXFIFOINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `TEARINGEFFECTINTERRUPTCLEAR` writer - Write '1' to this field to clear the TearingEffect interrupt."]
pub type TEARINGEFFECTINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `READCHANNELDONEINTERRUPTCLEAR` writer - Write '1' to this field to clear the ReadChannelDone interrupt."]
pub type READCHANNELDONEINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
#[doc = "Field `WRITECHANNELDONEINTERRUPTCLEAR` writer - Write '1' to this field to clear the WriteChannelDone interrupt."]
pub type WRITECHANNELDONEINTERRUPTCLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTCLEAR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write '1' to this field to clear the SequencerError interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sequencererrorinterruptclear(&mut self) -> SEQUENCERERRORINTERRUPTCLEAR_W<0> {
        SEQUENCERERRORINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 1 - Write '1' to this field to clear the SequencerSync interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sequencersyncinterruptclear(&mut self) -> SEQUENCERSYNCINTERRUPTCLEAR_W<1> {
        SEQUENCERSYNCINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 2 - Write '1' to this field to clear the TxFifo interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn instrfifointerruptclear(&mut self) -> INSTRFIFOINTERRUPTCLEAR_W<2> {
        INSTRFIFOINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 3 - Write '1' to this field to clear the RxFifo interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifointerruptclear(&mut self) -> RXFIFOINTERRUPTCLEAR_W<3> {
        RXFIFOINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 4 - Write '1' to this field to clear the TearingEffect interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tearingeffectinterruptclear(&mut self) -> TEARINGEFFECTINTERRUPTCLEAR_W<4> {
        TEARINGEFFECTINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 5 - Write '1' to this field to clear the ReadChannelDone interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn readchanneldoneinterruptclear(&mut self) -> READCHANNELDONEINTERRUPTCLEAR_W<5> {
        READCHANNELDONEINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Bit 6 - Write '1' to this field to clear the WriteChannelDone interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn writechanneldoneinterruptclear(&mut self) -> WRITECHANNELDONEINTERRUPTCLEAR_W<6> {
        WRITECHANNELDONEINTERRUPTCLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Bus Interface interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interruptclear](index.html) module"]
pub struct INTERRUPTCLEAR_SPEC;
impl crate::RegisterSpec for INTERRUPTCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [interruptclear::W](W) writer structure"]
impl crate::Writable for INTERRUPTCLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPTCLEAR to value 0"]
impl crate::Resettable for INTERRUPTCLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
