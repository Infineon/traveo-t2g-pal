#[doc = "Register `NETWORK_CONTROL` reader"]
pub struct R(crate::R<NETWORK_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NETWORK_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NETWORK_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NETWORK_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NETWORK_CONTROL` writer"]
pub struct W(crate::W<NETWORK_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NETWORK_CONTROL_SPEC>;
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
impl From<crate::W<NETWORK_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NETWORK_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOOPBACK` reader - Loopback - controls the loopback output pin."]
pub type LOOPBACK_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK` writer - Loopback - controls the loopback output pin."]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `LOOPBACK_LOCAL` reader - Loopback local - asserts the loopback_local signal to the system clock generator. Also connects txd to rxd, tx_en to rx_dv and forces full duplex mode. Bit 11 of the network configuration register must be set low to disable TBI mode when in internal loopback. rx_clk and tx_clk may malfunction as the GEM is switched into and out of internal loopback. It is important that receive and transmit circuits have already been disabled when making the switch into and out of internal loopback. Local loopback functionality is optional and may not be supported by some instantiations of the GEM."]
pub type LOOPBACK_LOCAL_R = crate::BitReader<bool>;
#[doc = "Field `LOOPBACK_LOCAL` writer - Loopback local - asserts the loopback_local signal to the system clock generator. Also connects txd to rxd, tx_en to rx_dv and forces full duplex mode. Bit 11 of the network configuration register must be set low to disable TBI mode when in internal loopback. rx_clk and tx_clk may malfunction as the GEM is switched into and out of internal loopback. It is important that receive and transmit circuits have already been disabled when making the switch into and out of internal loopback. Local loopback functionality is optional and may not be supported by some instantiations of the GEM."]
pub type LOOPBACK_LOCAL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `ENABLE_RECEIVE` reader - Receive enable - when set, it enables the GEM to receive data. When reset frame reception will stop immediately and the receive pipeline will be cleared. The receive queue pointer register is unaffected."]
pub type ENABLE_RECEIVE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_RECEIVE` writer - Receive enable - when set, it enables the GEM to receive data. When reset frame reception will stop immediately and the receive pipeline will be cleared. The receive queue pointer register is unaffected."]
pub type ENABLE_RECEIVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `ENABLE_TRANSMIT` reader - Transmit enable - when set, it enables the GEM transmitter to send data. When reset transmission will stop immediately, the transmit pipeline and control registers will be cleared and the transmit queue pointer register will reset to point to the start of the transmit descriptor list."]
pub type ENABLE_TRANSMIT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_TRANSMIT` writer - Transmit enable - when set, it enables the GEM transmitter to send data. When reset transmission will stop immediately, the transmit pipeline and control registers will be cleared and the transmit queue pointer register will reset to point to the start of the transmit descriptor list."]
pub type ENABLE_TRANSMIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `MAN_PORT_EN` reader - Management port enable - set to one to enable the management port. When zero forces mdio to high impedance state and mdc low."]
pub type MAN_PORT_EN_R = crate::BitReader<bool>;
#[doc = "Field `MAN_PORT_EN` writer - Management port enable - set to one to enable the management port. When zero forces mdio to high impedance state and mdc low."]
pub type MAN_PORT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `CLEAR_ALL_STATS_REGS` writer - Clear statistics registers - this bit is write only. Writing a one clears the statistics registers. Self clearing register."]
pub type CLEAR_ALL_STATS_REGS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `INC_ALL_STATS_REGS` writer - Incremental statistics registers - this bit is write only. Writing a one increments all the statistics registers by one for test purposes. Self clearing register."]
pub type INC_ALL_STATS_REGS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `STATS_WRITE_EN` reader - Write enable for statistics registers - setting this bit to one means the statistics registers can be written for functional test purposes."]
pub type STATS_WRITE_EN_R = crate::BitReader<bool>;
#[doc = "Field `STATS_WRITE_EN` writer - Write enable for statistics registers - setting this bit to one means the statistics registers can be written for functional test purposes."]
pub type STATS_WRITE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `BACK_PRESSURE` reader - Back pressure if set in 10M or 100M half duplex mode will force collisions on all received frames. Ignored in gigabit half duplex mode."]
pub type BACK_PRESSURE_R = crate::BitReader<bool>;
#[doc = "Field `BACK_PRESSURE` writer - Back pressure if set in 10M or 100M half duplex mode will force collisions on all received frames. Ignored in gigabit half duplex mode."]
pub type BACK_PRESSURE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TX_START_PCLK` writer - Start transmission - writing one to this bit starts transmission."]
pub type TX_START_PCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TX_HALT_PCLK` writer - Transmit halt - writing one to this bit halts transmission as soon as any ongoing frame transmission ends."]
pub type TX_HALT_PCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TX_PAUSE_FRAME_REQ` writer - Transmit pause frame - writing one to this bit causes a pause frame to be transmitted. Self clearing register."]
pub type TX_PAUSE_FRAME_REQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TX_PAUSE_FRAME_ZERO` writer - Transmit zero quantum pause frame - writing one to this bit causes a pause frame with zero quantum to be transmitted."]
pub type TX_PAUSE_FRAME_ZERO_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `REMOVED_13` reader - Write ignore, read 0"]
pub type REMOVED_13_R = crate::BitReader<bool>;
#[doc = "Field `REMOVED_14` reader - Write ignore, read 0"]
pub type REMOVED_14_R = crate::BitReader<bool>;
#[doc = "Field `STORE_RX_TS` reader - Store receive time stamp to memory. Setting this bit to one will cause the CRC of every received frame to be replaced with the value of the nanoseconds field of the 1588 timer that was captured as the receive frame passed the message time stamp point. Set to zero for normal operation."]
pub type STORE_RX_TS_R = crate::BitReader<bool>;
#[doc = "Field `STORE_RX_TS` writer - Store receive time stamp to memory. Setting this bit to one will cause the CRC of every received frame to be replaced with the value of the nanoseconds field of the 1588 timer that was captured as the receive frame passed the message time stamp point. Set to zero for normal operation."]
pub type STORE_RX_TS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `PFC_ENABLE` reader - Enable PFC Priority Based Pause Reception capabilities. Setting this bit will enable PFC negotiation and recognition of priority based pause frames."]
pub type PFC_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PFC_ENABLE` writer - Enable PFC Priority Based Pause Reception capabilities. Setting this bit will enable PFC negotiation and recognition of priority based pause frames."]
pub type PFC_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TRANSMIT_PFC_PRIORITY_BASED_PAUSE_FRAME` writer - Write a one to transmit PFC priority based pause frame. Takes the values stored in the Transmit PFC Pause Register. Self clearing register."]
pub type TRANSMIT_PFC_PRIORITY_BASED_PAUSE_FRAME_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `FLUSH_RX_PKT_PCLK` writer - Flush the next packet from the external RX DPRAM. Writing one to this bit will only have an effect if the DMA is not currently writing a packet already stored in the DPRAM to memory. Self clearing register."]
pub type FLUSH_RX_PKT_PCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TX_LPI_EN` reader - Enable LPI transmission when set LPI (low power idle) is immediately transmitted. LPI is transmitted even if bit 3 transmit enable is disabled. Setting this bit also sends a pause signal to the transmit datapath."]
pub type TX_LPI_EN_R = crate::BitReader<bool>;
#[doc = "Field `TX_LPI_EN` writer - Enable LPI transmission when set LPI (low power idle) is immediately transmitted. LPI is transmitted even if bit 3 transmit enable is disabled. Setting this bit also sends a pause signal to the transmit datapath."]
pub type TX_LPI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `PTP_UNICAST_ENA` reader - Enable detection of unicast PTP unicast frames."]
pub type PTP_UNICAST_ENA_R = crate::BitReader<bool>;
#[doc = "Field `PTP_UNICAST_ENA` writer - Enable detection of unicast PTP unicast frames."]
pub type PTP_UNICAST_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `ALT_SGMII_MODE` reader - Alternative sgmii mode. If asserted with sgmii_mode in the network control register the ACK bit is driven before ability detect during transfer of status information from the PHY to the MAC."]
pub type ALT_SGMII_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ALT_SGMII_MODE` writer - Alternative sgmii mode. If asserted with sgmii_mode in the network control register the ACK bit is driven before ability detect during transfer of status information from the PHY to the MAC."]
pub type ALT_SGMII_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `STORE_UDP_OFFSET` reader - N/A"]
pub type STORE_UDP_OFFSET_R = crate::BitReader<bool>;
#[doc = "Field `STORE_UDP_OFFSET` writer - N/A"]
pub type STORE_UDP_OFFSET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `EXT_TSU_PORT_ENABLE` reader - Write ignore, read 0"]
pub type EXT_TSU_PORT_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ONE_STEP_SYNC_MODE` reader - 1588 One Step Sync Mode. Write 1 to enable. Replace timestamp field in the 1588 header for TX Sync Frames with current TSU timer value."]
pub type ONE_STEP_SYNC_MODE_R = crate::BitReader<bool>;
#[doc = "Field `ONE_STEP_SYNC_MODE` writer - 1588 One Step Sync Mode. Write 1 to enable. Replace timestamp field in the 1588 header for TX Sync Frames with current TSU timer value."]
pub type ONE_STEP_SYNC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `PFC_CTRL` reader - 'Enable multiple PFC pause quantums, one per pause priority'"]
pub type PFC_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `PFC_CTRL` writer - 'Enable multiple PFC pause quantums, one per pause priority'"]
pub type PFC_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `EXT_RXQ_SEL_EN` reader - Enable external selection of receive queue. When this bit is high the ext_match1, ext_match2, ext_match3 and ext_match4 inputs will determine which receive queue a frame is routed to. This will be the case regardless of the state of the external address match enable bit 9 of the network config register. Note that receive frames will be dropped unless they are matched by the internal frame filtering functionality. If the external address match enable bit 9 in the network config register is set frames may be matched by an external address match filter as long as one of the ext_match1, ext_match2, ext_match3 and ext_match4 inputs is asserted early enough. When set ext_rxq_sel_en takes precedence over the existing screener functionality. This bit is only relevant if priority queuing is configured."]
pub type EXT_RXQ_SEL_EN_R = crate::BitReader<bool>;
#[doc = "Field `EXT_RXQ_SEL_EN` writer - Enable external selection of receive queue. When this bit is high the ext_match1, ext_match2, ext_match3 and ext_match4 inputs will determine which receive queue a frame is routed to. This will be the case regardless of the state of the external address match enable bit 9 of the network config register. Note that receive frames will be dropped unless they are matched by the internal frame filtering functionality. If the external address match enable bit 9 in the network config register is set frames may be matched by an external address match filter as long as one of the ext_match1, ext_match2, ext_match3 and ext_match4 inputs is asserted early enough. When set ext_rxq_sel_en takes precedence over the existing screener functionality. This bit is only relevant if priority queuing is configured."]
pub type EXT_RXQ_SEL_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `OSS_CORRECTION_FIELD` reader - 1588 One Step Correction Field Update. Set this bit high to enable updating the correction field of PTP 1588 version 2 sync frames by adding current TSU timer value."]
pub type OSS_CORRECTION_FIELD_R = crate::BitReader<bool>;
#[doc = "Field `OSS_CORRECTION_FIELD` writer - 1588 One Step Correction Field Update. Set this bit high to enable updating the correction field of PTP 1588 version 2 sync frames by adding current TSU timer value."]
pub type OSS_CORRECTION_FIELD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `SEL_MII_ON_RGMII` reader - If the RGMII interface being used set this bit high to configure the interface for MII operation."]
pub type SEL_MII_ON_RGMII_R = crate::BitReader<bool>;
#[doc = "Field `SEL_MII_ON_RGMII` writer - If the RGMII interface being used set this bit high to configure the interface for MII operation."]
pub type SEL_MII_ON_RGMII_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `TWO_PT_FIVE_GIG` reader - 2.5G operation selected - setting this bit high drives the speed_mode\\[3\\]
top level output pin high and also adjusts the link timer in the PCS auto-negotiation block to ensure it delivers 10ms for 2500BASE-X and 1.6ms in SGMII mode, and also ensures int_moderation counts 800ns periods with the speeded up MAC clocks."]
pub type TWO_PT_FIVE_GIG_R = crate::BitReader<bool>;
#[doc = "Field `TWO_PT_FIVE_GIG` writer - 2.5G operation selected - setting this bit high drives the speed_mode\\[3\\]
top level output pin high and also adjusts the link timer in the PCS auto-negotiation block to ensure it delivers 10ms for 2500BASE-X and 1.6ms in SGMII mode, and also ensures int_moderation counts 800ns periods with the speeded up MAC clocks."]
pub type TWO_PT_FIVE_GIG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `IFG_EATS_QAV_CREDIT` reader - Setting this bit high modifies the CBS algorithm so the IFG/IPG associated with a transmit frame counts towards its 802.1Qav credit."]
pub type IFG_EATS_QAV_CREDIT_R = crate::BitReader<bool>;
#[doc = "Field `IFG_EATS_QAV_CREDIT` writer - Setting this bit high modifies the CBS algorithm so the IFG/IPG associated with a transmit frame counts towards its 802.1Qav credit."]
pub type IFG_EATS_QAV_CREDIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NETWORK_CONTROL_SPEC, bool, O>;
#[doc = "Field `EXT_RXQ_RSVD_31` reader - N/A"]
pub type EXT_RXQ_RSVD_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Loopback - controls the loopback output pin."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Loopback local - asserts the loopback_local signal to the system clock generator. Also connects txd to rxd, tx_en to rx_dv and forces full duplex mode. Bit 11 of the network configuration register must be set low to disable TBI mode when in internal loopback. rx_clk and tx_clk may malfunction as the GEM is switched into and out of internal loopback. It is important that receive and transmit circuits have already been disabled when making the switch into and out of internal loopback. Local loopback functionality is optional and may not be supported by some instantiations of the GEM."]
    #[inline(always)]
    pub fn loopback_local(&self) -> LOOPBACK_LOCAL_R {
        LOOPBACK_LOCAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive enable - when set, it enables the GEM to receive data. When reset frame reception will stop immediately and the receive pipeline will be cleared. The receive queue pointer register is unaffected."]
    #[inline(always)]
    pub fn enable_receive(&self) -> ENABLE_RECEIVE_R {
        ENABLE_RECEIVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit enable - when set, it enables the GEM transmitter to send data. When reset transmission will stop immediately, the transmit pipeline and control registers will be cleared and the transmit queue pointer register will reset to point to the start of the transmit descriptor list."]
    #[inline(always)]
    pub fn enable_transmit(&self) -> ENABLE_TRANSMIT_R {
        ENABLE_TRANSMIT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Management port enable - set to one to enable the management port. When zero forces mdio to high impedance state and mdc low."]
    #[inline(always)]
    pub fn man_port_en(&self) -> MAN_PORT_EN_R {
        MAN_PORT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Write enable for statistics registers - setting this bit to one means the statistics registers can be written for functional test purposes."]
    #[inline(always)]
    pub fn stats_write_en(&self) -> STATS_WRITE_EN_R {
        STATS_WRITE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Back pressure if set in 10M or 100M half duplex mode will force collisions on all received frames. Ignored in gigabit half duplex mode."]
    #[inline(always)]
    pub fn back_pressure(&self) -> BACK_PRESSURE_R {
        BACK_PRESSURE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 13 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_13(&self) -> REMOVED_13_R {
        REMOVED_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write ignore, read 0"]
    #[inline(always)]
    pub fn removed_14(&self) -> REMOVED_14_R {
        REMOVED_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory. Setting this bit to one will cause the CRC of every received frame to be replaced with the value of the nanoseconds field of the 1588 timer that was captured as the receive frame passed the message time stamp point. Set to zero for normal operation."]
    #[inline(always)]
    pub fn store_rx_ts(&self) -> STORE_RX_TS_R {
        STORE_RX_TS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities. Setting this bit will enable PFC negotiation and recognition of priority based pause frames."]
    #[inline(always)]
    pub fn pfc_enable(&self) -> PFC_ENABLE_R {
        PFC_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted. LPI is transmitted even if bit 3 transmit enable is disabled. Setting this bit also sends a pause signal to the transmit datapath."]
    #[inline(always)]
    pub fn tx_lpi_en(&self) -> TX_LPI_EN_R {
        TX_LPI_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    pub fn ptp_unicast_ena(&self) -> PTP_UNICAST_ENA_R {
        PTP_UNICAST_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Alternative sgmii mode. If asserted with sgmii_mode in the network control register the ACK bit is driven before ability detect during transfer of status information from the PHY to the MAC."]
    #[inline(always)]
    pub fn alt_sgmii_mode(&self) -> ALT_SGMII_MODE_R {
        ALT_SGMII_MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    pub fn store_udp_offset(&self) -> STORE_UDP_OFFSET_R {
        STORE_UDP_OFFSET_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Write ignore, read 0"]
    #[inline(always)]
    pub fn ext_tsu_port_enable(&self) -> EXT_TSU_PORT_ENABLE_R {
        EXT_TSU_PORT_ENABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode. Write 1 to enable. Replace timestamp field in the 1588 header for TX Sync Frames with current TSU timer value."]
    #[inline(always)]
    pub fn one_step_sync_mode(&self) -> ONE_STEP_SYNC_MODE_R {
        ONE_STEP_SYNC_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 'Enable multiple PFC pause quantums, one per pause priority'"]
    #[inline(always)]
    pub fn pfc_ctrl(&self) -> PFC_CTRL_R {
        PFC_CTRL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable external selection of receive queue. When this bit is high the ext_match1, ext_match2, ext_match3 and ext_match4 inputs will determine which receive queue a frame is routed to. This will be the case regardless of the state of the external address match enable bit 9 of the network config register. Note that receive frames will be dropped unless they are matched by the internal frame filtering functionality. If the external address match enable bit 9 in the network config register is set frames may be matched by an external address match filter as long as one of the ext_match1, ext_match2, ext_match3 and ext_match4 inputs is asserted early enough. When set ext_rxq_sel_en takes precedence over the existing screener functionality. This bit is only relevant if priority queuing is configured."]
    #[inline(always)]
    pub fn ext_rxq_sel_en(&self) -> EXT_RXQ_SEL_EN_R {
        EXT_RXQ_SEL_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1588 One Step Correction Field Update. Set this bit high to enable updating the correction field of PTP 1588 version 2 sync frames by adding current TSU timer value."]
    #[inline(always)]
    pub fn oss_correction_field(&self) -> OSS_CORRECTION_FIELD_R {
        OSS_CORRECTION_FIELD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - If the RGMII interface being used set this bit high to configure the interface for MII operation."]
    #[inline(always)]
    pub fn sel_mii_on_rgmii(&self) -> SEL_MII_ON_RGMII_R {
        SEL_MII_ON_RGMII_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 2.5G operation selected - setting this bit high drives the speed_mode\\[3\\]
top level output pin high and also adjusts the link timer in the PCS auto-negotiation block to ensure it delivers 10ms for 2500BASE-X and 1.6ms in SGMII mode, and also ensures int_moderation counts 800ns periods with the speeded up MAC clocks."]
    #[inline(always)]
    pub fn two_pt_five_gig(&self) -> TWO_PT_FIVE_GIG_R {
        TWO_PT_FIVE_GIG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Setting this bit high modifies the CBS algorithm so the IFG/IPG associated with a transmit frame counts towards its 802.1Qav credit."]
    #[inline(always)]
    pub fn ifg_eats_qav_credit(&self) -> IFG_EATS_QAV_CREDIT_R {
        IFG_EATS_QAV_CREDIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn ext_rxq_rsvd_31(&self) -> EXT_RXQ_RSVD_31_R {
        EXT_RXQ_RSVD_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Loopback - controls the loopback output pin."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<0> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 1 - Loopback local - asserts the loopback_local signal to the system clock generator. Also connects txd to rxd, tx_en to rx_dv and forces full duplex mode. Bit 11 of the network configuration register must be set low to disable TBI mode when in internal loopback. rx_clk and tx_clk may malfunction as the GEM is switched into and out of internal loopback. It is important that receive and transmit circuits have already been disabled when making the switch into and out of internal loopback. Local loopback functionality is optional and may not be supported by some instantiations of the GEM."]
    #[inline(always)]
    #[must_use]
    pub fn loopback_local(&mut self) -> LOOPBACK_LOCAL_W<1> {
        LOOPBACK_LOCAL_W::new(self)
    }
    #[doc = "Bit 2 - Receive enable - when set, it enables the GEM to receive data. When reset frame reception will stop immediately and the receive pipeline will be cleared. The receive queue pointer register is unaffected."]
    #[inline(always)]
    #[must_use]
    pub fn enable_receive(&mut self) -> ENABLE_RECEIVE_W<2> {
        ENABLE_RECEIVE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit enable - when set, it enables the GEM transmitter to send data. When reset transmission will stop immediately, the transmit pipeline and control registers will be cleared and the transmit queue pointer register will reset to point to the start of the transmit descriptor list."]
    #[inline(always)]
    #[must_use]
    pub fn enable_transmit(&mut self) -> ENABLE_TRANSMIT_W<3> {
        ENABLE_TRANSMIT_W::new(self)
    }
    #[doc = "Bit 4 - Management port enable - set to one to enable the management port. When zero forces mdio to high impedance state and mdc low."]
    #[inline(always)]
    #[must_use]
    pub fn man_port_en(&mut self) -> MAN_PORT_EN_W<4> {
        MAN_PORT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Clear statistics registers - this bit is write only. Writing a one clears the statistics registers. Self clearing register."]
    #[inline(always)]
    #[must_use]
    pub fn clear_all_stats_regs(&mut self) -> CLEAR_ALL_STATS_REGS_W<5> {
        CLEAR_ALL_STATS_REGS_W::new(self)
    }
    #[doc = "Bit 6 - Incremental statistics registers - this bit is write only. Writing a one increments all the statistics registers by one for test purposes. Self clearing register."]
    #[inline(always)]
    #[must_use]
    pub fn inc_all_stats_regs(&mut self) -> INC_ALL_STATS_REGS_W<6> {
        INC_ALL_STATS_REGS_W::new(self)
    }
    #[doc = "Bit 7 - Write enable for statistics registers - setting this bit to one means the statistics registers can be written for functional test purposes."]
    #[inline(always)]
    #[must_use]
    pub fn stats_write_en(&mut self) -> STATS_WRITE_EN_W<7> {
        STATS_WRITE_EN_W::new(self)
    }
    #[doc = "Bit 8 - Back pressure if set in 10M or 100M half duplex mode will force collisions on all received frames. Ignored in gigabit half duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn back_pressure(&mut self) -> BACK_PRESSURE_W<8> {
        BACK_PRESSURE_W::new(self)
    }
    #[doc = "Bit 9 - Start transmission - writing one to this bit starts transmission."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start_pclk(&mut self) -> TX_START_PCLK_W<9> {
        TX_START_PCLK_W::new(self)
    }
    #[doc = "Bit 10 - Transmit halt - writing one to this bit halts transmission as soon as any ongoing frame transmission ends."]
    #[inline(always)]
    #[must_use]
    pub fn tx_halt_pclk(&mut self) -> TX_HALT_PCLK_W<10> {
        TX_HALT_PCLK_W::new(self)
    }
    #[doc = "Bit 11 - Transmit pause frame - writing one to this bit causes a pause frame to be transmitted. Self clearing register."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_frame_req(&mut self) -> TX_PAUSE_FRAME_REQ_W<11> {
        TX_PAUSE_FRAME_REQ_W::new(self)
    }
    #[doc = "Bit 12 - Transmit zero quantum pause frame - writing one to this bit causes a pause frame with zero quantum to be transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_frame_zero(&mut self) -> TX_PAUSE_FRAME_ZERO_W<12> {
        TX_PAUSE_FRAME_ZERO_W::new(self)
    }
    #[doc = "Bit 15 - Store receive time stamp to memory. Setting this bit to one will cause the CRC of every received frame to be replaced with the value of the nanoseconds field of the 1588 timer that was captured as the receive frame passed the message time stamp point. Set to zero for normal operation."]
    #[inline(always)]
    #[must_use]
    pub fn store_rx_ts(&mut self) -> STORE_RX_TS_W<15> {
        STORE_RX_TS_W::new(self)
    }
    #[doc = "Bit 16 - Enable PFC Priority Based Pause Reception capabilities. Setting this bit will enable PFC negotiation and recognition of priority based pause frames."]
    #[inline(always)]
    #[must_use]
    pub fn pfc_enable(&mut self) -> PFC_ENABLE_W<16> {
        PFC_ENABLE_W::new(self)
    }
    #[doc = "Bit 17 - Write a one to transmit PFC priority based pause frame. Takes the values stored in the Transmit PFC Pause Register. Self clearing register."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_pfc_priority_based_pause_frame(
        &mut self,
    ) -> TRANSMIT_PFC_PRIORITY_BASED_PAUSE_FRAME_W<17> {
        TRANSMIT_PFC_PRIORITY_BASED_PAUSE_FRAME_W::new(self)
    }
    #[doc = "Bit 18 - Flush the next packet from the external RX DPRAM. Writing one to this bit will only have an effect if the DMA is not currently writing a packet already stored in the DPRAM to memory. Self clearing register."]
    #[inline(always)]
    #[must_use]
    pub fn flush_rx_pkt_pclk(&mut self) -> FLUSH_RX_PKT_PCLK_W<18> {
        FLUSH_RX_PKT_PCLK_W::new(self)
    }
    #[doc = "Bit 19 - Enable LPI transmission when set LPI (low power idle) is immediately transmitted. LPI is transmitted even if bit 3 transmit enable is disabled. Setting this bit also sends a pause signal to the transmit datapath."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lpi_en(&mut self) -> TX_LPI_EN_W<19> {
        TX_LPI_EN_W::new(self)
    }
    #[doc = "Bit 20 - Enable detection of unicast PTP unicast frames."]
    #[inline(always)]
    #[must_use]
    pub fn ptp_unicast_ena(&mut self) -> PTP_UNICAST_ENA_W<20> {
        PTP_UNICAST_ENA_W::new(self)
    }
    #[doc = "Bit 21 - Alternative sgmii mode. If asserted with sgmii_mode in the network control register the ACK bit is driven before ability detect during transfer of status information from the PHY to the MAC."]
    #[inline(always)]
    #[must_use]
    pub fn alt_sgmii_mode(&mut self) -> ALT_SGMII_MODE_W<21> {
        ALT_SGMII_MODE_W::new(self)
    }
    #[doc = "Bit 22 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn store_udp_offset(&mut self) -> STORE_UDP_OFFSET_W<22> {
        STORE_UDP_OFFSET_W::new(self)
    }
    #[doc = "Bit 24 - 1588 One Step Sync Mode. Write 1 to enable. Replace timestamp field in the 1588 header for TX Sync Frames with current TSU timer value."]
    #[inline(always)]
    #[must_use]
    pub fn one_step_sync_mode(&mut self) -> ONE_STEP_SYNC_MODE_W<24> {
        ONE_STEP_SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 25 - 'Enable multiple PFC pause quantums, one per pause priority'"]
    #[inline(always)]
    #[must_use]
    pub fn pfc_ctrl(&mut self) -> PFC_CTRL_W<25> {
        PFC_CTRL_W::new(self)
    }
    #[doc = "Bit 26 - Enable external selection of receive queue. When this bit is high the ext_match1, ext_match2, ext_match3 and ext_match4 inputs will determine which receive queue a frame is routed to. This will be the case regardless of the state of the external address match enable bit 9 of the network config register. Note that receive frames will be dropped unless they are matched by the internal frame filtering functionality. If the external address match enable bit 9 in the network config register is set frames may be matched by an external address match filter as long as one of the ext_match1, ext_match2, ext_match3 and ext_match4 inputs is asserted early enough. When set ext_rxq_sel_en takes precedence over the existing screener functionality. This bit is only relevant if priority queuing is configured."]
    #[inline(always)]
    #[must_use]
    pub fn ext_rxq_sel_en(&mut self) -> EXT_RXQ_SEL_EN_W<26> {
        EXT_RXQ_SEL_EN_W::new(self)
    }
    #[doc = "Bit 27 - 1588 One Step Correction Field Update. Set this bit high to enable updating the correction field of PTP 1588 version 2 sync frames by adding current TSU timer value."]
    #[inline(always)]
    #[must_use]
    pub fn oss_correction_field(&mut self) -> OSS_CORRECTION_FIELD_W<27> {
        OSS_CORRECTION_FIELD_W::new(self)
    }
    #[doc = "Bit 28 - If the RGMII interface being used set this bit high to configure the interface for MII operation."]
    #[inline(always)]
    #[must_use]
    pub fn sel_mii_on_rgmii(&mut self) -> SEL_MII_ON_RGMII_W<28> {
        SEL_MII_ON_RGMII_W::new(self)
    }
    #[doc = "Bit 29 - 2.5G operation selected - setting this bit high drives the speed_mode\\[3\\]
top level output pin high and also adjusts the link timer in the PCS auto-negotiation block to ensure it delivers 10ms for 2500BASE-X and 1.6ms in SGMII mode, and also ensures int_moderation counts 800ns periods with the speeded up MAC clocks."]
    #[inline(always)]
    #[must_use]
    pub fn two_pt_five_gig(&mut self) -> TWO_PT_FIVE_GIG_W<29> {
        TWO_PT_FIVE_GIG_W::new(self)
    }
    #[doc = "Bit 30 - Setting this bit high modifies the CBS algorithm so the IFG/IPG associated with a transmit frame counts towards its 802.1Qav credit."]
    #[inline(always)]
    #[must_use]
    pub fn ifg_eats_qav_credit(&mut self) -> IFG_EATS_QAV_CREDIT_W<30> {
        IFG_EATS_QAV_CREDIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The network control register contains general MAC control functions for both receiver and transmitter.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [network_control](index.html) module"]
pub struct NETWORK_CONTROL_SPEC;
impl crate::RegisterSpec for NETWORK_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [network_control::R](R) reader structure"]
impl crate::Readable for NETWORK_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [network_control::W](W) writer structure"]
impl crate::Writable for NETWORK_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NETWORK_CONTROL to value 0"]
impl crate::Resettable for NETWORK_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
