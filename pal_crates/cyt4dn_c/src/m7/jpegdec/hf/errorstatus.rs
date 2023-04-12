#[doc = "Register `ERRORSTATUS` reader"]
pub struct R(crate::R<ERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRORSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRORREADAXI` reader - N/A"]
pub type ERRORREADAXI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRORWRITEAXI` reader - N/A"]
pub type ERRORWRITEAXI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRORSTOREOVERLENGTH0` reader - The decoded pixel data is overflowed from the region defined in StoreLength0. The decoding operation is stopped and cannot be resumed."]
pub type ERRORSTOREOVERLENGTH0_R = crate::BitReader<bool>;
#[doc = "Field `ERRORSTOREOVERLENGTH1` reader - The decoded pixel data is overflowed from the region defined in StoreLength1. The decoding operation is stopped and cannot be resumed."]
pub type ERRORSTOREOVERLENGTH1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:1 - N/A"]
    #[inline(always)]
    pub fn errorreadaxi(&self) -> ERRORREADAXI_R {
        ERRORREADAXI_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - N/A"]
    #[inline(always)]
    pub fn errorwriteaxi(&self) -> ERRORWRITEAXI_R {
        ERRORWRITEAXI_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - The decoded pixel data is overflowed from the region defined in StoreLength0. The decoding operation is stopped and cannot be resumed."]
    #[inline(always)]
    pub fn errorstoreoverlength0(&self) -> ERRORSTOREOVERLENGTH0_R {
        ERRORSTOREOVERLENGTH0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - The decoded pixel data is overflowed from the region defined in StoreLength1. The decoding operation is stopped and cannot be resumed."]
    #[inline(always)]
    pub fn errorstoreoverlength1(&self) -> ERRORSTOREOVERLENGTH1_R {
        ERRORSTOREOVERLENGTH1_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "Error status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorstatus](index.html) module"]
pub struct ERRORSTATUS_SPEC;
impl crate::RegisterSpec for ERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [errorstatus::R](R) reader structure"]
impl crate::Readable for ERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ERRORSTATUS to value 0"]
impl crate::Resettable for ERRORSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
