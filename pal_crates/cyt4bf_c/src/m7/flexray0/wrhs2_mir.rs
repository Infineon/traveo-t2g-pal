#[doc = "Register `WRHS2_MIR` reader"]
pub struct R(crate::R<WRHS2_MIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRHS2_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRHS2_MIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRHS2_MIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRHS2_MIR` writer"]
pub struct W(crate::W<WRHS2_MIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRHS2_MIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WRHS2_MIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRHS2_MIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC` reader - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Configuration not required Transmit Buffer: Header CRC calculated and configured by the Host For calculation of the header CRC the payload length of the frame send on the bus has to be considered. In static segment the payload length of all frames is configured by MHDC.SFDL\\[6:0\\]."]
pub type CRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CRC` writer - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Configuration not required Transmit Buffer: Header CRC calculated and configured by the Host For calculation of the header CRC the payload length of the frame send on the bus has to be considered. In static segment the payload length of all frames is configured by MHDC.SFDL\\[6:0\\]."]
pub type CRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRHS2_MIR_SPEC, u16, u16, 11, O>;
#[doc = "Field `PLC` reader - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host. During static segment the static frame payload length as configured by MHDC.SFDL\\[6:0\\]
defines the payload length for all static frames. If the payload length configured by PLC\\[6:0\\]
is shorter than this value padding bytes are inserted to ensure that frames have proper physical length. The padding pattern is logical zero (see also Section 5.8.3 Transmit Buffers)."]
pub type PLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLC` writer - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host. During static segment the static frame payload length as configured by MHDC.SFDL\\[6:0\\]
defines the payload length for all static frames. If the payload length configured by PLC\\[6:0\\]
is shorter than this value padding bytes are inserted to ensure that frames have proper physical length. The padding pattern is logical zero (see also Section 5.8.3 Transmit Buffers)."]
pub type PLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WRHS2_MIR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:10 - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Configuration not required Transmit Buffer: Header CRC calculated and configured by the Host For calculation of the header CRC the payload length of the frame send on the bus has to be considered. In static segment the payload length of all frames is configured by MHDC.SFDL\\[6:0\\]."]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host. During static segment the static frame payload length as configured by MHDC.SFDL\\[6:0\\]
defines the payload length for all static frames. If the payload length configured by PLC\\[6:0\\]
is shorter than this value padding bytes are inserted to ensure that frames have proper physical length. The padding pattern is logical zero (see also Section 5.8.3 Transmit Buffers)."]
    #[inline(always)]
    pub fn plc(&self) -> PLC_R {
        PLC_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Header CRC (vRF!Header!HeaderCRC) Receive Buffer: Configuration not required Transmit Buffer: Header CRC calculated and configured by the Host For calculation of the header CRC the payload length of the frame send on the bus has to be considered. In static segment the payload length of all frames is configured by MHDC.SFDL\\[6:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<0> {
        CRC_W::new(self)
    }
    #[doc = "Bits 16:22 - Payload Length Configured Length of data section (number of 2-byte words) as configured by the Host. During static segment the static frame payload length as configured by MHDC.SFDL\\[6:0\\]
defines the payload length for all static frames. If the payload length configured by PLC\\[6:0\\]
is shorter than this value padding bytes are inserted to ensure that frames have proper physical length. The padding pattern is logical zero (see also Section 5.8.3 Transmit Buffers)."]
    #[inline(always)]
    #[must_use]
    pub fn plc(&mut self) -> PLC_W<16> {
        PLC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Header Section 2 (mirror)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrhs2_mir](index.html) module"]
pub struct WRHS2_MIR_SPEC;
impl crate::RegisterSpec for WRHS2_MIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wrhs2_mir::R](R) reader structure"]
impl crate::Readable for WRHS2_MIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wrhs2_mir::W](W) writer structure"]
impl crate::Writable for WRHS2_MIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRHS2_MIR to value 0"]
impl crate::Resettable for WRHS2_MIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
