#[doc = "Register `SHAD_LPMR2` reader"]
pub struct R(crate::R<SHAD_LPMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR2_FS0_RL` reader - Shadow Read latency FS0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type SHAD_LPMR2_FS0_RL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR2_FS0_WL` reader - Shadow Write latency FS0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type SHAD_LPMR2_FS0_WL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR2_FS0_WLS` reader - Shadow Write latency set FS0 Values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type SHAD_LPMR2_FS0_WLS_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR2_WRLEV` reader - Shadow Write Leveling Enable FS0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type SHAD_LPMR2_WRLEV_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR2_FS1_RL` reader - Shadow Read latency FS1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type SHAD_LPMR2_FS1_RL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR2_FS1_WL` reader - Shadow Write latency FS1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type SHAD_LPMR2_FS1_WL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR2_FS1_WLS` reader - Shadow Write latency set FS1 Values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type SHAD_LPMR2_FS1_WLS_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR2_RSVD` reader - N/A"]
pub type SHAD_LPMR2_RSVD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - Shadow Read latency FS0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    pub fn shad_lpmr2_fs0_rl(&self) -> SHAD_LPMR2_FS0_RL_R {
        SHAD_LPMR2_FS0_RL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Shadow Write latency FS0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    pub fn shad_lpmr2_fs0_wl(&self) -> SHAD_LPMR2_FS0_WL_R {
        SHAD_LPMR2_FS0_WL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Shadow Write latency set FS0 Values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    pub fn shad_lpmr2_fs0_wls(&self) -> SHAD_LPMR2_FS0_WLS_R {
        SHAD_LPMR2_FS0_WLS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Shadow Write Leveling Enable FS0 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn shad_lpmr2_wrlev(&self) -> SHAD_LPMR2_WRLEV_R {
        SHAD_LPMR2_WRLEV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Shadow Read latency FS1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    pub fn shad_lpmr2_fs1_rl(&self) -> SHAD_LPMR2_FS1_RL_R {
        SHAD_LPMR2_FS1_RL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Shadow Write latency FS1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    pub fn shad_lpmr2_fs1_wl(&self) -> SHAD_LPMR2_FS1_WL_R {
        SHAD_LPMR2_FS1_WL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Shadow Write latency set FS1 Values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    pub fn shad_lpmr2_fs1_wls(&self) -> SHAD_LPMR2_FS1_WLS_R {
        SHAD_LPMR2_FS1_WLS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr2_rsvd(&self) -> SHAD_LPMR2_RSVD_R {
        SHAD_LPMR2_RSVD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr2](index.html) module"]
pub struct SHAD_LPMR2_SPEC;
impl crate::RegisterSpec for SHAD_LPMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr2::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR2 to value 0"]
impl crate::Resettable for SHAD_LPMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
