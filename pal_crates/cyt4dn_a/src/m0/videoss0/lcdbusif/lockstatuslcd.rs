#[doc = "Register `LOCKSTATUSLCD` reader"]
pub struct R(crate::R<LOCKSTATUSLCD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTATUSLCD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTATUSLCD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTATUSLCD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKSTATUSLCD` reader - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
pub type LOCKSTATUSLCD_R = crate::BitReader<bool>;
#[doc = "Field `PRIVILEGESTATUSLCD` reader - Current status of privilege protection: 0 = inactive , 1 = active."]
pub type PRIVILEGESTATUSLCD_R = crate::BitReader<bool>;
#[doc = "Field `FREEZESTATUSLCD` reader - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
pub type FREEZESTATUSLCD_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
    #[inline(always)]
    pub fn lockstatuslcd(&self) -> LOCKSTATUSLCD_R {
        LOCKSTATUSLCD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Current status of privilege protection: 0 = inactive , 1 = active."]
    #[inline(always)]
    pub fn privilegestatuslcd(&self) -> PRIVILEGESTATUSLCD_R {
        PRIVILEGESTATUSLCD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
    #[inline(always)]
    pub fn freezestatuslcd(&self) -> FREEZESTATUSLCD_R {
        FREEZESTATUSLCD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Protection status of this address block.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatuslcd](index.html) module"]
pub struct LOCKSTATUSLCD_SPEC;
impl crate::RegisterSpec for LOCKSTATUSLCD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstatuslcd::R](R) reader structure"]
impl crate::Readable for LOCKSTATUSLCD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTATUSLCD to value 0"]
impl crate::Resettable for LOCKSTATUSLCD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
