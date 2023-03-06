#[doc = "Register `PTS0` reader"]
pub struct R(crate::R<PTS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DLLERR` reader - DLL reset error. Dllerr\\[3:0\\]
is for data module \\[3:0\\], and dllerr\\[5:4\\]
is for control module\\[1:0\\]. 1 means the DLL reset process of corresponding module is not completed successfully."]
pub type DLLERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQRDERR` reader - Read VREF-DQ training error on rank 0. One bit for each data slice. 1 means PHY cannot find suitable VREF setting for its receiver"]
pub type VREFDQRDERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFCAERR` reader - VREFCA training error on rank 0. vrefcaerr\\[1\\]
is for LPDDR4 channel B and vrefcaerr\\[0\\]
is for LPDDR4 channel A. 1 means VREFCA training for corresponding LPDDR4 channel is not completed successfully"]
pub type VREFCAERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTERR` reader - Gate training error on rank 0. One bit for each data slice. 1 means gate training is not completed successfully"]
pub type GTERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRLVLERR` reader - Write leveling error on rank 0. One bit for each data slice. 1 means write leveling is not completed successfully"]
pub type WRLVLERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQWRERR` reader - Write VREF-DQ training error on rank 0. One bit for each data slice. 1 means VREF training for DRAM is not completed successfully"]
pub type VREFDQWRERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDLVLDMERR` reader - DM read data eye training error on rank 0. One bit for each DM signal. 1 means data eye training for corresponding DM is not completed successfully"]
pub type RDLVLDMERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SANCHKERR` reader - Write/Read sanity check error on rank 0. One bit for each data slice. 1 means read pattern and write pattern for corresponding data slice are not match"]
pub type SANCHKERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - DLL reset error. Dllerr\\[3:0\\]
is for data module \\[3:0\\], and dllerr\\[5:4\\]
is for control module\\[1:0\\]. 1 means the DLL reset process of corresponding module is not completed successfully."]
    #[inline(always)]
    pub fn dllerr(&self) -> DLLERR_R {
        DLLERR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Read VREF-DQ training error on rank 0. One bit for each data slice. 1 means PHY cannot find suitable VREF setting for its receiver"]
    #[inline(always)]
    pub fn vrefdqrderr(&self) -> VREFDQRDERR_R {
        VREFDQRDERR_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - VREFCA training error on rank 0. vrefcaerr\\[1\\]
is for LPDDR4 channel B and vrefcaerr\\[0\\]
is for LPDDR4 channel A. 1 means VREFCA training for corresponding LPDDR4 channel is not completed successfully"]
    #[inline(always)]
    pub fn vrefcaerr(&self) -> VREFCAERR_R {
        VREFCAERR_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Gate training error on rank 0. One bit for each data slice. 1 means gate training is not completed successfully"]
    #[inline(always)]
    pub fn gterr(&self) -> GTERR_R {
        GTERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Write leveling error on rank 0. One bit for each data slice. 1 means write leveling is not completed successfully"]
    #[inline(always)]
    pub fn wrlvlerr(&self) -> WRLVLERR_R {
        WRLVLERR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Write VREF-DQ training error on rank 0. One bit for each data slice. 1 means VREF training for DRAM is not completed successfully"]
    #[inline(always)]
    pub fn vrefdqwrerr(&self) -> VREFDQWRERR_R {
        VREFDQWRERR_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DM read data eye training error on rank 0. One bit for each DM signal. 1 means data eye training for corresponding DM is not completed successfully"]
    #[inline(always)]
    pub fn rdlvldmerr(&self) -> RDLVLDMERR_R {
        RDLVLDMERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Write/Read sanity check error on rank 0. One bit for each data slice. 1 means read pattern and write pattern for corresponding data slice are not match"]
    #[inline(always)]
    pub fn sanchkerr(&self) -> SANCHKERR_R {
        SANCHKERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "PHY Training Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pts0](index.html) module"]
pub struct PTS0_SPEC;
impl crate::RegisterSpec for PTS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pts0::R](R) reader structure"]
impl crate::Readable for PTS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTS0 to value 0"]
impl crate::Resettable for PTS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
