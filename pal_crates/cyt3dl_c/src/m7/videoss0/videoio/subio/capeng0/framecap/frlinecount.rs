#[doc = "Register `FRLINECOUNT` reader"]
pub struct R(crate::R<FRLINECOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRLINECOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRLINECOUNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRLINECOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRLINECOUNT` reader - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
pub type FRLINECOUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTWIDTHT_MSBS` reader - MSBs of CmSts1.TotWidthT. Bit locked when MdrCmrDone=1."]
pub type TOTWIDTHT_MSBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACTWIDTHT_MSBS` reader - MSBs of CmSts1.ActWidthT. Bit locked when MdrCmrDone=1."]
pub type ACTWIDTHT_MSBS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:13 - Shows current line number of input timing. Incremented with leading edge of hsync signal, set to 0 with leading edge of vsync signal."]
    #[inline(always)]
    pub fn frlinecount(&self) -> FRLINECOUNT_R {
        FRLINECOUNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:20 - MSBs of CmSts1.TotWidthT. Bit locked when MdrCmrDone=1."]
    #[inline(always)]
    pub fn totwidtht_msbs(&self) -> TOTWIDTHT_MSBS_R {
        TOTWIDTHT_MSBS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - MSBs of CmSts1.ActWidthT. Bit locked when MdrCmrDone=1."]
    #[inline(always)]
    pub fn actwidtht_msbs(&self) -> ACTWIDTHT_MSBS_R {
        ACTWIDTHT_MSBS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "FrameCap line counter register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frlinecount](index.html) module"]
pub struct FRLINECOUNT_SPEC;
impl crate::RegisterSpec for FRLINECOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frlinecount::R](R) reader structure"]
impl crate::Readable for FRLINECOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FRLINECOUNT to value 0"]
impl crate::Resettable for FRLINECOUNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
