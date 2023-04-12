#[doc = "Register `BISTSTT0` reader"]
pub struct R(crate::R<BISTSTT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BISTSTT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BISTSTT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BISTSTT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BISTERR_CA` reader - BIST Error status of CA A '1' indicates an error on the respective CA line."]
pub type BISTERR_CA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BISTERR_DM` reader - BIST Error status of DM A '1' indicates an error on the respective DM line."]
pub type BISTERR_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BISTERR_CKE` reader - BIST Error status of CKE A '1' indicates an error on the respective CKE line."]
pub type BISTERR_CKE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BISTERR_CS_N` reader - BIST Error status of CS_N A '1' indicates an error on the respective CS line."]
pub type BISTERR_CS_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BISTERR_RESET_N` reader - BIST Error status of RESET_N A '1' indicates an error on the respective RST line."]
pub type BISTERR_RESET_N_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BISTERR_ODT` reader - BIST Error status of ODT A '1' indicates an error on the respective ODT line."]
pub type BISTERR_ODT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - BIST Error status of CA A '1' indicates an error on the respective CA line."]
    #[inline(always)]
    pub fn bisterr_ca(&self) -> BISTERR_CA_R {
        BISTERR_CA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - BIST Error status of DM A '1' indicates an error on the respective DM line."]
    #[inline(always)]
    pub fn bisterr_dm(&self) -> BISTERR_DM_R {
        BISTERR_DM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - BIST Error status of CKE A '1' indicates an error on the respective CKE line."]
    #[inline(always)]
    pub fn bisterr_cke(&self) -> BISTERR_CKE_R {
        BISTERR_CKE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - BIST Error status of CS_N A '1' indicates an error on the respective CS line."]
    #[inline(always)]
    pub fn bisterr_cs_n(&self) -> BISTERR_CS_N_R {
        BISTERR_CS_N_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - BIST Error status of RESET_N A '1' indicates an error on the respective RST line."]
    #[inline(always)]
    pub fn bisterr_reset_n(&self) -> BISTERR_RESET_N_R {
        BISTERR_RESET_N_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - BIST Error status of ODT A '1' indicates an error on the respective ODT line."]
    #[inline(always)]
    pub fn bisterr_odt(&self) -> BISTERR_ODT_R {
        BISTERR_ODT_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[doc = "BIST Status 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biststt0](index.html) module"]
pub struct BISTSTT0_SPEC;
impl crate::RegisterSpec for BISTSTT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biststt0::R](R) reader structure"]
impl crate::Readable for BISTSTT0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BISTSTT0 to value 0"]
impl crate::Resettable for BISTSTT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
