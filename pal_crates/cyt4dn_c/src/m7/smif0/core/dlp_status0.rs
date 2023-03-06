#[doc = "Register `DLP_STATUS0` reader"]
pub struct R(crate::R<DLP_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLP_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLP_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLP_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT0` reader - In SDR mode: Number of delay line taps for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal values: 0 to 16. In DDR mode: Similar to SDR except the MSB and LSB data of the DDR pair are evaluated separately and this field reflects the least number of successful taps between those two evaluations."]
pub type DATA_BIT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT1` reader - Same as the DATA_BIT0 description except for spi_data\\[1\\]"]
pub type DATA_BIT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT2` reader - Same as the DATA_BIT0 description except for spi_data\\[2\\]"]
pub type DATA_BIT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT3` reader - Same as the DATA_BIT0 description except for spi_data\\[3\\]"]
pub type DATA_BIT3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:4 - In SDR mode: Number of delay line taps for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal values: 0 to 16. In DDR mode: Similar to SDR except the MSB and LSB data of the DDR pair are evaluated separately and this field reflects the least number of successful taps between those two evaluations."]
    #[inline(always)]
    pub fn data_bit0(&self) -> DATA_BIT0_R {
        DATA_BIT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Same as the DATA_BIT0 description except for spi_data\\[1\\]"]
    #[inline(always)]
    pub fn data_bit1(&self) -> DATA_BIT1_R {
        DATA_BIT1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Same as the DATA_BIT0 description except for spi_data\\[2\\]"]
    #[inline(always)]
    pub fn data_bit2(&self) -> DATA_BIT2_R {
        DATA_BIT2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Same as the DATA_BIT0 description except for spi_data\\[3\\]"]
    #[inline(always)]
    pub fn data_bit3(&self) -> DATA_BIT3_R {
        DATA_BIT3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "DLP Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlp_status0](index.html) module"]
pub struct DLP_STATUS0_SPEC;
impl crate::RegisterSpec for DLP_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlp_status0::R](R) reader structure"]
impl crate::Readable for DLP_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLP_STATUS0 to value 0"]
impl crate::Resettable for DLP_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
