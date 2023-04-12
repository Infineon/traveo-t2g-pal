#[doc = "Register `SYNCCONTROL` writer"]
pub struct W(crate::W<SYNCCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNCCONTROL_SPEC>;
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
impl From<crate::W<SYNCCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNCCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCIDTRIGGER` writer - Trigger SyncIdComplete\\[0:7\\]
and load SyncIdRequest\\[0:7\\]
into SyncIdStatus\\[0:7\\]. Is executed after store operation is completed or no operation is active on the corresponding TB."]
pub type SYNCIDTRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYNCCONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SEQUENCECOMPLETE` writer - Trigger SeqComplete interrupt if BlitEng is idle or the last operation has been completed."]
pub type SEQUENCECOMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNCCONTROL_SPEC, bool, O>;
#[doc = "Field `CACHEINVALIDATIONSRC` writer - Triggers invalidation of the GfxCache for FetchSrc with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
pub type CACHEINVALIDATIONSRC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYNCCONTROL_SPEC, bool, O>;
#[doc = "Field `CACHEINVALIDATIONDST` writer - Triggers invalidation of the GfxCache for FetchDst with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
pub type CACHEINVALIDATIONDST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYNCCONTROL_SPEC, bool, O>;
#[doc = "Field `CACHEINVALIDATIONMASK` writer - Triggers invalidation of the GfxCache for FetchMask with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
pub type CACHEINVALIDATIONMASK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SYNCCONTROL_SPEC, bool, O>;
impl W {
    #[doc = "Bits 0:7 - Trigger SyncIdComplete\\[0:7\\]
and load SyncIdRequest\\[0:7\\]
into SyncIdStatus\\[0:7\\]. Is executed after store operation is completed or no operation is active on the corresponding TB."]
    #[inline(always)]
    #[must_use]
    pub fn syncidtrigger(&mut self) -> SYNCIDTRIGGER_W<0> {
        SYNCIDTRIGGER_W::new(self)
    }
    #[doc = "Bit 8 - Trigger SeqComplete interrupt if BlitEng is idle or the last operation has been completed."]
    #[inline(always)]
    #[must_use]
    pub fn sequencecomplete(&mut self) -> SEQUENCECOMPLETE_W<8> {
        SEQUENCECOMPLETE_W::new(self)
    }
    #[doc = "Bit 16 - Triggers invalidation of the GfxCache for FetchSrc with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinvalidationsrc(&mut self) -> CACHEINVALIDATIONSRC_W<16> {
        CACHEINVALIDATIONSRC_W::new(self)
    }
    #[doc = "Bit 17 - Triggers invalidation of the GfxCache for FetchDst with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinvalidationdst(&mut self) -> CACHEINVALIDATIONDST_W<17> {
        CACHEINVALIDATIONDST_W::new(self)
    }
    #[doc = "Bit 18 - Triggers invalidation of the GfxCache for FetchMask with the next operation start. Operation can be any IBO or LBO fill, fetch or store."]
    #[inline(always)]
    #[must_use]
    pub fn cacheinvalidationmask(&mut self) -> CACHEINVALIDATIONMASK_W<18> {
        CACHEINVALIDATIONMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Triggers interrupts for sequence complete of specific TB of global sequence complete interrupt.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [synccontrol](index.html) module"]
pub struct SYNCCONTROL_SPEC;
impl crate::RegisterSpec for SYNCCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [synccontrol::W](W) writer structure"]
impl crate::Writable for SYNCCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNCCONTROL to value 0"]
impl crate::Resettable for SYNCCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
