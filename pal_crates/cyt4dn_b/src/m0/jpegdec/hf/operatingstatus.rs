#[doc = "Register `OPERATINGSTATUS` reader"]
pub struct R(crate::R<OPERATINGSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPERATINGSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPERATINGSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPERATINGSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OPERATIONMODE` reader - N/A"]
pub type OPERATIONMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FETCHUNITACTIVE` reader - Fetch Unit is operating."]
pub type FETCHUNITACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `FETCHAXIACTIVE` reader - There is an active AXI read transaction by Fetch Unit"]
pub type FETCHAXIACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `STOREUNITACTIVE` reader - Store Unit is operating."]
pub type STOREUNITACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `STOREAXIACTIVE` reader - There is an active AXI write transaction by Store Unit"]
pub type STOREAXIACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn operationmode(&self) -> OPERATIONMODE_R {
        OPERATIONMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 16 - Fetch Unit is operating."]
    #[inline(always)]
    pub fn fetchunitactive(&self) -> FETCHUNITACTIVE_R {
        FETCHUNITACTIVE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - There is an active AXI read transaction by Fetch Unit"]
    #[inline(always)]
    pub fn fetchaxiactive(&self) -> FETCHAXIACTIVE_R {
        FETCHAXIACTIVE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Store Unit is operating."]
    #[inline(always)]
    pub fn storeunitactive(&self) -> STOREUNITACTIVE_R {
        STOREUNITACTIVE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - There is an active AXI write transaction by Store Unit"]
    #[inline(always)]
    pub fn storeaxiactive(&self) -> STOREAXIACTIVE_R {
        STOREAXIACTIVE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Operating status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [operatingstatus](index.html) module"]
pub struct OPERATINGSTATUS_SPEC;
impl crate::RegisterSpec for OPERATINGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [operatingstatus::R](R) reader structure"]
impl crate::Readable for OPERATINGSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OPERATINGSTATUS to value 0"]
impl crate::Resettable for OPERATINGSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
