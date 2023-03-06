#[doc = "Register `PBSR` reader"]
pub struct R(crate::R<PBSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BIST_DONE` reader - PHY BIST done status"]
pub type BIST_DONE_R = crate::BitReader<bool>;
#[doc = "Field `BIST_ERR` reader - PHY BIST error status"]
pub type BIST_ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - PHY BIST done status"]
    #[inline(always)]
    pub fn bist_done(&self) -> BIST_DONE_R {
        BIST_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PHY BIST error status"]
    #[inline(always)]
    pub fn bist_err(&self) -> BIST_ERR_R {
        BIST_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "PHY BIST Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsr](index.html) module"]
pub struct PBSR_SPEC;
impl crate::RegisterSpec for PBSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbsr::R](R) reader structure"]
impl crate::Readable for PBSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PBSR to value 0"]
impl crate::Resettable for PBSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
