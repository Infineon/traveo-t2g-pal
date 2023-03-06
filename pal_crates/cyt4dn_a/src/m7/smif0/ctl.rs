#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XIP_MODE` reader - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XIP_MODE_R = crate::BitReader<XIP_MODE_A>;
#[doc = "Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XIP_MODE_A {
    #[doc = "0: '0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    MMIO_MODE = 0,
    #[doc = "1: '1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    XIP_MODE = 1,
}
impl From<XIP_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl XIP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_MODE_A {
        match self.bits {
            false => XIP_MODE_A::MMIO_MODE,
            true => XIP_MODE_A::XIP_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MMIO_MODE`"]
    #[inline(always)]
    pub fn is_mmio_mode(&self) -> bool {
        *self == XIP_MODE_A::MMIO_MODE
    }
    #[doc = "Checks if the value of the field is `XIP_MODE`"]
    #[inline(always)]
    pub fn is_xip_mode(&self) -> bool {
        *self == XIP_MODE_A::XIP_MODE
    }
}
#[doc = "Field `XIP_MODE` writer - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
pub type XIP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, XIP_MODE_A, O>;
impl<'a, const O: u8> XIP_MODE_W<'a, O> {
    #[doc = "'0': MMIO mode. Individual MMIO accesses to TX and RX FIFOs are used to generate a sequence of SPI transfers. This mode of operation allows for large flexibility in terms of the SPI transfers that can be generated."]
    #[inline(always)]
    pub fn mmio_mode(self) -> &'a mut W {
        self.variant(XIP_MODE_A::MMIO_MODE)
    }
    #[doc = "'1': XIP mode. eXecute-In-Place mode: incoming read and write transfers over the AHB-Lite bus infrastructure are automatically translated in SPI transfers to read data from and write data to a device. This mode of operation allow for efficient device read and write operations. This mode is only supported in SPI_MODE."]
    #[inline(always)]
    pub fn xip_mode(self) -> &'a mut W {
        self.variant(XIP_MODE_A::XIP_MODE)
    }
}
#[doc = "Field `CLOCK_IF_TX_SEL` reader - Specifies device interface transmitter clock options. '0': SDR. Memory interface clock 'spihb_clk_out' is divided (by 2) interface clock 'clk_if', memory interface data signals are driven by divided (by 2) interface clock 'clk_if' with different phase than 'spihb_clk_out'. Results in driving memory interface data signals at falling 'spihb_clk_out' edge. '1': DDR. Memory interface clock 'spihb_clk_out' is divided (by 2) inverted interface clock 'clk_if_inv', memory interface data signals are driven with interface clock 'clk_if'. Results in driving memory interface data signals 90 degrees before rising and falling 'spihb_clk_out' edge."]
pub type CLOCK_IF_TX_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_IF_TX_SEL` writer - Specifies device interface transmitter clock options. '0': SDR. Memory interface clock 'spihb_clk_out' is divided (by 2) interface clock 'clk_if', memory interface data signals are driven by divided (by 2) interface clock 'clk_if' with different phase than 'spihb_clk_out'. Results in driving memory interface data signals at falling 'spihb_clk_out' edge. '1': DDR. Memory interface clock 'spihb_clk_out' is divided (by 2) inverted interface clock 'clk_if_inv', memory interface data signals are driven with interface clock 'clk_if'. Results in driving memory interface data signals 90 degrees before rising and falling 'spihb_clk_out' edge."]
pub type CLOCK_IF_TX_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DELAY_LINE_SEL` reader - Specifies the delay line used for RX data capturing with - output / feedback clock based capturing (when CLOCK_IF_RX_SEL = \\[0..3\\]
and DELAY_TAP_ENABLED = 1) - internal clock based capturing (when CLOCK_IF_RX_SEL = \\[4..5\\], INT_CLOCK_CAPTURE_PRESENT = 1 and DELAY_TAP_ENABLED = 1) - RWDS based capturing (when CLOCK_IF_RX_SEL = \\[6..7\\])"]
pub type DELAY_LINE_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELAY_LINE_SEL` writer - Specifies the delay line used for RX data capturing with - output / feedback clock based capturing (when CLOCK_IF_RX_SEL = \\[0..3\\]
and DELAY_TAP_ENABLED = 1) - internal clock based capturing (when CLOCK_IF_RX_SEL = \\[4..5\\], INT_CLOCK_CAPTURE_PRESENT = 1 and DELAY_TAP_ENABLED = 1) - RWDS based capturing (when CLOCK_IF_RX_SEL = \\[6..7\\])"]
pub type DELAY_LINE_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DELAY_TAP_ENABLED` reader - Delay Line Tap Enable. '0': Disabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are not used. The read data is directly captured by the RX data FIFO capture clock as specified by CLOCK_IF_RX_SEL and INT_CLOCK_CAPTURE_CYCLE. '1': Enabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are used. Note that in RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]), DELAY_TAP_ENABLED must be set to '1' as the RWDS based capture scheme requires to use the delay line. If the output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) or the RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]) is selected then the register DELAY_TAP_SEL is used to select the capture clock. If the internal clock based capture scheme (CLOCK_IF_RX_SEL = \\[4..5\\]
and INT_CLOCK_CAPTURE_PRESENT = 1) is selected then the registers INT_CLOCK_DELAY_TAP_SEL0/1 are used to select the capture clock per data bit line (read data is captured by the capture logic and afterwards transferred to the RX data FIFO)."]
pub type DELAY_TAP_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `DELAY_TAP_ENABLED` writer - Delay Line Tap Enable. '0': Disabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are not used. The read data is directly captured by the RX data FIFO capture clock as specified by CLOCK_IF_RX_SEL and INT_CLOCK_CAPTURE_CYCLE. '1': Enabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are used. Note that in RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]), DELAY_TAP_ENABLED must be set to '1' as the RWDS based capture scheme requires to use the delay line. If the output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) or the RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]) is selected then the register DELAY_TAP_SEL is used to select the capture clock. If the internal clock based capture scheme (CLOCK_IF_RX_SEL = \\[4..5\\]
and INT_CLOCK_CAPTURE_PRESENT = 1) is selected then the registers INT_CLOCK_DELAY_TAP_SEL0/1 are used to select the capture clock per data bit line (read data is captured by the capture logic and afterwards transferred to the RX data FIFO)."]
pub type DELAY_TAP_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `INT_CLOCK_DL_ENABLED` reader - Data Learning Enable for internal RX clock based on Data Learning Pattern. Only applies when CLOCK_IF_RX_SEL = \\[4..5\\]
for selecting the internal clock based capture scheme and when DELAY_TAP_ENABLED = 1. '0': Disabled. The delay line tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are not modified by HW. '1': Enabled. The delay linle tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are modified by HW based on the data learning pattern. From all capture clock delay line taps producing a match between the expected data learning pattern in register DLP and the captured data learning pattern from the memory device the middle one is selected."]
pub type INT_CLOCK_DL_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `INT_CLOCK_DL_ENABLED` writer - Data Learning Enable for internal RX clock based on Data Learning Pattern. Only applies when CLOCK_IF_RX_SEL = \\[4..5\\]
for selecting the internal clock based capture scheme and when DELAY_TAP_ENABLED = 1. '0': Disabled. The delay line tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are not modified by HW. '1': Enabled. The delay linle tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are modified by HW based on the data learning pattern. From all capture clock delay line taps producing a match between the expected data learning pattern in register DLP and the captured data learning pattern from the memory device the middle one is selected."]
pub type INT_CLOCK_DL_ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `INT_CLOCK_CAPTURE_CYCLE` reader - N/A"]
pub type INT_CLOCK_CAPTURE_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT_CLOCK_CAPTURE_CYCLE` writer - N/A"]
pub type INT_CLOCK_CAPTURE_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLOCK_IF_RX_SEL` reader - N/A"]
pub type CLOCK_IF_RX_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLOCK_IF_RX_SEL` writer - N/A"]
pub type CLOCK_IF_RX_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DESELECT_DELAY` reader - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DESELECT_DELAY` writer - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SELECT_SETUP_DELAY` reader - Specifies the duration between 'spi_select_out\\[\\]' becomes low/'0') to 1st 'spi_clk_out' edge: '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1 memory interface clock cycle (= 2 clk_if cycle) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
pub type SELECT_SETUP_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT_SETUP_DELAY` writer - Specifies the duration between 'spi_select_out\\[\\]' becomes low/'0') to 1st 'spi_clk_out' edge: '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1 memory interface clock cycle (= 2 clk_if cycle) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
pub type SELECT_SETUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SELECT_HOLD_DELAY` reader - Specifies the duration between last 'spi_clk_out' edge to 'spi_select_out\\[\\]' becomes high/'1'): '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1/2 memory interface clock cycle (= 1 clk_if cycles) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
pub type SELECT_HOLD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT_HOLD_DELAY` writer - Specifies the duration between last 'spi_clk_out' edge to 'spi_select_out\\[\\]' becomes high/'1'): '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1/2 memory interface clock cycle (= 1 clk_if cycles) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
pub type SELECT_HOLD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLOCK` reader - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BLOCK_R = crate::BitReader<BLOCK_A>;
#[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCK_A {
    #[doc = "0: 0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BUS_ERROR = 0,
    #[doc = "1: 1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    WAIT_STATES = 1,
}
impl From<BLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: BLOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl BLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLOCK_A {
        match self.bits {
            false => BLOCK_A::BUS_ERROR,
            true => BLOCK_A::WAIT_STATES,
        }
    }
    #[doc = "Checks if the value of the field is `BUS_ERROR`"]
    #[inline(always)]
    pub fn is_bus_error(&self) -> bool {
        *self == BLOCK_A::BUS_ERROR
    }
    #[doc = "Checks if the value of the field is `WAIT_STATES`"]
    #[inline(always)]
    pub fn is_wait_states(&self) -> bool {
        *self == BLOCK_A::WAIT_STATES
    }
}
#[doc = "Field `BLOCK` writer - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
pub type BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, BLOCK_A, O>;
impl<'a, const O: u8> BLOCK_W<'a, O> {
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn bus_error(self) -> &'a mut W {
        self.variant(BLOCK_A::BUS_ERROR)
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency)."]
    #[inline(always)]
    pub fn wait_states(self) -> &'a mut W {
        self.variant(BLOCK_A::WAIT_STATES)
    }
}
#[doc = "Field `CLOCK_IF_SEL` reader - Specifies the clock source for 'clk_if'. Must be 0 (clk_hf) before entering DeepSleep; can be returned to 1 (clk_pll) afterwards. '0': clk_hf is used to create clk_if '1': clk_pll is used to create clk_if"]
pub type CLOCK_IF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_IF_SEL` writer - Specifies the clock source for 'clk_if'. Must be 0 (clk_hf) before entering DeepSleep; can be returned to 1 (clk_pll) afterwards. '0': clk_hf is used to create clk_if '1': clk_pll is used to create clk_if"]
pub type CLOCK_IF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Notes: - Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Notes: - Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: N/A"]
    DISABLED = 0,
    #[doc = "1: N/A"]
    ENABLED = 1,
}
impl From<ENABLED_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLED_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLED_A {
        match self.bits {
            false => ENABLED_A::DISABLED,
            true => ENABLED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLED_A::ENABLED
    }
}
#[doc = "Field `ENABLED` writer - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Notes: - Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, ENABLED_A, O>;
impl<'a, const O: u8> ENABLED_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    pub fn xip_mode(&self) -> XIP_MODE_R {
        XIP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Specifies device interface transmitter clock options. '0': SDR. Memory interface clock 'spihb_clk_out' is divided (by 2) interface clock 'clk_if', memory interface data signals are driven by divided (by 2) interface clock 'clk_if' with different phase than 'spihb_clk_out'. Results in driving memory interface data signals at falling 'spihb_clk_out' edge. '1': DDR. Memory interface clock 'spihb_clk_out' is divided (by 2) inverted interface clock 'clk_if_inv', memory interface data signals are driven with interface clock 'clk_if'. Results in driving memory interface data signals 90 degrees before rising and falling 'spihb_clk_out' edge."]
    #[inline(always)]
    pub fn clock_if_tx_sel(&self) -> CLOCK_IF_TX_SEL_R {
        CLOCK_IF_TX_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Specifies the delay line used for RX data capturing with - output / feedback clock based capturing (when CLOCK_IF_RX_SEL = \\[0..3\\]
and DELAY_TAP_ENABLED = 1) - internal clock based capturing (when CLOCK_IF_RX_SEL = \\[4..5\\], INT_CLOCK_CAPTURE_PRESENT = 1 and DELAY_TAP_ENABLED = 1) - RWDS based capturing (when CLOCK_IF_RX_SEL = \\[6..7\\])"]
    #[inline(always)]
    pub fn delay_line_sel(&self) -> DELAY_LINE_SEL_R {
        DELAY_LINE_SEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Delay Line Tap Enable. '0': Disabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are not used. The read data is directly captured by the RX data FIFO capture clock as specified by CLOCK_IF_RX_SEL and INT_CLOCK_CAPTURE_CYCLE. '1': Enabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are used. Note that in RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]), DELAY_TAP_ENABLED must be set to '1' as the RWDS based capture scheme requires to use the delay line. If the output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) or the RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]) is selected then the register DELAY_TAP_SEL is used to select the capture clock. If the internal clock based capture scheme (CLOCK_IF_RX_SEL = \\[4..5\\]
and INT_CLOCK_CAPTURE_PRESENT = 1) is selected then the registers INT_CLOCK_DELAY_TAP_SEL0/1 are used to select the capture clock per data bit line (read data is captured by the capture logic and afterwards transferred to the RX data FIFO)."]
    #[inline(always)]
    pub fn delay_tap_enabled(&self) -> DELAY_TAP_ENABLED_R {
        DELAY_TAP_ENABLED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Learning Enable for internal RX clock based on Data Learning Pattern. Only applies when CLOCK_IF_RX_SEL = \\[4..5\\]
for selecting the internal clock based capture scheme and when DELAY_TAP_ENABLED = 1. '0': Disabled. The delay line tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are not modified by HW. '1': Enabled. The delay linle tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are modified by HW based on the data learning pattern. From all capture clock delay line taps producing a match between the expected data learning pattern in register DLP and the captured data learning pattern from the memory device the middle one is selected."]
    #[inline(always)]
    pub fn int_clock_dl_enabled(&self) -> INT_CLOCK_DL_ENABLED_R {
        INT_CLOCK_DL_ENABLED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    pub fn int_clock_capture_cycle(&self) -> INT_CLOCK_CAPTURE_CYCLE_R {
        INT_CLOCK_CAPTURE_CYCLE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - N/A"]
    #[inline(always)]
    pub fn clock_if_rx_sel(&self) -> CLOCK_IF_RX_SEL_R {
        CLOCK_IF_RX_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&self) -> DESELECT_DELAY_R {
        DESELECT_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Specifies the duration between 'spi_select_out\\[\\]' becomes low/'0') to 1st 'spi_clk_out' edge: '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1 memory interface clock cycle (= 2 clk_if cycle) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
    #[inline(always)]
    pub fn select_setup_delay(&self) -> SELECT_SETUP_DELAY_R {
        SELECT_SETUP_DELAY_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Specifies the duration between last 'spi_clk_out' edge to 'spi_select_out\\[\\]' becomes high/'1'): '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1/2 memory interface clock cycle (= 1 clk_if cycles) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
    #[inline(always)]
    pub fn select_hold_delay(&self) -> SELECT_HOLD_DELAY_R {
        SELECT_HOLD_DELAY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Specifies the clock source for 'clk_if'. Must be 0 (clk_hf) before entering DeepSleep; can be returned to 1 (clk_pll) afterwards. '0': clk_hf is used to create clk_if '1': clk_pll is used to create clk_if"]
    #[inline(always)]
    pub fn clock_if_sel(&self) -> CLOCK_IF_SEL_R {
        CLOCK_IF_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Notes: - Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of operation. Note: this field should only be changed when the IP is disabled or when STATUS.BUSY is '0' and SW should not be executing from the XIP interface or MMIO interface."]
    #[inline(always)]
    #[must_use]
    pub fn xip_mode(&mut self) -> XIP_MODE_W<0> {
        XIP_MODE_W::new(self)
    }
    #[doc = "Bit 4 - Specifies device interface transmitter clock options. '0': SDR. Memory interface clock 'spihb_clk_out' is divided (by 2) interface clock 'clk_if', memory interface data signals are driven by divided (by 2) interface clock 'clk_if' with different phase than 'spihb_clk_out'. Results in driving memory interface data signals at falling 'spihb_clk_out' edge. '1': DDR. Memory interface clock 'spihb_clk_out' is divided (by 2) inverted interface clock 'clk_if_inv', memory interface data signals are driven with interface clock 'clk_if'. Results in driving memory interface data signals 90 degrees before rising and falling 'spihb_clk_out' edge."]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_tx_sel(&mut self) -> CLOCK_IF_TX_SEL_W<4> {
        CLOCK_IF_TX_SEL_W::new(self)
    }
    #[doc = "Bits 5:7 - Specifies the delay line used for RX data capturing with - output / feedback clock based capturing (when CLOCK_IF_RX_SEL = \\[0..3\\]
and DELAY_TAP_ENABLED = 1) - internal clock based capturing (when CLOCK_IF_RX_SEL = \\[4..5\\], INT_CLOCK_CAPTURE_PRESENT = 1 and DELAY_TAP_ENABLED = 1) - RWDS based capturing (when CLOCK_IF_RX_SEL = \\[6..7\\])"]
    #[inline(always)]
    #[must_use]
    pub fn delay_line_sel(&mut self) -> DELAY_LINE_SEL_W<5> {
        DELAY_LINE_SEL_W::new(self)
    }
    #[doc = "Bit 8 - Delay Line Tap Enable. '0': Disabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are not used. The read data is directly captured by the RX data FIFO capture clock as specified by CLOCK_IF_RX_SEL and INT_CLOCK_CAPTURE_CYCLE. '1': Enabled. The delay line tap selections specified in registers DELAY_TAP_SEL or INT_CLOCK_DELAY_TAP_SEL0/1 are used. Note that in RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]), DELAY_TAP_ENABLED must be set to '1' as the RWDS based capture scheme requires to use the delay line. If the output / feedback clock based capture scheme (CLOCK_IF_RX_SEL = \\[0..3\\]) or the RWDS based capture scheme (CLOCK_IF_RX_SEL = \\[6..7\\]) is selected then the register DELAY_TAP_SEL is used to select the capture clock. If the internal clock based capture scheme (CLOCK_IF_RX_SEL = \\[4..5\\]
and INT_CLOCK_CAPTURE_PRESENT = 1) is selected then the registers INT_CLOCK_DELAY_TAP_SEL0/1 are used to select the capture clock per data bit line (read data is captured by the capture logic and afterwards transferred to the RX data FIFO)."]
    #[inline(always)]
    #[must_use]
    pub fn delay_tap_enabled(&mut self) -> DELAY_TAP_ENABLED_W<8> {
        DELAY_TAP_ENABLED_W::new(self)
    }
    #[doc = "Bit 9 - Data Learning Enable for internal RX clock based on Data Learning Pattern. Only applies when CLOCK_IF_RX_SEL = \\[4..5\\]
for selecting the internal clock based capture scheme and when DELAY_TAP_ENABLED = 1. '0': Disabled. The delay line tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are not modified by HW. '1': Enabled. The delay linle tap selections specified in registers INT_CLOCK_DELAY_TAP_SEL0/1 are modified by HW based on the data learning pattern. From all capture clock delay line taps producing a match between the expected data learning pattern in register DLP and the captured data learning pattern from the memory device the middle one is selected."]
    #[inline(always)]
    #[must_use]
    pub fn int_clock_dl_enabled(&mut self) -> INT_CLOCK_DL_ENABLED_W<9> {
        INT_CLOCK_DL_ENABLED_W::new(self)
    }
    #[doc = "Bits 10:11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn int_clock_capture_cycle(&mut self) -> INT_CLOCK_CAPTURE_CYCLE_W<10> {
        INT_CLOCK_CAPTURE_CYCLE_W::new(self)
    }
    #[doc = "Bits 12:14 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_rx_sel(&mut self) -> CLOCK_IF_RX_SEL_W<12> {
        CLOCK_IF_RX_SEL_W::new(self)
    }
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    #[must_use]
    pub fn deselect_delay(&mut self) -> DESELECT_DELAY_W<16> {
        DESELECT_DELAY_W::new(self)
    }
    #[doc = "Bits 20:21 - Specifies the duration between 'spi_select_out\\[\\]' becomes low/'0') to 1st 'spi_clk_out' edge: '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1 memory interface clock cycle (= 2 clk_if cycle) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn select_setup_delay(&mut self) -> SELECT_SETUP_DELAY_W<20> {
        SELECT_SETUP_DELAY_W::new(self)
    }
    #[doc = "Bits 22:23 - Specifies the duration between last 'spi_clk_out' edge to 'spi_select_out\\[\\]' becomes high/'1'): '0': 0 memory interface clock cycles + min. duration (see below). '1': 1 memory interface clock cycle + min. duration (see below). '2': 2 memory interface clock cycles + min. duration (see below). '3': 3 memory interface clock cycles + min. duration (see below). In addition to the number of cycles selected here there is a min. duration of: - 1/2 memory interface clock cycle (= 1 clk_if cycles) for SDR timing (CLOCK_IF_TX_SEL = 0) - 1/4 memory interface clock cycle (= 1/2 clk_if cycle) for DDR timing (CLOCK_IF_TX_SEL = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn select_hold_delay(&mut self) -> SELECT_HOLD_DELAY_W<22> {
        SELECT_HOLD_DELAY_W::new(self)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. Note: the FIFOs can only be accessed in MMIO_MODE. This field is not used for test controller accesses."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<24> {
        BLOCK_W::new(self)
    }
    #[doc = "Bit 25 - Specifies the clock source for 'clk_if'. Must be 0 (clk_hf) before entering DeepSleep; can be returned to 1 (clk_pll) afterwards. '0': clk_hf is used to create clk_if '1': clk_pll is used to create clk_if"]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_sel(&mut self) -> CLOCK_IF_SEL_W<25> {
        CLOCK_IF_SEL_W::new(self)
    }
    #[doc = "Bit 31 - IP enable: '0': Disabled. All non-retention registers are reset to their default value when the IP is disabled. When the IP is disabled, the XIP accesses produce AHB-Lite bus errors. '1': Enabled. Notes: - Before disabling the IP, SW should ensure that the IP is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset."]
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
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0050_3400"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0050_3400;
}
