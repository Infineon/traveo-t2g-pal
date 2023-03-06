#[doc = "Register `INTERRUPTENABLE` reader"]
pub struct R(crate::R<INTERRUPTENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPTENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPTENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPTENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERRUPTENABLE` writer"]
pub struct W(crate::W<INTERRUPTENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERRUPTENABLE_SPEC>;
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
impl From<crate::W<INTERRUPTENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERRUPTENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQUENCERERRORINTERRUPTENABLE` reader - If set to '1' the SequencerError interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type SEQUENCERERRORINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SEQUENCERERRORINTERRUPTENABLE` writer - If set to '1' the SequencerError interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type SEQUENCERERRORINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `SEQUENCERSYNCINTERRUPTENABLE` reader - If set to '1' the SequencerSync interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type SEQUENCERSYNCINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `SEQUENCERSYNCINTERRUPTENABLE` writer - If set to '1' the SequencerSync interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type SEQUENCERSYNCINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `INSTRFIFOINTERRUPTENABLE` reader - If set to '1' the TxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type INSTRFIFOINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `INSTRFIFOINTERRUPTENABLE` writer - If set to '1' the TxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type INSTRFIFOINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `RXFIFOINTERRUPTENABLE` reader - If set to '1' the RxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type RXFIFOINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOINTERRUPTENABLE` writer - If set to '1' the RxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type RXFIFOINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `TEARINGEFFECTINTERRUPTENABLE` reader - If set to '1' the TearingEffect interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type TEARINGEFFECTINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TEARINGEFFECTINTERRUPTENABLE` writer - If set to '1' the TearingEffect interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type TEARINGEFFECTINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `READCHANNELDONEINTERRUPTENABLE` reader - If set to '1' the ReadChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type READCHANNELDONEINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `READCHANNELDONEINTERRUPTENABLE` writer - If set to '1' the ReadChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type READCHANNELDONEINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
#[doc = "Field `WRITECHANNELDONEINTERRUPTENABLE` reader - If set to '1' the WriteChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type WRITECHANNELDONEINTERRUPTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WRITECHANNELDONEINTERRUPTENABLE` writer - If set to '1' the WriteChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
pub type WRITECHANNELDONEINTERRUPTENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INTERRUPTENABLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If set to '1' the SequencerError interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn sequencererrorinterruptenable(&self) -> SEQUENCERERRORINTERRUPTENABLE_R {
        SEQUENCERERRORINTERRUPTENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If set to '1' the SequencerSync interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn sequencersyncinterruptenable(&self) -> SEQUENCERSYNCINTERRUPTENABLE_R {
        SEQUENCERSYNCINTERRUPTENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to '1' the TxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn instrfifointerruptenable(&self) -> INSTRFIFOINTERRUPTENABLE_R {
        INSTRFIFOINTERRUPTENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set to '1' the RxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn rxfifointerruptenable(&self) -> RXFIFOINTERRUPTENABLE_R {
        RXFIFOINTERRUPTENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set to '1' the TearingEffect interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn tearingeffectinterruptenable(&self) -> TEARINGEFFECTINTERRUPTENABLE_R {
        TEARINGEFFECTINTERRUPTENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set to '1' the ReadChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn readchanneldoneinterruptenable(&self) -> READCHANNELDONEINTERRUPTENABLE_R {
        READCHANNELDONEINTERRUPTENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If set to '1' the WriteChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    pub fn writechanneldoneinterruptenable(&self) -> WRITECHANNELDONEINTERRUPTENABLE_R {
        WRITECHANNELDONEINTERRUPTENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to '1' the SequencerError interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn sequencererrorinterruptenable(&mut self) -> SEQUENCERERRORINTERRUPTENABLE_W<0> {
        SEQUENCERERRORINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 1 - If set to '1' the SequencerSync interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn sequencersyncinterruptenable(&mut self) -> SEQUENCERSYNCINTERRUPTENABLE_W<1> {
        SEQUENCERSYNCINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 2 - If set to '1' the TxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn instrfifointerruptenable(&mut self) -> INSTRFIFOINTERRUPTENABLE_W<2> {
        INSTRFIFOINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 3 - If set to '1' the RxFifo interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifointerruptenable(&mut self) -> RXFIFOINTERRUPTENABLE_W<3> {
        RXFIFOINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 4 - If set to '1' the TearingEffect interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn tearingeffectinterruptenable(&mut self) -> TEARINGEFFECTINTERRUPTENABLE_W<4> {
        TEARINGEFFECTINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 5 - If set to '1' the ReadChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn readchanneldoneinterruptenable(&mut self) -> READCHANNELDONEINTERRUPTENABLE_W<5> {
        READCHANNELDONEINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Bit 6 - If set to '1' the WriteChannelDone interupt is enabled, if set to '0' the interrupt is disabled and only the InterruptStatus register can be used to see the status."]
    #[inline(always)]
    #[must_use]
    pub fn writechanneldoneinterruptenable(&mut self) -> WRITECHANNELDONEINTERRUPTENABLE_W<6> {
        WRITECHANNELDONEINTERRUPTENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LCD Bus Interface interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interruptenable](index.html) module"]
pub struct INTERRUPTENABLE_SPEC;
impl crate::RegisterSpec for INTERRUPTENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interruptenable::R](R) reader structure"]
impl crate::Readable for INTERRUPTENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [interruptenable::W](W) writer structure"]
impl crate::Writable for INTERRUPTENABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERRUPTENABLE to value 0"]
impl crate::Resettable for INTERRUPTENABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
