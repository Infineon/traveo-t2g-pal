#[doc = "Register `PRTC1` reader"]
pub struct R(crate::R<PRTC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRTC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRTC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRTC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRTC1` writer"]
pub struct W(crate::W<PRTC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRTC1_SPEC>;
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
impl From<crate::W<PRTC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRTC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSST` reader - Transmission Start Sequence Transmitter (gdTSSTransmitter) Configures the duration of the Transmission Start Sequence (TSS) in terms of bit times (1 bit time = 4 uT = 100ns@10Mbps). Must be identical in all nodes of a cluster. Valid values are 3 to 15 bit times."]
pub type TSST_R = crate::FieldReader<u8, TSST_A>;
#[doc = "Transmission Start Sequence Transmitter (gdTSSTransmitter) Configures the duration of the Transmission Start Sequence (TSS) in terms of bit times (1 bit time = 4 uT = 100ns@10Mbps). Must be identical in all nodes of a cluster. Valid values are 3 to 15 bit times.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TSST_A {
    #[doc = "3: N/A"]
    MIN = 3,
    #[doc = "15: N/A"]
    MAX = 15,
}
impl From<TSST_A> for u8 {
    #[inline(always)]
    fn from(variant: TSST_A) -> Self {
        variant as _
    }
}
impl TSST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSST_A> {
        match self.bits {
            3 => Some(TSST_A::MIN),
            15 => Some(TSST_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == TSST_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == TSST_A::MAX
    }
}
#[doc = "Field `TSST` writer - Transmission Start Sequence Transmitter (gdTSSTransmitter) Configures the duration of the Transmission Start Sequence (TSS) in terms of bit times (1 bit time = 4 uT = 100ns@10Mbps). Must be identical in all nodes of a cluster. Valid values are 3 to 15 bit times."]
pub type TSST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC1_SPEC, u8, TSST_A, 4, O>;
impl<'a, const O: u8> TSST_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(TSST_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(TSST_A::MAX)
    }
}
#[doc = "Field `CASM` reader - Collision Avoidance Symbol Max (gdCASRxLowMax) Configures the upper limit of the acceptance window for a collision avoidance symbol (CAS). CASM6 is fixed to '1'. Valid values are 67 to 99 bit times, corresponding to a CASM range (without CASM6) of 3 to 35."]
pub type CASM_R = crate::FieldReader<u8, CASM_A>;
#[doc = "Collision Avoidance Symbol Max (gdCASRxLowMax) Configures the upper limit of the acceptance window for a collision avoidance symbol (CAS). CASM6 is fixed to '1'. Valid values are 67 to 99 bit times, corresponding to a CASM range (without CASM6) of 3 to 35.\n\nValue on reset: 35"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CASM_A {
    #[doc = "3: N/A"]
    MIN = 3,
    #[doc = "35: N/A"]
    MAX = 35,
}
impl From<CASM_A> for u8 {
    #[inline(always)]
    fn from(variant: CASM_A) -> Self {
        variant as _
    }
}
impl CASM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CASM_A> {
        match self.bits {
            3 => Some(CASM_A::MIN),
            35 => Some(CASM_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == CASM_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == CASM_A::MAX
    }
}
#[doc = "Field `CASM` writer - Collision Avoidance Symbol Max (gdCASRxLowMax) Configures the upper limit of the acceptance window for a collision avoidance symbol (CAS). CASM6 is fixed to '1'. Valid values are 67 to 99 bit times, corresponding to a CASM range (without CASM6) of 3 to 35."]
pub type CASM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC1_SPEC, u8, CASM_A, 6, O>;
impl<'a, const O: u8> CASM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(CASM_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(CASM_A::MAX)
    }
}
#[doc = "Field `CASM6` reader - Part of CASM\\[6:0\\], but fixed to 1"]
pub type CASM6_R = crate::BitReader<bool>;
#[doc = "Field `SPP` reader - Strobe Point Position Defines the sample count value for strobing. The strobed bit value is set to the voted value when the sample count is incremented to the value configured by SPP\\[1:0\\]. 00, 11= Sample 5 (default) 01 = Sample 4 10 = Sample 6"]
pub type SPP_R = crate::FieldReader<u8, SPP_A>;
#[doc = "Strobe Point Position Defines the sample count value for strobing. The strobed bit value is set to the voted value when the sample count is incremented to the value configured by SPP\\[1:0\\]. 00, 11= Sample 5 (default) 01 = Sample 4 10 = Sample 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPP_A {
    #[doc = "0: N/A"]
    SAMPLE_5 = 0,
    #[doc = "3: N/A"]
    SAMPLE_5_MIRROR = 3,
    #[doc = "1: N/A"]
    SAMPLE_4 = 1,
    #[doc = "2: N/A"]
    SAMPLE_6 = 2,
}
impl From<SPP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPP_A) -> Self {
        variant as _
    }
}
impl SPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPP_A {
        match self.bits {
            0 => SPP_A::SAMPLE_5,
            3 => SPP_A::SAMPLE_5_MIRROR,
            1 => SPP_A::SAMPLE_4,
            2 => SPP_A::SAMPLE_6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_5`"]
    #[inline(always)]
    pub fn is_sample_5(&self) -> bool {
        *self == SPP_A::SAMPLE_5
    }
    #[doc = "Checks if the value of the field is `SAMPLE_5_MIRROR`"]
    #[inline(always)]
    pub fn is_sample_5_mirror(&self) -> bool {
        *self == SPP_A::SAMPLE_5_MIRROR
    }
    #[doc = "Checks if the value of the field is `SAMPLE_4`"]
    #[inline(always)]
    pub fn is_sample_4(&self) -> bool {
        *self == SPP_A::SAMPLE_4
    }
    #[doc = "Checks if the value of the field is `SAMPLE_6`"]
    #[inline(always)]
    pub fn is_sample_6(&self) -> bool {
        *self == SPP_A::SAMPLE_6
    }
}
#[doc = "Field `SPP` writer - Strobe Point Position Defines the sample count value for strobing. The strobed bit value is set to the voted value when the sample count is incremented to the value configured by SPP\\[1:0\\]. 00, 11= Sample 5 (default) 01 = Sample 4 10 = Sample 6"]
pub type SPP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRTC1_SPEC, u8, SPP_A, 2, O>;
impl<'a, const O: u8> SPP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sample_5(self) -> &'a mut W {
        self.variant(SPP_A::SAMPLE_5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sample_5_mirror(self) -> &'a mut W {
        self.variant(SPP_A::SAMPLE_5_MIRROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sample_4(self) -> &'a mut W {
        self.variant(SPP_A::SAMPLE_4)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn sample_6(self) -> &'a mut W {
        self.variant(SPP_A::SAMPLE_6)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler (gdSampleClockPeriod, pSamplesPerMicrotick) The Baud Rate Prescaler configures the baud rate on the FlexRay bus. The baud rates listed below are valid with a sample clock eray_sclk = 80 MHz. One bit time always consists of 8 samples independent of the configured baud rate. 00 = 10 MBit/s (default) gdSampleClockPeriod = 12.5 ns = 1 - eray_sclk pSamplesPerMicrotick = 2 (1 uT = 25 ns) 01 = 5 MBit/s gdSampleClockPeriod = 25 ns = 2 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 25 ns) 10, 11 = 2.5 MBit/s gdSampleClockPeriod = 50 ns = 4 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 50 ns)"]
pub type BRP_R = crate::FieldReader<u8, BRP_A>;
#[doc = "Baud Rate Prescaler (gdSampleClockPeriod, pSamplesPerMicrotick) The Baud Rate Prescaler configures the baud rate on the FlexRay bus. The baud rates listed below are valid with a sample clock eray_sclk = 80 MHz. One bit time always consists of 8 samples independent of the configured baud rate. 00 = 10 MBit/s (default) gdSampleClockPeriod = 12.5 ns = 1 - eray_sclk pSamplesPerMicrotick = 2 (1 uT = 25 ns) 01 = 5 MBit/s gdSampleClockPeriod = 25 ns = 2 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 25 ns) 10, 11 = 2.5 MBit/s gdSampleClockPeriod = 50 ns = 4 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 50 ns)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BRP_A {
    #[doc = "0: N/A"]
    BAUD_10_MBPS = 0,
    #[doc = "1: N/A"]
    BAUD_5_MBPS = 1,
    #[doc = "2: N/A"]
    BAUD_2P5_MBPS = 2,
    #[doc = "3: N/A"]
    BAUD_2P5_MBPS_MIRROR = 3,
}
impl From<BRP_A> for u8 {
    #[inline(always)]
    fn from(variant: BRP_A) -> Self {
        variant as _
    }
}
impl BRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRP_A {
        match self.bits {
            0 => BRP_A::BAUD_10_MBPS,
            1 => BRP_A::BAUD_5_MBPS,
            2 => BRP_A::BAUD_2P5_MBPS,
            3 => BRP_A::BAUD_2P5_MBPS_MIRROR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BAUD_10_MBPS`"]
    #[inline(always)]
    pub fn is_baud_10_mbps(&self) -> bool {
        *self == BRP_A::BAUD_10_MBPS
    }
    #[doc = "Checks if the value of the field is `BAUD_5_MBPS`"]
    #[inline(always)]
    pub fn is_baud_5_mbps(&self) -> bool {
        *self == BRP_A::BAUD_5_MBPS
    }
    #[doc = "Checks if the value of the field is `BAUD_2P5_MBPS`"]
    #[inline(always)]
    pub fn is_baud_2p5_mbps(&self) -> bool {
        *self == BRP_A::BAUD_2P5_MBPS
    }
    #[doc = "Checks if the value of the field is `BAUD_2P5_MBPS_MIRROR`"]
    #[inline(always)]
    pub fn is_baud_2p5_mbps_mirror(&self) -> bool {
        *self == BRP_A::BAUD_2P5_MBPS_MIRROR
    }
}
#[doc = "Field `BRP` writer - Baud Rate Prescaler (gdSampleClockPeriod, pSamplesPerMicrotick) The Baud Rate Prescaler configures the baud rate on the FlexRay bus. The baud rates listed below are valid with a sample clock eray_sclk = 80 MHz. One bit time always consists of 8 samples independent of the configured baud rate. 00 = 10 MBit/s (default) gdSampleClockPeriod = 12.5 ns = 1 - eray_sclk pSamplesPerMicrotick = 2 (1 uT = 25 ns) 01 = 5 MBit/s gdSampleClockPeriod = 25 ns = 2 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 25 ns) 10, 11 = 2.5 MBit/s gdSampleClockPeriod = 50 ns = 4 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 50 ns)"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRTC1_SPEC, u8, BRP_A, 2, O>;
impl<'a, const O: u8> BRP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn baud_10_mbps(self) -> &'a mut W {
        self.variant(BRP_A::BAUD_10_MBPS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn baud_5_mbps(self) -> &'a mut W {
        self.variant(BRP_A::BAUD_5_MBPS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn baud_2p5_mbps(self) -> &'a mut W {
        self.variant(BRP_A::BAUD_2P5_MBPS)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn baud_2p5_mbps_mirror(self) -> &'a mut W {
        self.variant(BRP_A::BAUD_2P5_MBPS_MIRROR)
    }
}
#[doc = "Field `RXW` reader - Wakeup Symbol Receive Window Length (gdWakeupSymbolRxWindow) Configures the number of bit times used by the node to test the duration of the received wakeup pattern. Must be identical in all nodes of a cluster. Valid values are 76 to 301 bit times."]
pub type RXW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXW` writer - Wakeup Symbol Receive Window Length (gdWakeupSymbolRxWindow) Configures the number of bit times used by the node to test the duration of the received wakeup pattern. Must be identical in all nodes of a cluster. Valid values are 76 to 301 bit times."]
pub type RXW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC1_SPEC, u16, u16, 9, O>;
#[doc = "Field `RWP` reader - Repetitions of Tx Wakeup Pattern (pWakeupPattern) Configures the number of repetitions (sequences) of the Tx wakeup symbol. Valid values are 2 to 63."]
pub type RWP_R = crate::FieldReader<u8, RWP_A>;
#[doc = "Repetitions of Tx Wakeup Pattern (pWakeupPattern) Configures the number of repetitions (sequences) of the Tx wakeup symbol. Valid values are 2 to 63.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RWP_A {
    #[doc = "2: N/A"]
    MIN = 2,
}
impl From<RWP_A> for u8 {
    #[inline(always)]
    fn from(variant: RWP_A) -> Self {
        variant as _
    }
}
impl RWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RWP_A> {
        match self.bits {
            2 => Some(RWP_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == RWP_A::MIN
    }
}
#[doc = "Field `RWP` writer - Repetitions of Tx Wakeup Pattern (pWakeupPattern) Configures the number of repetitions (sequences) of the Tx wakeup symbol. Valid values are 2 to 63."]
pub type RWP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC1_SPEC, u8, RWP_A, 6, O>;
impl<'a, const O: u8> RWP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(RWP_A::MIN)
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmission Start Sequence Transmitter (gdTSSTransmitter) Configures the duration of the Transmission Start Sequence (TSS) in terms of bit times (1 bit time = 4 uT = 100ns@10Mbps). Must be identical in all nodes of a cluster. Valid values are 3 to 15 bit times."]
    #[inline(always)]
    pub fn tsst(&self) -> TSST_R {
        TSST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:9 - Collision Avoidance Symbol Max (gdCASRxLowMax) Configures the upper limit of the acceptance window for a collision avoidance symbol (CAS). CASM6 is fixed to '1'. Valid values are 67 to 99 bit times, corresponding to a CASM range (without CASM6) of 3 to 35."]
    #[inline(always)]
    pub fn casm(&self) -> CASM_R {
        CASM_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bit 10 - Part of CASM\\[6:0\\], but fixed to 1"]
    #[inline(always)]
    pub fn casm6(&self) -> CASM6_R {
        CASM6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Strobe Point Position Defines the sample count value for strobing. The strobed bit value is set to the voted value when the sample count is incremented to the value configured by SPP\\[1:0\\]. 00, 11= Sample 5 (default) 01 = Sample 4 10 = Sample 6"]
    #[inline(always)]
    pub fn spp(&self) -> SPP_R {
        SPP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Baud Rate Prescaler (gdSampleClockPeriod, pSamplesPerMicrotick) The Baud Rate Prescaler configures the baud rate on the FlexRay bus. The baud rates listed below are valid with a sample clock eray_sclk = 80 MHz. One bit time always consists of 8 samples independent of the configured baud rate. 00 = 10 MBit/s (default) gdSampleClockPeriod = 12.5 ns = 1 - eray_sclk pSamplesPerMicrotick = 2 (1 uT = 25 ns) 01 = 5 MBit/s gdSampleClockPeriod = 25 ns = 2 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 25 ns) 10, 11 = 2.5 MBit/s gdSampleClockPeriod = 50 ns = 4 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 50 ns)"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:24 - Wakeup Symbol Receive Window Length (gdWakeupSymbolRxWindow) Configures the number of bit times used by the node to test the duration of the received wakeup pattern. Must be identical in all nodes of a cluster. Valid values are 76 to 301 bit times."]
    #[inline(always)]
    pub fn rxw(&self) -> RXW_R {
        RXW_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 26:31 - Repetitions of Tx Wakeup Pattern (pWakeupPattern) Configures the number of repetitions (sequences) of the Tx wakeup symbol. Valid values are 2 to 63."]
    #[inline(always)]
    pub fn rwp(&self) -> RWP_R {
        RWP_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmission Start Sequence Transmitter (gdTSSTransmitter) Configures the duration of the Transmission Start Sequence (TSS) in terms of bit times (1 bit time = 4 uT = 100ns@10Mbps). Must be identical in all nodes of a cluster. Valid values are 3 to 15 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn tsst(&mut self) -> TSST_W<0> {
        TSST_W::new(self)
    }
    #[doc = "Bits 4:9 - Collision Avoidance Symbol Max (gdCASRxLowMax) Configures the upper limit of the acceptance window for a collision avoidance symbol (CAS). CASM6 is fixed to '1'. Valid values are 67 to 99 bit times, corresponding to a CASM range (without CASM6) of 3 to 35."]
    #[inline(always)]
    #[must_use]
    pub fn casm(&mut self) -> CASM_W<4> {
        CASM_W::new(self)
    }
    #[doc = "Bits 12:13 - Strobe Point Position Defines the sample count value for strobing. The strobed bit value is set to the voted value when the sample count is incremented to the value configured by SPP\\[1:0\\]. 00, 11= Sample 5 (default) 01 = Sample 4 10 = Sample 6"]
    #[inline(always)]
    #[must_use]
    pub fn spp(&mut self) -> SPP_W<12> {
        SPP_W::new(self)
    }
    #[doc = "Bits 14:15 - Baud Rate Prescaler (gdSampleClockPeriod, pSamplesPerMicrotick) The Baud Rate Prescaler configures the baud rate on the FlexRay bus. The baud rates listed below are valid with a sample clock eray_sclk = 80 MHz. One bit time always consists of 8 samples independent of the configured baud rate. 00 = 10 MBit/s (default) gdSampleClockPeriod = 12.5 ns = 1 - eray_sclk pSamplesPerMicrotick = 2 (1 uT = 25 ns) 01 = 5 MBit/s gdSampleClockPeriod = 25 ns = 2 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 25 ns) 10, 11 = 2.5 MBit/s gdSampleClockPeriod = 50 ns = 4 - eray_sclk pSamplesPerMicrotick = 1 (1 uT = 50 ns)"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<14> {
        BRP_W::new(self)
    }
    #[doc = "Bits 16:24 - Wakeup Symbol Receive Window Length (gdWakeupSymbolRxWindow) Configures the number of bit times used by the node to test the duration of the received wakeup pattern. Must be identical in all nodes of a cluster. Valid values are 76 to 301 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn rxw(&mut self) -> RXW_W<16> {
        RXW_W::new(self)
    }
    #[doc = "Bits 26:31 - Repetitions of Tx Wakeup Pattern (pWakeupPattern) Configures the number of repetitions (sequences) of the Tx wakeup symbol. Valid values are 2 to 63."]
    #[inline(always)]
    #[must_use]
    pub fn rwp(&mut self) -> RWP_W<26> {
        RWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRT Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prtc1](index.html) module"]
pub struct PRTC1_SPEC;
impl crate::RegisterSpec for PRTC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prtc1::R](R) reader structure"]
impl crate::Readable for PRTC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prtc1::W](W) writer structure"]
impl crate::Writable for PRTC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRTC1 to value 0x084c_0633"]
impl crate::Resettable for PRTC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x084c_0633;
}
