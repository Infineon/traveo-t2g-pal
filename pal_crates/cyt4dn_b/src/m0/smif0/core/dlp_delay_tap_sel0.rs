#[doc = "Register `DLP_DELAY_TAP_SEL0` reader"]
pub struct R(crate::R<DLP_DELAY_TAP_SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLP_DELAY_TAP_SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLP_DELAY_TAP_SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLP_DELAY_TAP_SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT0_TAP_SEL` reader - DLP tap selection for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). In SDR mode: The upper 4 bits of this field are zero, while the lower 4 bits represent the winning tap selection for all received bits on this data line. In DDR mode: The upper 4 bits of this field represent the winning tap selection for the bits received in the MSB data of the DDR pair, while the lower 4 bits represent the winning tap selection for the bits received in the LSB data of the DDR pair on this data line. A 'winning tap selection' in either case is mapped as {tap15 of neg SDL, tap13 of neg SDL,..., tap1 of neg SDL, tap15 of pos SDL, tap13 of pos SDL,..., tap1 of pos SDL} for values 15 to 0, respectively."]
pub type DATA_BIT0_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT1_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[1\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT1_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT2_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[2\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT2_TAP_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT3_TAP_SEL` reader - Delay line tap selection for the IP's spi_data\\[3\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
pub type DATA_BIT3_TAP_SEL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - DLP tap selection for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). In SDR mode: The upper 4 bits of this field are zero, while the lower 4 bits represent the winning tap selection for all received bits on this data line. In DDR mode: The upper 4 bits of this field represent the winning tap selection for the bits received in the MSB data of the DDR pair, while the lower 4 bits represent the winning tap selection for the bits received in the LSB data of the DDR pair on this data line. A 'winning tap selection' in either case is mapped as {tap15 of neg SDL, tap13 of neg SDL,..., tap1 of neg SDL, tap15 of pos SDL, tap13 of pos SDL,..., tap1 of pos SDL} for values 15 to 0, respectively."]
    #[inline(always)]
    pub fn data_bit0_tap_sel(&self) -> DATA_BIT0_TAP_SEL_R {
        DATA_BIT0_TAP_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay line tap selection for the IP's spi_data\\[1\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit1_tap_sel(&self) -> DATA_BIT1_TAP_SEL_R {
        DATA_BIT1_TAP_SEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Delay line tap selection for the IP's spi_data\\[2\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit2_tap_sel(&self) -> DATA_BIT2_TAP_SEL_R {
        DATA_BIT2_TAP_SEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Delay line tap selection for the IP's spi_data\\[3\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL). See DATA_BIT0_TAP_SEL."]
    #[inline(always)]
    pub fn data_bit3_tap_sel(&self) -> DATA_BIT3_TAP_SEL_R {
        DATA_BIT3_TAP_SEL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DLP Delay Tap Select Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlp_delay_tap_sel0](index.html) module"]
pub struct DLP_DELAY_TAP_SEL0_SPEC;
impl crate::RegisterSpec for DLP_DELAY_TAP_SEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dlp_delay_tap_sel0::R](R) reader structure"]
impl crate::Readable for DLP_DELAY_TAP_SEL0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLP_DELAY_TAP_SEL0 to value 0"]
impl crate::Resettable for DLP_DELAY_TAP_SEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
