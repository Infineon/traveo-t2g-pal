#[doc = "Register `RDHS2` reader"]
pub struct R(crate::R<RDHS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDHS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDHS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDHS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRC` reader - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Header CRC updated from received data frames Transmit Buffer: Header CRC calculated and configured by the Host"]
pub type CRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLC` reader - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host."]
pub type PLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLR` reader - Payload Length Received (vRF!Header!Length) Payload length value updated from received data frames (exception: if message buffer belongs to the receive FIFO PLR\\[6:0\\]
is also updated from received null frames) When a message is stored into a message buffer the following behaviour with respect to payload length received and payload length configured is implemented: PLR\\[6:0\\]
> PLC\\[6:0\\]: The payload data stored in the message buffer is truncated to the payload length configured if PLC\\[6:0\\]
even or else truncated to PLC\\[6:0\\]
+ 1. PLR\\[6:0\\]
&lt;= PLC\\[6:0\\]: The received payload data is stored into the message buffers data section. The remaining data bytes of the data section as configured by PLC\\[6:0\\]
are filled with undefined data PLR\\[6:0\\]
= zero: The message buffer's data section is filled with undefined data PLC\\[6:0\\]
= zero: Message buffer has no data section configured. No data is stored into the message buffer's data section."]
pub type PLR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:10 - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Header CRC updated from received data frames Transmit Buffer: Header CRC calculated and configured by the Host"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host."]
    #[inline(always)]
    pub fn plc(&self) -> PLC_R {
        PLC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Payload Length Received (vRF!Header!Length) Payload length value updated from received data frames (exception: if message buffer belongs to the receive FIFO PLR\\[6:0\\]
is also updated from received null frames) When a message is stored into a message buffer the following behaviour with respect to payload length received and payload length configured is implemented: PLR\\[6:0\\]
> PLC\\[6:0\\]: The payload data stored in the message buffer is truncated to the payload length configured if PLC\\[6:0\\]
even or else truncated to PLC\\[6:0\\]
+ 1. PLR\\[6:0\\]
&lt;= PLC\\[6:0\\]: The received payload data is stored into the message buffers data section. The remaining data bytes of the data section as configured by PLC\\[6:0\\]
are filled with undefined data PLR\\[6:0\\]
= zero: The message buffer's data section is filled with undefined data PLC\\[6:0\\]
= zero: Message buffer has no data section configured. No data is stored into the message buffer's data section."]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Read Header Section 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdhs2](index.html) module"]
pub struct RDHS2_SPEC;
impl crate::RegisterSpec for RDHS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdhs2::R](R) reader structure"]
impl crate::Readable for RDHS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RDHS2 to value 0"]
impl crate::Resettable for RDHS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
