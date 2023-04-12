#[doc = "Register `LDTS` reader"]
pub struct R(crate::R<LDTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LDTA` reader - Last Dynamic Transmission Channel A Value of vSlotCounter\\[A\\]
at the time of the last frame transmission on channel A in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no frame was transmitted during the dynamic segment."]
pub type LDTA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LDTB` reader - Last Dynamic Transmission Channel B Value of vSlotCounter\\[B\\]
at the time of the last frame transmission on channel B in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no frame was transmitted during the dynamic segment."]
pub type LDTB_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Last Dynamic Transmission Channel A Value of vSlotCounter\\[A\\]
at the time of the last frame transmission on channel A in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldta(&self) -> LDTA_R {
        LDTA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Last Dynamic Transmission Channel B Value of vSlotCounter\\[B\\]
at the time of the last frame transmission on channel B in the dynamic segment of this node. It is updated at the end of the dynamic segment and is reset to zero if no frame was transmitted during the dynamic segment."]
    #[inline(always)]
    pub fn ldtb(&self) -> LDTB_R {
        LDTB_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "Last Dynamic Transmit Slot\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldts](index.html) module"]
pub struct LDTS_SPEC;
impl crate::RegisterSpec for LDTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldts::R](R) reader structure"]
impl crate::Readable for LDTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LDTS to value 0"]
impl crate::Resettable for LDTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
