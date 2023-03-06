#[doc = "Register `DL_STATUS1` reader"]
pub struct R(crate::R<DL_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DL_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DL_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DL_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT4` reader - Number of delay line taps for data bit 4 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT5` reader - Number of delay line taps for data bit 5 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT6` reader - Number of delay line taps for data bit 6 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT7` reader - Number of delay line taps for data bit 7 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of delay line taps for data bit 4 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit4(&self) -> DATA_BIT4_R {
        DATA_BIT4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of delay line taps for data bit 5 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit5(&self) -> DATA_BIT5_R {
        DATA_BIT5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of delay line taps for data bit 6 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit6(&self) -> DATA_BIT6_R {
        DATA_BIT6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of delay line taps for data bit 7 with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit7(&self) -> DATA_BIT7_R {
        DATA_BIT7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Data Learning Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dl_status1](index.html) module"]
pub struct DL_STATUS1_SPEC;
impl crate::RegisterSpec for DL_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dl_status1::R](R) reader structure"]
impl crate::Readable for DL_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DL_STATUS1 to value 0"]
impl crate::Resettable for DL_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
