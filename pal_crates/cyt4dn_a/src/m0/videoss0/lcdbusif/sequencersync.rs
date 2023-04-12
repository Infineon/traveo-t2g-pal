#[doc = "Register `SEQUENCERSYNC` writer"]
pub struct W(crate::W<SEQUENCERSYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQUENCERSYNC_SPEC>;
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
impl From<crate::W<SEQUENCERSYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQUENCERSYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEQUENCERACK` writer - A write to this bit will allow the sequencer to continue from the WAIT_ACK instruction."]
pub type SEQUENCERACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQUENCERSYNC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - A write to this bit will allow the sequencer to continue from the WAIT_ACK instruction."]
    #[inline(always)]
    #[must_use]
    pub fn sequencerack(&mut self) -> SEQUENCERACK_W<0> {
        SEQUENCERACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer synchronization register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sequencersync](index.html) module"]
pub struct SEQUENCERSYNC_SPEC;
impl crate::RegisterSpec for SEQUENCERSYNC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sequencersync::W](W) writer structure"]
impl crate::Writable for SEQUENCERSYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQUENCERSYNC to value 0"]
impl crate::Resettable for SEQUENCERSYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
