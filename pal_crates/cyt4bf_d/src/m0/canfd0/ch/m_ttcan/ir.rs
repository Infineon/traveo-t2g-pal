#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
pub type RF0N_R = crate::BitReader<bool>;
#[doc = "Field `RF0N` writer - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
pub type RF0N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
pub type RF0W_R = crate::BitReader<bool>;
#[doc = "Field `RF0W` writer - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
pub type RF0W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
pub type RF0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0F` writer - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
pub type RF0F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF0L_` reader - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
pub type RF0L__R = crate::BitReader<bool>;
#[doc = "Field `RF0L_` writer - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
pub type RF0L__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
pub type RF1N_R = crate::BitReader<bool>;
#[doc = "Field `RF1N` writer - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
pub type RF1N_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
pub type RF1W_R = crate::BitReader<bool>;
#[doc = "Field `RF1W` writer - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
pub type RF1W_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1F` reader - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
pub type RF1F_R = crate::BitReader<bool>;
#[doc = "Field `RF1F` writer - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
pub type RF1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `RF1L_` reader - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
pub type RF1L__R = crate::BitReader<bool>;
#[doc = "Field `RF1L_` writer - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
pub type RF1L__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `HPM` reader - High Priority Message 0= No high priority message received 1= High priority message received"]
pub type HPM_R = crate::BitReader<bool>;
#[doc = "Field `HPM` writer - High Priority Message 0= No high priority message received 1= High priority message received"]
pub type HPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TC` reader - Transmission Completed 0= No transmission completed 1= Transmission completed"]
pub type TC_R = crate::BitReader<bool>;
#[doc = "Field `TC` writer - Transmission Completed 0= No transmission completed 1= Transmission completed"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
pub type TCF_R = crate::BitReader<bool>;
#[doc = "Field `TCF` writer - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TFE` reader - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
pub type TFE_R = crate::BitReader<bool>;
#[doc = "Field `TFE` writer - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
pub type TEFN_R = crate::BitReader<bool>;
#[doc = "Field `TEFN` writer - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
pub type TEFN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
pub type TEFW_R = crate::BitReader<bool>;
#[doc = "Field `TEFW` writer - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
pub type TEFW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
pub type TEFF_R = crate::BitReader<bool>;
#[doc = "Field `TEFF` writer - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
pub type TEFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TEFL_` reader - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
pub type TEFL__R = crate::BitReader<bool>;
#[doc = "Field `TEFL_` writer - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
pub type TEFL__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TSW` reader - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
pub type TSW_R = crate::BitReader<bool>;
#[doc = "Field `TSW` writer - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
pub type TSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `MRAF` reader - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
pub type MRAF_R = crate::BitReader<bool>;
#[doc = "Field `MRAF` writer - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
pub type MRAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `TOO` reader - Timeout Occurred 0= No timeout 1= Timeout reached"]
pub type TOO_R = crate::BitReader<bool>;
#[doc = "Field `TOO` writer - Timeout Occurred 0= No timeout 1= Timeout reached"]
pub type TOO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
pub type DRX_R = crate::BitReader<bool>;
#[doc = "Field `DRX` writer - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
pub type DRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BEC` reader - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
pub type BEC_R = crate::BitReader<bool>;
#[doc = "Field `BEC` writer - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
pub type BEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BEU` reader - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
pub type BEU_R = crate::BitReader<bool>;
#[doc = "Field `BEU` writer - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
pub type BEU_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ELO` reader - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
pub type ELO_R = crate::BitReader<bool>;
#[doc = "Field `ELO` writer - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
pub type ELO_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EP_` reader - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
pub type EP__R = crate::BitReader<bool>;
#[doc = "Field `EP_` writer - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
pub type EP__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `EW_` reader - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
pub type EW__R = crate::BitReader<bool>;
#[doc = "Field `EW_` writer - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
pub type EW__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `BO_` reader - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
pub type BO__R = crate::BitReader<bool>;
#[doc = "Field `BO_` writer - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
pub type BO__W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `WDI` reader - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
pub type WDI_R = crate::BitReader<bool>;
#[doc = "Field `WDI` writer - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
pub type WDI_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
pub type PEA_R = crate::BitReader<bool>;
#[doc = "Field `PEA` writer - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
pub type PEA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
pub type PED_R = crate::BitReader<bool>;
#[doc = "Field `PED` writer - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
pub type PED_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `ARA` reader - N/A"]
pub type ARA_R = crate::BitReader<bool>;
#[doc = "Field `ARA` writer - N/A"]
pub type ARA_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    pub fn rf0l_(&self) -> RF0L__R {
        RF0L__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    pub fn rf1l_(&self) -> RF1L__R {
        RF1L__R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    pub fn tefl_(&self) -> TEFL__R {
        TEFL__R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    pub fn bec(&self) -> BEC_R {
        BEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    pub fn beu(&self) -> BEU_R {
        BEU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    pub fn ep_(&self) -> EP__R {
        EP__R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    pub fn ew_(&self) -> EW__R {
        EW__R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    pub fn bo_(&self) -> BO__R {
        BO__R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx FIFO 0 New Message 0= No new message written to Rx FIFO 0 1= New message written to Rx FIFO 0"]
    #[inline(always)]
    #[must_use]
    pub fn rf0n(&mut self) -> RF0N_W<0> {
        RF0N_W::new(self)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached 0= Rx FIFO 0 fill level below watermark 1= Rx FIFO 0 fill level reached watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rf0w(&mut self) -> RF0W_W<1> {
        RF0W_W::new(self)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full 0= Rx FIFO 0 not full 1= Rx FIFO 0 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf0f(&mut self) -> RF0F_W<2> {
        RF0F_W::new(self)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost 0= No Rx FIFO 0 message lost 1= Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size zero"]
    #[inline(always)]
    #[must_use]
    pub fn rf0l_(&mut self) -> RF0L__W<3> {
        RF0L__W::new(self)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message 0= No new message written to Rx FIFO 1 1= New message written to Rx FIFO 1"]
    #[inline(always)]
    #[must_use]
    pub fn rf1n(&mut self) -> RF1N_W<4> {
        RF1N_W::new(self)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached 0= Rx FIFO 1 fill level below watermark 1= Rx FIFO 1 fill level reached watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rf1w(&mut self) -> RF1W_W<5> {
        RF1W_W::new(self)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full 0= Rx FIFO 1 not full 1= Rx FIFO 1 full"]
    #[inline(always)]
    #[must_use]
    pub fn rf1f(&mut self) -> RF1F_W<6> {
        RF1F_W::new(self)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost 0= No Rx FIFO 1 message lost 1= Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size zero"]
    #[inline(always)]
    #[must_use]
    pub fn rf1l_(&mut self) -> RF1L__W<7> {
        RF1L__W::new(self)
    }
    #[doc = "Bit 8 - High Priority Message 0= No high priority message received 1= High priority message received"]
    #[inline(always)]
    #[must_use]
    pub fn hpm(&mut self) -> HPM_W<8> {
        HPM_W::new(self)
    }
    #[doc = "Bit 9 - Transmission Completed 0= No transmission completed 1= Transmission completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<9> {
        TC_W::new(self)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished 0= No transmission cancellation finished 1= Transmission cancellation finished"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<10> {
        TCF_W::new(self)
    }
    #[doc = "Bit 11 - Tx FIFO Empty 0= Tx FIFO non-empty 1= Tx FIFO empty"]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<11> {
        TFE_W::new(self)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry 0= Tx Event FIFO unchanged 1= Tx Handler wrote Tx Event FIFO element"]
    #[inline(always)]
    #[must_use]
    pub fn tefn(&mut self) -> TEFN_W<12> {
        TEFN_W::new(self)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached 0= Tx Event FIFO fill level below watermark 1= Tx Event FIFO fill level reached watermark"]
    #[inline(always)]
    #[must_use]
    pub fn tefw(&mut self) -> TEFW_W<13> {
        TEFW_W::new(self)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full 0= Tx Event FIFO not full 1= Tx Event FIFO full"]
    #[inline(always)]
    #[must_use]
    pub fn teff(&mut self) -> TEFF_W<14> {
        TEFF_W::new(self)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost 0= No Tx Event FIFO element lost 1= Tx Event FIFO element lost, also set after write attempt to Tx Event FIFO of size zero"]
    #[inline(always)]
    #[must_use]
    pub fn tefl_(&mut self) -> TEFL__W<15> {
        TEFL__W::new(self)
    }
    #[doc = "Bit 16 - Timestamp Wraparound 0= No timestamp counter wrap-around 1= Timestamp counter wrapped around"]
    #[inline(always)]
    #[must_use]
    pub fn tsw(&mut self) -> TSW_W<16> {
        TSW_W::new(self)
    }
    #[doc = "Bit 17 - Message RAM Access Failure The flag is set, when the Rx Handler - has not completed acceptance filtering or storage of an accepted message until the arbitration field of the following message has been received. In this case acceptance filtering or message storage is aborted and the Rx Handler starts processing of the following message. - was not able to write a message to the Message RAM. In this case message storage is aborted. In both cases the FIFO put index is not updated resp. the New Data flag for a dedicated Rx Buffer is not set, a partly stored message is overwritten when the next message is stored to this location. The flag is also set when the Tx Handler was not able to read a message from the Message RAM in time. In this case message transmission is aborted. In case of a Tx Handler access failure the M_TTCAN is switched into Restricted Operation Mode (see Section 3.1.5). To leave Restricted Operation Mode, the Host CPU has to reset CCCR.ASM. 0= No Message RAM access failure occurred 1= Message RAM access failure occurred"]
    #[inline(always)]
    #[must_use]
    pub fn mraf(&mut self) -> MRAF_W<17> {
        MRAF_W::new(self)
    }
    #[doc = "Bit 18 - Timeout Occurred 0= No timeout 1= Timeout reached"]
    #[inline(always)]
    #[must_use]
    pub fn too(&mut self) -> TOO_W<18> {
        TOO_W::new(self)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer The flag is set whenever a received message has been stored into a dedicated Rx Buffer. 0= No Rx Buffer updated 1= At least one received message stored into a Rx Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn drx(&mut self) -> DRX_W<19> {
        DRX_W::new(self)
    }
    #[doc = "Bit 20 - M_TTCAN reports correctable ECC fault to the generic fault structure, this bit always reads as 0. Bit Error Corrected Message RAM bit error detected and corrected. Controlled by input signal m_ttcan_aeim_berr\\[0\\]
generated by an optional external parity / ECC logic attached to the Message RAM. 0= No bit error detected when reading from Message RAM 1= Bit error detected and corrected (e.g. ECC)"]
    #[inline(always)]
    #[must_use]
    pub fn bec(&mut self) -> BEC_W<20> {
        BEC_W::new(self)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Message RAM bit error detected, uncorrected. Controlled by input signal m_ttcan_aeim_berr\\[1\\]
generated by an optional external parity / ECC logic attached to the Message RAM. An uncorrected Message RAM bit error sets CCCR.INIT to '1'. This is done to avoid transmission of corrupted data. 0= No bit error detected when reading from Message RAM 1= Bit error detected, uncorrected (e.g. parity logic)"]
    #[inline(always)]
    #[must_use]
    pub fn beu(&mut self) -> BEU_W<21> {
        BEU_W::new(self)
    }
    #[doc = "Bit 22 - Error Logging Overflow 0= CAN Error Logging Counter did not overflow 1= Overflow of CAN Error Logging Counter occurred"]
    #[inline(always)]
    #[must_use]
    pub fn elo(&mut self) -> ELO_W<22> {
        ELO_W::new(self)
    }
    #[doc = "Bit 23 - Error Passive 0= Error_Passive status unchanged 1= Error_Passive status changed"]
    #[inline(always)]
    #[must_use]
    pub fn ep_(&mut self) -> EP__W<23> {
        EP__W::new(self)
    }
    #[doc = "Bit 24 - Warning Status 0= Error_Warning status unchanged 1= Error_Warning status changed"]
    #[inline(always)]
    #[must_use]
    pub fn ew_(&mut self) -> EW__W<24> {
        EW__W::new(self)
    }
    #[doc = "Bit 25 - Bus_Off Status 0= Bus_Off status unchanged 1= Bus_Off status changed"]
    #[inline(always)]
    #[must_use]
    pub fn bo_(&mut self) -> BO__W<25> {
        BO__W::new(self)
    }
    #[doc = "Bit 26 - Watchdog Interrupt 0= No Message RAM Watchdog event occurred 1= Message RAM Watchdog event due to missing READY"]
    #[inline(always)]
    #[must_use]
    pub fn wdi(&mut self) -> WDI_W<26> {
        WDI_W::new(self)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used) 0= No protocol error in arbitration phase 1= Protocol error in arbitration phase detected (PSR.LEC != 0,7)"]
    #[inline(always)]
    #[must_use]
    pub fn pea(&mut self) -> PEA_W<27> {
        PEA_W::new(self)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used) 0= No protocol error in data phase 1= Protocol error in data phase detected (PSR.DLEC != 0,7)"]
    #[inline(always)]
    #[must_use]
    pub fn ped(&mut self) -> PED_W<28> {
        PED_W::new(self)
    }
    #[doc = "Bit 29 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ara(&mut self) -> ARA_W<29> {
        ARA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
