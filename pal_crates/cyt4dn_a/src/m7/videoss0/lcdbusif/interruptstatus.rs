#[doc = "Register `INTERRUPTSTATUS` reader"]
pub struct R(crate::R<INTERRUPTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERRUPTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERRUPTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERRUPTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEQUENCERERRORINTERRUPTSTATUS` reader - Status of the SequencerError interrupt."]
pub type SEQUENCERERRORINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `SEQUENCERSYNCINTERRUPTSTATUS` reader - Status of the SequencerSync interrupt."]
pub type SEQUENCERSYNCINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `INSTRFIFOINTERRUPTSTATUS` reader - Status of the TxFifo interrupt."]
pub type INSTRFIFOINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOINTERRUPTSTATUS` reader - Status of the RxFifo interrupt."]
pub type RXFIFOINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `TEARINGEFFECTINTERRUPTSTATUS` reader - Status of the TearingEffect interrupt."]
pub type TEARINGEFFECTINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `READCHANNELDONEINTERRUPTSTATUS` reader - Status of the ReadChannelDone interrupt."]
pub type READCHANNELDONEINTERRUPTSTATUS_R = crate::BitReader<bool>;
#[doc = "Field `WRITECHANNELDONEINTERRUPTSTATUS` reader - Status of the WriteChannelDone interrupt."]
pub type WRITECHANNELDONEINTERRUPTSTATUS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Status of the SequencerError interrupt."]
    #[inline(always)]
    pub fn sequencererrorinterruptstatus(&self) -> SEQUENCERERRORINTERRUPTSTATUS_R {
        SEQUENCERERRORINTERRUPTSTATUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the SequencerSync interrupt."]
    #[inline(always)]
    pub fn sequencersyncinterruptstatus(&self) -> SEQUENCERSYNCINTERRUPTSTATUS_R {
        SEQUENCERSYNCINTERRUPTSTATUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the TxFifo interrupt."]
    #[inline(always)]
    pub fn instrfifointerruptstatus(&self) -> INSTRFIFOINTERRUPTSTATUS_R {
        INSTRFIFOINTERRUPTSTATUS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the RxFifo interrupt."]
    #[inline(always)]
    pub fn rxfifointerruptstatus(&self) -> RXFIFOINTERRUPTSTATUS_R {
        RXFIFOINTERRUPTSTATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the TearingEffect interrupt."]
    #[inline(always)]
    pub fn tearingeffectinterruptstatus(&self) -> TEARINGEFFECTINTERRUPTSTATUS_R {
        TEARINGEFFECTINTERRUPTSTATUS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the ReadChannelDone interrupt."]
    #[inline(always)]
    pub fn readchanneldoneinterruptstatus(&self) -> READCHANNELDONEINTERRUPTSTATUS_R {
        READCHANNELDONEINTERRUPTSTATUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the WriteChannelDone interrupt."]
    #[inline(always)]
    pub fn writechanneldoneinterruptstatus(&self) -> WRITECHANNELDONEINTERRUPTSTATUS_R {
        WRITECHANNELDONEINTERRUPTSTATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "LCD Bus Interface interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [interruptstatus](index.html) module"]
pub struct INTERRUPTSTATUS_SPEC;
impl crate::RegisterSpec for INTERRUPTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [interruptstatus::R](R) reader structure"]
impl crate::Readable for INTERRUPTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTERRUPTSTATUS to value 0"]
impl crate::Resettable for INTERRUPTSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
