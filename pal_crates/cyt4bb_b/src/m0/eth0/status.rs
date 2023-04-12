#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFC_NEGOTIATE` reader - Identifies that PFC priority based pause flow control has been negotiated. 0 - No PFC priority based pause frames have yet been received, flow control is being handled using classic 802.3 pause frames. 1 - At least one PFC priority based pause frames has been received. All subsequent 802.3 pause frames will be dropped."]
pub type PFC_NEGOTIATE_R = crate::BitReader<bool>;
#[doc = "Field `RX_PFC_PAUSED` reader - Each bit corresponds to a priority indicated within the PFC priority based pause frame. Each bit is set when a PFC priority based pause frame has been received, and the associated priority pause time quantum is non-zero. Each bit is cleared when the associated pause time identified by the received pause time quantum has elapsed."]
pub type RX_PFC_PAUSED_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Identifies that PFC priority based pause flow control has been negotiated. 0 - No PFC priority based pause frames have yet been received, flow control is being handled using classic 802.3 pause frames. 1 - At least one PFC priority based pause frames has been received. All subsequent 802.3 pause frames will be dropped."]
    #[inline(always)]
    pub fn pfc_negotiate(&self) -> PFC_NEGOTIATE_R {
        PFC_NEGOTIATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - Each bit corresponds to a priority indicated within the PFC priority based pause frame. Each bit is set when a PFC priority based pause frame has been received, and the associated priority pause time quantum is non-zero. Each bit is cleared when the associated pause time identified by the received pause time quantum has elapsed."]
    #[inline(always)]
    pub fn rx_pfc_paused(&self) -> RX_PFC_PAUSED_R {
        RX_PFC_PAUSED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "MXETH Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
