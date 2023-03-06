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
    #[doc = "1: ''1': Arbitration mode. Arbitrates XIP vs. MMIO accesses such that any ongoing, pending, or 'merged' transactions on XIP always take priority over any MMIO accesses to the device. However, if an MMIO access is in process it cannot be interrupted until it completes (as defined by sending the 'last command' via TX_CMD_MMIO_FIFO_WR). If MMIO accesses are not happening at all, though, then the XIP accesses endure no extra arbitration latency."]
    ARB_MODE = 1,
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
            true => XIP_MODE_A::ARB_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `MMIO_MODE`"]
    #[inline(always)]
    pub fn is_mmio_mode(&self) -> bool {
        *self == XIP_MODE_A::MMIO_MODE
    }
    #[doc = "Checks if the value of the field is `ARB_MODE`"]
    #[inline(always)]
    pub fn is_arb_mode(&self) -> bool {
        *self == XIP_MODE_A::ARB_MODE
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
    #[doc = "''1': Arbitration mode. Arbitrates XIP vs. MMIO accesses such that any ongoing, pending, or 'merged' transactions on XIP always take priority over any MMIO accesses to the device. However, if an MMIO access is in process it cannot be interrupted until it completes (as defined by sending the 'last command' via TX_CMD_MMIO_FIFO_WR). If MMIO accesses are not happening at all, though, then the XIP accesses endure no extra arbitration latency."]
    #[inline(always)]
    pub fn arb_mode(self) -> &'a mut W {
        self.variant(XIP_MODE_A::ARB_MODE)
    }
}
#[doc = "Field `DESELECT_DELAY` reader - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DESELECT_DELAY` writer - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
pub type DESELECT_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 3, O>;
#[doc = "Field `SELECT_SETUP_DELAY` reader - Specifies the duration between 'spi_select_out\\[\\]' activating (going low) to the first 'spi_clk_out' rising edge before considering any MDL delay due to CTL2.MDL_TAP_SEL. Subtract that delay from the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after subtracting the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 add 1 PLL clock cycle to these values. DDR or SDR mode is determined by the mode of the first phase of the transaction. '0': 2 PLL clock cycles '1': 2 PLL clock cycles + 1 memory interface clock cycles '2': 2 PLL clock cycles + 2 memory interface clock cycles '3': 2 PLL clock cycles + 3 memory interface clock cycles"]
pub type SELECT_SETUP_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT_SETUP_DELAY` writer - Specifies the duration between 'spi_select_out\\[\\]' activating (going low) to the first 'spi_clk_out' rising edge before considering any MDL delay due to CTL2.MDL_TAP_SEL. Subtract that delay from the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after subtracting the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 add 1 PLL clock cycle to these values. DDR or SDR mode is determined by the mode of the first phase of the transaction. '0': 2 PLL clock cycles '1': 2 PLL clock cycles + 1 memory interface clock cycles '2': 2 PLL clock cycles + 2 memory interface clock cycles '3': 2 PLL clock cycles + 3 memory interface clock cycles"]
pub type SELECT_SETUP_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SELECT_HOLD_DELAY` reader - Specifies the duration between the last 'spi_clk_out' falling edge and 'spi_select_out\\[\\]' deactivating (going high) before considering any MDL delay due to CTL2.MDL_TAP_SEL. Add that delay to the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after adding the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 subtract 1 PLL clock cycle from these values. DDR or SDR mode is determined by the mode of the last phase of the transaction. '0': 1 memory interface clock cycle + CTL2.CLKOUT_DIV adjustment '1': 2 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '2': 3 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '3': 4 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment For the following settings of CTL2.CLKOUT_DIV apply these adjustments to the above: 0 (DIV2): subtract 1 PLL clock cycle 1 (DIV4): no adjustment needed 2 (DIV8): add 2 PLL clock cycles 3 (DIV16): add 6 PLL clock cycles"]
pub type SELECT_HOLD_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELECT_HOLD_DELAY` writer - Specifies the duration between the last 'spi_clk_out' falling edge and 'spi_select_out\\[\\]' deactivating (going high) before considering any MDL delay due to CTL2.MDL_TAP_SEL. Add that delay to the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after adding the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 subtract 1 PLL clock cycle from these values. DDR or SDR mode is determined by the mode of the last phase of the transaction. '0': 1 memory interface clock cycle + CTL2.CLKOUT_DIV adjustment '1': 2 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '2': 3 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '3': 4 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment For the following settings of CTL2.CLKOUT_DIV apply these adjustments to the above: 0 (DIV2): subtract 1 PLL clock cycle 1 (DIV4): no adjustment needed 2 (DIV8): add 2 PLL clock cycles 3 (DIV16): add 6 PLL clock cycles"]
pub type SELECT_HOLD_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLOCK` reader - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. This field is not used for test controller accesses."]
pub type BLOCK_R = crate::BitReader<BLOCK_A>;
#[doc = "Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. This field is not used for test controller accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLOCK_A {
    #[doc = "0: 0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    BUS_ERROR = 0,
    #[doc = "1: 1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency). This can be particularly true if ARB_MODE=1 and an MMIO access must wait a potentially long time behind a XIP access."]
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
#[doc = "Field `BLOCK` writer - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. This field is not used for test controller accesses."]
pub type BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, BLOCK_A, O>;
impl<'a, const O: u8> BLOCK_W<'a, O> {
    #[doc = "0': Generate an AHB-Lite bus error. This option is useful when SW decides to use polling on STATUS.TR_BUSY to determine if a interface transfer is no longer busy (transfer is completed). This option adds SW complexity, but limits the number of AHB-Lite wait states (and limits ISR latency)."]
    #[inline(always)]
    pub fn bus_error(self) -> &'a mut W {
        self.variant(BLOCK_A::BUS_ERROR)
    }
    #[doc = "1': Introduce wait states. This setting potentially locks up the AHB-Lite infrastructure and may increase the CPU interrupt latency.This option is useful when SW performs TX/RX data FIFO accesses immediately after a command is setup using the TX format FIFO. This option has low SW complexity, but may result in a significant number of AHB-Lite wait states (and may increase ISR latency). This can be particularly true if ARB_MODE=1 and an MMIO access must wait a potentially long time behind a XIP access."]
    #[inline(always)]
    pub fn wait_states(self) -> &'a mut W {
        self.variant(BLOCK_A::WAIT_STATES)
    }
}
#[doc = "Field `CLOCK_IF_SEL` reader - Specifies the clock source for 'clk_if'. Must be 0 (clk_pll_normal) before entering DeepSleep; can be returned to 1 (clk_pll_direct) afterwards. '0': clk_pll_normal is used to create clk_if '1': clk_pll_direct is used to create clk_if"]
pub type CLOCK_IF_SEL_R = crate::BitReader<bool>;
#[doc = "Field `CLOCK_IF_SEL` writer - Specifies the clock source for 'clk_if'. Must be 0 (clk_pll_normal) before entering DeepSleep; can be returned to 1 (clk_pll_direct) afterwards. '0': clk_pll_normal is used to create clk_if '1': clk_pll_direct is used to create clk_if"]
pub type CLOCK_IF_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - SMIF core enable: '0': Disabled. All non-retention registers are reset to their default value when this SMIF core is disabled. When this SMIF core is disabled, the XIP accesses produce AHB-Lite bus errors and AXI error responses. '1': Enabled. Notes: - Before disabling or resetting this SMIF core, SW should ensure that this SMIF core is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset. - The DLL is powered up when CTL.ENABLED is set to 1. It takes 8us for the DLL to stabilize and lock to the freerunning PLL clock at which point STATUS.DLL_LOCKED and INTR.DLL_LOCK will be set. Wait another 4us beyond that before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "SMIF core enable: '0': Disabled. All non-retention registers are reset to their default value when this SMIF core is disabled. When this SMIF core is disabled, the XIP accesses produce AHB-Lite bus errors and AXI error responses. '1': Enabled. Notes: - Before disabling or resetting this SMIF core, SW should ensure that this SMIF core is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset. - The DLL is powered up when CTL.ENABLED is set to 1. It takes 8us for the DLL to stabilize and lock to the freerunning PLL clock at which point STATUS.DLL_LOCKED and INTR.DLL_LOCK will be set. Wait another 4us beyond that before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values.\n\nValue on reset: 0"]
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
#[doc = "Field `ENABLED` writer - SMIF core enable: '0': Disabled. All non-retention registers are reset to their default value when this SMIF core is disabled. When this SMIF core is disabled, the XIP accesses produce AHB-Lite bus errors and AXI error responses. '1': Enabled. Notes: - Before disabling or resetting this SMIF core, SW should ensure that this SMIF core is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset. - The DLL is powered up when CTL.ENABLED is set to 1. It takes 8us for the DLL to stabilize and lock to the freerunning PLL clock at which point STATUS.DLL_LOCKED and INTR.DLL_LOCK will be set. Wait another 4us beyond that before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values."]
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
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    pub fn deselect_delay(&self) -> DESELECT_DELAY_R {
        DESELECT_DELAY_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Specifies the duration between 'spi_select_out\\[\\]' activating (going low) to the first 'spi_clk_out' rising edge before considering any MDL delay due to CTL2.MDL_TAP_SEL. Subtract that delay from the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after subtracting the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 add 1 PLL clock cycle to these values. DDR or SDR mode is determined by the mode of the first phase of the transaction. '0': 2 PLL clock cycles '1': 2 PLL clock cycles + 1 memory interface clock cycles '2': 2 PLL clock cycles + 2 memory interface clock cycles '3': 2 PLL clock cycles + 3 memory interface clock cycles"]
    #[inline(always)]
    pub fn select_setup_delay(&self) -> SELECT_SETUP_DELAY_R {
        SELECT_SETUP_DELAY_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Specifies the duration between the last 'spi_clk_out' falling edge and 'spi_select_out\\[\\]' deactivating (going high) before considering any MDL delay due to CTL2.MDL_TAP_SEL. Add that delay to the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after adding the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 subtract 1 PLL clock cycle from these values. DDR or SDR mode is determined by the mode of the last phase of the transaction. '0': 1 memory interface clock cycle + CTL2.CLKOUT_DIV adjustment '1': 2 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '2': 3 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '3': 4 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment For the following settings of CTL2.CLKOUT_DIV apply these adjustments to the above: 0 (DIV2): subtract 1 PLL clock cycle 1 (DIV4): no adjustment needed 2 (DIV8): add 2 PLL clock cycles 3 (DIV16): add 6 PLL clock cycles"]
    #[inline(always)]
    pub fn select_hold_delay(&self) -> SELECT_HOLD_DELAY_R {
        SELECT_HOLD_DELAY_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. This field is not used for test controller accesses."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Specifies the clock source for 'clk_if'. Must be 0 (clk_pll_normal) before entering DeepSleep; can be returned to 1 (clk_pll_direct) afterwards. '0': clk_pll_normal is used to create clk_if '1': clk_pll_direct is used to create clk_if"]
    #[inline(always)]
    pub fn clock_if_sel(&self) -> CLOCK_IF_SEL_R {
        CLOCK_IF_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - SMIF core enable: '0': Disabled. All non-retention registers are reset to their default value when this SMIF core is disabled. When this SMIF core is disabled, the XIP accesses produce AHB-Lite bus errors and AXI error responses. '1': Enabled. Notes: - Before disabling or resetting this SMIF core, SW should ensure that this SMIF core is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset. - The DLL is powered up when CTL.ENABLED is set to 1. It takes 8us for the DLL to stabilize and lock to the freerunning PLL clock at which point STATUS.DLL_LOCKED and INTR.DLL_LOCK will be set. Wait another 4us beyond that before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values."]
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
    #[doc = "Bits 16:18 - Specifies the minimum duration of SPI deselection ('spi_select_out\\[\\]' is high/'1') in between SPI transfers: '0': 1 memory interface clock cycle. '1': 2 memory interface clock cycles. '2': 3 memory interface clock cycles. '3': 4 memory interface clock cycles. '4': 5 memory interface clock cycles. '5': 6 memory interface clock cycles. '6': 7 memory interface clock cycles. '7': 8 memory interface clock cycles. During SPI deselection, 'spi_select_out\\[\\]' are '1'/inactive, 'spi_data_out\\[\\]' are '1' and 'spi_clk_out' is '0'/inactive."]
    #[inline(always)]
    #[must_use]
    pub fn deselect_delay(&mut self) -> DESELECT_DELAY_W<16> {
        DESELECT_DELAY_W::new(self)
    }
    #[doc = "Bits 20:21 - Specifies the duration between 'spi_select_out\\[\\]' activating (going low) to the first 'spi_clk_out' rising edge before considering any MDL delay due to CTL2.MDL_TAP_SEL. Subtract that delay from the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after subtracting the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 add 1 PLL clock cycle to these values. DDR or SDR mode is determined by the mode of the first phase of the transaction. '0': 2 PLL clock cycles '1': 2 PLL clock cycles + 1 memory interface clock cycles '2': 2 PLL clock cycles + 2 memory interface clock cycles '3': 2 PLL clock cycles + 3 memory interface clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn select_setup_delay(&mut self) -> SELECT_SETUP_DELAY_W<20> {
        SELECT_SETUP_DELAY_W::new(self)
    }
    #[doc = "Bits 22:23 - Specifies the duration between the last 'spi_clk_out' falling edge and 'spi_select_out\\[\\]' deactivating (going high) before considering any MDL delay due to CTL2.MDL_TAP_SEL. Add that delay to the below values, plus allow some margin for combinatorial variances between spi_select_out\\[\\]
and spi_clk_out. i.e., if after adding the MDL delay the timing would barely meet the memory datasheet requirement then it is recommended to go to the next higher setting. These values apply for DDR mode, or SDR mode with CTL2.TX_SDR_EXTRA_SETUP=0. For SDR mode with CTL2.TX_SDR_EXTRA_SETUP=1 subtract 1 PLL clock cycle from these values. DDR or SDR mode is determined by the mode of the last phase of the transaction. '0': 1 memory interface clock cycle + CTL2.CLKOUT_DIV adjustment '1': 2 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '2': 3 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment '3': 4 memory interface clock cycles + CTL2.CLKOUT_DIV adjustment For the following settings of CTL2.CLKOUT_DIV apply these adjustments to the above: 0 (DIV2): subtract 1 PLL clock cycle 1 (DIV4): no adjustment needed 2 (DIV8): add 2 PLL clock cycles 3 (DIV16): add 6 PLL clock cycles"]
    #[inline(always)]
    #[must_use]
    pub fn select_hold_delay(&mut self) -> SELECT_HOLD_DELAY_W<22> {
        SELECT_HOLD_DELAY_W::new(self)
    }
    #[doc = "Bit 24 - Specifies what happens for MMIO interface read accesses to an empty RX data FIFO or to a full TX format/data FIFO. This field is not used for test controller accesses."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<24> {
        BLOCK_W::new(self)
    }
    #[doc = "Bit 25 - Specifies the clock source for 'clk_if'. Must be 0 (clk_pll_normal) before entering DeepSleep; can be returned to 1 (clk_pll_direct) afterwards. '0': clk_pll_normal is used to create clk_if '1': clk_pll_direct is used to create clk_if"]
    #[inline(always)]
    #[must_use]
    pub fn clock_if_sel(&mut self) -> CLOCK_IF_SEL_W<25> {
        CLOCK_IF_SEL_W::new(self)
    }
    #[doc = "Bit 31 - SMIF core enable: '0': Disabled. All non-retention registers are reset to their default value when this SMIF core is disabled. When this SMIF core is disabled, the XIP accesses produce AHB-Lite bus errors and AXI error responses. '1': Enabled. Notes: - Before disabling or resetting this SMIF core, SW should ensure that this SMIF core is NOT busy (STATUS.BUSY is '0'), otherwise illegal interface transfers may occur. - After CTL.ENABLED is set to 1 it takes up to 20 clk_if cycles until the memory interface registers are realeased from reset. - The DLL is powered up when CTL.ENABLED is set to 1. It takes 8us for the DLL to stabilize and lock to the freerunning PLL clock at which point STATUS.DLL_LOCKED and INTR.DLL_LOCK will be set. Wait another 4us beyond that before executing any transactions on the memory interface; the MDL and SDLs require that extra time after the lock indication to reach their final stable delay values."]
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
#[doc = "`reset()` method sets CTL to value 0x0050_0000"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0050_0000;
}
