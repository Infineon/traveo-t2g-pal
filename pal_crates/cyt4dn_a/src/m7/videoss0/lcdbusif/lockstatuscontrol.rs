#[doc = "Register `LOCKSTATUSCONTROL` reader"]
pub struct R(crate::R<LOCKSTATUSCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCKSTATUSCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCKSTATUSCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCKSTATUSCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LOCKSTATUSCONTROL` reader - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
pub type LOCKSTATUSCONTROL_R = crate::BitReader<bool>;
#[doc = "Field `PRIVILEGESTATUSCONTROL` reader - Current status of privilege protection: 0 = inactive , 1 = active."]
pub type PRIVILEGESTATUSCONTROL_R = crate::BitReader<bool>;
#[doc = "Field `FREEZESTATUSCONTROL` reader - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
pub type FREEZESTATUSCONTROL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Current status of lock protection: 0 = inactive (unlock counter > 0), 1 = active (unlock counter == 0)."]
    #[inline(always)]
    pub fn lockstatuscontrol(&self) -> LOCKSTATUSCONTROL_R {
        LOCKSTATUSCONTROL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Current status of privilege protection: 0 = inactive , 1 = active."]
    #[inline(always)]
    pub fn privilegestatuscontrol(&self) -> PRIVILEGESTATUSCONTROL_R {
        PRIVILEGESTATUSCONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Current freeze status: 0 = protection status can be changed, 1 = cannot be changed."]
    #[inline(always)]
    pub fn freezestatuscontrol(&self) -> FREEZESTATUSCONTROL_R {
        FREEZESTATUSCONTROL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Protection status of this address block.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockstatuscontrol](index.html) module"]
pub struct LOCKSTATUSCONTROL_SPEC;
impl crate::RegisterSpec for LOCKSTATUSCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lockstatuscontrol::R](R) reader structure"]
impl crate::Readable for LOCKSTATUSCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOCKSTATUSCONTROL to value 0"]
impl crate::Resettable for LOCKSTATUSCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
