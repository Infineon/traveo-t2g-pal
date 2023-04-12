#[doc = "Register `FGCHSTAT` reader"]
pub struct R(crate::R<FGCHSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGCHSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGCHSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGCHSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PFIFOEMPTY` reader - Read request to empty primary pixel FIFO detected. (Bit locked when 1, clear by using ClrPrimStat)."]
pub type PFIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `PRIMSYNCSTAT` reader - Current status primary channel synchronization (0 = out of sync (frame tearing), 1 = in sync (normal operation); not locked)."]
pub type PRIMSYNCSTAT_R = crate::BitReader<bool>;
#[doc = "Field `SFIFOEMPTY` reader - Read request to empty secondary pixel FIFO detected. (bit locked when 1, clear by using ClrSecStat)."]
pub type SFIFOEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SKEWRANGEERR` reader - The secondary channel skew value has run out of the limit defined by SyncRangeLow and SyncRangeHigh. (bit locked when 1, clear by using ClrSecStat)."]
pub type SKEWRANGEERR_R = crate::BitReader<bool>;
#[doc = "Field `SECSYNCSTAT` reader - Current status secondary channel synchronization (0 = out of sync, 1 = in sync; not locked)."]
pub type SECSYNCSTAT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Read request to empty primary pixel FIFO detected. (Bit locked when 1, clear by using ClrPrimStat)."]
    #[inline(always)]
    pub fn pfifoempty(&self) -> PFIFOEMPTY_R {
        PFIFOEMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Current status primary channel synchronization (0 = out of sync (frame tearing), 1 = in sync (normal operation); not locked)."]
    #[inline(always)]
    pub fn primsyncstat(&self) -> PRIMSYNCSTAT_R {
        PRIMSYNCSTAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Read request to empty secondary pixel FIFO detected. (bit locked when 1, clear by using ClrSecStat)."]
    #[inline(always)]
    pub fn sfifoempty(&self) -> SFIFOEMPTY_R {
        SFIFOEMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The secondary channel skew value has run out of the limit defined by SyncRangeLow and SyncRangeHigh. (bit locked when 1, clear by using ClrSecStat)."]
    #[inline(always)]
    pub fn skewrangeerr(&self) -> SKEWRANGEERR_R {
        SKEWRANGEERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Current status secondary channel synchronization (0 = out of sync, 1 = in sync; not locked)."]
    #[inline(always)]
    pub fn secsyncstat(&self) -> SECSYNCSTAT_R {
        SECSYNCSTAT_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "FrameGen Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgchstat](index.html) module"]
pub struct FGCHSTAT_SPEC;
impl crate::RegisterSpec for FGCHSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgchstat::R](R) reader structure"]
impl crate::Readable for FGCHSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FGCHSTAT to value 0"]
impl crate::Resettable for FGCHSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
