#[doc = "Register `LPMR2` reader"]
pub struct R(crate::R<LPMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR2` writer"]
pub struct W(crate::W<LPMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR2_SPEC>;
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
impl From<crate::W<LPMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_RL` reader - Read latency for frequency set 0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type FS0_RL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_RL` writer - Read latency for frequency set 0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type FS0_RL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_WL` reader - Write latency for frequency set 0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type FS0_WL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_WL` writer - Write latency for frequency set 0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type FS0_WL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_WLS` reader - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type FS0_WLS_R = crate::BitReader<bool>;
#[doc = "Field `FS0_WLS` writer - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type FS0_WLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR2_SPEC, bool, O>;
#[doc = "Field `WRLEV` reader - Write Leveling Enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WRLEV_R = crate::BitReader<bool>;
#[doc = "Field `WRLEV` writer - Write Leveling Enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WRLEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR2_SPEC, bool, O>;
#[doc = "Field `FS1_RL` reader - Read latency for frequency set 1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type FS1_RL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_RL` writer - Read latency for frequency set 1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
pub type FS1_RL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_WL` reader - Write latency for frequency set 1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type FS1_WL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_WL` writer - Write latency for frequency set 1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
pub type FS1_WL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_WLS` reader - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type FS1_WLS_R = crate::BitReader<bool>;
#[doc = "Field `FS1_WLS` writer - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
pub type FS1_WLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPMR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Read latency for frequency set 0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    pub fn fs0_rl(&self) -> FS0_RL_R {
        FS0_RL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Write latency for frequency set 0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    pub fn fs0_wl(&self) -> FS0_WL_R {
        FS0_WL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    pub fn fs0_wls(&self) -> FS0_WLS_R {
        FS0_WLS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Leveling Enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn wrlev(&self) -> WRLEV_R {
        WRLEV_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Read latency for frequency set 1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    pub fn fs1_rl(&self) -> FS1_RL_R {
        FS1_RL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Write latency for frequency set 1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    pub fn fs1_wl(&self) -> FS1_WL_R {
        FS1_WL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    pub fn fs1_wls(&self) -> FS1_WLS_R {
        FS1_WLS_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read latency for frequency set 0 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_rl(&mut self) -> FS0_RL_W<0> {
        FS0_RL_W::new(self)
    }
    #[doc = "Bits 3:5 - Write latency for frequency set 0 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_wl(&mut self) -> FS0_WL_W<3> {
        FS0_WL_W::new(self)
    }
    #[doc = "Bit 6 - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_wls(&mut self) -> FS0_WLS_W<6> {
        FS0_WLS_W::new(self)
    }
    #[doc = "Bit 7 - Write Leveling Enable Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn wrlev(&mut self) -> WRLEV_W<7> {
        WRLEV_W::new(self)
    }
    #[doc = "Bits 8:10 - Read latency for frequency set 1 Read Latency values according to JESD209-4: RL &amp; nRTP for DBI-RD Disabled (MR3 OP\\[6\\]=0B) 000B: RL=6, nRTP = 8 (Default) 001B: RL=10, nRTP = 8 010B: RL=14, nRTP = 8 011B: RL=20, nRTP = 8 100B: RL=24, nRTP = 10 101B: RL=28, nRTP = 12 110B: RL=32, nRTP = 14 111B: RL=36, nRTP = 16 RL &amp; nRTP for DBI-RD Enabled (MR3 OP\\[6\\]=1B) 000B: RL=6, nRTP = 8 001B: RL=12, nRTP = 8 010B: RL=16, nRTP = 8 011B: RL=22, nRTP = 8 100B: RL=28, nRTP = 10 101B: RL=32, nRTP = 12 110B: RL=36, nRTP = 14 111B: RL=40, nRTP = 16"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_rl(&mut self) -> FS1_RL_W<8> {
        FS1_RL_W::new(self)
    }
    #[doc = "Bits 11:13 - Write latency for frequency set 1 Write Latency values according to JESD209-4: WL Set 'A' (MR2 OP\\[6\\]=0B) 000B: WL=4 (Default) 001B: WL=6 010B: WL=8 011B: WL=10 100B: WL=12 101B: WL=14 110B: WL=16 111B: WL=18 WL Set 'B' (MR2 OP\\[6\\]=1B) 000B: WL=4 001B: WL=8 010B: WL=12 011B: WL=18 100B: WL=22 101B: WL=26 110B: WL=30 111B: WL=34"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_wl(&mut self) -> FS1_WL_W<11> {
        FS1_WL_W::new(self)
    }
    #[doc = "Bit 14 - Write Latency Set values according to JESD209-4: WL_SET_A = 0B WL Set 'A' (default) WL_SET_B = 1B WL Set 'B'"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_wls(&mut self) -> FS1_WLS_W<14> {
        FS1_WLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr2](index.html) module"]
pub struct LPMR2_SPEC;
impl crate::RegisterSpec for LPMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr2::R](R) reader structure"]
impl crate::Readable for LPMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr2::W](W) writer structure"]
impl crate::Writable for LPMR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR2 to value 0x2d2d"]
impl crate::Resettable for LPMR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2d2d;
}
