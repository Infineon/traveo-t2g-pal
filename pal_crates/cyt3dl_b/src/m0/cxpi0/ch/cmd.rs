#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_HEADER` reader - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error (such as bit error if bit_ignore=0, arbitration loss, tx data length error. For timeout please refer to SAS) is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. The above is for transmission of PID without prior transmission of PTYPE. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered CXPI message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. SW clears this field to '0', when it wants to cancel a pending request. Note that if SW clears this field to '0' while HW is already in the middle of the request, then the cancel will be ignored. The cancel request can happen when HW is pending a retry when it is still servicing the current transaction. Or the cancel request can occur when HW is checking IFS/bus idle-ness. Note that if PTYPE (TX_HEADER) is transmitted follow by receive response (RX_RESPONSE) or no response, HW clears TX_HEADER right after transmitting PTYPE."]
pub type TX_HEADER_R = crate::BitReader<bool>;
#[doc = "Field `TX_HEADER` writer - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error (such as bit error if bit_ignore=0, arbitration loss, tx data length error. For timeout please refer to SAS) is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. The above is for transmission of PID without prior transmission of PTYPE. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered CXPI message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. SW clears this field to '0', when it wants to cancel a pending request. Note that if SW clears this field to '0' while HW is already in the middle of the request, then the cancel will be ignored. The cancel request can happen when HW is pending a retry when it is still servicing the current transaction. Or the cancel request can occur when HW is checking IFS/bus idle-ness. Note that if PTYPE (TX_HEADER) is transmitted follow by receive response (RX_RESPONSE) or no response, HW clears TX_HEADER right after transmitting PTYPE."]
pub type TX_HEADER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TX_RESPONSE` reader - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. SW can also clear this field to '0' if SW wants to cancel the TX_RESPONSE. The response is transmitted when the CRC are transmitted (INTR.TX_RESPONSE_DONE)."]
pub type TX_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `TX_RESPONSE` writer - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. SW can also clear this field to '0' if SW wants to cancel the TX_RESPONSE. The response is transmitted when the CRC are transmitted (INTR.TX_RESPONSE_DONE)."]
pub type TX_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - SW sets this field to '1' to direct HW to sleep mode. HW transits from Normal to Sleep upon SLEEP=1 and both TX and RX is idle. HW sets this field to '0' when it is in Sleep mode. Note that, SW needs to manage the entry to sleep mode by checking the conditions are met before initiating sleep mode e.g. all slave nodes supports sleep and on transmitting sleep frames to indicate sleep to all slave nodes.SW shall not program SLEEP=1 when HW is executing TX_WAKEUP_PULSE command. If SW programs SLEEP=1 during HW executing TX_WAKEUP_PULSE command, it will cause HW to abruptly stop transmitting wakeup pulse as below: 1. HW not clearing TX_WAKEUP_PULSE 2. HW not setting TX_WAKEUP_DONE 3. HW outputting TX_OUT=0."]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - SW sets this field to '1' to direct HW to sleep mode. HW transits from Normal to Sleep upon SLEEP=1 and both TX and RX is idle. HW sets this field to '0' when it is in Sleep mode. Note that, SW needs to manage the entry to sleep mode by checking the conditions are met before initiating sleep mode e.g. all slave nodes supports sleep and on transmitting sleep frames to indicate sleep to all slave nodes.SW shall not program SLEEP=1 when HW is executing TX_WAKEUP_PULSE command. If SW programs SLEEP=1 during HW executing TX_WAKEUP_PULSE command, it will cause HW to abruptly stop transmitting wakeup pulse as below: 1. HW not clearing TX_WAKEUP_PULSE 2. HW not setting TX_WAKEUP_DONE 3. HW outputting TX_OUT=0."]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `WAKE_TO_STANDBY` reader - SW sets this field to '1' to direct HW to wake up from Sleep mode to Standby mode. SW clears this field to '0' from '1' when it wants to direct HW from Standby to Normal mode. HW clears this field to '0' when it is in Normal mode or back to Sleep mode from Standby mode. For the case of CXPI master mode, HW will move its power mode from Sleep->Standby when this field is set to '1'. When SW clears this field is from '1' to '0' in Standby mode, HW will move to Normal mode while HW starts transmitting clock. To transmit wake pulse, SW need to program TX_WAKE_PULSE accordingly in Standby mode before entering Normal. For the case of CXPI slave mode and PWM mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW will wait for detection of clock before moving from Standby->Normal. SW clearing this field to '0' will have no effect. For the case of CXPI slave mode and NRZ mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW needs to be directed by SW by clearing this field to '0' to move from Standby->Normal after SW has detected clock through another IP (e.g. MXTCPWM)."]
pub type WAKE_TO_STANDBY_R = crate::BitReader<bool>;
#[doc = "Field `WAKE_TO_STANDBY` writer - SW sets this field to '1' to direct HW to wake up from Sleep mode to Standby mode. SW clears this field to '0' from '1' when it wants to direct HW from Standby to Normal mode. HW clears this field to '0' when it is in Normal mode or back to Sleep mode from Standby mode. For the case of CXPI master mode, HW will move its power mode from Sleep->Standby when this field is set to '1'. When SW clears this field is from '1' to '0' in Standby mode, HW will move to Normal mode while HW starts transmitting clock. To transmit wake pulse, SW need to program TX_WAKE_PULSE accordingly in Standby mode before entering Normal. For the case of CXPI slave mode and PWM mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW will wait for detection of clock before moving from Standby->Normal. SW clearing this field to '0' will have no effect. For the case of CXPI slave mode and NRZ mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW needs to be directed by SW by clearing this field to '0' to move from Standby->Normal after SW has detected clock through another IP (e.g. MXTCPWM)."]
pub type WAKE_TO_STANDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TX_WAKE_PULSE` reader - SW sets this field to '1' to direct HW to send wake up pulse. HW will transmit wake up pulse in Standby state only. HW will ignore this field when it's not in Standby state. '1' - HW will drive CXPI bus to low for period dictated by T_WAKEUP_LENGTH. '0:'- No wake up pulse HW clears this field to '0' after it transmit the pulse per T_WAKEUP_LENGTH. For the case where more than 1 wake pulses are required, SW is expected to set this field to '1' again (after this field is cleared to '0') per the number of times the wake pulse is required."]
pub type TX_WAKE_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `TX_WAKE_PULSE` writer - SW sets this field to '1' to direct HW to send wake up pulse. HW will transmit wake up pulse in Standby state only. HW will ignore this field when it's not in Standby state. '1' - HW will drive CXPI bus to low for period dictated by T_WAKEUP_LENGTH. '0:'- No wake up pulse HW clears this field to '0' after it transmit the pulse per T_WAKEUP_LENGTH. For the case where more than 1 wake pulses are required, SW is expected to set this field to '1' again (after this field is cleared to '0') per the number of times the wake pulse is required."]
pub type TX_WAKE_PULSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `IFS_WAIT` reader - SW sets this field to '1' to wait for IFS. HW clears this field to '0' after it detects logical '1' based on IFS. HW will keep this field to '1' if it detects logical '0' before fulfilling the number of logical '1' required. The intention of this bit is to provide capability for SW to direct HW to check bus idle-ness before transmitting. Without setting this bit before directing HW to send header, HW will not check bus idle before transmitting. Besides that, this bit will also provide SW an option to configure HW to wait for IFS before sending again PTYPE or PID to fulfill IFS if there is no response from other nodes. Note: SW needs to configure valid IFS values before setting IFS_WAIT=1."]
pub type IFS_WAIT_R = crate::BitReader<bool>;
#[doc = "Field `IFS_WAIT` writer - SW sets this field to '1' to wait for IFS. HW clears this field to '0' after it detects logical '1' based on IFS. HW will keep this field to '1' if it detects logical '0' before fulfilling the number of logical '1' required. The intention of this bit is to provide capability for SW to direct HW to check bus idle-ness before transmitting. Without setting this bit before directing HW to send header, HW will not check bus idle before transmitting. Besides that, this bit will also provide SW an option to configure HW to wait for IFS before sending again PTYPE or PID to fulfill IFS if there is no response from other nodes. Note: SW needs to configure valid IFS values before setting IFS_WAIT=1."]
pub type IFS_WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RX_HEADER` reader - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences. (Not set to '0' when an error is detected). -RX_HEADER -RX_HEADER, TX_RESPONSE -RX_HEADER, RX_RESPONSE The above applies for cases of receiving PID without prior receiving PTYPE. The header is received when the PID field STOP bits are received (INTR.RX_HEADER_PID_DONE and INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID/PTYPE field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has lower priority than RX_RESPONSE hence RX_RESPONSE need to be clear to '0' by SW) to transmit a response. Note that, for cases with RXPIDZERO_CHECK_EN=1 for slave in polling mode, RX_HEADER is cleared upon receiving PTYPE if the next course of action is to transmit PID. If the next course of action is to receive PID, then the RX_HEADER is cleared upon completion of response."]
pub type RX_HEADER_R = crate::BitReader<bool>;
#[doc = "Field `RX_HEADER` writer - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences. (Not set to '0' when an error is detected). -RX_HEADER -RX_HEADER, TX_RESPONSE -RX_HEADER, RX_RESPONSE The above applies for cases of receiving PID without prior receiving PTYPE. The header is received when the PID field STOP bits are received (INTR.RX_HEADER_PID_DONE and INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID/PTYPE field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has lower priority than RX_RESPONSE hence RX_RESPONSE need to be clear to '0' by SW) to transmit a response. Note that, for cases with RXPIDZERO_CHECK_EN=1 for slave in polling mode, RX_HEADER is cleared upon receiving PTYPE if the next course of action is to transmit PID. If the next course of action is to receive PID, then the RX_HEADER is cleared upon completion of response."]
pub type RX_HEADER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RX_RESPONSE` reader - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). -TX_HEADER, RX_RESPONSE -RX_HEADER, RX_RESPONSE SW can set this field to '1' to be conservative on receiving response i.e. IBS=0 whenever it is receiving PID. If the PID corresponds to transmit response, the SW can then clear this field to '0' and set TX_RESPONSE=1 to direct HW to send response. The response is received after CRC are received (INTR.RX_RESPONSE_DONE)."]
pub type RX_RESPONSE_R = crate::BitReader<bool>;
#[doc = "Field `RX_RESPONSE` writer - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). -TX_HEADER, RX_RESPONSE -RX_HEADER, RX_RESPONSE SW can set this field to '1' to be conservative on receiving response i.e. IBS=0 whenever it is receiving PID. If the PID corresponds to transmit response, the SW can then clear this field to '0' and set TX_RESPONSE=1 to direct HW to send response. The response is received after CRC are received (INTR.RX_RESPONSE_DONE)."]
pub type RX_RESPONSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error (such as bit error if bit_ignore=0, arbitration loss, tx data length error. For timeout please refer to SAS) is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. The above is for transmission of PID without prior transmission of PTYPE. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered CXPI message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. SW clears this field to '0', when it wants to cancel a pending request. Note that if SW clears this field to '0' while HW is already in the middle of the request, then the cancel will be ignored. The cancel request can happen when HW is pending a retry when it is still servicing the current transaction. Or the cancel request can occur when HW is checking IFS/bus idle-ness. Note that if PTYPE (TX_HEADER) is transmitted follow by receive response (RX_RESPONSE) or no response, HW clears TX_HEADER right after transmitting PTYPE."]
    #[inline(always)]
    pub fn tx_header(&self) -> TX_HEADER_R {
        TX_HEADER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. SW can also clear this field to '0' if SW wants to cancel the TX_RESPONSE. The response is transmitted when the CRC are transmitted (INTR.TX_RESPONSE_DONE)."]
    #[inline(always)]
    pub fn tx_response(&self) -> TX_RESPONSE_R {
        TX_RESPONSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SW sets this field to '1' to direct HW to sleep mode. HW transits from Normal to Sleep upon SLEEP=1 and both TX and RX is idle. HW sets this field to '0' when it is in Sleep mode. Note that, SW needs to manage the entry to sleep mode by checking the conditions are met before initiating sleep mode e.g. all slave nodes supports sleep and on transmitting sleep frames to indicate sleep to all slave nodes.SW shall not program SLEEP=1 when HW is executing TX_WAKEUP_PULSE command. If SW programs SLEEP=1 during HW executing TX_WAKEUP_PULSE command, it will cause HW to abruptly stop transmitting wakeup pulse as below: 1. HW not clearing TX_WAKEUP_PULSE 2. HW not setting TX_WAKEUP_DONE 3. HW outputting TX_OUT=0."]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SW sets this field to '1' to direct HW to wake up from Sleep mode to Standby mode. SW clears this field to '0' from '1' when it wants to direct HW from Standby to Normal mode. HW clears this field to '0' when it is in Normal mode or back to Sleep mode from Standby mode. For the case of CXPI master mode, HW will move its power mode from Sleep->Standby when this field is set to '1'. When SW clears this field is from '1' to '0' in Standby mode, HW will move to Normal mode while HW starts transmitting clock. To transmit wake pulse, SW need to program TX_WAKE_PULSE accordingly in Standby mode before entering Normal. For the case of CXPI slave mode and PWM mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW will wait for detection of clock before moving from Standby->Normal. SW clearing this field to '0' will have no effect. For the case of CXPI slave mode and NRZ mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW needs to be directed by SW by clearing this field to '0' to move from Standby->Normal after SW has detected clock through another IP (e.g. MXTCPWM)."]
    #[inline(always)]
    pub fn wake_to_standby(&self) -> WAKE_TO_STANDBY_R {
        WAKE_TO_STANDBY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SW sets this field to '1' to direct HW to send wake up pulse. HW will transmit wake up pulse in Standby state only. HW will ignore this field when it's not in Standby state. '1' - HW will drive CXPI bus to low for period dictated by T_WAKEUP_LENGTH. '0:'- No wake up pulse HW clears this field to '0' after it transmit the pulse per T_WAKEUP_LENGTH. For the case where more than 1 wake pulses are required, SW is expected to set this field to '1' again (after this field is cleared to '0') per the number of times the wake pulse is required."]
    #[inline(always)]
    pub fn tx_wake_pulse(&self) -> TX_WAKE_PULSE_R {
        TX_WAKE_PULSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SW sets this field to '1' to wait for IFS. HW clears this field to '0' after it detects logical '1' based on IFS. HW will keep this field to '1' if it detects logical '0' before fulfilling the number of logical '1' required. The intention of this bit is to provide capability for SW to direct HW to check bus idle-ness before transmitting. Without setting this bit before directing HW to send header, HW will not check bus idle before transmitting. Besides that, this bit will also provide SW an option to configure HW to wait for IFS before sending again PTYPE or PID to fulfill IFS if there is no response from other nodes. Note: SW needs to configure valid IFS values before setting IFS_WAIT=1."]
    #[inline(always)]
    pub fn ifs_wait(&self) -> IFS_WAIT_R {
        IFS_WAIT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences. (Not set to '0' when an error is detected). -RX_HEADER -RX_HEADER, TX_RESPONSE -RX_HEADER, RX_RESPONSE The above applies for cases of receiving PID without prior receiving PTYPE. The header is received when the PID field STOP bits are received (INTR.RX_HEADER_PID_DONE and INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID/PTYPE field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has lower priority than RX_RESPONSE hence RX_RESPONSE need to be clear to '0' by SW) to transmit a response. Note that, for cases with RXPIDZERO_CHECK_EN=1 for slave in polling mode, RX_HEADER is cleared upon receiving PTYPE if the next course of action is to transmit PID. If the next course of action is to receive PID, then the RX_HEADER is cleared upon completion of response."]
    #[inline(always)]
    pub fn rx_header(&self) -> RX_HEADER_R {
        RX_HEADER_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). -TX_HEADER, RX_RESPONSE -RX_HEADER, RX_RESPONSE SW can set this field to '1' to be conservative on receiving response i.e. IBS=0 whenever it is receiving PID. If the PID corresponds to transmit response, the SW can then clear this field to '0' and set TX_RESPONSE=1 to direct HW to send response. The response is received after CRC are received (INTR.RX_RESPONSE_DONE)."]
    #[inline(always)]
    pub fn rx_response(&self) -> RX_RESPONSE_R {
        RX_RESPONSE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SW sets this field to '1' to transmit a header. HW sets this field to '0' on successful completion of ANY of the following legal command sequences (also set to '0' when an error (such as bit error if bit_ignore=0, arbitration loss, tx data length error. For timeout please refer to SAS) is detected): - TX_HEADER - TX_HEADER, TX_RESPONSE. - TX_HEADER, RX_RESPONSE. The above is for transmission of PID without prior transmission of PTYPE. The header is transmitted when the PID field STOP bits are transmitted (INTR.TX_HEADER_DONE). HW sets this field to '1', when the 'tr_cmd_tx_header' input trigger is activated. This allows for time triggered CXPI message transfer. HW driven time triggered transfer eliminates the jitter that is typically associated with SW driven transfer. SW clears this field to '0', when it wants to cancel a pending request. Note that if SW clears this field to '0' while HW is already in the middle of the request, then the cancel will be ignored. The cancel request can happen when HW is pending a retry when it is still servicing the current transaction. Or the cancel request can occur when HW is checking IFS/bus idle-ness. Note that if PTYPE (TX_HEADER) is transmitted follow by receive response (RX_RESPONSE) or no response, HW clears TX_HEADER right after transmitting PTYPE."]
    #[inline(always)]
    #[must_use]
    pub fn tx_header(&mut self) -> TX_HEADER_W<0> {
        TX_HEADER_W::new(self)
    }
    #[doc = "Bit 1 - SW sets this field to '1' to transmit a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (also set to '0' when an error is detected). - TX_HEADER, TX_RESPONSE. - RX_HEADER, TX_RESPONSE. SW can also clear this field to '0' if SW wants to cancel the TX_RESPONSE. The response is transmitted when the CRC are transmitted (INTR.TX_RESPONSE_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_response(&mut self) -> TX_RESPONSE_W<1> {
        TX_RESPONSE_W::new(self)
    }
    #[doc = "Bit 2 - SW sets this field to '1' to direct HW to sleep mode. HW transits from Normal to Sleep upon SLEEP=1 and both TX and RX is idle. HW sets this field to '0' when it is in Sleep mode. Note that, SW needs to manage the entry to sleep mode by checking the conditions are met before initiating sleep mode e.g. all slave nodes supports sleep and on transmitting sleep frames to indicate sleep to all slave nodes.SW shall not program SLEEP=1 when HW is executing TX_WAKEUP_PULSE command. If SW programs SLEEP=1 during HW executing TX_WAKEUP_PULSE command, it will cause HW to abruptly stop transmitting wakeup pulse as below: 1. HW not clearing TX_WAKEUP_PULSE 2. HW not setting TX_WAKEUP_DONE 3. HW outputting TX_OUT=0."]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<2> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 3 - SW sets this field to '1' to direct HW to wake up from Sleep mode to Standby mode. SW clears this field to '0' from '1' when it wants to direct HW from Standby to Normal mode. HW clears this field to '0' when it is in Normal mode or back to Sleep mode from Standby mode. For the case of CXPI master mode, HW will move its power mode from Sleep->Standby when this field is set to '1'. When SW clears this field is from '1' to '0' in Standby mode, HW will move to Normal mode while HW starts transmitting clock. To transmit wake pulse, SW need to program TX_WAKE_PULSE accordingly in Standby mode before entering Normal. For the case of CXPI slave mode and PWM mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW will wait for detection of clock before moving from Standby->Normal. SW clearing this field to '0' will have no effect. For the case of CXPI slave mode and NRZ mode, HW will move power mode from Sleep->Standby when this field is set to '1'. HW needs to be directed by SW by clearing this field to '0' to move from Standby->Normal after SW has detected clock through another IP (e.g. MXTCPWM)."]
    #[inline(always)]
    #[must_use]
    pub fn wake_to_standby(&mut self) -> WAKE_TO_STANDBY_W<3> {
        WAKE_TO_STANDBY_W::new(self)
    }
    #[doc = "Bit 4 - SW sets this field to '1' to direct HW to send wake up pulse. HW will transmit wake up pulse in Standby state only. HW will ignore this field when it's not in Standby state. '1' - HW will drive CXPI bus to low for period dictated by T_WAKEUP_LENGTH. '0:'- No wake up pulse HW clears this field to '0' after it transmit the pulse per T_WAKEUP_LENGTH. For the case where more than 1 wake pulses are required, SW is expected to set this field to '1' again (after this field is cleared to '0') per the number of times the wake pulse is required."]
    #[inline(always)]
    #[must_use]
    pub fn tx_wake_pulse(&mut self) -> TX_WAKE_PULSE_W<4> {
        TX_WAKE_PULSE_W::new(self)
    }
    #[doc = "Bit 5 - SW sets this field to '1' to wait for IFS. HW clears this field to '0' after it detects logical '1' based on IFS. HW will keep this field to '1' if it detects logical '0' before fulfilling the number of logical '1' required. The intention of this bit is to provide capability for SW to direct HW to check bus idle-ness before transmitting. Without setting this bit before directing HW to send header, HW will not check bus idle before transmitting. Besides that, this bit will also provide SW an option to configure HW to wait for IFS before sending again PTYPE or PID to fulfill IFS if there is no response from other nodes. Note: SW needs to configure valid IFS values before setting IFS_WAIT=1."]
    #[inline(always)]
    #[must_use]
    pub fn ifs_wait(&mut self) -> IFS_WAIT_W<5> {
        IFS_WAIT_W::new(self)
    }
    #[doc = "Bit 8 - SW sets this field to '1' to receive a header. HW sets this field to '0' on successful completion of the ANY of the legal command sequences. (Not set to '0' when an error is detected). -RX_HEADER -RX_HEADER, TX_RESPONSE -RX_HEADER, RX_RESPONSE The above applies for cases of receiving PID without prior receiving PTYPE. The header is received when the PID field STOP bits are received (INTR.RX_HEADER_PID_DONE and INTR.RX_HEADER_DONE). Typically, a slave node SW sets both RX_HEADER and RX_RESPONSE to '1', anticipating a transfer of a response from the master node to this slave node. After receipt of the header PID/PTYPE field (INTR.RX_HEADER_PID_DONE is activated), the slave node may decide to set TX_RESPONSE to '1' (which has lower priority than RX_RESPONSE hence RX_RESPONSE need to be clear to '0' by SW) to transmit a response. Note that, for cases with RXPIDZERO_CHECK_EN=1 for slave in polling mode, RX_HEADER is cleared upon receiving PTYPE if the next course of action is to transmit PID. If the next course of action is to receive PID, then the RX_HEADER is cleared upon completion of response."]
    #[inline(always)]
    #[must_use]
    pub fn rx_header(&mut self) -> RX_HEADER_W<8> {
        RX_HEADER_W::new(self)
    }
    #[doc = "Bit 9 - SW sets this field to '1' to receive a response. HW sets this field to '0' on successful completion of ANY of the legal command sequences (NOT set to '0' when an error is detected). -TX_HEADER, RX_RESPONSE -RX_HEADER, RX_RESPONSE SW can set this field to '1' to be conservative on receiving response i.e. IBS=0 whenever it is receiving PID. If the PID corresponds to transmit response, the SW can then clear this field to '0' and set TX_RESPONSE=1 to direct HW to send response. The response is received after CRC are received (INTR.RX_RESPONSE_DONE)."]
    #[inline(always)]
    #[must_use]
    pub fn rx_response(&mut self) -> RX_RESPONSE_W<9> {
        RX_RESPONSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
