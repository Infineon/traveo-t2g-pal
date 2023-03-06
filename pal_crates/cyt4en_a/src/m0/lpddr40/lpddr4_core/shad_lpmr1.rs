#[doc = "Register `SHAD_LPMR1` reader"]
pub struct R(crate::R<SHAD_LPMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR1_FS0_BL` reader - N/A"]
pub type SHAD_LPMR1_FS0_BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR1_FS0_WPRE` reader - N/A"]
pub type SHAD_LPMR1_FS0_WPRE_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR1_FS0_RPRE` reader - Shadow Read preamble FS0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type SHAD_LPMR1_FS0_RPRE_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR1_FS0_NWR` reader - Shadow Write recovery to pre-charge FS0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type SHAD_LPMR1_FS0_NWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR1_FS0_RPST` reader - Shadow Read postamble FS0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type SHAD_LPMR1_FS0_RPST_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR1_FS1_BL` reader - N/A"]
pub type SHAD_LPMR1_FS1_BL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR1_FS1_WPRE` reader - N/A"]
pub type SHAD_LPMR1_FS1_WPRE_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR1_FS1_RPRE` reader - Shadow Read preamble FS1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
pub type SHAD_LPMR1_FS1_RPRE_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR1_FS1_NWR` reader - Shadow Write recovery to pre-charge FS1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
pub type SHAD_LPMR1_FS1_NWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR1_FS1_RPST` reader - Shadow Read postamble FS1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
pub type SHAD_LPMR1_FS1_RPST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr1_fs0_bl(&self) -> SHAD_LPMR1_FS0_BL_R {
        SHAD_LPMR1_FS0_BL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr1_fs0_wpre(&self) -> SHAD_LPMR1_FS0_WPRE_R {
        SHAD_LPMR1_FS0_WPRE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Shadow Read preamble FS0 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    pub fn shad_lpmr1_fs0_rpre(&self) -> SHAD_LPMR1_FS0_RPRE_R {
        SHAD_LPMR1_FS0_RPRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Shadow Write recovery to pre-charge FS0 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    pub fn shad_lpmr1_fs0_nwr(&self) -> SHAD_LPMR1_FS0_NWR_R {
        SHAD_LPMR1_FS0_NWR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Shadow Read postamble FS0 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    pub fn shad_lpmr1_fs0_rpst(&self) -> SHAD_LPMR1_FS0_RPST_R {
        SHAD_LPMR1_FS0_RPST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr1_fs1_bl(&self) -> SHAD_LPMR1_FS1_BL_R {
        SHAD_LPMR1_FS1_BL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr1_fs1_wpre(&self) -> SHAD_LPMR1_FS1_WPRE_R {
        SHAD_LPMR1_FS1_WPRE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shadow Read preamble FS1 Read Preamble values according to JESD209-4: RD_PREAMBLE_STATIC = 0B RD Pre-amble = Static (default) RD_PREAMBLE_TOGGLE = 1B RD Pre-amble = Toggle"]
    #[inline(always)]
    pub fn shad_lpmr1_fs1_rpre(&self) -> SHAD_LPMR1_FS1_RPRE_R {
        SHAD_LPMR1_FS1_RPRE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Shadow Write recovery to pre-charge FS1 Write Recovery values according to JESD209-4: NWR6 = 000B nWR = 6 (default) NWR10 = 001B nWR = 10 NWR16 = 010B nWR = 16 NWR20 = 011B nWR = 20 NWR24 = 100B nWR = 24 NWR30 = 101B nWR = 30 NWR34 = 110B nWR = 34 NWR40 = 111B nWR = 40"]
    #[inline(always)]
    pub fn shad_lpmr1_fs1_nwr(&self) -> SHAD_LPMR1_FS1_NWR_R {
        SHAD_LPMR1_FS1_NWR_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Shadow Read postamble FS1 Read Postamble values according to JESD209-4: RD_POSTAMBLE_0P5 = 0B RD Post-amble = 0.5*tCK (default) RD_POSTAMBLE_1P5 = 1B RD Post-amble = 1.5*tCK"]
    #[inline(always)]
    pub fn shad_lpmr1_fs1_rpst(&self) -> SHAD_LPMR1_FS1_RPST_R {
        SHAD_LPMR1_FS1_RPST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr1](index.html) module"]
pub struct SHAD_LPMR1_SPEC;
impl crate::RegisterSpec for SHAD_LPMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr1::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR1 to value 0"]
impl crate::Resettable for SHAD_LPMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
