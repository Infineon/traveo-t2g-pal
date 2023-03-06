#[doc = "Register `CTL1` reader"]
pub struct R(crate::R<CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL1` writer"]
pub struct W(crate::W<CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL1_SPEC>;
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
impl From<crate::W<CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_NR` reader - Number of data fields (minus 1) in the response (not including the checksum): '0': 1 data field. '1': 2 data fields. ... '7': 8 data fields. Note: master and slave nodes need to agree upon the number of data fields before message transfer. In RX_RESPONSE case, When PID (header) is received, firmware has the time of one response data byte, to modify CTL1.DATA_NR."]
pub type DATA_NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_NR` writer - Number of data fields (minus 1) in the response (not including the checksum): '0': 1 data field. '1': 2 data fields. ... '7': 8 data fields. Note: master and slave nodes need to agree upon the number of data fields before message transfer. In RX_RESPONSE case, When PID (header) is received, firmware has the time of one response data byte, to modify CTL1.DATA_NR."]
pub type DATA_NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CHECKSUM_ENHANCED` reader - Checksum mode: '0': Classic mode. PID field is NOT included in the checksum calculation. '1': Enhanced mode. PID field is included in the checksum calculation. This mode requires special attention when the master node transmits the header and a (different) slave node transmits the response: the slave node will use the calculated partial checksum over the received PID field as a starting point for the calculation over the to be transmitted data fields. Note: If the frame identifier ID\\[5:0\\]
is 0x3c or 0x3d, the classic mode will ALWAYS be used for transmission and assumed for reception, independent of the CHECKSUM_ENHANCED value."]
pub type CHECKSUM_ENHANCED_R = crate::BitReader<bool>;
#[doc = "Field `CHECKSUM_ENHANCED` writer - Checksum mode: '0': Classic mode. PID field is NOT included in the checksum calculation. '1': Enhanced mode. PID field is included in the checksum calculation. This mode requires special attention when the master node transmits the header and a (different) slave node transmits the response: the slave node will use the calculated partial checksum over the received PID field as a starting point for the calculation over the to be transmitted data fields. Note: If the frame identifier ID\\[5:0\\]
is 0x3c or 0x3d, the classic mode will ALWAYS be used for transmission and assumed for reception, independent of the CHECKSUM_ENHANCED value."]
pub type CHECKSUM_ENHANCED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL1_SPEC, bool, O>;
#[doc = "Field `FRAME_TIMEOUT` reader - Specifies the maximum allowed length (timeout value) for a frame, frame header or frame response in bit periods. The LIN specification prescribes to set the maximum length to 1.4x the nominal length (Theader_max = 1.4 x Theader_nom and Tresponse_max = 1.4 x Tresponse_nom). The nominal header length Theader_nom is 34 bit periods and the nominal response length Tresponse_nom is 10 * (data_nr + 1) bit periods (data_nr is the number of data fields) Note: the LIN specification specifies the following: 'Tools and tests shall check the Tframe_max (= Theader_max + Tresponse_max). Nodes shall not check this time. The receiving node of the frame shall accept the frame up to the next frame slot (i.e. next break field), even if it is longer then Tframe_max).'"]
pub type FRAME_TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAME_TIMEOUT` writer - Specifies the maximum allowed length (timeout value) for a frame, frame header or frame response in bit periods. The LIN specification prescribes to set the maximum length to 1.4x the nominal length (Theader_max = 1.4 x Theader_nom and Tresponse_max = 1.4 x Tresponse_nom). The nominal header length Theader_nom is 34 bit periods and the nominal response length Tresponse_nom is 10 * (data_nr + 1) bit periods (data_nr is the number of data fields) Note: the LIN specification specifies the following: 'Tools and tests shall check the Tframe_max (= Theader_max + Tresponse_max). Nodes shall not check this time. The receiving node of the frame shall accept the frame up to the next frame slot (i.e. next break field), even if it is longer then Tframe_max).'"]
pub type FRAME_TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 8, O>;
#[doc = "Field `FRAME_TIMEOUT_SEL` reader - Specifies the frame timeout mode: '0': No timeout functionality (default value). '1': Frame mode: detects timeout from the start of break field to checksum field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34+20 bit periods (header and a response with 1 data field). '2': Frame header mode: detects timeout from the start of break field to PID field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34 bit periods (header). '3': Frame response mode: detects timeout from the PID field STOP bits (exclusive) to checksum field STOP bits (the response space is included in the frame response). The minimum FRAME_TIMEOUT value is 20 bit periods (response with 1 data field)."]
pub type FRAME_TIMEOUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAME_TIMEOUT_SEL` writer - Specifies the frame timeout mode: '0': No timeout functionality (default value). '1': Frame mode: detects timeout from the start of break field to checksum field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34+20 bit periods (header and a response with 1 data field). '2': Frame header mode: detects timeout from the start of break field to PID field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34 bit periods (header). '3': Frame response mode: detects timeout from the PID field STOP bits (exclusive) to checksum field STOP bits (the response space is included in the frame response). The minimum FRAME_TIMEOUT value is 20 bit periods (response with 1 data field)."]
pub type FRAME_TIMEOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Number of data fields (minus 1) in the response (not including the checksum): '0': 1 data field. '1': 2 data fields. ... '7': 8 data fields. Note: master and slave nodes need to agree upon the number of data fields before message transfer. In RX_RESPONSE case, When PID (header) is received, firmware has the time of one response data byte, to modify CTL1.DATA_NR."]
    #[inline(always)]
    pub fn data_nr(&self) -> DATA_NR_R {
        DATA_NR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Checksum mode: '0': Classic mode. PID field is NOT included in the checksum calculation. '1': Enhanced mode. PID field is included in the checksum calculation. This mode requires special attention when the master node transmits the header and a (different) slave node transmits the response: the slave node will use the calculated partial checksum over the received PID field as a starting point for the calculation over the to be transmitted data fields. Note: If the frame identifier ID\\[5:0\\]
is 0x3c or 0x3d, the classic mode will ALWAYS be used for transmission and assumed for reception, independent of the CHECKSUM_ENHANCED value."]
    #[inline(always)]
    pub fn checksum_enhanced(&self) -> CHECKSUM_ENHANCED_R {
        CHECKSUM_ENHANCED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Specifies the maximum allowed length (timeout value) for a frame, frame header or frame response in bit periods. The LIN specification prescribes to set the maximum length to 1.4x the nominal length (Theader_max = 1.4 x Theader_nom and Tresponse_max = 1.4 x Tresponse_nom). The nominal header length Theader_nom is 34 bit periods and the nominal response length Tresponse_nom is 10 * (data_nr + 1) bit periods (data_nr is the number of data fields) Note: the LIN specification specifies the following: 'Tools and tests shall check the Tframe_max (= Theader_max + Tresponse_max). Nodes shall not check this time. The receiving node of the frame shall accept the frame up to the next frame slot (i.e. next break field), even if it is longer then Tframe_max).'"]
    #[inline(always)]
    pub fn frame_timeout(&self) -> FRAME_TIMEOUT_R {
        FRAME_TIMEOUT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Specifies the frame timeout mode: '0': No timeout functionality (default value). '1': Frame mode: detects timeout from the start of break field to checksum field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34+20 bit periods (header and a response with 1 data field). '2': Frame header mode: detects timeout from the start of break field to PID field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34 bit periods (header). '3': Frame response mode: detects timeout from the PID field STOP bits (exclusive) to checksum field STOP bits (the response space is included in the frame response). The minimum FRAME_TIMEOUT value is 20 bit periods (response with 1 data field)."]
    #[inline(always)]
    pub fn frame_timeout_sel(&self) -> FRAME_TIMEOUT_SEL_R {
        FRAME_TIMEOUT_SEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of data fields (minus 1) in the response (not including the checksum): '0': 1 data field. '1': 2 data fields. ... '7': 8 data fields. Note: master and slave nodes need to agree upon the number of data fields before message transfer. In RX_RESPONSE case, When PID (header) is received, firmware has the time of one response data byte, to modify CTL1.DATA_NR."]
    #[inline(always)]
    #[must_use]
    pub fn data_nr(&mut self) -> DATA_NR_W<0> {
        DATA_NR_W::new(self)
    }
    #[doc = "Bit 8 - Checksum mode: '0': Classic mode. PID field is NOT included in the checksum calculation. '1': Enhanced mode. PID field is included in the checksum calculation. This mode requires special attention when the master node transmits the header and a (different) slave node transmits the response: the slave node will use the calculated partial checksum over the received PID field as a starting point for the calculation over the to be transmitted data fields. Note: If the frame identifier ID\\[5:0\\]
is 0x3c or 0x3d, the classic mode will ALWAYS be used for transmission and assumed for reception, independent of the CHECKSUM_ENHANCED value."]
    #[inline(always)]
    #[must_use]
    pub fn checksum_enhanced(&mut self) -> CHECKSUM_ENHANCED_W<8> {
        CHECKSUM_ENHANCED_W::new(self)
    }
    #[doc = "Bits 16:23 - Specifies the maximum allowed length (timeout value) for a frame, frame header or frame response in bit periods. The LIN specification prescribes to set the maximum length to 1.4x the nominal length (Theader_max = 1.4 x Theader_nom and Tresponse_max = 1.4 x Tresponse_nom). The nominal header length Theader_nom is 34 bit periods and the nominal response length Tresponse_nom is 10 * (data_nr + 1) bit periods (data_nr is the number of data fields) Note: the LIN specification specifies the following: 'Tools and tests shall check the Tframe_max (= Theader_max + Tresponse_max). Nodes shall not check this time. The receiving node of the frame shall accept the frame up to the next frame slot (i.e. next break field), even if it is longer then Tframe_max).'"]
    #[inline(always)]
    #[must_use]
    pub fn frame_timeout(&mut self) -> FRAME_TIMEOUT_W<16> {
        FRAME_TIMEOUT_W::new(self)
    }
    #[doc = "Bits 24:25 - Specifies the frame timeout mode: '0': No timeout functionality (default value). '1': Frame mode: detects timeout from the start of break field to checksum field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34+20 bit periods (header and a response with 1 data field). '2': Frame header mode: detects timeout from the start of break field to PID field STOP bits (inclusive). The minimum FRAME_TIMEOUT value is 34 bit periods (header). '3': Frame response mode: detects timeout from the PID field STOP bits (exclusive) to checksum field STOP bits (the response space is included in the frame response). The minimum FRAME_TIMEOUT value is 20 bit periods (response with 1 data field)."]
    #[inline(always)]
    #[must_use]
    pub fn frame_timeout_sel(&mut self) -> FRAME_TIMEOUT_SEL_W<24> {
        FRAME_TIMEOUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl1](index.html) module"]
pub struct CTL1_SPEC;
impl crate::RegisterSpec for CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl1::R](R) reader structure"]
impl crate::Readable for CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl1::W](W) writer structure"]
impl crate::Writable for CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL1 to value 0"]
impl crate::Resettable for CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
