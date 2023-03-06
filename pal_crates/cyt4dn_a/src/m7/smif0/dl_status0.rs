#[doc = "Register `DL_STATUS0` reader"]
pub struct R(crate::R<DL_STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DL_STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DL_STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DL_STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA_BIT0` reader - Number of delay line taps for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT1` reader - Number of delay line taps for the IP's spi_data\\[1\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT2` reader - Number of delay line taps for the IP's spi_data\\[2\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_BIT3` reader - Number of delay line taps for the IP's spi_data\\[3\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
pub type DATA_BIT3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of delay line taps for the IP's spi_data\\[0\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit0(&self) -> DATA_BIT0_R {
        DATA_BIT0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of delay line taps for the IP's spi_data\\[1\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit1(&self) -> DATA_BIT1_R {
        DATA_BIT1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of delay line taps for the IP's spi_data\\[2\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit2(&self) -> DATA_BIT2_R {
        DATA_BIT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of delay line taps for the IP's spi_data\\[3\\]
(note the mapping of spi_data to IO in DEVICE.CTL.DATA_SEL) with correctly captured DLP in last read transaction. Legal range: \\[0, 255\\]."]
    #[inline(always)]
    pub fn data_bit3(&self) -> DATA_BIT3_R {
        DATA_BIT3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Data Learning Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dl_status0](index.html) module"]
pub struct DL_STATUS0_SPEC;
impl crate::RegisterSpec for DL_STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dl_status0::R](R) reader structure"]
impl crate::Readable for DL_STATUS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DL_STATUS0 to value 0"]
impl crate::Resettable for DL_STATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
