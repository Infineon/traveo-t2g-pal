#[doc = "Register `RXPID_FI` reader"]
pub struct R(crate::R<RXPID_FI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXPID_FI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXPID_FI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXPID_FI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PID` reader - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Reception: Received PID field. SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE. Note that, this field can be use by SW to check PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and bit\\[7\\]
is the parity bit of the frame type. This parity bit is send by the transmitting node."]
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FI` reader - Frame Information. This is the byte that is received as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT."]
pub type FI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLCEXT` reader - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[15:12\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
pub type DLCEXT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Header protected identifier (PID). - Bits 6 downto 0: frame identifier ID\\[6:0\\]. - Bits 7: is odd parity bit. - PID\\[7\\]
= ! (ID\\[6\\]
^ ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[2\\]
^ ID\\[1\\]
^ ID\\[0\\]) Reception: Received PID field. SW uses the PID field to determine how to handle the response for a received frame header: TX_RESPONSE or RX_RESPONSE. Note that, this field can be use by SW to check PType byte as the HW handles both PID and PType the same way. The frame type would occupy bit\\[6:0\\]
and bit\\[7\\]
is the parity bit of the frame type. This parity bit is send by the transmitting node."]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Frame Information. This is the byte that is received as Frame Information. Per CXPI spec, FI\\[7:4\\]
denotes the data length count (DLC). FI\\[3:2\\]
denotes Network Management. Bit\\[3\\]
- wakeup.ind Bit\\[2\\]
- sleep.ind FI\\[1:0\\]
denotes CT."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data Length Count Extension. This field is intended for payload of more than 12B. This field is only valid if DLC=4'b1111 (FI\\[15:12\\]). The value specified in this field will be the new payload size. Valid values are 0-255."]
    #[inline(always)]
    pub fn dlcext(&self) -> DLCEXT_R {
        DLCEXT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "RXPID and Frame Information\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxpid_fi](index.html) module"]
pub struct RXPID_FI_SPEC;
impl crate::RegisterSpec for RXPID_FI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxpid_fi::R](R) reader structure"]
impl crate::Readable for RXPID_FI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXPID_FI to value 0"]
impl crate::Resettable for RXPID_FI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
