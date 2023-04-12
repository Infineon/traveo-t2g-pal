#[doc = "Register `ERROR_CTL` reader"]
pub struct R(crate::R<ERROR_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERROR_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERROR_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERROR_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERROR_CTL` writer"]
pub struct W(crate::W<ERROR_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERROR_CTL_SPEC>;
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
impl From<crate::W<ERROR_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERROR_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_IDX` reader - Specifies the channel index of the channel to which HW injected channel transmitter errors applies."]
pub type CH_IDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH_IDX` writer - Specifies the channel index of the channel to which HW injected channel transmitter errors applies."]
pub type CH_IDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ERROR_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `TX_SYNC_ERROR` reader - The synchronization field is changed from 0x55 to 0x00. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR activation."]
pub type TX_SYNC_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_SYNC_ERROR` writer - The synchronization field is changed from 0x55 to 0x00. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR activation."]
pub type TX_SYNC_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_SYNC_STOP_ERROR` reader - The synchronization field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR or INTR.RX_HEADER_FRAME_ERROR activation."]
pub type TX_SYNC_STOP_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_SYNC_STOP_ERROR` writer - The synchronization field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR or INTR.RX_HEADER_FRAME_ERROR activation."]
pub type TX_SYNC_STOP_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_PARITY_ERROR` reader - In LIN mode, the PID parity bit P\\[1\\]
is inverted from !(ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) to (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]). At the receiver, this should result in INTR.RX_HEADER_PARITY_ERROR activation. In UART mode, a data field's parity bit is inverted."]
pub type TX_PARITY_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_PARITY_ERROR` writer - In LIN mode, the PID parity bit P\\[1\\]
is inverted from !(ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) to (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]). At the receiver, this should result in INTR.RX_HEADER_PARITY_ERROR activation. In UART mode, a data field's parity bit is inverted."]
pub type TX_PARITY_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_PID_STOP_ERROR` reader - The PID field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_FRAME_ERROR activation."]
pub type TX_PID_STOP_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_PID_STOP_ERROR` writer - The PID field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_FRAME_ERROR activation."]
pub type TX_PID_STOP_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_DATA_STOP_ERROR` reader - The data field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation. Note: Used in UART mode."]
pub type TX_DATA_STOP_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_DATA_STOP_ERROR` writer - The data field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation. Note: Used in UART mode."]
pub type TX_DATA_STOP_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_CHECKSUM_ERROR` reader - The checksum field is inverted. At the receiver, this should result in INTR.RX_RESPONSE_CHECKSUM_ERROR activation."]
pub type TX_CHECKSUM_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_CHECKSUM_ERROR` writer - The checksum field is inverted. At the receiver, this should result in INTR.RX_RESPONSE_CHECKSUM_ERROR activation."]
pub type TX_CHECKSUM_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `TX_CHECKSUM_STOP_ERROR` reader - The checksum field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation."]
pub type TX_CHECKSUM_STOP_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `TX_CHECKSUM_STOP_ERROR` writer - The checksum field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation."]
pub type TX_CHECKSUM_STOP_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Error injection enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Error injection enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERROR_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Specifies the channel index of the channel to which HW injected channel transmitter errors applies."]
    #[inline(always)]
    pub fn ch_idx(&self) -> CH_IDX_R {
        CH_IDX_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - The synchronization field is changed from 0x55 to 0x00. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR activation."]
    #[inline(always)]
    pub fn tx_sync_error(&self) -> TX_SYNC_ERROR_R {
        TX_SYNC_ERROR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The synchronization field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR or INTR.RX_HEADER_FRAME_ERROR activation."]
    #[inline(always)]
    pub fn tx_sync_stop_error(&self) -> TX_SYNC_STOP_ERROR_R {
        TX_SYNC_STOP_ERROR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - In LIN mode, the PID parity bit P\\[1\\]
is inverted from !(ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) to (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]). At the receiver, this should result in INTR.RX_HEADER_PARITY_ERROR activation. In UART mode, a data field's parity bit is inverted."]
    #[inline(always)]
    pub fn tx_parity_error(&self) -> TX_PARITY_ERROR_R {
        TX_PARITY_ERROR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The PID field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_FRAME_ERROR activation."]
    #[inline(always)]
    pub fn tx_pid_stop_error(&self) -> TX_PID_STOP_ERROR_R {
        TX_PID_STOP_ERROR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - The data field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation. Note: Used in UART mode."]
    #[inline(always)]
    pub fn tx_data_stop_error(&self) -> TX_DATA_STOP_ERROR_R {
        TX_DATA_STOP_ERROR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The checksum field is inverted. At the receiver, this should result in INTR.RX_RESPONSE_CHECKSUM_ERROR activation."]
    #[inline(always)]
    pub fn tx_checksum_error(&self) -> TX_CHECKSUM_ERROR_R {
        TX_CHECKSUM_ERROR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The checksum field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation."]
    #[inline(always)]
    pub fn tx_checksum_stop_error(&self) -> TX_CHECKSUM_STOP_ERROR_R {
        TX_CHECKSUM_STOP_ERROR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Specifies the channel index of the channel to which HW injected channel transmitter errors applies."]
    #[inline(always)]
    #[must_use]
    pub fn ch_idx(&mut self) -> CH_IDX_W<0> {
        CH_IDX_W::new(self)
    }
    #[doc = "Bit 16 - The synchronization field is changed from 0x55 to 0x00. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR activation."]
    #[inline(always)]
    #[must_use]
    pub fn tx_sync_error(&mut self) -> TX_SYNC_ERROR_W<16> {
        TX_SYNC_ERROR_W::new(self)
    }
    #[doc = "Bit 17 - The synchronization field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_SYNC_ERROR or INTR.RX_HEADER_FRAME_ERROR activation."]
    #[inline(always)]
    #[must_use]
    pub fn tx_sync_stop_error(&mut self) -> TX_SYNC_STOP_ERROR_W<17> {
        TX_SYNC_STOP_ERROR_W::new(self)
    }
    #[doc = "Bit 18 - In LIN mode, the PID parity bit P\\[1\\]
is inverted from !(ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]) to (ID\\[5\\]
^ ID\\[4\\]
^ ID\\[3\\]
^ ID\\[1\\]). At the receiver, this should result in INTR.RX_HEADER_PARITY_ERROR activation. In UART mode, a data field's parity bit is inverted."]
    #[inline(always)]
    #[must_use]
    pub fn tx_parity_error(&mut self) -> TX_PARITY_ERROR_W<18> {
        TX_PARITY_ERROR_W::new(self)
    }
    #[doc = "Bit 19 - The PID field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_HEADER_FRAME_ERROR activation."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pid_stop_error(&mut self) -> TX_PID_STOP_ERROR_W<19> {
        TX_PID_STOP_ERROR_W::new(self)
    }
    #[doc = "Bit 21 - The data field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation. Note: Used in UART mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_stop_error(&mut self) -> TX_DATA_STOP_ERROR_W<21> {
        TX_DATA_STOP_ERROR_W::new(self)
    }
    #[doc = "Bit 22 - The checksum field is inverted. At the receiver, this should result in INTR.RX_RESPONSE_CHECKSUM_ERROR activation."]
    #[inline(always)]
    #[must_use]
    pub fn tx_checksum_error(&mut self) -> TX_CHECKSUM_ERROR_W<22> {
        TX_CHECKSUM_ERROR_W::new(self)
    }
    #[doc = "Bit 23 - The checksum field STOP bits are inverted to '0'. At the receiver, this should result in INTR.RX_RESPONSE_FRAME_ERROR activation."]
    #[inline(always)]
    #[must_use]
    pub fn tx_checksum_stop_error(&mut self) -> TX_CHECKSUM_STOP_ERROR_W<23> {
        TX_CHECKSUM_STOP_ERROR_W::new(self)
    }
    #[doc = "Bit 31 - Error injection enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [error_ctl](index.html) module"]
pub struct ERROR_CTL_SPEC;
impl crate::RegisterSpec for ERROR_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [error_ctl::R](R) reader structure"]
impl crate::Readable for ERROR_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [error_ctl::W](W) writer structure"]
impl crate::Writable for ERROR_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERROR_CTL to value 0"]
impl crate::Resettable for ERROR_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
