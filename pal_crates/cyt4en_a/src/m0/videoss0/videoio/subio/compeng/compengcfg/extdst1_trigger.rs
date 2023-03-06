#[doc = "Register `EXTDST1_TRIGGER` writer"]
pub struct W(crate::W<EXTDST1_TRIGGER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST1_TRIGGER_SPEC>;
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
impl From<crate::W<EXTDST1_TRIGGER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST1_TRIGGER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST1_SYNC_TRIGGER` writer - Writing a '1' to this field triggers reconfiguration of the pipeline with endpoint extdst1"]
pub type EXTDST1_SYNC_TRIGGER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST1_TRIGGER_SPEC, bool, O>;
#[doc = "Field `EXTDST1_TRIGGER_SEQUENCE_COMPLETE` writer - By writing a '1' to this register field, you can trigger the extdst1 sequence complete interrupt that will occur as soon as the pipeline with the endpoint extdst1 is empty. This interrupt will also occur if the pipeline is already empty when this field is written. The interrupt will not occur if this field is not written. The interrupt will occur exactly as often as this field is written, assuming that this field is not written again until the interrupt has occured after a previous trigger."]
pub type EXTDST1_TRIGGER_SEQUENCE_COMPLETE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST1_TRIGGER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Writing a '1' to this field triggers reconfiguration of the pipeline with endpoint extdst1"]
    #[inline(always)]
    #[must_use]
    pub fn extdst1_sync_trigger(&mut self) -> EXTDST1_SYNC_TRIGGER_W<0> {
        EXTDST1_SYNC_TRIGGER_W::new(self)
    }
    #[doc = "Bit 4 - By writing a '1' to this register field, you can trigger the extdst1 sequence complete interrupt that will occur as soon as the pipeline with the endpoint extdst1 is empty. This interrupt will also occur if the pipeline is already empty when this field is written. The interrupt will not occur if this field is not written. The interrupt will occur exactly as often as this field is written, assuming that this field is not written again until the interrupt has occured after a previous trigger."]
    #[inline(always)]
    #[must_use]
    pub fn extdst1_trigger_sequence_complete(&mut self) -> EXTDST1_TRIGGER_SEQUENCE_COMPLETE_W<4> {
        EXTDST1_TRIGGER_SEQUENCE_COMPLETE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger bits for pixel engine configuration of extdst1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst1_trigger](index.html) module"]
pub struct EXTDST1_TRIGGER_SPEC;
impl crate::RegisterSpec for EXTDST1_TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [extdst1_trigger::W](W) writer structure"]
impl crate::Writable for EXTDST1_TRIGGER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST1_TRIGGER to value 0"]
impl crate::Resettable for EXTDST1_TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
