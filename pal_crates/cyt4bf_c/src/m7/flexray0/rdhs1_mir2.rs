#[doc = "Register `RDHS1_MIR2` reader"]
pub struct R(crate::R<RDHS1_MIR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDHS1_MIR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDHS1_MIR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDHS1_MIR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FID` reader - Frame ID"]
pub type FID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CYC` reader - Cycle Code"]
pub type CYC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHA` reader - Channel Filter Control A"]
pub type CHA_R = crate::BitReader<bool>;
#[doc = "Field `CHB` reader - Channel Filter Control B"]
pub type CHB_R = crate::BitReader<bool>;
#[doc = "Field `CFG` reader - Message Buffer Direction Configuration Bit"]
pub type CFG_R = crate::BitReader<bool>;
#[doc = "Field `PPIT` reader - Payload Preamble Indicator Transmit"]
pub type PPIT_R = crate::BitReader<bool>;
#[doc = "Field `TXM` reader - Transmission Mode"]
pub type TXM_R = crate::BitReader<bool>;
#[doc = "Field `MBI` reader - Message Buffer Interrupt"]
pub type MBI_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:10 - Frame ID"]
    #[inline(always)]
    pub fn fid(&self) -> FID_R {
        FID_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Cycle Code"]
    #[inline(always)]
    pub fn cyc(&self) -> CYC_R {
        CYC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Channel Filter Control A"]
    #[inline(always)]
    pub fn cha(&self) -> CHA_R {
        CHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel Filter Control B"]
    #[inline(always)]
    pub fn chb(&self) -> CHB_R {
        CHB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Message Buffer Direction Configuration Bit"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Payload Preamble Indicator Transmit"]
    #[inline(always)]
    pub fn ppit(&self) -> PPIT_R {
        PPIT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmission Mode"]
    #[inline(always)]
    pub fn txm(&self) -> TXM_R {
        TXM_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Message Buffer Interrupt"]
    #[inline(always)]
    pub fn mbi(&self) -> MBI_R {
        MBI_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Read Header Section 1 (2nd mirror)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdhs1_mir2](index.html) module"]
pub struct RDHS1_MIR2_SPEC;
impl crate::RegisterSpec for RDHS1_MIR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdhs1_mir2::R](R) reader structure"]
impl crate::Readable for RDHS1_MIR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDHS1_MIR2 to value 0"]
impl crate::Resettable for RDHS1_MIR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
