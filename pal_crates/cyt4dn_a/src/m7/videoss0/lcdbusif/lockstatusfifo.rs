#[doc = "Register `LOCKSTATUSFIFO` reader"]
pub struct R(crate::R<LOCKSTATUSFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTATUSFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTATUSFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTATUSFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKSTATUSFIFO` reader - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
pub type LOCKSTATUSFIFO_R = crate::BitReader<bool>;
#[doc = "Field `PRIVILEGESTATUSFIFO` reader - Current status of privilege protection: 0 = inactive , 1 = active."]
pub type PRIVILEGESTATUSFIFO_R = crate::BitReader<bool>;
#[doc = "Field `FREEZESTATUSFIFO` reader - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
pub type FREEZESTATUSFIFO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
    #[inline(always)]
    pub fn lockstatusfifo(&self) -> LOCKSTATUSFIFO_R {
        LOCKSTATUSFIFO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Current status of privilege protection: 0 = inactive , 1 = active."]
    #[inline(always)]
    pub fn privilegestatusfifo(&self) -> PRIVILEGESTATUSFIFO_R {
        PRIVILEGESTATUSFIFO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
    #[inline(always)]
    pub fn freezestatusfifo(&self) -> FREEZESTATUSFIFO_R {
        FREEZESTATUSFIFO_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Protection status of this address block.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatusfifo](index.html) module"]
pub struct LOCKSTATUSFIFO_SPEC;
impl crate::RegisterSpec for LOCKSTATUSFIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstatusfifo::R](R) reader structure"]
impl crate::Readable for LOCKSTATUSFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTATUSFIFO to value 0"]
impl crate::Resettable for LOCKSTATUSFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
