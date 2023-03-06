#[doc = "Register `POS` reader"]
pub struct R(crate::R<POS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PHYSETC` reader - PHY setting reload completed."]
pub type PHYSETC_R = crate::BitReader<bool>;
#[doc = "Field `PHYFSC` reader - PHY frequency change completed."]
pub type PHYFSC_R = crate::BitReader<bool>;
#[doc = "Field `PHYINITC` reader - PHY initialization completed"]
pub type PHYINITC_R = crate::BitReader<bool>;
#[doc = "Field `DLLRSTC` reader - DLL reset completed."]
pub type DLLRSTC_R = crate::BitReader<bool>;
#[doc = "Field `DRAMINITC` reader - DRAM initialization completed."]
pub type DRAMINITC_R = crate::BitReader<bool>;
#[doc = "Field `VREFDQRDC` reader - PHY VREF-DQ training completed."]
pub type VREFDQRDC_R = crate::BitReader<bool>;
#[doc = "Field `VREFCAC` reader - DRAM VREF-CA training completed."]
pub type VREFCAC_R = crate::BitReader<bool>;
#[doc = "Field `GTC` reader - Gate training completed for rank 0"]
pub type GTC_R = crate::BitReader<bool>;
#[doc = "Field `WRLVLC` reader - Write leveling completed for rank 0"]
pub type WRLVLC_R = crate::BitReader<bool>;
#[doc = "Field `RDLVLC` reader - Read data eye training completed for rank 0"]
pub type RDLVLC_R = crate::BitReader<bool>;
#[doc = "Field `VREFDQWRC` reader - DRAM VREF-DQ training completed for rank 0"]
pub type VREFDQWRC_R = crate::BitReader<bool>;
#[doc = "Field `DLYEVALC` reader - Delay evaluation completed for rank 0"]
pub type DLYEVALC_R = crate::BitReader<bool>;
#[doc = "Field `SANCHKC` reader - Write/read sanity check completed for rank 0"]
pub type SANCHKC_R = crate::BitReader<bool>;
#[doc = "Field `OFS` reader - Operating frequency set point."]
pub type OFS_R = crate::BitReader<bool>;
#[doc = "Field `FS0REQ` reader - Frequency set point 0 request."]
pub type FS0REQ_R = crate::BitReader<bool>;
#[doc = "Field `FS1REQ` reader - Frequency set point 1 request."]
pub type FS1REQ_R = crate::BitReader<bool>;
#[doc = "Field `CLKLOCKC` reader - MC-PHY clock phase locked."]
pub type CLKLOCKC_R = crate::BitReader<bool>;
#[doc = "Field `CMDDLYC` reader - PHY command bus delay loaded"]
pub type CMDDLYC_R = crate::BitReader<bool>;
#[doc = "Field `DQSDQC` reader - tDQS2DQ training completed for rank 0"]
pub type DQSDQC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PHY setting reload completed."]
    #[inline(always)]
    pub fn physetc(&self) -> PHYSETC_R {
        PHYSETC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY frequency change completed."]
    #[inline(always)]
    pub fn phyfsc(&self) -> PHYFSC_R {
        PHYFSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY initialization completed"]
    #[inline(always)]
    pub fn phyinitc(&self) -> PHYINITC_R {
        PHYINITC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DLL reset completed."]
    #[inline(always)]
    pub fn dllrstc(&self) -> DLLRSTC_R {
        DLLRSTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DRAM initialization completed."]
    #[inline(always)]
    pub fn draminitc(&self) -> DRAMINITC_R {
        DRAMINITC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHY VREF-DQ training completed."]
    #[inline(always)]
    pub fn vrefdqrdc(&self) -> VREFDQRDC_R {
        VREFDQRDC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DRAM VREF-CA training completed."]
    #[inline(always)]
    pub fn vrefcac(&self) -> VREFCAC_R {
        VREFCAC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Gate training completed for rank 0"]
    #[inline(always)]
    pub fn gtc(&self) -> GTC_R {
        GTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write leveling completed for rank 0"]
    #[inline(always)]
    pub fn wrlvlc(&self) -> WRLVLC_R {
        WRLVLC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read data eye training completed for rank 0"]
    #[inline(always)]
    pub fn rdlvlc(&self) -> RDLVLC_R {
        RDLVLC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DRAM VREF-DQ training completed for rank 0"]
    #[inline(always)]
    pub fn vrefdqwrc(&self) -> VREFDQWRC_R {
        VREFDQWRC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Delay evaluation completed for rank 0"]
    #[inline(always)]
    pub fn dlyevalc(&self) -> DLYEVALC_R {
        DLYEVALC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write/read sanity check completed for rank 0"]
    #[inline(always)]
    pub fn sanchkc(&self) -> SANCHKC_R {
        SANCHKC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Operating frequency set point."]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Frequency set point 0 request."]
    #[inline(always)]
    pub fn fs0req(&self) -> FS0REQ_R {
        FS0REQ_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Frequency set point 1 request."]
    #[inline(always)]
    pub fn fs1req(&self) -> FS1REQ_R {
        FS1REQ_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MC-PHY clock phase locked."]
    #[inline(always)]
    pub fn clklockc(&self) -> CLKLOCKC_R {
        CLKLOCKC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY command bus delay loaded"]
    #[inline(always)]
    pub fn cmddlyc(&self) -> CMDDLYC_R {
        CMDDLYC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - tDQS2DQ training completed for rank 0"]
    #[inline(always)]
    pub fn dqsdqc(&self) -> DQSDQC_R {
        DQSDQC_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "PHY Operation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pos](index.html) module"]
pub struct POS_SPEC;
impl crate::RegisterSpec for POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pos::R](R) reader structure"]
impl crate::Readable for POS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
