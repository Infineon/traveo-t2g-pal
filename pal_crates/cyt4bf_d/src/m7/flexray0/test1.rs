#[doc = "Register `TEST1` reader"]
pub struct R(crate::R<TEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST1` writer"]
pub struct W(crate::W<TEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST1_SPEC>;
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
impl From<crate::W<TEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRTEN` reader - Write Test Register Enable Enables write access to the test registers. To set the bit from '0' to '1' the test mode key has to be written as defined in \\[01\\]Section 4.3.3 Lock Register (LCK). The unlock sequence is not required when WRTEN is kept at '1' while other bits of the register are changed. The bit can be reset to '0' at any time. 1 = Write access to test registers enabled 0 = Write access to test registers disabled"]
pub type WRTEN_R = crate::BitReader<WRTEN_A>;
#[doc = "Write Test Register Enable Enables write access to the test registers. To set the bit from '0' to '1' the test mode key has to be written as defined in \\[01\\]Section 4.3.3 Lock Register (LCK). The unlock sequence is not required when WRTEN is kept at '1' while other bits of the register are changed. The bit can be reset to '0' at any time. 1 = Write access to test registers enabled 0 = Write access to test registers disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRTEN_A {
    #[doc = "0: N/A"]
    TEST_REG_WR_DISABLED = 0,
    #[doc = "1: N/A"]
    TEST_REG_WR_ENABLED = 1,
}
impl From<WRTEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WRTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRTEN_A {
        match self.bits {
            false => WRTEN_A::TEST_REG_WR_DISABLED,
            true => WRTEN_A::TEST_REG_WR_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `TEST_REG_WR_DISABLED`"]
    #[inline(always)]
    pub fn is_test_reg_wr_disabled(&self) -> bool {
        *self == WRTEN_A::TEST_REG_WR_DISABLED
    }
    #[doc = "Checks if the value of the field is `TEST_REG_WR_ENABLED`"]
    #[inline(always)]
    pub fn is_test_reg_wr_enabled(&self) -> bool {
        *self == WRTEN_A::TEST_REG_WR_ENABLED
    }
}
#[doc = "Field `WRTEN` writer - Write Test Register Enable Enables write access to the test registers. To set the bit from '0' to '1' the test mode key has to be written as defined in \\[01\\]Section 4.3.3 Lock Register (LCK). The unlock sequence is not required when WRTEN is kept at '1' while other bits of the register are changed. The bit can be reset to '0' at any time. 1 = Write access to test registers enabled 0 = Write access to test registers disabled"]
pub type WRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, WRTEN_A, O>;
impl<'a, const O: u8> WRTEN_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn test_reg_wr_disabled(self) -> &'a mut W {
        self.variant(WRTEN_A::TEST_REG_WR_DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn test_reg_wr_enabled(self) -> &'a mut W {
        self.variant(WRTEN_A::TEST_REG_WR_ENABLED)
    }
}
#[doc = "Field `ELBE` reader - External Loop Back Enable There are two possibilities to perform a Loop Back test. External Loop Back via physical layer or internal Loop Back for in-system self-test (default). In case of an internal Loop Back pins eray_txen1,2_n are in their inactive state, pins eray_txd1,2 are set to HIGH, pins eray_rxd1,2 are not evaluated. Bit ELBE is evaluated only when POC is in Loop Back Mode and test multiplexer control is in non-multiplexing mode TMC\\[1:0\\]
= '00'. 1 = External Loop Back 0 = Internal Loop Back (default)"]
pub type ELBE_R = crate::BitReader<ELBE_A>;
#[doc = "External Loop Back Enable There are two possibilities to perform a Loop Back test. External Loop Back via physical layer or internal Loop Back for in-system self-test (default). In case of an internal Loop Back pins eray_txen1,2_n are in their inactive state, pins eray_txd1,2 are set to HIGH, pins eray_rxd1,2 are not evaluated. Bit ELBE is evaluated only when POC is in Loop Back Mode and test multiplexer control is in non-multiplexing mode TMC\\[1:0\\]
= '00'. 1 = External Loop Back 0 = Internal Loop Back (default)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELBE_A {
    #[doc = "0: N/A"]
    INTERNAL_LOOP_BACK = 0,
    #[doc = "1: N/A"]
    EXTERNAL_LOOP_BACK = 1,
}
impl From<ELBE_A> for bool {
    #[inline(always)]
    fn from(variant: ELBE_A) -> Self {
        variant as u8 != 0
    }
}
impl ELBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELBE_A {
        match self.bits {
            false => ELBE_A::INTERNAL_LOOP_BACK,
            true => ELBE_A::EXTERNAL_LOOP_BACK,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL_LOOP_BACK`"]
    #[inline(always)]
    pub fn is_internal_loop_back(&self) -> bool {
        *self == ELBE_A::INTERNAL_LOOP_BACK
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_LOOP_BACK`"]
    #[inline(always)]
    pub fn is_external_loop_back(&self) -> bool {
        *self == ELBE_A::EXTERNAL_LOOP_BACK
    }
}
#[doc = "Field `ELBE` writer - External Loop Back Enable There are two possibilities to perform a Loop Back test. External Loop Back via physical layer or internal Loop Back for in-system self-test (default). In case of an internal Loop Back pins eray_txen1,2_n are in their inactive state, pins eray_txd1,2 are set to HIGH, pins eray_rxd1,2 are not evaluated. Bit ELBE is evaluated only when POC is in Loop Back Mode and test multiplexer control is in non-multiplexing mode TMC\\[1:0\\]
= '00'. 1 = External Loop Back 0 = Internal Loop Back (default)"]
pub type ELBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, ELBE_A, O>;
impl<'a, const O: u8> ELBE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn internal_loop_back(self) -> &'a mut W {
        self.variant(ELBE_A::INTERNAL_LOOP_BACK)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn external_loop_back(self) -> &'a mut W {
        self.variant(ELBE_A::EXTERNAL_LOOP_BACK)
    }
}
#[doc = "Field `TMC` reader - Test Multiplexer Control 00, 11= Normal signal path (default) 01 = RAM Test Mode - Internal busses are multiplexed to make all RAM blocks of the ERay module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing. 10 = I/O Test Mode - Output pins eray_txd1, eray_txd2, eray_txen1_n, eray_txen2_n, are driven to the values defined by bits TXA, TXB, TXENA, TXENB. The values applied to the input pins eray_rxd1, eray_rxd2 can be read from register bits RXA, RXB."]
pub type TMC_R = crate::FieldReader<u8, TMC_A>;
#[doc = "Test Multiplexer Control 00, 11= Normal signal path (default) 01 = RAM Test Mode - Internal busses are multiplexed to make all RAM blocks of the ERay module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing. 10 = I/O Test Mode - Output pins eray_txd1, eray_txd2, eray_txen1_n, eray_txen2_n, are driven to the values defined by bits TXA, TXB, TXENA, TXENB. The values applied to the input pins eray_rxd1, eray_rxd2 can be read from register bits RXA, RXB.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TMC_A {
    #[doc = "0: N/A"]
    NORMAL_SIGNAL_PATH = 0,
    #[doc = "3: N/A"]
    NORMAL_SIGNAL_PATH_MIRROR = 3,
    #[doc = "1: N/A"]
    RAM_TEST_MODE = 1,
    #[doc = "2: N/A"]
    IO_TEST_MODE = 2,
}
impl From<TMC_A> for u8 {
    #[inline(always)]
    fn from(variant: TMC_A) -> Self {
        variant as _
    }
}
impl TMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMC_A {
        match self.bits {
            0 => TMC_A::NORMAL_SIGNAL_PATH,
            3 => TMC_A::NORMAL_SIGNAL_PATH_MIRROR,
            1 => TMC_A::RAM_TEST_MODE,
            2 => TMC_A::IO_TEST_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_SIGNAL_PATH`"]
    #[inline(always)]
    pub fn is_normal_signal_path(&self) -> bool {
        *self == TMC_A::NORMAL_SIGNAL_PATH
    }
    #[doc = "Checks if the value of the field is `NORMAL_SIGNAL_PATH_MIRROR`"]
    #[inline(always)]
    pub fn is_normal_signal_path_mirror(&self) -> bool {
        *self == TMC_A::NORMAL_SIGNAL_PATH_MIRROR
    }
    #[doc = "Checks if the value of the field is `RAM_TEST_MODE`"]
    #[inline(always)]
    pub fn is_ram_test_mode(&self) -> bool {
        *self == TMC_A::RAM_TEST_MODE
    }
    #[doc = "Checks if the value of the field is `IO_TEST_MODE`"]
    #[inline(always)]
    pub fn is_io_test_mode(&self) -> bool {
        *self == TMC_A::IO_TEST_MODE
    }
}
#[doc = "Field `TMC` writer - Test Multiplexer Control 00, 11= Normal signal path (default) 01 = RAM Test Mode - Internal busses are multiplexed to make all RAM blocks of the ERay module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing. 10 = I/O Test Mode - Output pins eray_txd1, eray_txd2, eray_txen1_n, eray_txen2_n, are driven to the values defined by bits TXA, TXB, TXENA, TXENB. The values applied to the input pins eray_rxd1, eray_rxd2 can be read from register bits RXA, RXB."]
pub type TMC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TEST1_SPEC, u8, TMC_A, 2, O>;
impl<'a, const O: u8> TMC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn normal_signal_path(self) -> &'a mut W {
        self.variant(TMC_A::NORMAL_SIGNAL_PATH)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn normal_signal_path_mirror(self) -> &'a mut W {
        self.variant(TMC_A::NORMAL_SIGNAL_PATH_MIRROR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn ram_test_mode(self) -> &'a mut W {
        self.variant(TMC_A::RAM_TEST_MODE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn io_test_mode(self) -> &'a mut W {
        self.variant(TMC_A::IO_TEST_MODE)
    }
}
#[doc = "Field `AOA` reader - Activity on A AOA is set when there is activity on channel A. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOA is the inverse of zChannelIdle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOA should be ignored in all other POC states. 1 = Activity detected, channel A not idle 0 = No activity detected, channel A idle"]
pub type AOA_R = crate::BitReader<AOA_A>;
#[doc = "Activity on A AOA is set when there is activity on channel A. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOA is the inverse of zChannelIdle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOA should be ignored in all other POC states. 1 = Activity detected, channel A not idle 0 = No activity detected, channel A idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOA_A {
    #[doc = "0: N/A"]
    NOACT_DETECT_CH_A_IDLE = 0,
    #[doc = "1: N/A"]
    ACT_DETECT_CH_A_NOT_IDLE = 1,
}
impl From<AOA_A> for bool {
    #[inline(always)]
    fn from(variant: AOA_A) -> Self {
        variant as u8 != 0
    }
}
impl AOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOA_A {
        match self.bits {
            false => AOA_A::NOACT_DETECT_CH_A_IDLE,
            true => AOA_A::ACT_DETECT_CH_A_NOT_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT_DETECT_CH_A_IDLE`"]
    #[inline(always)]
    pub fn is_noact_detect_ch_a_idle(&self) -> bool {
        *self == AOA_A::NOACT_DETECT_CH_A_IDLE
    }
    #[doc = "Checks if the value of the field is `ACT_DETECT_CH_A_NOT_IDLE`"]
    #[inline(always)]
    pub fn is_act_detect_ch_a_not_idle(&self) -> bool {
        *self == AOA_A::ACT_DETECT_CH_A_NOT_IDLE
    }
}
#[doc = "Field `AOB` reader - Activity on B AOB is set when there is activity on channel B. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOB is the inverse of zChannel- Idle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOB should be ignored in all other POC states. 1 = Activity detected, channel B not idle 0 = No activity detected, channel B idle"]
pub type AOB_R = crate::BitReader<AOB_A>;
#[doc = "Activity on B AOB is set when there is activity on channel B. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOB is the inverse of zChannel- Idle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOB should be ignored in all other POC states. 1 = Activity detected, channel B not idle 0 = No activity detected, channel B idle\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AOB_A {
    #[doc = "0: N/A"]
    NOACT_DETECT_CH_B_IDLE = 0,
    #[doc = "1: N/A"]
    ACT_DETECT_CH_B_NOT_IDLE = 1,
}
impl From<AOB_A> for bool {
    #[inline(always)]
    fn from(variant: AOB_A) -> Self {
        variant as u8 != 0
    }
}
impl AOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AOB_A {
        match self.bits {
            false => AOB_A::NOACT_DETECT_CH_B_IDLE,
            true => AOB_A::ACT_DETECT_CH_B_NOT_IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `NOACT_DETECT_CH_B_IDLE`"]
    #[inline(always)]
    pub fn is_noact_detect_ch_b_idle(&self) -> bool {
        *self == AOB_A::NOACT_DETECT_CH_B_IDLE
    }
    #[doc = "Checks if the value of the field is `ACT_DETECT_CH_B_NOT_IDLE`"]
    #[inline(always)]
    pub fn is_act_detect_ch_b_not_idle(&self) -> bool {
        *self == AOB_A::ACT_DETECT_CH_B_NOT_IDLE
    }
}
#[doc = "Field `RXA` reader - Monitor Channel A Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd1 = '0' 1 = eray_rxd1 = '1'"]
pub type RXA_R = crate::BitReader<RXA_A>;
#[doc = "Monitor Channel A Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd1 = '0' 1 = eray_rxd1 = '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXA_A {
    #[doc = "0: N/A"]
    ERAY_RXD1_0 = 0,
    #[doc = "1: N/A"]
    ERAY_RXD1_1 = 1,
}
impl From<RXA_A> for bool {
    #[inline(always)]
    fn from(variant: RXA_A) -> Self {
        variant as u8 != 0
    }
}
impl RXA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXA_A {
        match self.bits {
            false => RXA_A::ERAY_RXD1_0,
            true => RXA_A::ERAY_RXD1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_RXD1_0`"]
    #[inline(always)]
    pub fn is_eray_rxd1_0(&self) -> bool {
        *self == RXA_A::ERAY_RXD1_0
    }
    #[doc = "Checks if the value of the field is `ERAY_RXD1_1`"]
    #[inline(always)]
    pub fn is_eray_rxd1_1(&self) -> bool {
        *self == RXA_A::ERAY_RXD1_1
    }
}
#[doc = "Field `RXB` reader - Monitor Channel B Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd2 = '0' 1 = eray_rxd2 = '1'"]
pub type RXB_R = crate::BitReader<RXB_A>;
#[doc = "Monitor Channel B Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd2 = '0' 1 = eray_rxd2 = '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXB_A {
    #[doc = "0: N/A"]
    ERAY_RXD2_0 = 0,
    #[doc = "1: N/A"]
    ERAY_RXD2_1 = 1,
}
impl From<RXB_A> for bool {
    #[inline(always)]
    fn from(variant: RXB_A) -> Self {
        variant as u8 != 0
    }
}
impl RXB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXB_A {
        match self.bits {
            false => RXB_A::ERAY_RXD2_0,
            true => RXB_A::ERAY_RXD2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_RXD2_0`"]
    #[inline(always)]
    pub fn is_eray_rxd2_0(&self) -> bool {
        *self == RXB_A::ERAY_RXD2_0
    }
    #[doc = "Checks if the value of the field is `ERAY_RXD2_1`"]
    #[inline(always)]
    pub fn is_eray_rxd2_1(&self) -> bool {
        *self == RXB_A::ERAY_RXD2_1
    }
}
#[doc = "Field `TXA` reader - Control of Channel A Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd1 pin drives a '0' 1 = eray_txd1 pin drives a '1'"]
pub type TXA_R = crate::BitReader<TXA_A>;
#[doc = "Control of Channel A Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd1 pin drives a '0' 1 = eray_txd1 pin drives a '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXA_A {
    #[doc = "0: N/A"]
    ERAY_TXD1_0 = 0,
    #[doc = "1: N/A"]
    ERAY_TXD1_1 = 1,
}
impl From<TXA_A> for bool {
    #[inline(always)]
    fn from(variant: TXA_A) -> Self {
        variant as u8 != 0
    }
}
impl TXA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXA_A {
        match self.bits {
            false => TXA_A::ERAY_TXD1_0,
            true => TXA_A::ERAY_TXD1_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_TXD1_0`"]
    #[inline(always)]
    pub fn is_eray_txd1_0(&self) -> bool {
        *self == TXA_A::ERAY_TXD1_0
    }
    #[doc = "Checks if the value of the field is `ERAY_TXD1_1`"]
    #[inline(always)]
    pub fn is_eray_txd1_1(&self) -> bool {
        *self == TXA_A::ERAY_TXD1_1
    }
}
#[doc = "Field `TXA` writer - Control of Channel A Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd1 pin drives a '0' 1 = eray_txd1 pin drives a '1'"]
pub type TXA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, TXA_A, O>;
impl<'a, const O: u8> TXA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txd1_0(self) -> &'a mut W {
        self.variant(TXA_A::ERAY_TXD1_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txd1_1(self) -> &'a mut W {
        self.variant(TXA_A::ERAY_TXD1_1)
    }
}
#[doc = "Field `TXB` reader - Control of Channel B Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd2 pin drives a '0' 1 = eray_txd2 pin drives a '1'"]
pub type TXB_R = crate::BitReader<TXB_A>;
#[doc = "Control of Channel B Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd2 pin drives a '0' 1 = eray_txd2 pin drives a '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXB_A {
    #[doc = "0: N/A"]
    ERAY_TXD2_0 = 0,
    #[doc = "1: N/A"]
    ERAY_TXD2_1 = 1,
}
impl From<TXB_A> for bool {
    #[inline(always)]
    fn from(variant: TXB_A) -> Self {
        variant as u8 != 0
    }
}
impl TXB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXB_A {
        match self.bits {
            false => TXB_A::ERAY_TXD2_0,
            true => TXB_A::ERAY_TXD2_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_TXD2_0`"]
    #[inline(always)]
    pub fn is_eray_txd2_0(&self) -> bool {
        *self == TXB_A::ERAY_TXD2_0
    }
    #[doc = "Checks if the value of the field is `ERAY_TXD2_1`"]
    #[inline(always)]
    pub fn is_eray_txd2_1(&self) -> bool {
        *self == TXB_A::ERAY_TXD2_1
    }
}
#[doc = "Field `TXB` writer - Control of Channel B Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd2 pin drives a '0' 1 = eray_txd2 pin drives a '1'"]
pub type TXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, TXB_A, O>;
impl<'a, const O: u8> TXB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txd2_0(self) -> &'a mut W {
        self.variant(TXB_A::ERAY_TXD2_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txd2_1(self) -> &'a mut W {
        self.variant(TXB_A::ERAY_TXD2_1)
    }
}
#[doc = "Field `TXENA` reader - Control of Channel A Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen1_n pin drives a '0' 1 = eray_txen1_n pin drives a '1'"]
pub type TXENA_R = crate::BitReader<TXENA_A>;
#[doc = "Control of Channel A Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen1_n pin drives a '0' 1 = eray_txen1_n pin drives a '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENA_A {
    #[doc = "0: N/A"]
    ERAY_TXEN1_N_0 = 0,
    #[doc = "1: N/A"]
    ERAY_TXEN1_N_1 = 1,
}
impl From<TXENA_A> for bool {
    #[inline(always)]
    fn from(variant: TXENA_A) -> Self {
        variant as u8 != 0
    }
}
impl TXENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENA_A {
        match self.bits {
            false => TXENA_A::ERAY_TXEN1_N_0,
            true => TXENA_A::ERAY_TXEN1_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_TXEN1_N_0`"]
    #[inline(always)]
    pub fn is_eray_txen1_n_0(&self) -> bool {
        *self == TXENA_A::ERAY_TXEN1_N_0
    }
    #[doc = "Checks if the value of the field is `ERAY_TXEN1_N_1`"]
    #[inline(always)]
    pub fn is_eray_txen1_n_1(&self) -> bool {
        *self == TXENA_A::ERAY_TXEN1_N_1
    }
}
#[doc = "Field `TXENA` writer - Control of Channel A Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen1_n pin drives a '0' 1 = eray_txen1_n pin drives a '1'"]
pub type TXENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, TXENA_A, O>;
impl<'a, const O: u8> TXENA_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txen1_n_0(self) -> &'a mut W {
        self.variant(TXENA_A::ERAY_TXEN1_N_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txen1_n_1(self) -> &'a mut W {
        self.variant(TXENA_A::ERAY_TXEN1_N_1)
    }
}
#[doc = "Field `TXENB` reader - Control of Channel B Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen2_n pin drives a '0' 1 = eray_txen2_n pin drives a '1'"]
pub type TXENB_R = crate::BitReader<TXENB_A>;
#[doc = "Control of Channel B Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen2_n pin drives a '0' 1 = eray_txen2_n pin drives a '1'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXENB_A {
    #[doc = "0: N/A"]
    ERAY_TXEN2_N_0 = 0,
    #[doc = "1: N/A"]
    ERAY_TXEN2_N_1 = 1,
}
impl From<TXENB_A> for bool {
    #[inline(always)]
    fn from(variant: TXENB_A) -> Self {
        variant as u8 != 0
    }
}
impl TXENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXENB_A {
        match self.bits {
            false => TXENB_A::ERAY_TXEN2_N_0,
            true => TXENB_A::ERAY_TXEN2_N_1,
        }
    }
    #[doc = "Checks if the value of the field is `ERAY_TXEN2_N_0`"]
    #[inline(always)]
    pub fn is_eray_txen2_n_0(&self) -> bool {
        *self == TXENB_A::ERAY_TXEN2_N_0
    }
    #[doc = "Checks if the value of the field is `ERAY_TXEN2_N_1`"]
    #[inline(always)]
    pub fn is_eray_txen2_n_1(&self) -> bool {
        *self == TXENB_A::ERAY_TXEN2_N_1
    }
}
#[doc = "Field `TXENB` writer - Control of Channel B Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen2_n pin drives a '0' 1 = eray_txen2_n pin drives a '1'"]
pub type TXENB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST1_SPEC, TXENB_A, O>;
impl<'a, const O: u8> TXENB_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txen2_n_0(self) -> &'a mut W {
        self.variant(TXENB_A::ERAY_TXEN2_N_0)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn eray_txen2_n_1(self) -> &'a mut W {
        self.variant(TXENB_A::ERAY_TXEN2_N_1)
    }
}
#[doc = "Field `CERA` reader - N/A"]
pub type CERA_R = crate::FieldReader<u8, CERA_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CERA_A {
    #[doc = "0: N/A"]
    NO_CODING_ERR = 0,
    #[doc = "1: N/A"]
    HEADER_CRC_ERR = 1,
    #[doc = "2: N/A"]
    FRAME_CRC_ERR = 2,
    #[doc = "3: N/A"]
    FRAME_START_SEQ_FSS_LONG = 3,
    #[doc = "4: N/A"]
    FIRST_BIT_OF_BSS_LOW = 4,
    #[doc = "5: N/A"]
    SECOND_BIT_OF_BSS_HIGH = 5,
    #[doc = "6: N/A"]
    FIRST_BIT_OF_FES_HIGH = 6,
    #[doc = "7: N/A"]
    SECOND_BIT_OF_FES_LOW = 7,
    #[doc = "8: N/A"]
    CAS_MTS_SYMBOL_SHORT = 8,
    #[doc = "9: N/A"]
    CAS_MTS_SYMBOL_LONG = 9,
}
impl From<CERA_A> for u8 {
    #[inline(always)]
    fn from(variant: CERA_A) -> Self {
        variant as _
    }
}
impl CERA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CERA_A> {
        match self.bits {
            0 => Some(CERA_A::NO_CODING_ERR),
            1 => Some(CERA_A::HEADER_CRC_ERR),
            2 => Some(CERA_A::FRAME_CRC_ERR),
            3 => Some(CERA_A::FRAME_START_SEQ_FSS_LONG),
            4 => Some(CERA_A::FIRST_BIT_OF_BSS_LOW),
            5 => Some(CERA_A::SECOND_BIT_OF_BSS_HIGH),
            6 => Some(CERA_A::FIRST_BIT_OF_FES_HIGH),
            7 => Some(CERA_A::SECOND_BIT_OF_FES_LOW),
            8 => Some(CERA_A::CAS_MTS_SYMBOL_SHORT),
            9 => Some(CERA_A::CAS_MTS_SYMBOL_LONG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CODING_ERR`"]
    #[inline(always)]
    pub fn is_no_coding_err(&self) -> bool {
        *self == CERA_A::NO_CODING_ERR
    }
    #[doc = "Checks if the value of the field is `HEADER_CRC_ERR`"]
    #[inline(always)]
    pub fn is_header_crc_err(&self) -> bool {
        *self == CERA_A::HEADER_CRC_ERR
    }
    #[doc = "Checks if the value of the field is `FRAME_CRC_ERR`"]
    #[inline(always)]
    pub fn is_frame_crc_err(&self) -> bool {
        *self == CERA_A::FRAME_CRC_ERR
    }
    #[doc = "Checks if the value of the field is `FRAME_START_SEQ_FSS_LONG`"]
    #[inline(always)]
    pub fn is_frame_start_seq_fss_long(&self) -> bool {
        *self == CERA_A::FRAME_START_SEQ_FSS_LONG
    }
    #[doc = "Checks if the value of the field is `FIRST_BIT_OF_BSS_LOW`"]
    #[inline(always)]
    pub fn is_first_bit_of_bss_low(&self) -> bool {
        *self == CERA_A::FIRST_BIT_OF_BSS_LOW
    }
    #[doc = "Checks if the value of the field is `SECOND_BIT_OF_BSS_HIGH`"]
    #[inline(always)]
    pub fn is_second_bit_of_bss_high(&self) -> bool {
        *self == CERA_A::SECOND_BIT_OF_BSS_HIGH
    }
    #[doc = "Checks if the value of the field is `FIRST_BIT_OF_FES_HIGH`"]
    #[inline(always)]
    pub fn is_first_bit_of_fes_high(&self) -> bool {
        *self == CERA_A::FIRST_BIT_OF_FES_HIGH
    }
    #[doc = "Checks if the value of the field is `SECOND_BIT_OF_FES_LOW`"]
    #[inline(always)]
    pub fn is_second_bit_of_fes_low(&self) -> bool {
        *self == CERA_A::SECOND_BIT_OF_FES_LOW
    }
    #[doc = "Checks if the value of the field is `CAS_MTS_SYMBOL_SHORT`"]
    #[inline(always)]
    pub fn is_cas_mts_symbol_short(&self) -> bool {
        *self == CERA_A::CAS_MTS_SYMBOL_SHORT
    }
    #[doc = "Checks if the value of the field is `CAS_MTS_SYMBOL_LONG`"]
    #[inline(always)]
    pub fn is_cas_mts_symbol_long(&self) -> bool {
        *self == CERA_A::CAS_MTS_SYMBOL_LONG
    }
}
#[doc = "Field `CERB` reader - N/A"]
pub type CERB_R = crate::FieldReader<u8, CERB_A>;
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CERB_A {
    #[doc = "0: N/A"]
    NO_CODING_ERR = 0,
    #[doc = "1: N/A"]
    HEADER_CRC_ERR = 1,
    #[doc = "2: N/A"]
    FRAME_CRC_ERR = 2,
    #[doc = "3: N/A"]
    FRAME_START_SEQ_FSS_LONG = 3,
    #[doc = "4: N/A"]
    FIRST_BIT_OF_BSS_LOW = 4,
    #[doc = "5: N/A"]
    SECOND_BIT_OF_BSS_HIGH = 5,
    #[doc = "6: N/A"]
    FIRST_BIT_OF_FES_HIGH = 6,
    #[doc = "7: N/A"]
    SECOND_BIT_OF_FES_LOW = 7,
    #[doc = "8: N/A"]
    CAS_MTS_SYMBOL_SHORT = 8,
    #[doc = "9: N/A"]
    CAS_MTS_SYMBOL_LONG = 9,
}
impl From<CERB_A> for u8 {
    #[inline(always)]
    fn from(variant: CERB_A) -> Self {
        variant as _
    }
}
impl CERB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CERB_A> {
        match self.bits {
            0 => Some(CERB_A::NO_CODING_ERR),
            1 => Some(CERB_A::HEADER_CRC_ERR),
            2 => Some(CERB_A::FRAME_CRC_ERR),
            3 => Some(CERB_A::FRAME_START_SEQ_FSS_LONG),
            4 => Some(CERB_A::FIRST_BIT_OF_BSS_LOW),
            5 => Some(CERB_A::SECOND_BIT_OF_BSS_HIGH),
            6 => Some(CERB_A::FIRST_BIT_OF_FES_HIGH),
            7 => Some(CERB_A::SECOND_BIT_OF_FES_LOW),
            8 => Some(CERB_A::CAS_MTS_SYMBOL_SHORT),
            9 => Some(CERB_A::CAS_MTS_SYMBOL_LONG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CODING_ERR`"]
    #[inline(always)]
    pub fn is_no_coding_err(&self) -> bool {
        *self == CERB_A::NO_CODING_ERR
    }
    #[doc = "Checks if the value of the field is `HEADER_CRC_ERR`"]
    #[inline(always)]
    pub fn is_header_crc_err(&self) -> bool {
        *self == CERB_A::HEADER_CRC_ERR
    }
    #[doc = "Checks if the value of the field is `FRAME_CRC_ERR`"]
    #[inline(always)]
    pub fn is_frame_crc_err(&self) -> bool {
        *self == CERB_A::FRAME_CRC_ERR
    }
    #[doc = "Checks if the value of the field is `FRAME_START_SEQ_FSS_LONG`"]
    #[inline(always)]
    pub fn is_frame_start_seq_fss_long(&self) -> bool {
        *self == CERB_A::FRAME_START_SEQ_FSS_LONG
    }
    #[doc = "Checks if the value of the field is `FIRST_BIT_OF_BSS_LOW`"]
    #[inline(always)]
    pub fn is_first_bit_of_bss_low(&self) -> bool {
        *self == CERB_A::FIRST_BIT_OF_BSS_LOW
    }
    #[doc = "Checks if the value of the field is `SECOND_BIT_OF_BSS_HIGH`"]
    #[inline(always)]
    pub fn is_second_bit_of_bss_high(&self) -> bool {
        *self == CERB_A::SECOND_BIT_OF_BSS_HIGH
    }
    #[doc = "Checks if the value of the field is `FIRST_BIT_OF_FES_HIGH`"]
    #[inline(always)]
    pub fn is_first_bit_of_fes_high(&self) -> bool {
        *self == CERB_A::FIRST_BIT_OF_FES_HIGH
    }
    #[doc = "Checks if the value of the field is `SECOND_BIT_OF_FES_LOW`"]
    #[inline(always)]
    pub fn is_second_bit_of_fes_low(&self) -> bool {
        *self == CERB_A::SECOND_BIT_OF_FES_LOW
    }
    #[doc = "Checks if the value of the field is `CAS_MTS_SYMBOL_SHORT`"]
    #[inline(always)]
    pub fn is_cas_mts_symbol_short(&self) -> bool {
        *self == CERB_A::CAS_MTS_SYMBOL_SHORT
    }
    #[doc = "Checks if the value of the field is `CAS_MTS_SYMBOL_LONG`"]
    #[inline(always)]
    pub fn is_cas_mts_symbol_long(&self) -> bool {
        *self == CERB_A::CAS_MTS_SYMBOL_LONG
    }
}
impl R {
    #[doc = "Bit 0 - Write Test Register Enable Enables write access to the test registers. To set the bit from '0' to '1' the test mode key has to be written as defined in \\[01\\]Section 4.3.3 Lock Register (LCK). The unlock sequence is not required when WRTEN is kept at '1' while other bits of the register are changed. The bit can be reset to '0' at any time. 1 = Write access to test registers enabled 0 = Write access to test registers disabled"]
    #[inline(always)]
    pub fn wrten(&self) -> WRTEN_R {
        WRTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Loop Back Enable There are two possibilities to perform a Loop Back test. External Loop Back via physical layer or internal Loop Back for in-system self-test (default). In case of an internal Loop Back pins eray_txen1,2_n are in their inactive state, pins eray_txd1,2 are set to HIGH, pins eray_rxd1,2 are not evaluated. Bit ELBE is evaluated only when POC is in Loop Back Mode and test multiplexer control is in non-multiplexing mode TMC\\[1:0\\]
= '00'. 1 = External Loop Back 0 = Internal Loop Back (default)"]
    #[inline(always)]
    pub fn elbe(&self) -> ELBE_R {
        ELBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Test Multiplexer Control 00, 11= Normal signal path (default) 01 = RAM Test Mode - Internal busses are multiplexed to make all RAM blocks of the ERay module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing. 10 = I/O Test Mode - Output pins eray_txd1, eray_txd2, eray_txen1_n, eray_txen2_n, are driven to the values defined by bits TXA, TXB, TXENA, TXENB. The values applied to the input pins eray_rxd1, eray_rxd2 can be read from register bits RXA, RXB."]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Activity on A AOA is set when there is activity on channel A. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOA is the inverse of zChannelIdle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOA should be ignored in all other POC states. 1 = Activity detected, channel A not idle 0 = No activity detected, channel A idle"]
    #[inline(always)]
    pub fn aoa(&self) -> AOA_R {
        AOA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Activity on B AOB is set when there is activity on channel B. It is reset when 11 consecutive '1' bits are detected or if the POC state is DEFAULT_CONFIG or CONFIG. During STARTUP, NORMAL_ACTIVE, or NORMAL_PASSIVE the function of AOB is the inverse of zChannel- Idle as specified in the FlexRay protocol spec v2.1, chapter 3, BITSTRB process. AOB should be ignored in all other POC states. 1 = Activity detected, channel B not idle 0 = No activity detected, channel B idle"]
    #[inline(always)]
    pub fn aob(&self) -> AOB_R {
        AOB_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Monitor Channel A Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd1 = '0' 1 = eray_rxd1 = '1'"]
    #[inline(always)]
    pub fn rxa(&self) -> RXA_R {
        RXA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Monitor Channel B Receive Pin This is used to test the interface to the physical layer (connectivity test) by reading the pin. 0 = eray_rxd2 = '0' 1 = eray_rxd2 = '1'"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Control of Channel A Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd1 pin drives a '0' 1 = eray_txd1 pin drives a '1'"]
    #[inline(always)]
    pub fn txa(&self) -> TXA_R {
        TXA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Control of Channel B Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd2 pin drives a '0' 1 = eray_txd2 pin drives a '1'"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Control of Channel A Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen1_n pin drives a '0' 1 = eray_txen1_n pin drives a '1'"]
    #[inline(always)]
    pub fn txena(&self) -> TXENA_R {
        TXENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Control of Channel B Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen2_n pin drives a '0' 1 = eray_txen2_n pin drives a '1'"]
    #[inline(always)]
    pub fn txenb(&self) -> TXENB_R {
        TXENB_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:27 - N/A"]
    #[inline(always)]
    pub fn cera(&self) -> CERA_R {
        CERA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - N/A"]
    #[inline(always)]
    pub fn cerb(&self) -> CERB_R {
        CERB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Write Test Register Enable Enables write access to the test registers. To set the bit from '0' to '1' the test mode key has to be written as defined in \\[01\\]Section 4.3.3 Lock Register (LCK). The unlock sequence is not required when WRTEN is kept at '1' while other bits of the register are changed. The bit can be reset to '0' at any time. 1 = Write access to test registers enabled 0 = Write access to test registers disabled"]
    #[inline(always)]
    #[must_use]
    pub fn wrten(&mut self) -> WRTEN_W<0> {
        WRTEN_W::new(self)
    }
    #[doc = "Bit 1 - External Loop Back Enable There are two possibilities to perform a Loop Back test. External Loop Back via physical layer or internal Loop Back for in-system self-test (default). In case of an internal Loop Back pins eray_txen1,2_n are in their inactive state, pins eray_txd1,2 are set to HIGH, pins eray_rxd1,2 are not evaluated. Bit ELBE is evaluated only when POC is in Loop Back Mode and test multiplexer control is in non-multiplexing mode TMC\\[1:0\\]
= '00'. 1 = External Loop Back 0 = Internal Loop Back (default)"]
    #[inline(always)]
    #[must_use]
    pub fn elbe(&mut self) -> ELBE_W<1> {
        ELBE_W::new(self)
    }
    #[doc = "Bits 4:5 - Test Multiplexer Control 00, 11= Normal signal path (default) 01 = RAM Test Mode - Internal busses are multiplexed to make all RAM blocks of the ERay module directly accessible by the Host. This mode is intended to enable testing of the embedded RAM blocks during production testing. 10 = I/O Test Mode - Output pins eray_txd1, eray_txd2, eray_txen1_n, eray_txen2_n, are driven to the values defined by bits TXA, TXB, TXENA, TXENB. The values applied to the input pins eray_rxd1, eray_rxd2 can be read from register bits RXA, RXB."]
    #[inline(always)]
    #[must_use]
    pub fn tmc(&mut self) -> TMC_W<4> {
        TMC_W::new(self)
    }
    #[doc = "Bit 18 - Control of Channel A Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd1 pin drives a '0' 1 = eray_txd1 pin drives a '1'"]
    #[inline(always)]
    #[must_use]
    pub fn txa(&mut self) -> TXA_W<18> {
        TXA_W::new(self)
    }
    #[doc = "Bit 19 - Control of Channel B Transmit Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txd2 pin drives a '0' 1 = eray_txd2 pin drives a '1'"]
    #[inline(always)]
    #[must_use]
    pub fn txb(&mut self) -> TXB_W<19> {
        TXB_W::new(self)
    }
    #[doc = "Bit 20 - Control of Channel A Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen1_n pin drives a '0' 1 = eray_txen1_n pin drives a '1'"]
    #[inline(always)]
    #[must_use]
    pub fn txena(&mut self) -> TXENA_W<20> {
        TXENA_W::new(self)
    }
    #[doc = "Bit 21 - Control of Channel B Transmit Enable Pin This is used to test the interface to the physical layer (connectivity test) by driving the pin. 0 = eray_txen2_n pin drives a '0' 1 = eray_txen2_n pin drives a '1'"]
    #[inline(always)]
    #[must_use]
    pub fn txenb(&mut self) -> TXENB_W<21> {
        TXENB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test1](index.html) module"]
pub struct TEST1_SPEC;
impl crate::RegisterSpec for TEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test1::R](R) reader structure"]
impl crate::Readable for TEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test1::W](W) writer structure"]
impl crate::Writable for TEST1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST1 to value 0x0300"]
impl crate::Resettable for TEST1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
