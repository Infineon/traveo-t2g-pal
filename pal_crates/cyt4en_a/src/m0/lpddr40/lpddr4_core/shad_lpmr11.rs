#[doc = "Register `SHAD_LPMR11` reader"]
pub struct R(crate::R<SHAD_LPMR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHAD_LPMR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHAD_LPMR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHAD_LPMR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHAD_LPMR11_FS0_DQODT` reader - N/A"]
pub type SHAD_LPMR11_FS0_DQODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR11_RSVD0` reader - N/A"]
pub type SHAD_LPMR11_RSVD0_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR11_FS0_CAODT` reader - N/A"]
pub type SHAD_LPMR11_FS0_CAODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR11_RSVD1` reader - N/A"]
pub type SHAD_LPMR11_RSVD1_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR11_FS1_DQODT` reader - N/A"]
pub type SHAD_LPMR11_FS1_DQODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR11_RSVD2` reader - N/A"]
pub type SHAD_LPMR11_RSVD2_R = crate::BitReader<bool>;
#[doc = "Field `SHAD_LPMR11_FS1_CAODT` reader - N/A"]
pub type SHAD_LPMR11_FS1_CAODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHAD_LPMR11_RSVD3` reader - N/A"]
pub type SHAD_LPMR11_RSVD3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_fs0_dqodt(&self) -> SHAD_LPMR11_FS0_DQODT_R {
        SHAD_LPMR11_FS0_DQODT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_rsvd0(&self) -> SHAD_LPMR11_RSVD0_R {
        SHAD_LPMR11_RSVD0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_fs0_caodt(&self) -> SHAD_LPMR11_FS0_CAODT_R {
        SHAD_LPMR11_FS0_CAODT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_rsvd1(&self) -> SHAD_LPMR11_RSVD1_R {
        SHAD_LPMR11_RSVD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_fs1_dqodt(&self) -> SHAD_LPMR11_FS1_DQODT_R {
        SHAD_LPMR11_FS1_DQODT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_rsvd2(&self) -> SHAD_LPMR11_RSVD2_R {
        SHAD_LPMR11_RSVD2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_fs1_caodt(&self) -> SHAD_LPMR11_FS1_CAODT_R {
        SHAD_LPMR11_FS1_CAODT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - N/A"]
    #[inline(always)]
    pub fn shad_lpmr11_rsvd3(&self) -> SHAD_LPMR11_RSVD3_R {
        SHAD_LPMR11_RSVD3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Shadow LPDDR Mode Register 11\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shad_lpmr11](index.html) module"]
pub struct SHAD_LPMR11_SPEC;
impl crate::RegisterSpec for SHAD_LPMR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shad_lpmr11::R](R) reader structure"]
impl crate::Readable for SHAD_LPMR11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SHAD_LPMR11 to value 0"]
impl crate::Resettable for SHAD_LPMR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
