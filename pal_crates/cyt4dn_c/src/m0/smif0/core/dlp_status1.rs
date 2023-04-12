#[doc = "Register `DLP_STATUS1` reader"]
pub struct R(crate::R<DLP_STATUS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLP_STATUS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLP_STATUS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLP_STATUS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT4` reader - Same as the DATA_BIT0 description except for spi_data\\[4\\]"]
pub type DATA_BIT4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT5` reader - Same as the DATA_BIT0 description except for spi_data\\[5\\]"]
pub type DATA_BIT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT6` reader - Same as the DATA_BIT0 description except for spi_data\\[6\\]"]
pub type DATA_BIT6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT7` reader - Same as the DATA_BIT0 description except for spi_data\\[7\\]"]
pub type DATA_BIT7_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - Same as the DATA_BIT0 description except for spi_data\\[4\\]"]
    #[inline(always)]
    pub fn data_bit4(&self) -> DATA_BIT4_R {
        DATA_BIT4_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as the DATA_BIT0 description except for spi_data\\[5\\]"]
    #[inline(always)]
    pub fn data_bit5(&self) -> DATA_BIT5_R {
        DATA_BIT5_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as the DATA_BIT0 description except for spi_data\\[6\\]"]
    #[inline(always)]
    pub fn data_bit6(&self) -> DATA_BIT6_R {
        DATA_BIT6_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as the DATA_BIT0 description except for spi_data\\[7\\]"]
    #[inline(always)]
    pub fn data_bit7(&self) -> DATA_BIT7_R {
        DATA_BIT7_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "DLP Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlp_status1](index.html) module"]
pub struct DLP_STATUS1_SPEC;
impl crate::RegisterSpec for DLP_STATUS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlp_status1::R](R) reader structure"]
impl crate::Readable for DLP_STATUS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLP_STATUS1 to value 0"]
impl crate::Resettable for DLP_STATUS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
