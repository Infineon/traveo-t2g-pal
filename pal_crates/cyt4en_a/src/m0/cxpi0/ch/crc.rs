#[doc = "Register `CRC` reader"]
pub struct R(crate::R<CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRC1` reader - CRC first byte for both CRC8 and CRC16. This is valid for both Normal frame and Long frame. HW will load this field with first byte of CRC upon receiving it."]
pub type RXCRC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCRC2` reader - CRC second byte of CRC16. This is valid only for Long frames. HW will load this field with second byte of CRC upon receiving it."]
pub type RXCRC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCRC1` reader - CRC first byte for both CRC8 and CRC16. This is valid for both Normal frame and Long frame. HW will load this field with first byte of CRC for transmit."]
pub type TXCRC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCRC2` reader - CRC second byte of CRC16. This is valid only for Long frames. HW will load this field with second byte of CRC for transmit."]
pub type TXCRC2_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CRC first byte for both CRC8 and CRC16. This is valid for both Normal frame and Long frame. HW will load this field with first byte of CRC upon receiving it."]
    #[inline(always)]
    pub fn rxcrc1(&self) -> RXCRC1_R {
        RXCRC1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CRC second byte of CRC16. This is valid only for Long frames. HW will load this field with second byte of CRC upon receiving it."]
    #[inline(always)]
    pub fn rxcrc2(&self) -> RXCRC2_R {
        RXCRC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CRC first byte for both CRC8 and CRC16. This is valid for both Normal frame and Long frame. HW will load this field with first byte of CRC for transmit."]
    #[inline(always)]
    pub fn txcrc1(&self) -> TXCRC1_R {
        TXCRC1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - CRC second byte of CRC16. This is valid only for Long frames. HW will load this field with second byte of CRC for transmit."]
    #[inline(always)]
    pub fn txcrc2(&self) -> TXCRC2_R {
        TXCRC2_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CRC\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc](index.html) module"]
pub struct CRC_SPEC;
impl crate::RegisterSpec for CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crc::R](R) reader structure"]
impl crate::Readable for CRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRC to value 0"]
impl crate::Resettable for CRC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
