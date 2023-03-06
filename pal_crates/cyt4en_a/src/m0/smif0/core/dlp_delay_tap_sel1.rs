#[doc = "Register `DLP_DELAY_TAP_SEL1` reader"]
pub struct R(crate::R<DLP_DELAY_TAP_SEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLP_DELAY_TAP_SEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLP_DELAY_TAP_SEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLP_DELAY_TAP_SEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT4_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[4\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT4_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT5_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[5\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT5_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT6_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[6\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT6_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT7_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[7\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT7_TAP_SEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Delay line tap selection for the IP's spi_data\\[4\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit4_tap_sel(&self) -> DATA_BIT4_TAP_SEL_R {
        DATA_BIT4_TAP_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for the IP's spi_data\\[5\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit5_tap_sel(&self) -> DATA_BIT5_TAP_SEL_R {
        DATA_BIT5_TAP_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for the IP's spi_data\\[6\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit6_tap_sel(&self) -> DATA_BIT6_TAP_SEL_R {
        DATA_BIT6_TAP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for the IP's spi_data\\[7\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit7_tap_sel(&self) -> DATA_BIT7_TAP_SEL_R {
        DATA_BIT7_TAP_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DLP Delay Tap Select Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlp_delay_tap_sel1](index.html) module"]
pub struct DLP_DELAY_TAP_SEL1_SPEC;
impl crate::RegisterSpec for DLP_DELAY_TAP_SEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlp_delay_tap_sel1::R](R) reader structure"]
impl crate::Readable for DLP_DELAY_TAP_SEL1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLP_DELAY_TAP_SEL1 to value 0"]
impl crate::Resettable for DLP_DELAY_TAP_SEL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
