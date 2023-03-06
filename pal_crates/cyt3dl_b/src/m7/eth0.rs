#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MXETH Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - MXETH Status Register"]
    pub status: STATUS,
    _reserved2: [u8; 0x0ff8],
    #[doc = "0x1000 - The network control register contains general MAC control functions for both receiver and transmitter."]
    pub network_control: NETWORK_CONTROL,
    #[doc = "0x1004 - The network configuration register contains functions for setting the mode of operation for the Gigabit Ethernet MAC."]
    pub network_config: NETWORK_CONFIG,
    #[doc = "0x1008 - The network status register returns status information with respect to the PHY management interface."]
    pub network_status: NETWORK_STATUS,
    #[doc = "0x100c - Not presents. Access to the register will return AHB error."]
    pub user_io_register: USER_IO_REGISTER,
    #[doc = "0x1010 - DMA Configuration Register"]
    pub dma_config: DMA_CONFIG,
    #[doc = "0x1014 - This register, when read, provides details of the status of a transmit. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register."]
    pub transmit_status: TRANSMIT_STATUS,
    #[doc = "0x1018 - This register holds the start address of the receive buffer queue (receive buffers descriptor list). The receive buffer queue base address must be initialized before receive is enabled through bit 2 of the network control register. Once reception is enabled, any write to the receive buffer queue base address register is ignored. Reading this register returns the location of the descriptor currently being accessed. This value increments as buffers are used. Software should not use this register for determining where to remove received frames from the queue as it constantly changes as new frames are received. Software should instead work its way through the buffer descriptor queue checking the used bits. In terms of AMBA (AXI) operation, the receive descriptors are read from memory using a single 32bit AXI access. When the datapath is configured at 64bit, the receive descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is written to using a single 64bit AXI access."]
    pub receive_q_ptr: RECEIVE_Q_PTR,
    #[doc = "0x101c - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
    pub transmit_q_ptr: TRANSMIT_Q_PTR,
    #[doc = "0x1020 - This register, when read provides details of the status of a receive. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register."]
    pub receive_status: RECEIVE_STATUS,
    #[doc = "0x1024 - If not configured for priority queueing, the GEM generates a single interrupt. This register indicates the source of this interrupt. The corresponding bit in the mask register must be clear for a bit to be set. If any bit is set in this register the ethernet_int signal will be asserted. For test purposes each bit can be set or reset by writing to the interrupt mask register. The default configuration is shown below whereby all bits are reset to zero on read. Changing the validity of the `gem_irq_read_clear define will instead require a one to be written to the appropriate bit in order to clear it. In this mode reading has no affect on the status of the bit."]
    pub int_status: INT_STATUS,
    #[doc = "0x1028 - At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
    pub int_enable: INT_ENABLE,
    #[doc = "0x102c - Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
    pub int_disable: INT_DISABLE,
    #[doc = "0x1030 - The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
    pub int_mask: INT_MASK,
    #[doc = "0x1034 - The PHY maintenance register is implemented as a shift register. Writing to the register starts a shift operation which is signalled as complete when bit-2 is set in the network status register. It takes about 2000 pclk cycles to complete, when MDC is set for pclk divide by 32 in the network configuration register. An interrupt is generated upon completion. During this time, the MSB of the register is output on the MDIO pin and the LSB updated from the MDIO pin with each MDC cycle. This causes transmission of a PHY management frame on MDIO. See Section 22.2.4.5 of the IEEE 802.3 standard. Reading during the shift operation will return the current contents of the shift register. At the end of management operation, the bits will have shifted back to their original locations. For a read operation, the data bits will be updated with data read from the PHY. It is important to write the correct values to the register to ensure a valid PHY management frame is produced. The MDIO interface can read IEEE 802.3 clause 45 PHYs as well as clause 22 PHYs. To read clause 45 PHYs, bit 30 should be written with a 0 rather than a 1. For a description of MDC generation, see Network Configuration Register."]
    pub phy_management: PHY_MANAGEMENT,
    #[doc = "0x1038 - Received Pause Quantum Register"]
    pub pause_time: PAUSE_TIME,
    #[doc = "0x103c - Transmit Pause Quantum Register"]
    pub tx_pause_quantum: TX_PAUSE_QUANTUM,
    #[doc = "0x1040 - Partial store and forward is only applicable when using the DMA configured in SRAM based packet buffer mode. It is also not available when using multi buffer frames. TX Partial Store and Forward"]
    pub pbuf_txcutthru: PBUF_TXCUTTHRU,
    #[doc = "0x1044 - RX Partial Store and Forward"]
    pub pbuf_rxcutthru: PBUF_RXCUTTHRU,
    #[doc = "0x1048 - Maximum Jumbo Frame Size."]
    pub jumbo_max_length: JUMBO_MAX_LENGTH,
    #[doc = "0x104c - Not presents."]
    pub external_fifo_interface: EXTERNAL_FIFO_INTERFACE,
    _reserved22: [u8; 0x04],
    #[doc = "0x1054 - Used to set the maximum amount of outstanding transactions on the AXI bus between AR / R channels and AW / W channels. Cannot be more than the depth of the configured AXI pipeline FIFO (defined in verilog defs.v)"]
    pub axi_max_pipeline: AXI_MAX_PIPELINE,
    #[doc = "0x1058 - Not presents. Access to the register will return AHB error."]
    pub rsc_control: RSC_CONTROL,
    #[doc = "0x105c - Used to moderate the number of transmit and receive complete interrupts issued. With interrupt moderation enabled receive and transmit interrupts are not generated immediately a frame is transmitted or received. Instead when a receive or transmit event occurs a timer is started and the interrupt is asserted after it times out. This limits the frequency with which the CPU receives interrupts. When interrupt moderation is enabled interrupt status bit one is always used for receive and bit 7 is always used for transmit even when priority queuing is enabled. With interrupt moderation 800ns periods are counted. GEM determines what constitutes an 800ns period by looking at the tbi (bit 11), gigabit bit (10) and speed (bit 0) bits in the network configuration register and counting tx_clk cycles. Bit 0 needs to be set to 1 for 100M operation."]
    pub int_moderation: INT_MODERATION,
    #[doc = "0x1060 - Used to pause transmission after deassertion of tx_lpi_en. Each unit in this register corresponds to 64ns in gigabit mode, 320ns in 100M mode and 3200ns at 10M. After tx_lpi_en is deasserted transmission will pause for the set time."]
    pub sys_wake_time: SYS_WAKE_TIME,
    _reserved26: [u8; 0x1c],
    #[doc = "0x1080 - The unicast hash enable and the multicast hash enable bits in the network configuration register enable the reception of hash matched frames. Hash Register Bottom (31 to 0 bits)"]
    pub hash_bottom: HASH_BOTTOM,
    #[doc = "0x1084 - Hash Register Top (63 to 32 bits)"]
    pub hash_top: HASH_TOP,
    #[doc = "0x1088 - The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
    pub spec_add1_bottom: SPEC_ADD1_BOTTOM,
    #[doc = "0x108c - Specific Address Top"]
    pub spec_add1_top: SPEC_ADD1_TOP,
    #[doc = "0x1090 - The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
    pub spec_add2_bottom: SPEC_ADD2_BOTTOM,
    #[doc = "0x1094 - Specific Address Top"]
    pub spec_add2_top: SPEC_ADD2_TOP,
    #[doc = "0x1098 - The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
    pub spec_add3_bottom: SPEC_ADD3_BOTTOM,
    #[doc = "0x109c - Specific Address Top"]
    pub spec_add3_top: SPEC_ADD3_TOP,
    #[doc = "0x10a0 - The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
    pub spec_add4_bottom: SPEC_ADD4_BOTTOM,
    #[doc = "0x10a4 - Specific Address Top"]
    pub spec_add4_top: SPEC_ADD4_TOP,
    #[doc = "0x10a8 - Type ID Match 1"]
    pub spec_type1: SPEC_TYPE1,
    #[doc = "0x10ac - Type ID Match 2"]
    pub spec_type2: SPEC_TYPE2,
    #[doc = "0x10b0 - Type ID Match 3"]
    pub spec_type3: SPEC_TYPE3,
    #[doc = "0x10b4 - Type ID Match 4"]
    pub spec_type4: SPEC_TYPE4,
    #[doc = "0x10b8 - Wake on LAN Register. Presents in design, but feature is not supported."]
    pub wol_register: WOL_REGISTER,
    #[doc = "0x10bc - IPG stretch register"]
    pub stretch_ratio: STRETCH_RATIO,
    #[doc = "0x10c0 - Stacked VLAN Register"]
    pub stacked_vlan: STACKED_VLAN,
    #[doc = "0x10c4 - Transmit PFC Pause Register"]
    pub tx_pfc_pause: TX_PFC_PAUSE,
    #[doc = "0x10c8 - Specific Address Mask 1 Bottom (31 to 0 bits)"]
    pub mask_add1_bottom: MASK_ADD1_BOTTOM,
    #[doc = "0x10cc - Specific Address Mask 1 Top (47 to 32 bits)"]
    pub mask_add1_top: MASK_ADD1_TOP,
    #[doc = "0x10d0 - Receive DMA Data Buffer Address Mask"]
    pub dma_addr_or_mask: DMA_ADDR_OR_MASK,
    #[doc = "0x10d4 - PTP RX unicast IP destination address"]
    pub rx_ptp_unicast: RX_PTP_UNICAST,
    #[doc = "0x10d8 - PTP TX unicast IP destination address"]
    pub tx_ptp_unicast: TX_PTP_UNICAST,
    #[doc = "0x10dc - TSU timer comparison value nanoseconds"]
    pub tsu_nsec_cmp: TSU_NSEC_CMP,
    #[doc = "0x10e0 - TSU timer comparison value seconds (31 to 0 bits)"]
    pub tsu_sec_cmp: TSU_SEC_CMP,
    #[doc = "0x10e4 - TSU timer comparison value seconds (47 to 32 bits)"]
    pub tsu_msb_sec_cmp: TSU_MSB_SEC_CMP,
    #[doc = "0x10e8 - PTP Event Frame Transmitted Seconds Register (47 to 32 bits)"]
    pub tsu_ptp_tx_msb_sec: TSU_PTP_TX_MSB_SEC,
    #[doc = "0x10ec - PTP Event Frame Received Seconds Register (47 to 32 bits)"]
    pub tsu_ptp_rx_msb_sec: TSU_PTP_RX_MSB_SEC,
    #[doc = "0x10f0 - PTP Peer Event Frame Transmitted Seconds Register (47 to 32 bits)"]
    pub tsu_peer_tx_msb_sec: TSU_PEER_TX_MSB_SEC,
    #[doc = "0x10f4 - PTP Peer Event Frame Received Seconds Register (47 to 32 bits)"]
    pub tsu_peer_rx_msb_sec: TSU_PEER_RX_MSB_SEC,
    #[doc = "0x10f8 - The fill levels for the TX &amp; RX packet buffers can be read using this register, including the fill level for each queue in the TX direction."]
    pub dpram_fill_dbg: DPRAM_FILL_DBG,
    #[doc = "0x10fc - This register indicates a Cadence module identification number and module revision. The value of this register is read only as defined by `gem_revision_reg_value"]
    pub revision_reg: REVISION_REG,
    #[doc = "0x1100 - Octets Transmitted lower bits (31 to 0 bits)"]
    pub octets_txed_bottom: OCTETS_TXED_BOTTOM,
    #[doc = "0x1104 - Octets Transmitted higher bits (47 to 32 bits)"]
    pub octets_txed_top: OCTETS_TXED_TOP,
    #[doc = "0x1108 - Frames Transmitted"]
    pub frames_txed_ok: FRAMES_TXED_OK,
    #[doc = "0x110c - Broadcast Frames Transmitted"]
    pub broadcast_txed: BROADCAST_TXED,
    #[doc = "0x1110 - Multicast Frames Transmitted"]
    pub multicast_txed: MULTICAST_TXED,
    #[doc = "0x1114 - Pause Frames Transmitted"]
    pub pause_frames_txed: PAUSE_FRAMES_TXED,
    #[doc = "0x1118 - 64 Byte Frames Transmitted"]
    pub frames_txed_64: FRAMES_TXED_64,
    #[doc = "0x111c - 65 to 127 Byte Frames Transmitted"]
    pub frames_txed_65: FRAMES_TXED_65,
    #[doc = "0x1120 - 128 to 255 Byte Frames Transmitted"]
    pub frames_txed_128: FRAMES_TXED_128,
    #[doc = "0x1124 - 256 to 511 Byte Frames Transmitted"]
    pub frames_txed_256: FRAMES_TXED_256,
    #[doc = "0x1128 - 512 to 1023 Byte Frames Transmitted"]
    pub frames_txed_512: FRAMES_TXED_512,
    #[doc = "0x112c - 1024 to 1518 Byte Frames Transmitted"]
    pub frames_txed_1024: FRAMES_TXED_1024,
    #[doc = "0x1130 - Greater Than 1518 Byte Frames Transmitted"]
    pub frames_txed_1519: FRAMES_TXED_1519,
    #[doc = "0x1134 - Transmit Under Runs"]
    pub tx_underruns: TX_UNDERRUNS,
    #[doc = "0x1138 - Single Collision Frames. Presents in design but not support."]
    pub single_collisions: SINGLE_COLLISIONS,
    #[doc = "0x113c - Multiple Collision Frames. Presents in design but not support."]
    pub multiple_collisions: MULTIPLE_COLLISIONS,
    #[doc = "0x1140 - Excessive Collisions. Presents in design but not support."]
    pub excessive_collisions: EXCESSIVE_COLLISIONS,
    #[doc = "0x1144 - Late Collisions. Presents in design but not support."]
    pub late_collisions: LATE_COLLISIONS,
    #[doc = "0x1148 - Deferred Transmission Frames. Presents in design but not support."]
    pub deferred_frames: DEFERRED_FRAMES,
    #[doc = "0x114c - Carrier Sense Errors. Presents in design but not support."]
    pub crs_errors: CRS_ERRORS,
    #[doc = "0x1150 - Octets Received (31 to 0 bits)"]
    pub octets_rxed_bottom: OCTETS_RXED_BOTTOM,
    #[doc = "0x1154 - Octets Received (47 to 32 bits)"]
    pub octets_rxed_top: OCTETS_RXED_TOP,
    #[doc = "0x1158 - Frames Received"]
    pub frames_rxed_ok: FRAMES_RXED_OK,
    #[doc = "0x115c - Broadcast Frames Received"]
    pub broadcast_rxed: BROADCAST_RXED,
    #[doc = "0x1160 - Multicast Frames Received"]
    pub multicast_rxed: MULTICAST_RXED,
    #[doc = "0x1164 - Pause Frames Received"]
    pub pause_frames_rxed: PAUSE_FRAMES_RXED,
    #[doc = "0x1168 - 64 Byte Frames Received"]
    pub frames_rxed_64: FRAMES_RXED_64,
    #[doc = "0x116c - 65 to 127 Byte Frames Received"]
    pub frames_rxed_65: FRAMES_RXED_65,
    #[doc = "0x1170 - 128 to 255 Byte Frames Received"]
    pub frames_rxed_128: FRAMES_RXED_128,
    #[doc = "0x1174 - 256 to 511 Byte Frames Received"]
    pub frames_rxed_256: FRAMES_RXED_256,
    #[doc = "0x1178 - 512 to 1023 Byte Frames Received"]
    pub frames_rxed_512: FRAMES_RXED_512,
    #[doc = "0x117c - 1024 to 1518 Byte Frames Received"]
    pub frames_rxed_1024: FRAMES_RXED_1024,
    #[doc = "0x1180 - 1519 to maximum Byte Frames Received"]
    pub frames_rxed_1519: FRAMES_RXED_1519,
    #[doc = "0x1184 - Undersized Frames Received"]
    pub undersize_frames: UNDERSIZE_FRAMES,
    #[doc = "0x1188 - Oversize Frames Received"]
    pub excessive_rx_length: EXCESSIVE_RX_LENGTH,
    #[doc = "0x118c - Jabbers Received"]
    pub rx_jabbers: RX_JABBERS,
    #[doc = "0x1190 - Frame Check Sequence Errors"]
    pub fcs_errors: FCS_ERRORS,
    #[doc = "0x1194 - Length Field Frame Errors"]
    pub rx_length_errors: RX_LENGTH_ERRORS,
    #[doc = "0x1198 - Receive Symbol Errors"]
    pub rx_symbol_errors: RX_SYMBOL_ERRORS,
    #[doc = "0x119c - Alignment Errors"]
    pub alignment_errors: ALIGNMENT_ERRORS,
    #[doc = "0x11a0 - Receive Resource Errors"]
    pub rx_resource_errors: RX_RESOURCE_ERRORS,
    #[doc = "0x11a4 - Receive Overruns"]
    pub rx_overruns: RX_OVERRUNS,
    #[doc = "0x11a8 - IP Header Checksum Errors"]
    pub rx_ip_ck_errors: RX_IP_CK_ERRORS,
    #[doc = "0x11ac - TCP Checksum Errors"]
    pub rx_tcp_ck_errors: RX_TCP_CK_ERRORS,
    #[doc = "0x11b0 - UDP Checksum Errors"]
    pub rx_udp_ck_errors: RX_UDP_CK_ERRORS,
    #[doc = "0x11b4 - Receive DMA Flushed Packets"]
    pub auto_flushed_pkts: AUTO_FLUSHED_PKTS,
    _reserved104: [u8; 0x04],
    #[doc = "0x11bc - 1588 Timer Increment Register sub nsec"]
    pub tsu_timer_incr_sub_nsec: TSU_TIMER_INCR_SUB_NSEC,
    #[doc = "0x11c0 - 1588 Timer Seconds Register (47 to 32 bits)"]
    pub tsu_timer_msb_sec: TSU_TIMER_MSB_SEC,
    #[doc = "0x11c4 - 1588 Timer Sync Strobe Seconds Register (47 to 32 bits)"]
    pub tsu_strobe_msb_sec: TSU_STROBE_MSB_SEC,
    #[doc = "0x11c8 - 1588 Timer Sync Strobe Seconds Register (31 to 0 bits)"]
    pub tsu_strobe_sec: TSU_STROBE_SEC,
    #[doc = "0x11cc - 1588 Timer Sync Strobe Nanoseconds Register"]
    pub tsu_strobe_nsec: TSU_STROBE_NSEC,
    #[doc = "0x11d0 - 1588 Timer Seconds Register (31 to 0 bits)"]
    pub tsu_timer_sec: TSU_TIMER_SEC,
    #[doc = "0x11d4 - 1588 Timer Nanoseconds Register"]
    pub tsu_timer_nsec: TSU_TIMER_NSEC,
    #[doc = "0x11d8 - This register is used to adjust the value of the timer in the TSU. It allows an integral number of nanoseconds to be added or subtracted from the timer in a one-off operation. This register returns all zeroes when read."]
    pub tsu_timer_adjust: TSU_TIMER_ADJUST,
    #[doc = "0x11dc - 1588 Timer Increment Register"]
    pub tsu_timer_incr: TSU_TIMER_INCR,
    #[doc = "0x11e0 - PTP Event Frame Transmitted Seconds Register (31 to 0 bits)"]
    pub tsu_ptp_tx_sec: TSU_PTP_TX_SEC,
    #[doc = "0x11e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub tsu_ptp_tx_nsec: TSU_PTP_TX_NSEC,
    #[doc = "0x11e8 - PTP Event Frame Received Seconds Register (31 to 0 bits)"]
    pub tsu_ptp_rx_sec: TSU_PTP_RX_SEC,
    #[doc = "0x11ec - PTP Event Frame Received Nanoseconds Register"]
    pub tsu_ptp_rx_nsec: TSU_PTP_RX_NSEC,
    #[doc = "0x11f0 - PTP Peer Event Frame Transmitted Seconds Register (31 to 0 bits)"]
    pub tsu_peer_tx_sec: TSU_PEER_TX_SEC,
    #[doc = "0x11f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub tsu_peer_tx_nsec: TSU_PEER_TX_NSEC,
    #[doc = "0x11f8 - PTP Peer Event Frame Received Seconds Register (31 to 0 bits)"]
    pub tsu_peer_rx_sec: TSU_PEER_RX_SEC,
    #[doc = "0x11fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub tsu_peer_rx_nsec: TSU_PEER_RX_NSEC,
    #[doc = "0x1200 - Not presents. Access to the register returns AHB error."]
    pub pcs_control: PCS_CONTROL,
    #[doc = "0x1204 - Not presents. Access to the register returns AHB error."]
    pub pcs_status: PCS_STATUS,
    _reserved123: [u8; 0x08],
    #[doc = "0x1210 - Not presents. Access to the register returns AHB error."]
    pub pcs_an_adv: PCS_AN_ADV,
    #[doc = "0x1214 - Not presents. Access to the register returns AHB error."]
    pub pcs_an_lp_base: PCS_AN_LP_BASE,
    #[doc = "0x1218 - Not presents. Access to the register returns AHB error."]
    pub pcs_an_exp: PCS_AN_EXP,
    #[doc = "0x121c - Not presents. Access to the register returns AHB error."]
    pub pcs_an_np_tx: PCS_AN_NP_TX,
    #[doc = "0x1220 - Not presents. Access to the register returns AHB error."]
    pub pcs_an_lp_np: PCS_AN_LP_NP,
    _reserved128: [u8; 0x18],
    #[doc = "0x123c - Not presents. Access to the register returns AHB error."]
    pub pcs_an_ext_status: PCS_AN_EXT_STATUS,
    _reserved129: [u8; 0x20],
    #[doc = "0x1260 - Transmit Pause Quantum Register 1"]
    pub tx_pause_quantum1: TX_PAUSE_QUANTUM1,
    #[doc = "0x1264 - Transmit Pause Quantum Register 2"]
    pub tx_pause_quantum2: TX_PAUSE_QUANTUM2,
    #[doc = "0x1268 - Transmit Pause Quantum Register 3"]
    pub tx_pause_quantum3: TX_PAUSE_QUANTUM3,
    _reserved132: [u8; 0x04],
    #[doc = "0x1270 - Received LPI transitions"]
    pub rx_lpi: RX_LPI,
    #[doc = "0x1274 - Received LPI time"]
    pub rx_lpi_time: RX_LPI_TIME,
    #[doc = "0x1278 - Transmit LPI transitions"]
    pub tx_lpi: TX_LPI,
    #[doc = "0x127c - Transmit LPI time"]
    pub tx_lpi_time: TX_LPI_TIME,
    #[doc = "0x1280 - The GEM_GXL(3PIP) has many parameterisation options to configure the IP during compilation stage. This is achieved using Verilog define compiler directives in an include file called mxeth_defs.v."]
    pub designcfg_debug1: DESIGNCFG_DEBUG1,
    #[doc = "0x1284 - Design Configuration Register 2"]
    pub designcfg_debug2: DESIGNCFG_DEBUG2,
    #[doc = "0x1288 - Design Configuration Register 3"]
    pub designcfg_debug3: DESIGNCFG_DEBUG3,
    #[doc = "0x128c - Design Configuration Register 4"]
    pub designcfg_debug4: DESIGNCFG_DEBUG4,
    #[doc = "0x1290 - Design Configuration Register 5"]
    pub designcfg_debug5: DESIGNCFG_DEBUG5,
    #[doc = "0x1294 - Design Configuration Register 6"]
    pub designcfg_debug6: DESIGNCFG_DEBUG6,
    #[doc = "0x1298 - Design Configuration Register 7"]
    pub designcfg_debug7: DESIGNCFG_DEBUG7,
    #[doc = "0x129c - Design Configuration Register 8"]
    pub designcfg_debug8: DESIGNCFG_DEBUG8,
    #[doc = "0x12a0 - Design Configuration Register 9"]
    pub designcfg_debug9: DESIGNCFG_DEBUG9,
    #[doc = "0x12a4 - Design Configuration Register 10"]
    pub designcfg_debug10: DESIGNCFG_DEBUG10,
    _reserved146: [u8; 0x58],
    #[doc = "0x1300 - Specific address registers 5 ~ 36 doesn't present. Access to the register returns AHB error."]
    pub spec_add5_bottom: SPEC_ADD5_BOTTOM,
    #[doc = "0x1304 - Specific address registers 5 ~ 36 doesn't present. Access to the register returns AHB error."]
    pub spec_add5_top: SPEC_ADD5_TOP,
    _reserved148: [u8; 0xf0],
    #[doc = "0x13f8 - Not presents."]
    pub spec_add36_bottom: SPEC_ADD36_BOTTOM,
    #[doc = "0x13fc - Not presents."]
    pub spec_add36_top: SPEC_ADD36_TOP,
    #[doc = "0x1400 - Priority queue Interrupt Status Register"]
    pub int_q1_status: INT_Q1_STATUS,
    #[doc = "0x1404 - Priority queue Interrupt Status Register"]
    pub int_q2_status: INT_Q2_STATUS,
    #[doc = "0x1408 - int_q3_status to int_q15_status doesn't present. Access to the register returns AHB error."]
    pub int_q3_status: INT_Q3_STATUS,
    _reserved153: [u8; 0x2c],
    #[doc = "0x1438 - Not presents."]
    pub int_q15_status: INT_Q15_STATUS,
    _reserved154: [u8; 0x04],
    #[doc = "0x1440 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
    pub transmit_q1_ptr: TRANSMIT_Q1_PTR,
    #[doc = "0x1444 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
    pub transmit_q2_ptr: TRANSMIT_Q2_PTR,
    #[doc = "0x1448 - transmit_q3_ptr to transmit_q15_ptr doesn't present. Access to the register returns AHB error."]
    pub transmit_q3_ptr: TRANSMIT_Q3_PTR,
    _reserved157: [u8; 0x2c],
    #[doc = "0x1478 - Not presents."]
    pub transmit_q15_ptr: TRANSMIT_Q15_PTR,
    _reserved158: [u8; 0x04],
    #[doc = "0x1480 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
    pub receive_q1_ptr: RECEIVE_Q1_PTR,
    #[doc = "0x1484 - This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
    pub receive_q2_ptr: RECEIVE_Q2_PTR,
    #[doc = "0x1488 - Not presents. Start address register doesn't present for queue3 ~ queue7."]
    pub receive_q3_ptr: RECEIVE_Q3_PTR,
    _reserved161: [u8; 0x0c],
    #[doc = "0x1498 - Not presents."]
    pub receive_q7_ptr: RECEIVE_Q7_PTR,
    _reserved162: [u8; 0x04],
    #[doc = "0x14a0 - Receive Buffer queue 1 Size"]
    pub dma_rxbuf_size_q1: DMA_RXBUF_SIZE_Q1,
    #[doc = "0x14a4 - Receive Buffer queue 2 Size"]
    pub dma_rxbuf_size_q2: DMA_RXBUF_SIZE_Q2,
    #[doc = "0x14a8 - dma_rxbuf_size_q3 to dma_rxbuf_size_q7 doesn't present."]
    pub dma_rxbuf_size_q3: DMA_RXBUF_SIZE_Q3,
    _reserved165: [u8; 0x0c],
    #[doc = "0x14b8 - Not presents."]
    pub dma_rxbuf_size_q7: DMA_RXBUF_SIZE_Q7,
    #[doc = "0x14bc - The IdleSlope value is defined as the rate of change of credit when a packet is waiting to be sent. This must not exceed the portTransmitRate which is dependent on the speed of operation, eg, portTranmsitRate. 1Gb/s = 32'h07735940 (125 Mbytes/s), 100Mb/sec = 32'h017D7840 (25 Mnibbles/s), 10Mb/sec = 32'h002625A0 (2.5 Mnibbles/s). If 50 percent of bandwidth was to be allocated to a particular queue in 1Gb/sec mode then the IdleSlope value for that queue would be calculated as 32'h07735940/2. Note that Credit-Based Shaping should be disabled prior to updating the IdleSlope values. As another example, for a 1722 audio packet with a payload of 6 samples per channel, the packet size would be 7 (preamble) + 1 (SFD) + 50 (packet header) + 6x4x2(payload) + 4 (CRC) = 110 bytes. For a rate of 8000 packets per second, the desired rate would 110 x 8000 bytes per second, so the programmed idleSlope value would be 880000 for a 1Gb/s connection, or 1760000 for a 100Mb/s or 10Mbs connection. See Figure 6.3 in the IEEE 1722 standard. In practice, the actual transmission rate will be vary slightly from that calculated. In this case, the idleSlope value should be recalibrated based on the variance between the measured and expected rate, and in this case very accurate transmission rates can be achieved."]
    pub cbs_control: CBS_CONTROL,
    #[doc = "0x14c0 - queue A is the highest priority queue. This would be queue 8 in an 8 queue configuration."]
    pub cbs_idleslope_q_a: CBS_IDLESLOPE_Q_A,
    #[doc = "0x14c4 - queue B is the 2nd highest priority queue. This would be queue 7 in an 8 queue configuration."]
    pub cbs_idleslope_q_b: CBS_IDLESLOPE_Q_B,
    #[doc = "0x14c8 - Upper 32 bits of transmit buffer descriptor queue base address."]
    pub upper_tx_q_base_addr: UPPER_TX_Q_BASE_ADDR,
    #[doc = "0x14cc - TX BD control register"]
    pub tx_bd_control: TX_BD_CONTROL,
    #[doc = "0x14d0 - RX BD control register"]
    pub rx_bd_control: RX_BD_CONTROL,
    #[doc = "0x14d4 - Upper 32 bits of receive buffer descriptor queue base address."]
    pub upper_rx_q_base_addr: UPPER_RX_Q_BASE_ADDR,
    _reserved173: [u8; 0x08],
    #[doc = "0x14e0 - Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_10m 12'h4e0 // 10M Port TX Rate *** HIDDEN Register ***'. Default value of cbs related hidden registers (0x14E0~0x14E8) are depicted in cbs_control register."]
    pub hidden_reg0: HIDDEN_REG0,
    #[doc = "0x14e4 - Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_100m 12'h4e4 // 100M Port TX Rate *** HIDDEN Register ***'"]
    pub hidden_reg1: HIDDEN_REG1,
    #[doc = "0x14e8 - Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_1g 12'h4e8 // 1G Port TX Rate *** HIDDEN Register ***'"]
    pub hidden_reg2: HIDDEN_REG2,
    #[doc = "0x14ec - Hidden registers defined in edma_defs.v '`define gem_wd_counter 12'h4ec // *** HIDDEN Register ***'."]
    pub hidden_reg3: HIDDEN_REG3,
    _reserved177: [u8; 0x08],
    #[doc = "0x14f8 - Hidden registers defined in edma_defs.v '`define gem_axi_tx_full_threshold0 12'h4f8 // AXI full threshold setting *** HIDDEN Register ***'. Note. When using AXI mode with a single port ram ( gem_spram == 1) mode and a 32b dma bus width ( gem_dma_bus_width == 32 or bits 22 to 21 of the network_config register are set to 0) the AXI hidden registers (0x14F8 and 0x14FC) need to be updated (these registers are used for fine tuning AXI dma bursts and normally should not be touched)."]
    pub hidden_reg4: HIDDEN_REG4,
    #[doc = "0x14fc - Hidden registers defined in edma_defs.v '`define gem_axi_tx_full_threshold1 12'h4fc // AXI full threshold setting *** HIDDEN Register ***'."]
    pub hidden_reg5: HIDDEN_REG5,
    #[doc = "0x1500 - Screening type 1 registers are used to allocate up to 16 priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11 to 4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27 to 12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 2 to 0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C."]
    pub screening_type_1_register_0: SCREENING_TYPE_1_REGISTER_0,
    #[doc = "0x1504 - screening type 1 register 1, same as screening_type_1_register_0"]
    pub screening_type_1_register_1: SCREENING_TYPE_1_REGISTER_1,
    #[doc = "0x1508 - screening type 1 register 2, same as screening_type_1_register_0"]
    pub screening_type_1_register_2: SCREENING_TYPE_1_REGISTER_2,
    #[doc = "0x150c - screening type 1 register 3, same as screening_type_1_register_0"]
    pub screening_type_1_register_3: SCREENING_TYPE_1_REGISTER_3,
    #[doc = "0x1510 - screening type 1 register 4, same as screening_type_1_register_0"]
    pub screening_type_1_register_4: SCREENING_TYPE_1_REGISTER_4,
    #[doc = "0x1514 - screening type 1 register 5, same as screening_type_1_register_0"]
    pub screening_type_1_register_5: SCREENING_TYPE_1_REGISTER_5,
    #[doc = "0x1518 - screening type 1 register 6, same as screening_type_1_register_0"]
    pub screening_type_1_register_6: SCREENING_TYPE_1_REGISTER_6,
    #[doc = "0x151c - screening type 1 register 7, same as screening_type_1_register_0"]
    pub screening_type_1_register_7: SCREENING_TYPE_1_REGISTER_7,
    #[doc = "0x1520 - screening type 1 register 8, same as screening_type_1_register_0"]
    pub screening_type_1_register_8: SCREENING_TYPE_1_REGISTER_8,
    #[doc = "0x1524 - screening type 1 register 9, same as screening_type_1_register_0"]
    pub screening_type_1_register_9: SCREENING_TYPE_1_REGISTER_9,
    #[doc = "0x1528 - screening type 1 register 10, same as screening_type_1_register_0"]
    pub screening_type_1_register_10: SCREENING_TYPE_1_REGISTER_10,
    #[doc = "0x152c - screening type 1 register 11, same as screening_type_1_register_0"]
    pub screening_type_1_register_11: SCREENING_TYPE_1_REGISTER_11,
    #[doc = "0x1530 - screening type 1 register 12, same as screening_type_1_register_0"]
    pub screening_type_1_register_12: SCREENING_TYPE_1_REGISTER_12,
    #[doc = "0x1534 - screening type 1 register 13, same as screening_type_1_register_0"]
    pub screening_type_1_register_13: SCREENING_TYPE_1_REGISTER_13,
    #[doc = "0x1538 - screening type 1 register 14, same as screening_type_1_register_0"]
    pub screening_type_1_register_14: SCREENING_TYPE_1_REGISTER_14,
    #[doc = "0x153c - screening type 1 register 15, same as screening_type_1_register_0"]
    pub screening_type_1_register_15: SCREENING_TYPE_1_REGISTER_15,
    #[doc = "0x1540 - Screener Type 2 match registers operate independently of screener type 1 registers and offer additional match capabilities, extending the capabilities into vendor specific protocols."]
    pub screening_type_2_register_0: SCREENING_TYPE_2_REGISTER_0,
    #[doc = "0x1544 - screening type 2 register 1, same as screening_type_2_register_0"]
    pub screening_type_2_register_1: SCREENING_TYPE_2_REGISTER_1,
    #[doc = "0x1548 - screening type 2 register 2, same as screening_type_2_register_0"]
    pub screening_type_2_register_2: SCREENING_TYPE_2_REGISTER_2,
    #[doc = "0x154c - screening type 2 register 3, same as screening_type_2_register_0"]
    pub screening_type_2_register_3: SCREENING_TYPE_2_REGISTER_3,
    #[doc = "0x1550 - screening type 2 register 4, same as screening_type_2_register_0"]
    pub screening_type_2_register_4: SCREENING_TYPE_2_REGISTER_4,
    #[doc = "0x1554 - screening type 2 register 5, same as screening_type_2_register_0"]
    pub screening_type_2_register_5: SCREENING_TYPE_2_REGISTER_5,
    #[doc = "0x1558 - screening type 2 register 6, same as screening_type_2_register_0"]
    pub screening_type_2_register_6: SCREENING_TYPE_2_REGISTER_6,
    #[doc = "0x155c - screening type 2 register 7, same as screening_type_2_register_0"]
    pub screening_type_2_register_7: SCREENING_TYPE_2_REGISTER_7,
    #[doc = "0x1560 - screening type 2 register 8, same as screening_type_2_register_0"]
    pub screening_type_2_register_8: SCREENING_TYPE_2_REGISTER_8,
    #[doc = "0x1564 - screening type 2 register 9, same as screening_type_2_register_0"]
    pub screening_type_2_register_9: SCREENING_TYPE_2_REGISTER_9,
    #[doc = "0x1568 - screening type 2 register 10, same as screening_type_2_register_0"]
    pub screening_type_2_register_10: SCREENING_TYPE_2_REGISTER_10,
    #[doc = "0x156c - screening type 2 register 11, same as screening_type_2_register_0"]
    pub screening_type_2_register_11: SCREENING_TYPE_2_REGISTER_11,
    #[doc = "0x1570 - screening type 2 register 12, same as screening_type_2_register_0"]
    pub screening_type_2_register_12: SCREENING_TYPE_2_REGISTER_12,
    #[doc = "0x1574 - screening type 2 register 13, same as screening_type_2_register_0"]
    pub screening_type_2_register_13: SCREENING_TYPE_2_REGISTER_13,
    #[doc = "0x1578 - screening type 2 register 14, same as screening_type_2_register_0"]
    pub screening_type_2_register_14: SCREENING_TYPE_2_REGISTER_14,
    #[doc = "0x157c - screening type 2 register 15, same as screening_type_2_register_0"]
    pub screening_type_2_register_15: SCREENING_TYPE_2_REGISTER_15,
    #[doc = "0x1580 - This register controls the transmit scheduling algorithm the user can select for each active transmit queue. By default all queues are initialized to fixed priority, with the top indexed queue having overall priority"]
    pub tx_sched_ctrl: TX_SCHED_CTRL,
    _reserved212: [u8; 0x0c],
    #[doc = "0x1590 - This register holds the DWRR weighting value or the ETS bandwidth percentage value used by the transmit scheduler for queues 0 to 3."]
    pub bw_rate_limit_q0to3: BW_RATE_LIMIT_Q0TO3,
    #[doc = "0x1594 - Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
    pub bw_rate_limit_q4to7: BW_RATE_LIMIT_Q4TO7,
    #[doc = "0x1598 - Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
    pub bw_rate_limit_q8to11: BW_RATE_LIMIT_Q8TO11,
    #[doc = "0x159c - Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
    pub bw_rate_limit_q12to15: BW_RATE_LIMIT_Q12TO15,
    #[doc = "0x15a0 - This register allows the user to distribute the Transmit SRAM used by the DMA across the priority queues, for queues 0 to 7. The SRAM itself is split into a number of evenly sized segments (this is defined in the verilog configuration defs file - for the configuration used to generate this register description, the total number of segments was set to '16'). Those segments can then be freely distributed across the active queues, in powers of 2. I.e. a value of 0 would mean 1 segment has been allocated to the queue. A value of 1 would mean 2 segments, a value of 2 means 4 segments and so on. The reset values of these registers are defined in the configuration defs file."]
    pub tx_q_seg_alloc_q0to7: TX_Q_SEG_ALLOC_Q0TO7,
    #[doc = "0x15a4 - Not presents. Access to the register returns AHB error."]
    pub tx_q_seg_alloc_q8to15: TX_Q_SEG_ALLOC_Q8TO15,
    _reserved218: [u8; 0x18],
    #[doc = "0x15c0 - receive_q8_ptr to receive_q15_ptr doesn't present. Access to the register returns AHB error."]
    pub receive_q8_ptr: RECEIVE_Q8_PTR,
    _reserved219: [u8; 0x18],
    #[doc = "0x15dc - Not presents."]
    pub receive_q15_ptr: RECEIVE_Q15_PTR,
    #[doc = "0x15e0 - dma_rxbuf_size_q8 to dma_rxbuf_size_q15 doesn't present. Access to the register returns AHB error."]
    pub dma_rxbuf_size_q8: DMA_RXBUF_SIZE_Q8,
    _reserved221: [u8; 0x18],
    #[doc = "0x15fc - Not presents."]
    pub dma_rxbuf_size_q15: DMA_RXBUF_SIZE_Q15,
    #[doc = "0x1600 - At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
    pub int_q1_enable: INT_Q1_ENABLE,
    #[doc = "0x1604 - At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
    pub int_q2_enable: INT_Q2_ENABLE,
    #[doc = "0x1608 - int_q3_enable to int_q7_enable doesn't present. Access to the register returns AHB error."]
    pub int_q3_enable: INT_Q3_ENABLE,
    _reserved225: [u8; 0x0c],
    #[doc = "0x1618 - Not presents."]
    pub int_q7_enable: INT_Q7_ENABLE,
    _reserved226: [u8; 0x04],
    #[doc = "0x1620 - Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
    pub int_q1_disable: INT_Q1_DISABLE,
    #[doc = "0x1624 - Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
    pub int_q2_disable: INT_Q2_DISABLE,
    #[doc = "0x1628 - int_q3_disable to int_q7_disable doesn't present. Access to the register returns AHB error."]
    pub int_q3_disable: INT_Q3_DISABLE,
    _reserved229: [u8; 0x0c],
    #[doc = "0x1638 - Not presents."]
    pub int_q7_disable: INT_Q7_DISABLE,
    _reserved230: [u8; 0x04],
    #[doc = "0x1640 - The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
    pub int_q1_mask: INT_Q1_MASK,
    #[doc = "0x1644 - The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
    pub int_q2_mask: INT_Q2_MASK,
    #[doc = "0x1648 - int_q3_mask to int_q7_mask doesn't present. Access to the register returns AHB error."]
    pub int_q3_mask: INT_Q3_MASK,
    _reserved233: [u8; 0x0c],
    #[doc = "0x1658 - Not presents."]
    pub int_q7_mask: INT_Q7_MASK,
    _reserved234: [u8; 0x04],
    #[doc = "0x1660 - int_q8_enable to int_q15_enable doesn't present. Access to the register returns AHB error."]
    pub int_q8_enable: INT_Q8_ENABLE,
    _reserved235: [u8; 0x18],
    #[doc = "0x167c - Not presents."]
    pub int_q15_enable: INT_Q15_ENABLE,
    #[doc = "0x1680 - int_q8_disable to int_q15_disable doesn't present. Access to the register returns AHB error."]
    pub int_q8_disable: INT_Q8_DISABLE,
    _reserved237: [u8; 0x18],
    #[doc = "0x169c - Not presents."]
    pub int_q15_disable: INT_Q15_DISABLE,
    #[doc = "0x16a0 - int_q8_mask to int_q15_mask doesn't present. Access to the register returns AHB error."]
    pub int_q8_mask: INT_Q8_MASK,
    _reserved239: [u8; 0x18],
    #[doc = "0x16bc - Not presents."]
    pub int_q15_mask: INT_Q15_MASK,
    _reserved240: [u8; 0x20],
    #[doc = "0x16e0 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_0: SCREENING_TYPE_2_ETHERTYPE_REG_0,
    #[doc = "0x16e4 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_1: SCREENING_TYPE_2_ETHERTYPE_REG_1,
    #[doc = "0x16e8 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_2: SCREENING_TYPE_2_ETHERTYPE_REG_2,
    #[doc = "0x16ec - Ethertype Register"]
    pub screening_type_2_ethertype_reg_3: SCREENING_TYPE_2_ETHERTYPE_REG_3,
    #[doc = "0x16f0 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_4: SCREENING_TYPE_2_ETHERTYPE_REG_4,
    #[doc = "0x16f4 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_5: SCREENING_TYPE_2_ETHERTYPE_REG_5,
    #[doc = "0x16f8 - Ethertype Register"]
    pub screening_type_2_ethertype_reg_6: SCREENING_TYPE_2_ETHERTYPE_REG_6,
    #[doc = "0x16fc - Ethertype Register"]
    pub screening_type_2_ethertype_reg_7: SCREENING_TYPE_2_ETHERTYPE_REG_7,
    #[doc = "0x1700 - 'Compare A, B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value if the mask is enabled, is equal to the COMPARE Value. Either a 16 bit comparison or a 32 bit comparison is done. This selection is made via the associated compare word1 register bit 9. If a 16 bit comparison is selected, then a 16 bit mask is also available to the user to select which bits should be compared. If the 32 bit compare option is selected, then no mask is available. The byte at the OFFSET number of bytes from the index start is compared thru bits 7 to 0 of the configured VALUE. The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15 to 8 of the configured VALUE and so on. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the therType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc. Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value it's not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. '"]
    pub type2_compare_0_word_0: TYPE2_COMPARE_0_WORD_0,
    #[doc = "0x1704 - 'Type2 Compare Word 1'"]
    pub type2_compare_0_word_1: TYPE2_COMPARE_0_WORD_1,
    #[doc = "0x1708 - same as type2_compare_0_word_0"]
    pub type2_compare_1_word_0: TYPE2_COMPARE_1_WORD_0,
    #[doc = "0x170c - same as type2_compare_0_word_1"]
    pub type2_compare_1_word_1: TYPE2_COMPARE_1_WORD_1,
    #[doc = "0x1710 - same as type2_compare_0_word_0"]
    pub type2_compare_2_word_0: TYPE2_COMPARE_2_WORD_0,
    #[doc = "0x1714 - same as type2_compare_0_word_1"]
    pub type2_compare_2_word_1: TYPE2_COMPARE_2_WORD_1,
    #[doc = "0x1718 - same as type2_compare_0_word_0"]
    pub type2_compare_3_word_0: TYPE2_COMPARE_3_WORD_0,
    #[doc = "0x171c - same as type2_compare_0_word_1"]
    pub type2_compare_3_word_1: TYPE2_COMPARE_3_WORD_1,
    #[doc = "0x1720 - same as type2_compare_0_word_0"]
    pub type2_compare_4_word_0: TYPE2_COMPARE_4_WORD_0,
    #[doc = "0x1724 - same as type2_compare_0_word_1"]
    pub type2_compare_4_word_1: TYPE2_COMPARE_4_WORD_1,
    #[doc = "0x1728 - same as type2_compare_0_word_0"]
    pub type2_compare_5_word_0: TYPE2_COMPARE_5_WORD_0,
    #[doc = "0x172c - same as type2_compare_0_word_1"]
    pub type2_compare_5_word_1: TYPE2_COMPARE_5_WORD_1,
    #[doc = "0x1730 - same as type2_compare_0_word_0"]
    pub type2_compare_6_word_0: TYPE2_COMPARE_6_WORD_0,
    #[doc = "0x1734 - same as type2_compare_0_word_1"]
    pub type2_compare_6_word_1: TYPE2_COMPARE_6_WORD_1,
    #[doc = "0x1738 - same as type2_compare_0_word_0"]
    pub type2_compare_7_word_0: TYPE2_COMPARE_7_WORD_0,
    #[doc = "0x173c - same as type2_compare_0_word_1"]
    pub type2_compare_7_word_1: TYPE2_COMPARE_7_WORD_1,
    #[doc = "0x1740 - same as type2_compare_0_word_0"]
    pub type2_compare_8_word_0: TYPE2_COMPARE_8_WORD_0,
    #[doc = "0x1744 - same as type2_compare_0_word_1"]
    pub type2_compare_8_word_1: TYPE2_COMPARE_8_WORD_1,
    #[doc = "0x1748 - same as type2_compare_0_word_0"]
    pub type2_compare_9_word_0: TYPE2_COMPARE_9_WORD_0,
    #[doc = "0x174c - same as type2_compare_0_word_1"]
    pub type2_compare_9_word_1: TYPE2_COMPARE_9_WORD_1,
    #[doc = "0x1750 - same as type2_compare_0_word_0"]
    pub type2_compare_10_word_0: TYPE2_COMPARE_10_WORD_0,
    #[doc = "0x1754 - same as type2_compare_0_word_1"]
    pub type2_compare_10_word_1: TYPE2_COMPARE_10_WORD_1,
    #[doc = "0x1758 - same as type2_compare_0_word_0"]
    pub type2_compare_11_word_0: TYPE2_COMPARE_11_WORD_0,
    #[doc = "0x175c - same as type2_compare_0_word_1"]
    pub type2_compare_11_word_1: TYPE2_COMPARE_11_WORD_1,
    #[doc = "0x1760 - same as type2_compare_0_word_0"]
    pub type2_compare_12_word_0: TYPE2_COMPARE_12_WORD_0,
    #[doc = "0x1764 - same as type2_compare_0_word_1"]
    pub type2_compare_12_word_1: TYPE2_COMPARE_12_WORD_1,
    #[doc = "0x1768 - same as type2_compare_0_word_0"]
    pub type2_compare_13_word_0: TYPE2_COMPARE_13_WORD_0,
    #[doc = "0x176c - same as type2_compare_0_word_1"]
    pub type2_compare_13_word_1: TYPE2_COMPARE_13_WORD_1,
    #[doc = "0x1770 - same as type2_compare_0_word_0"]
    pub type2_compare_14_word_0: TYPE2_COMPARE_14_WORD_0,
    #[doc = "0x1774 - same as type2_compare_0_word_1"]
    pub type2_compare_14_word_1: TYPE2_COMPARE_14_WORD_1,
    #[doc = "0x1778 - same as type2_compare_0_word_0"]
    pub type2_compare_15_word_0: TYPE2_COMPARE_15_WORD_0,
    #[doc = "0x177c - same as type2_compare_0_word_1"]
    pub type2_compare_15_word_1: TYPE2_COMPARE_15_WORD_1,
    #[doc = "0x1780 - same as type2_compare_0_word_0"]
    pub type2_compare_16_word_0: TYPE2_COMPARE_16_WORD_0,
    #[doc = "0x1784 - same as type2_compare_0_word_1"]
    pub type2_compare_16_word_1: TYPE2_COMPARE_16_WORD_1,
    #[doc = "0x1788 - same as type2_compare_0_word_0"]
    pub type2_compare_17_word_0: TYPE2_COMPARE_17_WORD_0,
    #[doc = "0x178c - same as type2_compare_0_word_1"]
    pub type2_compare_17_word_1: TYPE2_COMPARE_17_WORD_1,
    #[doc = "0x1790 - same as type2_compare_0_word_0"]
    pub type2_compare_18_word_0: TYPE2_COMPARE_18_WORD_0,
    #[doc = "0x1794 - same as type2_compare_0_word_1"]
    pub type2_compare_18_word_1: TYPE2_COMPARE_18_WORD_1,
    #[doc = "0x1798 - same as type2_compare_0_word_0"]
    pub type2_compare_19_word_0: TYPE2_COMPARE_19_WORD_0,
    #[doc = "0x179c - same as type2_compare_0_word_1"]
    pub type2_compare_19_word_1: TYPE2_COMPARE_19_WORD_1,
    #[doc = "0x17a0 - same as type2_compare_0_word_0"]
    pub type2_compare_20_word_0: TYPE2_COMPARE_20_WORD_0,
    #[doc = "0x17a4 - same as type2_compare_0_word_1"]
    pub type2_compare_20_word_1: TYPE2_COMPARE_20_WORD_1,
    #[doc = "0x17a8 - same as type2_compare_0_word_0"]
    pub type2_compare_21_word_0: TYPE2_COMPARE_21_WORD_0,
    #[doc = "0x17ac - same as type2_compare_0_word_1"]
    pub type2_compare_21_word_1: TYPE2_COMPARE_21_WORD_1,
    #[doc = "0x17b0 - same as type2_compare_0_word_0"]
    pub type2_compare_22_word_0: TYPE2_COMPARE_22_WORD_0,
    #[doc = "0x17b4 - same as type2_compare_0_word_1"]
    pub type2_compare_22_word_1: TYPE2_COMPARE_22_WORD_1,
    #[doc = "0x17b8 - same as type2_compare_0_word_0"]
    pub type2_compare_23_word_0: TYPE2_COMPARE_23_WORD_0,
    #[doc = "0x17bc - same as type2_compare_0_word_1"]
    pub type2_compare_23_word_1: TYPE2_COMPARE_23_WORD_1,
    #[doc = "0x17c0 - same as type2_compare_0_word_0"]
    pub type2_compare_24_word_0: TYPE2_COMPARE_24_WORD_0,
    #[doc = "0x17c4 - same as type2_compare_0_word_1"]
    pub type2_compare_24_word_1: TYPE2_COMPARE_24_WORD_1,
    #[doc = "0x17c8 - same as type2_compare_0_word_0"]
    pub type2_compare_25_word_0: TYPE2_COMPARE_25_WORD_0,
    #[doc = "0x17cc - same as type2_compare_0_word_1"]
    pub type2_compare_25_word_1: TYPE2_COMPARE_25_WORD_1,
    #[doc = "0x17d0 - same as type2_compare_0_word_0"]
    pub type2_compare_26_word_0: TYPE2_COMPARE_26_WORD_0,
    #[doc = "0x17d4 - same as type2_compare_0_word_1"]
    pub type2_compare_26_word_1: TYPE2_COMPARE_26_WORD_1,
    #[doc = "0x17d8 - same as type2_compare_0_word_0"]
    pub type2_compare_27_word_0: TYPE2_COMPARE_27_WORD_0,
    #[doc = "0x17dc - same as type2_compare_0_word_1"]
    pub type2_compare_27_word_1: TYPE2_COMPARE_27_WORD_1,
    #[doc = "0x17e0 - same as type2_compare_0_word_0"]
    pub type2_compare_28_word_0: TYPE2_COMPARE_28_WORD_0,
    #[doc = "0x17e4 - same as type2_compare_0_word_1"]
    pub type2_compare_28_word_1: TYPE2_COMPARE_28_WORD_1,
    #[doc = "0x17e8 - same as type2_compare_0_word_0"]
    pub type2_compare_29_word_0: TYPE2_COMPARE_29_WORD_0,
    #[doc = "0x17ec - same as type2_compare_0_word_1"]
    pub type2_compare_29_word_1: TYPE2_COMPARE_29_WORD_1,
    #[doc = "0x17f0 - same as type2_compare_0_word_0"]
    pub type2_compare_30_word_0: TYPE2_COMPARE_30_WORD_0,
    #[doc = "0x17f4 - same as type2_compare_0_word_1"]
    pub type2_compare_30_word_1: TYPE2_COMPARE_30_WORD_1,
    #[doc = "0x17f8 - same as type2_compare_0_word_0"]
    pub type2_compare_31_word_0: TYPE2_COMPARE_31_WORD_0,
    #[doc = "0x17fc - same as type2_compare_0_word_1"]
    pub type2_compare_31_word_1: TYPE2_COMPARE_31_WORD_1,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "MXETH Control Register"]
pub mod ctl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "MXETH Status Register"]
pub mod status;
#[doc = "NETWORK_CONTROL (rw) register accessor: an alias for `Reg<NETWORK_CONTROL_SPEC>`"]
pub type NETWORK_CONTROL = crate::Reg<network_control::NETWORK_CONTROL_SPEC>;
#[doc = "The network control register contains general MAC control functions for both receiver and transmitter."]
pub mod network_control;
#[doc = "NETWORK_CONFIG (rw) register accessor: an alias for `Reg<NETWORK_CONFIG_SPEC>`"]
pub type NETWORK_CONFIG = crate::Reg<network_config::NETWORK_CONFIG_SPEC>;
#[doc = "The network configuration register contains functions for setting the mode of operation for the Gigabit Ethernet MAC."]
pub mod network_config;
#[doc = "NETWORK_STATUS (r) register accessor: an alias for `Reg<NETWORK_STATUS_SPEC>`"]
pub type NETWORK_STATUS = crate::Reg<network_status::NETWORK_STATUS_SPEC>;
#[doc = "The network status register returns status information with respect to the PHY management interface."]
pub mod network_status;
#[doc = "USER_IO_REGISTER (r) register accessor: an alias for `Reg<USER_IO_REGISTER_SPEC>`"]
pub type USER_IO_REGISTER = crate::Reg<user_io_register::USER_IO_REGISTER_SPEC>;
#[doc = "Not presents. Access to the register will return AHB error."]
pub mod user_io_register;
#[doc = "DMA_CONFIG (rw) register accessor: an alias for `Reg<DMA_CONFIG_SPEC>`"]
pub type DMA_CONFIG = crate::Reg<dma_config::DMA_CONFIG_SPEC>;
#[doc = "DMA Configuration Register"]
pub mod dma_config;
#[doc = "TRANSMIT_STATUS (rw) register accessor: an alias for `Reg<TRANSMIT_STATUS_SPEC>`"]
pub type TRANSMIT_STATUS = crate::Reg<transmit_status::TRANSMIT_STATUS_SPEC>;
#[doc = "This register, when read, provides details of the status of a transmit. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register."]
pub mod transmit_status;
#[doc = "RECEIVE_Q_PTR (rw) register accessor: an alias for `Reg<RECEIVE_Q_PTR_SPEC>`"]
pub type RECEIVE_Q_PTR = crate::Reg<receive_q_ptr::RECEIVE_Q_PTR_SPEC>;
#[doc = "This register holds the start address of the receive buffer queue (receive buffers descriptor list). The receive buffer queue base address must be initialized before receive is enabled through bit 2 of the network control register. Once reception is enabled, any write to the receive buffer queue base address register is ignored. Reading this register returns the location of the descriptor currently being accessed. This value increments as buffers are used. Software should not use this register for determining where to remove received frames from the queue as it constantly changes as new frames are received. Software should instead work its way through the buffer descriptor queue checking the used bits. In terms of AMBA (AXI) operation, the receive descriptors are read from memory using a single 32bit AXI access. When the datapath is configured at 64bit, the receive descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is written to using a single 64bit AXI access."]
pub mod receive_q_ptr;
#[doc = "TRANSMIT_Q_PTR (rw) register accessor: an alias for `Reg<TRANSMIT_Q_PTR_SPEC>`"]
pub type TRANSMIT_Q_PTR = crate::Reg<transmit_q_ptr::TRANSMIT_Q_PTR_SPEC>;
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit, the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
pub mod transmit_q_ptr;
#[doc = "RECEIVE_STATUS (rw) register accessor: an alias for `Reg<RECEIVE_STATUS_SPEC>`"]
pub type RECEIVE_STATUS = crate::Reg<receive_status::RECEIVE_STATUS_SPEC>;
#[doc = "This register, when read provides details of the status of a receive. Once read, individual bits may be cleared by writing 1 to them. It is not possible to set a bit to 1 by writing to the register."]
pub mod receive_status;
#[doc = "INT_STATUS (rw) register accessor: an alias for `Reg<INT_STATUS_SPEC>`"]
pub type INT_STATUS = crate::Reg<int_status::INT_STATUS_SPEC>;
#[doc = "If not configured for priority queueing, the GEM generates a single interrupt. This register indicates the source of this interrupt. The corresponding bit in the mask register must be clear for a bit to be set. If any bit is set in this register the ethernet_int signal will be asserted. For test purposes each bit can be set or reset by writing to the interrupt mask register. The default configuration is shown below whereby all bits are reset to zero on read. Changing the validity of the `gem_irq_read_clear define will instead require a one to be written to the appropriate bit in order to clear it. In this mode reading has no affect on the status of the bit."]
pub mod int_status;
#[doc = "INT_ENABLE (w) register accessor: an alias for `Reg<INT_ENABLE_SPEC>`"]
pub type INT_ENABLE = crate::Reg<int_enable::INT_ENABLE_SPEC>;
#[doc = "At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
pub mod int_enable;
#[doc = "INT_DISABLE (rw) register accessor: an alias for `Reg<INT_DISABLE_SPEC>`"]
pub type INT_DISABLE = crate::Reg<int_disable::INT_DISABLE_SPEC>;
#[doc = "Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
pub mod int_disable;
#[doc = "INT_MASK (r) register accessor: an alias for `Reg<INT_MASK_SPEC>`"]
pub type INT_MASK = crate::Reg<int_mask::INT_MASK_SPEC>;
#[doc = "The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
pub mod int_mask;
#[doc = "PHY_MANAGEMENT (rw) register accessor: an alias for `Reg<PHY_MANAGEMENT_SPEC>`"]
pub type PHY_MANAGEMENT = crate::Reg<phy_management::PHY_MANAGEMENT_SPEC>;
#[doc = "The PHY maintenance register is implemented as a shift register. Writing to the register starts a shift operation which is signalled as complete when bit-2 is set in the network status register. It takes about 2000 pclk cycles to complete, when MDC is set for pclk divide by 32 in the network configuration register. An interrupt is generated upon completion. During this time, the MSB of the register is output on the MDIO pin and the LSB updated from the MDIO pin with each MDC cycle. This causes transmission of a PHY management frame on MDIO. See Section 22.2.4.5 of the IEEE 802.3 standard. Reading during the shift operation will return the current contents of the shift register. At the end of management operation, the bits will have shifted back to their original locations. For a read operation, the data bits will be updated with data read from the PHY. It is important to write the correct values to the register to ensure a valid PHY management frame is produced. The MDIO interface can read IEEE 802.3 clause 45 PHYs as well as clause 22 PHYs. To read clause 45 PHYs, bit 30 should be written with a 0 rather than a 1. For a description of MDC generation, see Network Configuration Register."]
pub mod phy_management;
#[doc = "PAUSE_TIME (r) register accessor: an alias for `Reg<PAUSE_TIME_SPEC>`"]
pub type PAUSE_TIME = crate::Reg<pause_time::PAUSE_TIME_SPEC>;
#[doc = "Received Pause Quantum Register"]
pub mod pause_time;
#[doc = "TX_PAUSE_QUANTUM (rw) register accessor: an alias for `Reg<TX_PAUSE_QUANTUM_SPEC>`"]
pub type TX_PAUSE_QUANTUM = crate::Reg<tx_pause_quantum::TX_PAUSE_QUANTUM_SPEC>;
#[doc = "Transmit Pause Quantum Register"]
pub mod tx_pause_quantum;
#[doc = "PBUF_TXCUTTHRU (rw) register accessor: an alias for `Reg<PBUF_TXCUTTHRU_SPEC>`"]
pub type PBUF_TXCUTTHRU = crate::Reg<pbuf_txcutthru::PBUF_TXCUTTHRU_SPEC>;
#[doc = "Partial store and forward is only applicable when using the DMA configured in SRAM based packet buffer mode. It is also not available when using multi buffer frames. TX Partial Store and Forward"]
pub mod pbuf_txcutthru;
#[doc = "PBUF_RXCUTTHRU (rw) register accessor: an alias for `Reg<PBUF_RXCUTTHRU_SPEC>`"]
pub type PBUF_RXCUTTHRU = crate::Reg<pbuf_rxcutthru::PBUF_RXCUTTHRU_SPEC>;
#[doc = "RX Partial Store and Forward"]
pub mod pbuf_rxcutthru;
#[doc = "JUMBO_MAX_LENGTH (rw) register accessor: an alias for `Reg<JUMBO_MAX_LENGTH_SPEC>`"]
pub type JUMBO_MAX_LENGTH = crate::Reg<jumbo_max_length::JUMBO_MAX_LENGTH_SPEC>;
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbo_max_length;
#[doc = "EXTERNAL_FIFO_INTERFACE (r) register accessor: an alias for `Reg<EXTERNAL_FIFO_INTERFACE_SPEC>`"]
pub type EXTERNAL_FIFO_INTERFACE =
    crate::Reg<external_fifo_interface::EXTERNAL_FIFO_INTERFACE_SPEC>;
#[doc = "Not presents."]
pub mod external_fifo_interface;
#[doc = "AXI_MAX_PIPELINE (rw) register accessor: an alias for `Reg<AXI_MAX_PIPELINE_SPEC>`"]
pub type AXI_MAX_PIPELINE = crate::Reg<axi_max_pipeline::AXI_MAX_PIPELINE_SPEC>;
#[doc = "Used to set the maximum amount of outstanding transactions on the AXI bus between AR / R channels and AW / W channels. Cannot be more than the depth of the configured AXI pipeline FIFO (defined in verilog defs.v)"]
pub mod axi_max_pipeline;
#[doc = "RSC_CONTROL (r) register accessor: an alias for `Reg<RSC_CONTROL_SPEC>`"]
pub type RSC_CONTROL = crate::Reg<rsc_control::RSC_CONTROL_SPEC>;
#[doc = "Not presents. Access to the register will return AHB error."]
pub mod rsc_control;
#[doc = "INT_MODERATION (rw) register accessor: an alias for `Reg<INT_MODERATION_SPEC>`"]
pub type INT_MODERATION = crate::Reg<int_moderation::INT_MODERATION_SPEC>;
#[doc = "Used to moderate the number of transmit and receive complete interrupts issued. With interrupt moderation enabled receive and transmit interrupts are not generated immediately a frame is transmitted or received. Instead when a receive or transmit event occurs a timer is started and the interrupt is asserted after it times out. This limits the frequency with which the CPU receives interrupts. When interrupt moderation is enabled interrupt status bit one is always used for receive and bit 7 is always used for transmit even when priority queuing is enabled. With interrupt moderation 800ns periods are counted. GEM determines what constitutes an 800ns period by looking at the tbi (bit 11), gigabit bit (10) and speed (bit 0) bits in the network configuration register and counting tx_clk cycles. Bit 0 needs to be set to 1 for 100M operation."]
pub mod int_moderation;
#[doc = "SYS_WAKE_TIME (rw) register accessor: an alias for `Reg<SYS_WAKE_TIME_SPEC>`"]
pub type SYS_WAKE_TIME = crate::Reg<sys_wake_time::SYS_WAKE_TIME_SPEC>;
#[doc = "Used to pause transmission after deassertion of tx_lpi_en. Each unit in this register corresponds to 64ns in gigabit mode, 320ns in 100M mode and 3200ns at 10M. After tx_lpi_en is deasserted transmission will pause for the set time."]
pub mod sys_wake_time;
#[doc = "HASH_BOTTOM (rw) register accessor: an alias for `Reg<HASH_BOTTOM_SPEC>`"]
pub type HASH_BOTTOM = crate::Reg<hash_bottom::HASH_BOTTOM_SPEC>;
#[doc = "The unicast hash enable and the multicast hash enable bits in the network configuration register enable the reception of hash matched frames. Hash Register Bottom (31 to 0 bits)"]
pub mod hash_bottom;
#[doc = "HASH_TOP (rw) register accessor: an alias for `Reg<HASH_TOP_SPEC>`"]
pub type HASH_TOP = crate::Reg<hash_top::HASH_TOP_SPEC>;
#[doc = "Hash Register Top (63 to 32 bits)"]
pub mod hash_top;
#[doc = "SPEC_ADD1_BOTTOM (rw) register accessor: an alias for `Reg<SPEC_ADD1_BOTTOM_SPEC>`"]
pub type SPEC_ADD1_BOTTOM = crate::Reg<spec_add1_bottom::SPEC_ADD1_BOTTOM_SPEC>;
#[doc = "The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
pub mod spec_add1_bottom;
#[doc = "SPEC_ADD1_TOP (rw) register accessor: an alias for `Reg<SPEC_ADD1_TOP_SPEC>`"]
pub type SPEC_ADD1_TOP = crate::Reg<spec_add1_top::SPEC_ADD1_TOP_SPEC>;
#[doc = "Specific Address Top"]
pub mod spec_add1_top;
#[doc = "SPEC_ADD2_BOTTOM (rw) register accessor: an alias for `Reg<SPEC_ADD2_BOTTOM_SPEC>`"]
pub type SPEC_ADD2_BOTTOM = crate::Reg<spec_add2_bottom::SPEC_ADD2_BOTTOM_SPEC>;
#[doc = "The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
pub mod spec_add2_bottom;
#[doc = "SPEC_ADD2_TOP (rw) register accessor: an alias for `Reg<SPEC_ADD2_TOP_SPEC>`"]
pub type SPEC_ADD2_TOP = crate::Reg<spec_add2_top::SPEC_ADD2_TOP_SPEC>;
#[doc = "Specific Address Top"]
pub mod spec_add2_top;
#[doc = "SPEC_ADD3_BOTTOM (rw) register accessor: an alias for `Reg<SPEC_ADD3_BOTTOM_SPEC>`"]
pub type SPEC_ADD3_BOTTOM = crate::Reg<spec_add3_bottom::SPEC_ADD3_BOTTOM_SPEC>;
#[doc = "The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
pub mod spec_add3_bottom;
#[doc = "SPEC_ADD3_TOP (rw) register accessor: an alias for `Reg<SPEC_ADD3_TOP_SPEC>`"]
pub type SPEC_ADD3_TOP = crate::Reg<spec_add3_top::SPEC_ADD3_TOP_SPEC>;
#[doc = "Specific Address Top"]
pub mod spec_add3_top;
#[doc = "SPEC_ADD4_BOTTOM (rw) register accessor: an alias for `Reg<SPEC_ADD4_BOTTOM_SPEC>`"]
pub type SPEC_ADD4_BOTTOM = crate::Reg<spec_add4_bottom::SPEC_ADD4_BOTTOM_SPEC>;
#[doc = "The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written."]
pub mod spec_add4_bottom;
#[doc = "SPEC_ADD4_TOP (rw) register accessor: an alias for `Reg<SPEC_ADD4_TOP_SPEC>`"]
pub type SPEC_ADD4_TOP = crate::Reg<spec_add4_top::SPEC_ADD4_TOP_SPEC>;
#[doc = "Specific Address Top"]
pub mod spec_add4_top;
#[doc = "SPEC_TYPE1 (rw) register accessor: an alias for `Reg<SPEC_TYPE1_SPEC>`"]
pub type SPEC_TYPE1 = crate::Reg<spec_type1::SPEC_TYPE1_SPEC>;
#[doc = "Type ID Match 1"]
pub mod spec_type1;
#[doc = "SPEC_TYPE2 (rw) register accessor: an alias for `Reg<SPEC_TYPE2_SPEC>`"]
pub type SPEC_TYPE2 = crate::Reg<spec_type2::SPEC_TYPE2_SPEC>;
#[doc = "Type ID Match 2"]
pub mod spec_type2;
#[doc = "SPEC_TYPE3 (rw) register accessor: an alias for `Reg<SPEC_TYPE3_SPEC>`"]
pub type SPEC_TYPE3 = crate::Reg<spec_type3::SPEC_TYPE3_SPEC>;
#[doc = "Type ID Match 3"]
pub mod spec_type3;
#[doc = "SPEC_TYPE4 (rw) register accessor: an alias for `Reg<SPEC_TYPE4_SPEC>`"]
pub type SPEC_TYPE4 = crate::Reg<spec_type4::SPEC_TYPE4_SPEC>;
#[doc = "Type ID Match 4"]
pub mod spec_type4;
#[doc = "WOL_REGISTER (rw) register accessor: an alias for `Reg<WOL_REGISTER_SPEC>`"]
pub type WOL_REGISTER = crate::Reg<wol_register::WOL_REGISTER_SPEC>;
#[doc = "Wake on LAN Register. Presents in design, but feature is not supported."]
pub mod wol_register;
#[doc = "STRETCH_RATIO (rw) register accessor: an alias for `Reg<STRETCH_RATIO_SPEC>`"]
pub type STRETCH_RATIO = crate::Reg<stretch_ratio::STRETCH_RATIO_SPEC>;
#[doc = "IPG stretch register"]
pub mod stretch_ratio;
#[doc = "STACKED_VLAN (rw) register accessor: an alias for `Reg<STACKED_VLAN_SPEC>`"]
pub type STACKED_VLAN = crate::Reg<stacked_vlan::STACKED_VLAN_SPEC>;
#[doc = "Stacked VLAN Register"]
pub mod stacked_vlan;
#[doc = "TX_PFC_PAUSE (rw) register accessor: an alias for `Reg<TX_PFC_PAUSE_SPEC>`"]
pub type TX_PFC_PAUSE = crate::Reg<tx_pfc_pause::TX_PFC_PAUSE_SPEC>;
#[doc = "Transmit PFC Pause Register"]
pub mod tx_pfc_pause;
#[doc = "MASK_ADD1_BOTTOM (rw) register accessor: an alias for `Reg<MASK_ADD1_BOTTOM_SPEC>`"]
pub type MASK_ADD1_BOTTOM = crate::Reg<mask_add1_bottom::MASK_ADD1_BOTTOM_SPEC>;
#[doc = "Specific Address Mask 1 Bottom (31 to 0 bits)"]
pub mod mask_add1_bottom;
#[doc = "MASK_ADD1_TOP (rw) register accessor: an alias for `Reg<MASK_ADD1_TOP_SPEC>`"]
pub type MASK_ADD1_TOP = crate::Reg<mask_add1_top::MASK_ADD1_TOP_SPEC>;
#[doc = "Specific Address Mask 1 Top (47 to 32 bits)"]
pub mod mask_add1_top;
#[doc = "DMA_ADDR_OR_MASK (rw) register accessor: an alias for `Reg<DMA_ADDR_OR_MASK_SPEC>`"]
pub type DMA_ADDR_OR_MASK = crate::Reg<dma_addr_or_mask::DMA_ADDR_OR_MASK_SPEC>;
#[doc = "Receive DMA Data Buffer Address Mask"]
pub mod dma_addr_or_mask;
#[doc = "RX_PTP_UNICAST (rw) register accessor: an alias for `Reg<RX_PTP_UNICAST_SPEC>`"]
pub type RX_PTP_UNICAST = crate::Reg<rx_ptp_unicast::RX_PTP_UNICAST_SPEC>;
#[doc = "PTP RX unicast IP destination address"]
pub mod rx_ptp_unicast;
#[doc = "TX_PTP_UNICAST (rw) register accessor: an alias for `Reg<TX_PTP_UNICAST_SPEC>`"]
pub type TX_PTP_UNICAST = crate::Reg<tx_ptp_unicast::TX_PTP_UNICAST_SPEC>;
#[doc = "PTP TX unicast IP destination address"]
pub mod tx_ptp_unicast;
#[doc = "TSU_NSEC_CMP (rw) register accessor: an alias for `Reg<TSU_NSEC_CMP_SPEC>`"]
pub type TSU_NSEC_CMP = crate::Reg<tsu_nsec_cmp::TSU_NSEC_CMP_SPEC>;
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsu_nsec_cmp;
#[doc = "TSU_SEC_CMP (rw) register accessor: an alias for `Reg<TSU_SEC_CMP_SPEC>`"]
pub type TSU_SEC_CMP = crate::Reg<tsu_sec_cmp::TSU_SEC_CMP_SPEC>;
#[doc = "TSU timer comparison value seconds (31 to 0 bits)"]
pub mod tsu_sec_cmp;
#[doc = "TSU_MSB_SEC_CMP (rw) register accessor: an alias for `Reg<TSU_MSB_SEC_CMP_SPEC>`"]
pub type TSU_MSB_SEC_CMP = crate::Reg<tsu_msb_sec_cmp::TSU_MSB_SEC_CMP_SPEC>;
#[doc = "TSU timer comparison value seconds (47 to 32 bits)"]
pub mod tsu_msb_sec_cmp;
#[doc = "TSU_PTP_TX_MSB_SEC (r) register accessor: an alias for `Reg<TSU_PTP_TX_MSB_SEC_SPEC>`"]
pub type TSU_PTP_TX_MSB_SEC = crate::Reg<tsu_ptp_tx_msb_sec::TSU_PTP_TX_MSB_SEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register (47 to 32 bits)"]
pub mod tsu_ptp_tx_msb_sec;
#[doc = "TSU_PTP_RX_MSB_SEC (r) register accessor: an alias for `Reg<TSU_PTP_RX_MSB_SEC_SPEC>`"]
pub type TSU_PTP_RX_MSB_SEC = crate::Reg<tsu_ptp_rx_msb_sec::TSU_PTP_RX_MSB_SEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register (47 to 32 bits)"]
pub mod tsu_ptp_rx_msb_sec;
#[doc = "TSU_PEER_TX_MSB_SEC (r) register accessor: an alias for `Reg<TSU_PEER_TX_MSB_SEC_SPEC>`"]
pub type TSU_PEER_TX_MSB_SEC = crate::Reg<tsu_peer_tx_msb_sec::TSU_PEER_TX_MSB_SEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register (47 to 32 bits)"]
pub mod tsu_peer_tx_msb_sec;
#[doc = "TSU_PEER_RX_MSB_SEC (r) register accessor: an alias for `Reg<TSU_PEER_RX_MSB_SEC_SPEC>`"]
pub type TSU_PEER_RX_MSB_SEC = crate::Reg<tsu_peer_rx_msb_sec::TSU_PEER_RX_MSB_SEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register (47 to 32 bits)"]
pub mod tsu_peer_rx_msb_sec;
#[doc = "DPRAM_FILL_DBG (rw) register accessor: an alias for `Reg<DPRAM_FILL_DBG_SPEC>`"]
pub type DPRAM_FILL_DBG = crate::Reg<dpram_fill_dbg::DPRAM_FILL_DBG_SPEC>;
#[doc = "The fill levels for the TX &amp; RX packet buffers can be read using this register, including the fill level for each queue in the TX direction."]
pub mod dpram_fill_dbg;
#[doc = "REVISION_REG (r) register accessor: an alias for `Reg<REVISION_REG_SPEC>`"]
pub type REVISION_REG = crate::Reg<revision_reg::REVISION_REG_SPEC>;
#[doc = "This register indicates a Cadence module identification number and module revision. The value of this register is read only as defined by `gem_revision_reg_value"]
pub mod revision_reg;
#[doc = "OCTETS_TXED_BOTTOM (r) register accessor: an alias for `Reg<OCTETS_TXED_BOTTOM_SPEC>`"]
pub type OCTETS_TXED_BOTTOM = crate::Reg<octets_txed_bottom::OCTETS_TXED_BOTTOM_SPEC>;
#[doc = "Octets Transmitted lower bits (31 to 0 bits)"]
pub mod octets_txed_bottom;
#[doc = "OCTETS_TXED_TOP (r) register accessor: an alias for `Reg<OCTETS_TXED_TOP_SPEC>`"]
pub type OCTETS_TXED_TOP = crate::Reg<octets_txed_top::OCTETS_TXED_TOP_SPEC>;
#[doc = "Octets Transmitted higher bits (47 to 32 bits)"]
pub mod octets_txed_top;
#[doc = "FRAMES_TXED_OK (r) register accessor: an alias for `Reg<FRAMES_TXED_OK_SPEC>`"]
pub type FRAMES_TXED_OK = crate::Reg<frames_txed_ok::FRAMES_TXED_OK_SPEC>;
#[doc = "Frames Transmitted"]
pub mod frames_txed_ok;
#[doc = "BROADCAST_TXED (r) register accessor: an alias for `Reg<BROADCAST_TXED_SPEC>`"]
pub type BROADCAST_TXED = crate::Reg<broadcast_txed::BROADCAST_TXED_SPEC>;
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcast_txed;
#[doc = "MULTICAST_TXED (r) register accessor: an alias for `Reg<MULTICAST_TXED_SPEC>`"]
pub type MULTICAST_TXED = crate::Reg<multicast_txed::MULTICAST_TXED_SPEC>;
#[doc = "Multicast Frames Transmitted"]
pub mod multicast_txed;
#[doc = "PAUSE_FRAMES_TXED (r) register accessor: an alias for `Reg<PAUSE_FRAMES_TXED_SPEC>`"]
pub type PAUSE_FRAMES_TXED = crate::Reg<pause_frames_txed::PAUSE_FRAMES_TXED_SPEC>;
#[doc = "Pause Frames Transmitted"]
pub mod pause_frames_txed;
#[doc = "FRAMES_TXED_64 (r) register accessor: an alias for `Reg<FRAMES_TXED_64_SPEC>`"]
pub type FRAMES_TXED_64 = crate::Reg<frames_txed_64::FRAMES_TXED_64_SPEC>;
#[doc = "64 Byte Frames Transmitted"]
pub mod frames_txed_64;
#[doc = "FRAMES_TXED_65 (r) register accessor: an alias for `Reg<FRAMES_TXED_65_SPEC>`"]
pub type FRAMES_TXED_65 = crate::Reg<frames_txed_65::FRAMES_TXED_65_SPEC>;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod frames_txed_65;
#[doc = "FRAMES_TXED_128 (r) register accessor: an alias for `Reg<FRAMES_TXED_128_SPEC>`"]
pub type FRAMES_TXED_128 = crate::Reg<frames_txed_128::FRAMES_TXED_128_SPEC>;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod frames_txed_128;
#[doc = "FRAMES_TXED_256 (r) register accessor: an alias for `Reg<FRAMES_TXED_256_SPEC>`"]
pub type FRAMES_TXED_256 = crate::Reg<frames_txed_256::FRAMES_TXED_256_SPEC>;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod frames_txed_256;
#[doc = "FRAMES_TXED_512 (r) register accessor: an alias for `Reg<FRAMES_TXED_512_SPEC>`"]
pub type FRAMES_TXED_512 = crate::Reg<frames_txed_512::FRAMES_TXED_512_SPEC>;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod frames_txed_512;
#[doc = "FRAMES_TXED_1024 (r) register accessor: an alias for `Reg<FRAMES_TXED_1024_SPEC>`"]
pub type FRAMES_TXED_1024 = crate::Reg<frames_txed_1024::FRAMES_TXED_1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod frames_txed_1024;
#[doc = "FRAMES_TXED_1519 (r) register accessor: an alias for `Reg<FRAMES_TXED_1519_SPEC>`"]
pub type FRAMES_TXED_1519 = crate::Reg<frames_txed_1519::FRAMES_TXED_1519_SPEC>;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod frames_txed_1519;
#[doc = "TX_UNDERRUNS (r) register accessor: an alias for `Reg<TX_UNDERRUNS_SPEC>`"]
pub type TX_UNDERRUNS = crate::Reg<tx_underruns::TX_UNDERRUNS_SPEC>;
#[doc = "Transmit Under Runs"]
pub mod tx_underruns;
#[doc = "SINGLE_COLLISIONS (r) register accessor: an alias for `Reg<SINGLE_COLLISIONS_SPEC>`"]
pub type SINGLE_COLLISIONS = crate::Reg<single_collisions::SINGLE_COLLISIONS_SPEC>;
#[doc = "Single Collision Frames. Presents in design but not support."]
pub mod single_collisions;
#[doc = "MULTIPLE_COLLISIONS (r) register accessor: an alias for `Reg<MULTIPLE_COLLISIONS_SPEC>`"]
pub type MULTIPLE_COLLISIONS = crate::Reg<multiple_collisions::MULTIPLE_COLLISIONS_SPEC>;
#[doc = "Multiple Collision Frames. Presents in design but not support."]
pub mod multiple_collisions;
#[doc = "EXCESSIVE_COLLISIONS (r) register accessor: an alias for `Reg<EXCESSIVE_COLLISIONS_SPEC>`"]
pub type EXCESSIVE_COLLISIONS = crate::Reg<excessive_collisions::EXCESSIVE_COLLISIONS_SPEC>;
#[doc = "Excessive Collisions. Presents in design but not support."]
pub mod excessive_collisions;
#[doc = "LATE_COLLISIONS (r) register accessor: an alias for `Reg<LATE_COLLISIONS_SPEC>`"]
pub type LATE_COLLISIONS = crate::Reg<late_collisions::LATE_COLLISIONS_SPEC>;
#[doc = "Late Collisions. Presents in design but not support."]
pub mod late_collisions;
#[doc = "DEFERRED_FRAMES (r) register accessor: an alias for `Reg<DEFERRED_FRAMES_SPEC>`"]
pub type DEFERRED_FRAMES = crate::Reg<deferred_frames::DEFERRED_FRAMES_SPEC>;
#[doc = "Deferred Transmission Frames. Presents in design but not support."]
pub mod deferred_frames;
#[doc = "CRS_ERRORS (r) register accessor: an alias for `Reg<CRS_ERRORS_SPEC>`"]
pub type CRS_ERRORS = crate::Reg<crs_errors::CRS_ERRORS_SPEC>;
#[doc = "Carrier Sense Errors. Presents in design but not support."]
pub mod crs_errors;
#[doc = "OCTETS_RXED_BOTTOM (r) register accessor: an alias for `Reg<OCTETS_RXED_BOTTOM_SPEC>`"]
pub type OCTETS_RXED_BOTTOM = crate::Reg<octets_rxed_bottom::OCTETS_RXED_BOTTOM_SPEC>;
#[doc = "Octets Received (31 to 0 bits)"]
pub mod octets_rxed_bottom;
#[doc = "OCTETS_RXED_TOP (r) register accessor: an alias for `Reg<OCTETS_RXED_TOP_SPEC>`"]
pub type OCTETS_RXED_TOP = crate::Reg<octets_rxed_top::OCTETS_RXED_TOP_SPEC>;
#[doc = "Octets Received (47 to 32 bits)"]
pub mod octets_rxed_top;
#[doc = "FRAMES_RXED_OK (r) register accessor: an alias for `Reg<FRAMES_RXED_OK_SPEC>`"]
pub type FRAMES_RXED_OK = crate::Reg<frames_rxed_ok::FRAMES_RXED_OK_SPEC>;
#[doc = "Frames Received"]
pub mod frames_rxed_ok;
#[doc = "BROADCAST_RXED (r) register accessor: an alias for `Reg<BROADCAST_RXED_SPEC>`"]
pub type BROADCAST_RXED = crate::Reg<broadcast_rxed::BROADCAST_RXED_SPEC>;
#[doc = "Broadcast Frames Received"]
pub mod broadcast_rxed;
#[doc = "MULTICAST_RXED (r) register accessor: an alias for `Reg<MULTICAST_RXED_SPEC>`"]
pub type MULTICAST_RXED = crate::Reg<multicast_rxed::MULTICAST_RXED_SPEC>;
#[doc = "Multicast Frames Received"]
pub mod multicast_rxed;
#[doc = "PAUSE_FRAMES_RXED (r) register accessor: an alias for `Reg<PAUSE_FRAMES_RXED_SPEC>`"]
pub type PAUSE_FRAMES_RXED = crate::Reg<pause_frames_rxed::PAUSE_FRAMES_RXED_SPEC>;
#[doc = "Pause Frames Received"]
pub mod pause_frames_rxed;
#[doc = "FRAMES_RXED_64 (r) register accessor: an alias for `Reg<FRAMES_RXED_64_SPEC>`"]
pub type FRAMES_RXED_64 = crate::Reg<frames_rxed_64::FRAMES_RXED_64_SPEC>;
#[doc = "64 Byte Frames Received"]
pub mod frames_rxed_64;
#[doc = "FRAMES_RXED_65 (r) register accessor: an alias for `Reg<FRAMES_RXED_65_SPEC>`"]
pub type FRAMES_RXED_65 = crate::Reg<frames_rxed_65::FRAMES_RXED_65_SPEC>;
#[doc = "65 to 127 Byte Frames Received"]
pub mod frames_rxed_65;
#[doc = "FRAMES_RXED_128 (r) register accessor: an alias for `Reg<FRAMES_RXED_128_SPEC>`"]
pub type FRAMES_RXED_128 = crate::Reg<frames_rxed_128::FRAMES_RXED_128_SPEC>;
#[doc = "128 to 255 Byte Frames Received"]
pub mod frames_rxed_128;
#[doc = "FRAMES_RXED_256 (r) register accessor: an alias for `Reg<FRAMES_RXED_256_SPEC>`"]
pub type FRAMES_RXED_256 = crate::Reg<frames_rxed_256::FRAMES_RXED_256_SPEC>;
#[doc = "256 to 511 Byte Frames Received"]
pub mod frames_rxed_256;
#[doc = "FRAMES_RXED_512 (r) register accessor: an alias for `Reg<FRAMES_RXED_512_SPEC>`"]
pub type FRAMES_RXED_512 = crate::Reg<frames_rxed_512::FRAMES_RXED_512_SPEC>;
#[doc = "512 to 1023 Byte Frames Received"]
pub mod frames_rxed_512;
#[doc = "FRAMES_RXED_1024 (r) register accessor: an alias for `Reg<FRAMES_RXED_1024_SPEC>`"]
pub type FRAMES_RXED_1024 = crate::Reg<frames_rxed_1024::FRAMES_RXED_1024_SPEC>;
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod frames_rxed_1024;
#[doc = "FRAMES_RXED_1519 (r) register accessor: an alias for `Reg<FRAMES_RXED_1519_SPEC>`"]
pub type FRAMES_RXED_1519 = crate::Reg<frames_rxed_1519::FRAMES_RXED_1519_SPEC>;
#[doc = "1519 to maximum Byte Frames Received"]
pub mod frames_rxed_1519;
#[doc = "UNDERSIZE_FRAMES (r) register accessor: an alias for `Reg<UNDERSIZE_FRAMES_SPEC>`"]
pub type UNDERSIZE_FRAMES = crate::Reg<undersize_frames::UNDERSIZE_FRAMES_SPEC>;
#[doc = "Undersized Frames Received"]
pub mod undersize_frames;
#[doc = "EXCESSIVE_RX_LENGTH (r) register accessor: an alias for `Reg<EXCESSIVE_RX_LENGTH_SPEC>`"]
pub type EXCESSIVE_RX_LENGTH = crate::Reg<excessive_rx_length::EXCESSIVE_RX_LENGTH_SPEC>;
#[doc = "Oversize Frames Received"]
pub mod excessive_rx_length;
#[doc = "RX_JABBERS (r) register accessor: an alias for `Reg<RX_JABBERS_SPEC>`"]
pub type RX_JABBERS = crate::Reg<rx_jabbers::RX_JABBERS_SPEC>;
#[doc = "Jabbers Received"]
pub mod rx_jabbers;
#[doc = "FCS_ERRORS (r) register accessor: an alias for `Reg<FCS_ERRORS_SPEC>`"]
pub type FCS_ERRORS = crate::Reg<fcs_errors::FCS_ERRORS_SPEC>;
#[doc = "Frame Check Sequence Errors"]
pub mod fcs_errors;
#[doc = "RX_LENGTH_ERRORS (r) register accessor: an alias for `Reg<RX_LENGTH_ERRORS_SPEC>`"]
pub type RX_LENGTH_ERRORS = crate::Reg<rx_length_errors::RX_LENGTH_ERRORS_SPEC>;
#[doc = "Length Field Frame Errors"]
pub mod rx_length_errors;
#[doc = "RX_SYMBOL_ERRORS (r) register accessor: an alias for `Reg<RX_SYMBOL_ERRORS_SPEC>`"]
pub type RX_SYMBOL_ERRORS = crate::Reg<rx_symbol_errors::RX_SYMBOL_ERRORS_SPEC>;
#[doc = "Receive Symbol Errors"]
pub mod rx_symbol_errors;
#[doc = "ALIGNMENT_ERRORS (r) register accessor: an alias for `Reg<ALIGNMENT_ERRORS_SPEC>`"]
pub type ALIGNMENT_ERRORS = crate::Reg<alignment_errors::ALIGNMENT_ERRORS_SPEC>;
#[doc = "Alignment Errors"]
pub mod alignment_errors;
#[doc = "RX_RESOURCE_ERRORS (r) register accessor: an alias for `Reg<RX_RESOURCE_ERRORS_SPEC>`"]
pub type RX_RESOURCE_ERRORS = crate::Reg<rx_resource_errors::RX_RESOURCE_ERRORS_SPEC>;
#[doc = "Receive Resource Errors"]
pub mod rx_resource_errors;
#[doc = "RX_OVERRUNS (r) register accessor: an alias for `Reg<RX_OVERRUNS_SPEC>`"]
pub type RX_OVERRUNS = crate::Reg<rx_overruns::RX_OVERRUNS_SPEC>;
#[doc = "Receive Overruns"]
pub mod rx_overruns;
#[doc = "RX_IP_CK_ERRORS (r) register accessor: an alias for `Reg<RX_IP_CK_ERRORS_SPEC>`"]
pub type RX_IP_CK_ERRORS = crate::Reg<rx_ip_ck_errors::RX_IP_CK_ERRORS_SPEC>;
#[doc = "IP Header Checksum Errors"]
pub mod rx_ip_ck_errors;
#[doc = "RX_TCP_CK_ERRORS (r) register accessor: an alias for `Reg<RX_TCP_CK_ERRORS_SPEC>`"]
pub type RX_TCP_CK_ERRORS = crate::Reg<rx_tcp_ck_errors::RX_TCP_CK_ERRORS_SPEC>;
#[doc = "TCP Checksum Errors"]
pub mod rx_tcp_ck_errors;
#[doc = "RX_UDP_CK_ERRORS (r) register accessor: an alias for `Reg<RX_UDP_CK_ERRORS_SPEC>`"]
pub type RX_UDP_CK_ERRORS = crate::Reg<rx_udp_ck_errors::RX_UDP_CK_ERRORS_SPEC>;
#[doc = "UDP Checksum Errors"]
pub mod rx_udp_ck_errors;
#[doc = "AUTO_FLUSHED_PKTS (r) register accessor: an alias for `Reg<AUTO_FLUSHED_PKTS_SPEC>`"]
pub type AUTO_FLUSHED_PKTS = crate::Reg<auto_flushed_pkts::AUTO_FLUSHED_PKTS_SPEC>;
#[doc = "Receive DMA Flushed Packets"]
pub mod auto_flushed_pkts;
#[doc = "TSU_TIMER_INCR_SUB_NSEC (rw) register accessor: an alias for `Reg<TSU_TIMER_INCR_SUB_NSEC_SPEC>`"]
pub type TSU_TIMER_INCR_SUB_NSEC =
    crate::Reg<tsu_timer_incr_sub_nsec::TSU_TIMER_INCR_SUB_NSEC_SPEC>;
#[doc = "1588 Timer Increment Register sub nsec"]
pub mod tsu_timer_incr_sub_nsec;
#[doc = "TSU_TIMER_MSB_SEC (rw) register accessor: an alias for `Reg<TSU_TIMER_MSB_SEC_SPEC>`"]
pub type TSU_TIMER_MSB_SEC = crate::Reg<tsu_timer_msb_sec::TSU_TIMER_MSB_SEC_SPEC>;
#[doc = "1588 Timer Seconds Register (47 to 32 bits)"]
pub mod tsu_timer_msb_sec;
#[doc = "TSU_STROBE_MSB_SEC (r) register accessor: an alias for `Reg<TSU_STROBE_MSB_SEC_SPEC>`"]
pub type TSU_STROBE_MSB_SEC = crate::Reg<tsu_strobe_msb_sec::TSU_STROBE_MSB_SEC_SPEC>;
#[doc = "1588 Timer Sync Strobe Seconds Register (47 to 32 bits)"]
pub mod tsu_strobe_msb_sec;
#[doc = "TSU_STROBE_SEC (r) register accessor: an alias for `Reg<TSU_STROBE_SEC_SPEC>`"]
pub type TSU_STROBE_SEC = crate::Reg<tsu_strobe_sec::TSU_STROBE_SEC_SPEC>;
#[doc = "1588 Timer Sync Strobe Seconds Register (31 to 0 bits)"]
pub mod tsu_strobe_sec;
#[doc = "TSU_STROBE_NSEC (r) register accessor: an alias for `Reg<TSU_STROBE_NSEC_SPEC>`"]
pub type TSU_STROBE_NSEC = crate::Reg<tsu_strobe_nsec::TSU_STROBE_NSEC_SPEC>;
#[doc = "1588 Timer Sync Strobe Nanoseconds Register"]
pub mod tsu_strobe_nsec;
#[doc = "TSU_TIMER_SEC (rw) register accessor: an alias for `Reg<TSU_TIMER_SEC_SPEC>`"]
pub type TSU_TIMER_SEC = crate::Reg<tsu_timer_sec::TSU_TIMER_SEC_SPEC>;
#[doc = "1588 Timer Seconds Register (31 to 0 bits)"]
pub mod tsu_timer_sec;
#[doc = "TSU_TIMER_NSEC (rw) register accessor: an alias for `Reg<TSU_TIMER_NSEC_SPEC>`"]
pub type TSU_TIMER_NSEC = crate::Reg<tsu_timer_nsec::TSU_TIMER_NSEC_SPEC>;
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsu_timer_nsec;
#[doc = "TSU_TIMER_ADJUST (w) register accessor: an alias for `Reg<TSU_TIMER_ADJUST_SPEC>`"]
pub type TSU_TIMER_ADJUST = crate::Reg<tsu_timer_adjust::TSU_TIMER_ADJUST_SPEC>;
#[doc = "This register is used to adjust the value of the timer in the TSU. It allows an integral number of nanoseconds to be added or subtracted from the timer in a one-off operation. This register returns all zeroes when read."]
pub mod tsu_timer_adjust;
#[doc = "TSU_TIMER_INCR (rw) register accessor: an alias for `Reg<TSU_TIMER_INCR_SPEC>`"]
pub type TSU_TIMER_INCR = crate::Reg<tsu_timer_incr::TSU_TIMER_INCR_SPEC>;
#[doc = "1588 Timer Increment Register"]
pub mod tsu_timer_incr;
#[doc = "TSU_PTP_TX_SEC (r) register accessor: an alias for `Reg<TSU_PTP_TX_SEC_SPEC>`"]
pub type TSU_PTP_TX_SEC = crate::Reg<tsu_ptp_tx_sec::TSU_PTP_TX_SEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Seconds Register (31 to 0 bits)"]
pub mod tsu_ptp_tx_sec;
#[doc = "TSU_PTP_TX_NSEC (r) register accessor: an alias for `Reg<TSU_PTP_TX_NSEC_SPEC>`"]
pub type TSU_PTP_TX_NSEC = crate::Reg<tsu_ptp_tx_nsec::TSU_PTP_TX_NSEC_SPEC>;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsu_ptp_tx_nsec;
#[doc = "TSU_PTP_RX_SEC (r) register accessor: an alias for `Reg<TSU_PTP_RX_SEC_SPEC>`"]
pub type TSU_PTP_RX_SEC = crate::Reg<tsu_ptp_rx_sec::TSU_PTP_RX_SEC_SPEC>;
#[doc = "PTP Event Frame Received Seconds Register (31 to 0 bits)"]
pub mod tsu_ptp_rx_sec;
#[doc = "TSU_PTP_RX_NSEC (r) register accessor: an alias for `Reg<TSU_PTP_RX_NSEC_SPEC>`"]
pub type TSU_PTP_RX_NSEC = crate::Reg<tsu_ptp_rx_nsec::TSU_PTP_RX_NSEC_SPEC>;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsu_ptp_rx_nsec;
#[doc = "TSU_PEER_TX_SEC (r) register accessor: an alias for `Reg<TSU_PEER_TX_SEC_SPEC>`"]
pub type TSU_PEER_TX_SEC = crate::Reg<tsu_peer_tx_sec::TSU_PEER_TX_SEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register (31 to 0 bits)"]
pub mod tsu_peer_tx_sec;
#[doc = "TSU_PEER_TX_NSEC (r) register accessor: an alias for `Reg<TSU_PEER_TX_NSEC_SPEC>`"]
pub type TSU_PEER_TX_NSEC = crate::Reg<tsu_peer_tx_nsec::TSU_PEER_TX_NSEC_SPEC>;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsu_peer_tx_nsec;
#[doc = "TSU_PEER_RX_SEC (r) register accessor: an alias for `Reg<TSU_PEER_RX_SEC_SPEC>`"]
pub type TSU_PEER_RX_SEC = crate::Reg<tsu_peer_rx_sec::TSU_PEER_RX_SEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Seconds Register (31 to 0 bits)"]
pub mod tsu_peer_rx_sec;
#[doc = "TSU_PEER_RX_NSEC (r) register accessor: an alias for `Reg<TSU_PEER_RX_NSEC_SPEC>`"]
pub type TSU_PEER_RX_NSEC = crate::Reg<tsu_peer_rx_nsec::TSU_PEER_RX_NSEC_SPEC>;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsu_peer_rx_nsec;
#[doc = "PCS_CONTROL (r) register accessor: an alias for `Reg<PCS_CONTROL_SPEC>`"]
pub type PCS_CONTROL = crate::Reg<pcs_control::PCS_CONTROL_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_control;
#[doc = "PCS_STATUS (r) register accessor: an alias for `Reg<PCS_STATUS_SPEC>`"]
pub type PCS_STATUS = crate::Reg<pcs_status::PCS_STATUS_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_status;
#[doc = "PCS_AN_ADV (r) register accessor: an alias for `Reg<PCS_AN_ADV_SPEC>`"]
pub type PCS_AN_ADV = crate::Reg<pcs_an_adv::PCS_AN_ADV_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_adv;
#[doc = "PCS_AN_LP_BASE (r) register accessor: an alias for `Reg<PCS_AN_LP_BASE_SPEC>`"]
pub type PCS_AN_LP_BASE = crate::Reg<pcs_an_lp_base::PCS_AN_LP_BASE_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_lp_base;
#[doc = "PCS_AN_EXP (r) register accessor: an alias for `Reg<PCS_AN_EXP_SPEC>`"]
pub type PCS_AN_EXP = crate::Reg<pcs_an_exp::PCS_AN_EXP_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_exp;
#[doc = "PCS_AN_NP_TX (r) register accessor: an alias for `Reg<PCS_AN_NP_TX_SPEC>`"]
pub type PCS_AN_NP_TX = crate::Reg<pcs_an_np_tx::PCS_AN_NP_TX_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_np_tx;
#[doc = "PCS_AN_LP_NP (r) register accessor: an alias for `Reg<PCS_AN_LP_NP_SPEC>`"]
pub type PCS_AN_LP_NP = crate::Reg<pcs_an_lp_np::PCS_AN_LP_NP_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_lp_np;
#[doc = "PCS_AN_EXT_STATUS (r) register accessor: an alias for `Reg<PCS_AN_EXT_STATUS_SPEC>`"]
pub type PCS_AN_EXT_STATUS = crate::Reg<pcs_an_ext_status::PCS_AN_EXT_STATUS_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod pcs_an_ext_status;
#[doc = "TX_PAUSE_QUANTUM1 (rw) register accessor: an alias for `Reg<TX_PAUSE_QUANTUM1_SPEC>`"]
pub type TX_PAUSE_QUANTUM1 = crate::Reg<tx_pause_quantum1::TX_PAUSE_QUANTUM1_SPEC>;
#[doc = "Transmit Pause Quantum Register 1"]
pub mod tx_pause_quantum1;
#[doc = "TX_PAUSE_QUANTUM2 (rw) register accessor: an alias for `Reg<TX_PAUSE_QUANTUM2_SPEC>`"]
pub type TX_PAUSE_QUANTUM2 = crate::Reg<tx_pause_quantum2::TX_PAUSE_QUANTUM2_SPEC>;
#[doc = "Transmit Pause Quantum Register 2"]
pub mod tx_pause_quantum2;
#[doc = "TX_PAUSE_QUANTUM3 (rw) register accessor: an alias for `Reg<TX_PAUSE_QUANTUM3_SPEC>`"]
pub type TX_PAUSE_QUANTUM3 = crate::Reg<tx_pause_quantum3::TX_PAUSE_QUANTUM3_SPEC>;
#[doc = "Transmit Pause Quantum Register 3"]
pub mod tx_pause_quantum3;
#[doc = "RX_LPI (r) register accessor: an alias for `Reg<RX_LPI_SPEC>`"]
pub type RX_LPI = crate::Reg<rx_lpi::RX_LPI_SPEC>;
#[doc = "Received LPI transitions"]
pub mod rx_lpi;
#[doc = "RX_LPI_TIME (r) register accessor: an alias for `Reg<RX_LPI_TIME_SPEC>`"]
pub type RX_LPI_TIME = crate::Reg<rx_lpi_time::RX_LPI_TIME_SPEC>;
#[doc = "Received LPI time"]
pub mod rx_lpi_time;
#[doc = "TX_LPI (r) register accessor: an alias for `Reg<TX_LPI_SPEC>`"]
pub type TX_LPI = crate::Reg<tx_lpi::TX_LPI_SPEC>;
#[doc = "Transmit LPI transitions"]
pub mod tx_lpi;
#[doc = "TX_LPI_TIME (r) register accessor: an alias for `Reg<TX_LPI_TIME_SPEC>`"]
pub type TX_LPI_TIME = crate::Reg<tx_lpi_time::TX_LPI_TIME_SPEC>;
#[doc = "Transmit LPI time"]
pub mod tx_lpi_time;
#[doc = "DESIGNCFG_DEBUG1 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG1_SPEC>`"]
pub type DESIGNCFG_DEBUG1 = crate::Reg<designcfg_debug1::DESIGNCFG_DEBUG1_SPEC>;
#[doc = "The GEM_GXL(3PIP) has many parameterisation options to configure the IP during compilation stage. This is achieved using Verilog define compiler directives in an include file called mxeth_defs.v."]
pub mod designcfg_debug1;
#[doc = "DESIGNCFG_DEBUG2 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG2_SPEC>`"]
pub type DESIGNCFG_DEBUG2 = crate::Reg<designcfg_debug2::DESIGNCFG_DEBUG2_SPEC>;
#[doc = "Design Configuration Register 2"]
pub mod designcfg_debug2;
#[doc = "DESIGNCFG_DEBUG3 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG3_SPEC>`"]
pub type DESIGNCFG_DEBUG3 = crate::Reg<designcfg_debug3::DESIGNCFG_DEBUG3_SPEC>;
#[doc = "Design Configuration Register 3"]
pub mod designcfg_debug3;
#[doc = "DESIGNCFG_DEBUG4 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG4_SPEC>`"]
pub type DESIGNCFG_DEBUG4 = crate::Reg<designcfg_debug4::DESIGNCFG_DEBUG4_SPEC>;
#[doc = "Design Configuration Register 4"]
pub mod designcfg_debug4;
#[doc = "DESIGNCFG_DEBUG5 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG5_SPEC>`"]
pub type DESIGNCFG_DEBUG5 = crate::Reg<designcfg_debug5::DESIGNCFG_DEBUG5_SPEC>;
#[doc = "Design Configuration Register 5"]
pub mod designcfg_debug5;
#[doc = "DESIGNCFG_DEBUG6 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG6_SPEC>`"]
pub type DESIGNCFG_DEBUG6 = crate::Reg<designcfg_debug6::DESIGNCFG_DEBUG6_SPEC>;
#[doc = "Design Configuration Register 6"]
pub mod designcfg_debug6;
#[doc = "DESIGNCFG_DEBUG7 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG7_SPEC>`"]
pub type DESIGNCFG_DEBUG7 = crate::Reg<designcfg_debug7::DESIGNCFG_DEBUG7_SPEC>;
#[doc = "Design Configuration Register 7"]
pub mod designcfg_debug7;
#[doc = "DESIGNCFG_DEBUG8 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG8_SPEC>`"]
pub type DESIGNCFG_DEBUG8 = crate::Reg<designcfg_debug8::DESIGNCFG_DEBUG8_SPEC>;
#[doc = "Design Configuration Register 8"]
pub mod designcfg_debug8;
#[doc = "DESIGNCFG_DEBUG9 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG9_SPEC>`"]
pub type DESIGNCFG_DEBUG9 = crate::Reg<designcfg_debug9::DESIGNCFG_DEBUG9_SPEC>;
#[doc = "Design Configuration Register 9"]
pub mod designcfg_debug9;
#[doc = "DESIGNCFG_DEBUG10 (r) register accessor: an alias for `Reg<DESIGNCFG_DEBUG10_SPEC>`"]
pub type DESIGNCFG_DEBUG10 = crate::Reg<designcfg_debug10::DESIGNCFG_DEBUG10_SPEC>;
#[doc = "Design Configuration Register 10"]
pub mod designcfg_debug10;
#[doc = "SPEC_ADD5_BOTTOM (r) register accessor: an alias for `Reg<SPEC_ADD5_BOTTOM_SPEC>`"]
pub type SPEC_ADD5_BOTTOM = crate::Reg<spec_add5_bottom::SPEC_ADD5_BOTTOM_SPEC>;
#[doc = "Specific address registers 5 ~ 36 doesn't present. Access to the register returns AHB error."]
pub mod spec_add5_bottom;
#[doc = "SPEC_ADD5_TOP (r) register accessor: an alias for `Reg<SPEC_ADD5_TOP_SPEC>`"]
pub type SPEC_ADD5_TOP = crate::Reg<spec_add5_top::SPEC_ADD5_TOP_SPEC>;
#[doc = "Specific address registers 5 ~ 36 doesn't present. Access to the register returns AHB error."]
pub mod spec_add5_top;
#[doc = "SPEC_ADD36_BOTTOM (r) register accessor: an alias for `Reg<SPEC_ADD36_BOTTOM_SPEC>`"]
pub type SPEC_ADD36_BOTTOM = crate::Reg<spec_add36_bottom::SPEC_ADD36_BOTTOM_SPEC>;
#[doc = "Not presents."]
pub mod spec_add36_bottom;
#[doc = "SPEC_ADD36_TOP (r) register accessor: an alias for `Reg<SPEC_ADD36_TOP_SPEC>`"]
pub type SPEC_ADD36_TOP = crate::Reg<spec_add36_top::SPEC_ADD36_TOP_SPEC>;
#[doc = "Not presents."]
pub mod spec_add36_top;
#[doc = "INT_Q1_STATUS (r) register accessor: an alias for `Reg<INT_Q1_STATUS_SPEC>`"]
pub type INT_Q1_STATUS = crate::Reg<int_q1_status::INT_Q1_STATUS_SPEC>;
#[doc = "Priority queue Interrupt Status Register"]
pub mod int_q1_status;
#[doc = "INT_Q2_STATUS (r) register accessor: an alias for `Reg<INT_Q2_STATUS_SPEC>`"]
pub type INT_Q2_STATUS = crate::Reg<int_q2_status::INT_Q2_STATUS_SPEC>;
#[doc = "Priority queue Interrupt Status Register"]
pub mod int_q2_status;
#[doc = "INT_Q3_STATUS (r) register accessor: an alias for `Reg<INT_Q3_STATUS_SPEC>`"]
pub type INT_Q3_STATUS = crate::Reg<int_q3_status::INT_Q3_STATUS_SPEC>;
#[doc = "int_q3_status to int_q15_status doesn't present. Access to the register returns AHB error."]
pub mod int_q3_status;
#[doc = "INT_Q15_STATUS (r) register accessor: an alias for `Reg<INT_Q15_STATUS_SPEC>`"]
pub type INT_Q15_STATUS = crate::Reg<int_q15_status::INT_Q15_STATUS_SPEC>;
#[doc = "Not presents."]
pub mod int_q15_status;
#[doc = "TRANSMIT_Q1_PTR (rw) register accessor: an alias for `Reg<TRANSMIT_Q1_PTR_SPEC>`"]
pub type TRANSMIT_Q1_PTR = crate::Reg<transmit_q1_ptr::TRANSMIT_Q1_PTR_SPEC>;
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
pub mod transmit_q1_ptr;
#[doc = "TRANSMIT_Q2_PTR (rw) register accessor: an alias for `Reg<TRANSMIT_Q2_PTR_SPEC>`"]
pub type TRANSMIT_Q2_PTR = crate::Reg<transmit_q2_ptr::TRANSMIT_Q2_PTR_SPEC>;
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
pub mod transmit_q2_ptr;
#[doc = "TRANSMIT_Q3_PTR (r) register accessor: an alias for `Reg<TRANSMIT_Q3_PTR_SPEC>`"]
pub type TRANSMIT_Q3_PTR = crate::Reg<transmit_q3_ptr::TRANSMIT_Q3_PTR_SPEC>;
#[doc = "transmit_q3_ptr to transmit_q15_ptr doesn't present. Access to the register returns AHB error."]
pub mod transmit_q3_ptr;
#[doc = "TRANSMIT_Q15_PTR (r) register accessor: an alias for `Reg<TRANSMIT_Q15_PTR_SPEC>`"]
pub type TRANSMIT_Q15_PTR = crate::Reg<transmit_q15_ptr::TRANSMIT_Q15_PTR_SPEC>;
#[doc = "Not presents."]
pub mod transmit_q15_ptr;
#[doc = "RECEIVE_Q1_PTR (rw) register accessor: an alias for `Reg<RECEIVE_Q1_PTR_SPEC>`"]
pub type RECEIVE_Q1_PTR = crate::Reg<receive_q1_ptr::RECEIVE_Q1_PTR_SPEC>;
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
pub mod receive_q1_ptr;
#[doc = "RECEIVE_Q2_PTR (rw) register accessor: an alias for `Reg<RECEIVE_Q2_PTR_SPEC>`"]
pub type RECEIVE_Q2_PTR = crate::Reg<receive_q2_ptr::RECEIVE_Q2_PTR_SPEC>;
#[doc = "This register holds the start address of the transmit buffer queue (transmit buffers descriptor list). The transmit buffer queue base address register must be initialized before transmit is started through bit 9 of the network control register. Once transmission has started, any write to the transmit buffer queue base address register is illegal and therefore ignored. Note that due to clock boundary synchronization, it takes a maximum of four pclk cycles from the writing of the transmit start bit before the transmitter is active. Writing to the transmit buffer queue base address register during this time may produce unpredictable results. Reading this register returns the location of the descriptor currently being accessed. Because the DMA can store data for multiple frames at once, this may not necessarily be pointing to the current frame being transmitted. In terms of AMBA AXI operation, the transmit descriptors are written to memory using a single 32bit AHB access. When the datapath is configured as 64bit , the transmit descriptors should be aligned at 64-bit boundaries and each pair of 32-bit descriptors is read from memory using a single AXI access."]
pub mod receive_q2_ptr;
#[doc = "RECEIVE_Q3_PTR (r) register accessor: an alias for `Reg<RECEIVE_Q3_PTR_SPEC>`"]
pub type RECEIVE_Q3_PTR = crate::Reg<receive_q3_ptr::RECEIVE_Q3_PTR_SPEC>;
#[doc = "Not presents. Start address register doesn't present for queue3 ~ queue7."]
pub mod receive_q3_ptr;
#[doc = "RECEIVE_Q7_PTR (r) register accessor: an alias for `Reg<RECEIVE_Q7_PTR_SPEC>`"]
pub type RECEIVE_Q7_PTR = crate::Reg<receive_q7_ptr::RECEIVE_Q7_PTR_SPEC>;
#[doc = "Not presents."]
pub mod receive_q7_ptr;
#[doc = "DMA_RXBUF_SIZE_Q1 (rw) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q1_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q1 = crate::Reg<dma_rxbuf_size_q1::DMA_RXBUF_SIZE_Q1_SPEC>;
#[doc = "Receive Buffer queue 1 Size"]
pub mod dma_rxbuf_size_q1;
#[doc = "DMA_RXBUF_SIZE_Q2 (rw) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q2_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q2 = crate::Reg<dma_rxbuf_size_q2::DMA_RXBUF_SIZE_Q2_SPEC>;
#[doc = "Receive Buffer queue 2 Size"]
pub mod dma_rxbuf_size_q2;
#[doc = "DMA_RXBUF_SIZE_Q3 (r) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q3_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q3 = crate::Reg<dma_rxbuf_size_q3::DMA_RXBUF_SIZE_Q3_SPEC>;
#[doc = "dma_rxbuf_size_q3 to dma_rxbuf_size_q7 doesn't present."]
pub mod dma_rxbuf_size_q3;
#[doc = "DMA_RXBUF_SIZE_Q7 (r) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q7_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q7 = crate::Reg<dma_rxbuf_size_q7::DMA_RXBUF_SIZE_Q7_SPEC>;
#[doc = "Not presents."]
pub mod dma_rxbuf_size_q7;
#[doc = "CBS_CONTROL (rw) register accessor: an alias for `Reg<CBS_CONTROL_SPEC>`"]
pub type CBS_CONTROL = crate::Reg<cbs_control::CBS_CONTROL_SPEC>;
#[doc = "The IdleSlope value is defined as the rate of change of credit when a packet is waiting to be sent. This must not exceed the portTransmitRate which is dependent on the speed of operation, eg, portTranmsitRate. 1Gb/s = 32'h07735940 (125 Mbytes/s), 100Mb/sec = 32'h017D7840 (25 Mnibbles/s), 10Mb/sec = 32'h002625A0 (2.5 Mnibbles/s). If 50 percent of bandwidth was to be allocated to a particular queue in 1Gb/sec mode then the IdleSlope value for that queue would be calculated as 32'h07735940/2. Note that Credit-Based Shaping should be disabled prior to updating the IdleSlope values. As another example, for a 1722 audio packet with a payload of 6 samples per channel, the packet size would be 7 (preamble) + 1 (SFD) + 50 (packet header) + 6x4x2(payload) + 4 (CRC) = 110 bytes. For a rate of 8000 packets per second, the desired rate would 110 x 8000 bytes per second, so the programmed idleSlope value would be 880000 for a 1Gb/s connection, or 1760000 for a 100Mb/s or 10Mbs connection. See Figure 6.3 in the IEEE 1722 standard. In practice, the actual transmission rate will be vary slightly from that calculated. In this case, the idleSlope value should be recalibrated based on the variance between the measured and expected rate, and in this case very accurate transmission rates can be achieved."]
pub mod cbs_control;
#[doc = "CBS_IDLESLOPE_Q_A (rw) register accessor: an alias for `Reg<CBS_IDLESLOPE_Q_A_SPEC>`"]
pub type CBS_IDLESLOPE_Q_A = crate::Reg<cbs_idleslope_q_a::CBS_IDLESLOPE_Q_A_SPEC>;
#[doc = "queue A is the highest priority queue. This would be queue 8 in an 8 queue configuration."]
pub mod cbs_idleslope_q_a;
#[doc = "CBS_IDLESLOPE_Q_B (rw) register accessor: an alias for `Reg<CBS_IDLESLOPE_Q_B_SPEC>`"]
pub type CBS_IDLESLOPE_Q_B = crate::Reg<cbs_idleslope_q_b::CBS_IDLESLOPE_Q_B_SPEC>;
#[doc = "queue B is the 2nd highest priority queue. This would be queue 7 in an 8 queue configuration."]
pub mod cbs_idleslope_q_b;
#[doc = "UPPER_TX_Q_BASE_ADDR (rw) register accessor: an alias for `Reg<UPPER_TX_Q_BASE_ADDR_SPEC>`"]
pub type UPPER_TX_Q_BASE_ADDR = crate::Reg<upper_tx_q_base_addr::UPPER_TX_Q_BASE_ADDR_SPEC>;
#[doc = "Upper 32 bits of transmit buffer descriptor queue base address."]
pub mod upper_tx_q_base_addr;
#[doc = "TX_BD_CONTROL (rw) register accessor: an alias for `Reg<TX_BD_CONTROL_SPEC>`"]
pub type TX_BD_CONTROL = crate::Reg<tx_bd_control::TX_BD_CONTROL_SPEC>;
#[doc = "TX BD control register"]
pub mod tx_bd_control;
#[doc = "RX_BD_CONTROL (rw) register accessor: an alias for `Reg<RX_BD_CONTROL_SPEC>`"]
pub type RX_BD_CONTROL = crate::Reg<rx_bd_control::RX_BD_CONTROL_SPEC>;
#[doc = "RX BD control register"]
pub mod rx_bd_control;
#[doc = "UPPER_RX_Q_BASE_ADDR (rw) register accessor: an alias for `Reg<UPPER_RX_Q_BASE_ADDR_SPEC>`"]
pub type UPPER_RX_Q_BASE_ADDR = crate::Reg<upper_rx_q_base_addr::UPPER_RX_Q_BASE_ADDR_SPEC>;
#[doc = "Upper 32 bits of receive buffer descriptor queue base address."]
pub mod upper_rx_q_base_addr;
#[doc = "HIDDEN_REG0 (rw) register accessor: an alias for `Reg<HIDDEN_REG0_SPEC>`"]
pub type HIDDEN_REG0 = crate::Reg<hidden_reg0::HIDDEN_REG0_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_10m 12'h4e0 // 10M Port TX Rate *** HIDDEN Register ***'. Default value of cbs related hidden registers (0x14E0~0x14E8) are depicted in cbs_control register."]
pub mod hidden_reg0;
#[doc = "HIDDEN_REG1 (rw) register accessor: an alias for `Reg<HIDDEN_REG1_SPEC>`"]
pub type HIDDEN_REG1 = crate::Reg<hidden_reg1::HIDDEN_REG1_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_100m 12'h4e4 // 100M Port TX Rate *** HIDDEN Register ***'"]
pub mod hidden_reg1;
#[doc = "HIDDEN_REG2 (rw) register accessor: an alias for `Reg<HIDDEN_REG2_SPEC>`"]
pub type HIDDEN_REG2 = crate::Reg<hidden_reg2::HIDDEN_REG2_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_cbs_port_tx_rate_1g 12'h4e8 // 1G Port TX Rate *** HIDDEN Register ***'"]
pub mod hidden_reg2;
#[doc = "HIDDEN_REG3 (rw) register accessor: an alias for `Reg<HIDDEN_REG3_SPEC>`"]
pub type HIDDEN_REG3 = crate::Reg<hidden_reg3::HIDDEN_REG3_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_wd_counter 12'h4ec // *** HIDDEN Register ***'."]
pub mod hidden_reg3;
#[doc = "HIDDEN_REG4 (rw) register accessor: an alias for `Reg<HIDDEN_REG4_SPEC>`"]
pub type HIDDEN_REG4 = crate::Reg<hidden_reg4::HIDDEN_REG4_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_axi_tx_full_threshold0 12'h4f8 // AXI full threshold setting *** HIDDEN Register ***'. Note. When using AXI mode with a single port ram ( gem_spram == 1) mode and a 32b dma bus width ( gem_dma_bus_width == 32 or bits 22 to 21 of the network_config register are set to 0) the AXI hidden registers (0x14F8 and 0x14FC) need to be updated (these registers are used for fine tuning AXI dma bursts and normally should not be touched)."]
pub mod hidden_reg4;
#[doc = "HIDDEN_REG5 (rw) register accessor: an alias for `Reg<HIDDEN_REG5_SPEC>`"]
pub type HIDDEN_REG5 = crate::Reg<hidden_reg5::HIDDEN_REG5_SPEC>;
#[doc = "Hidden registers defined in edma_defs.v '`define gem_axi_tx_full_threshold1 12'h4fc // AXI full threshold setting *** HIDDEN Register ***'."]
pub mod hidden_reg5;
#[doc = "SCREENING_TYPE_1_REGISTER_0 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_0_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_0 =
    crate::Reg<screening_type_1_register_0::SCREENING_TYPE_1_REGISTER_0_SPEC>;
#[doc = "Screening type 1 registers are used to allocate up to 16 priority queues to received frames based on certain IP or UDP fields of incoming frames. Firstly, when DS/TC match enable is set (bit 28), the DS (Differentiated Services) field of the received IPv4 header or TCfield (traffic class) of IPv6 headers are matched against bits 11 to 4. Secondly, when UDP port match enable is set (bit 29), the UDP Destination Port of the received UDP frame is matched against bits 27 to 12. Both UDP and DS/TC matching can be enabled simultaneously or individually. If a match is successful, then the queue value programmed in bits 2 to 0 is allocated to the frame. The required number of Type 1 screening registers is configured in the gem defines file. Up to 16 type 1 screening registers have been allocated APB address space between 0x500 and 0x53C."]
pub mod screening_type_1_register_0;
#[doc = "SCREENING_TYPE_1_REGISTER_1 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_1_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_1 =
    crate::Reg<screening_type_1_register_1::SCREENING_TYPE_1_REGISTER_1_SPEC>;
#[doc = "screening type 1 register 1, same as screening_type_1_register_0"]
pub mod screening_type_1_register_1;
#[doc = "SCREENING_TYPE_1_REGISTER_2 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_2_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_2 =
    crate::Reg<screening_type_1_register_2::SCREENING_TYPE_1_REGISTER_2_SPEC>;
#[doc = "screening type 1 register 2, same as screening_type_1_register_0"]
pub mod screening_type_1_register_2;
#[doc = "SCREENING_TYPE_1_REGISTER_3 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_3_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_3 =
    crate::Reg<screening_type_1_register_3::SCREENING_TYPE_1_REGISTER_3_SPEC>;
#[doc = "screening type 1 register 3, same as screening_type_1_register_0"]
pub mod screening_type_1_register_3;
#[doc = "SCREENING_TYPE_1_REGISTER_4 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_4_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_4 =
    crate::Reg<screening_type_1_register_4::SCREENING_TYPE_1_REGISTER_4_SPEC>;
#[doc = "screening type 1 register 4, same as screening_type_1_register_0"]
pub mod screening_type_1_register_4;
#[doc = "SCREENING_TYPE_1_REGISTER_5 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_5_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_5 =
    crate::Reg<screening_type_1_register_5::SCREENING_TYPE_1_REGISTER_5_SPEC>;
#[doc = "screening type 1 register 5, same as screening_type_1_register_0"]
pub mod screening_type_1_register_5;
#[doc = "SCREENING_TYPE_1_REGISTER_6 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_6_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_6 =
    crate::Reg<screening_type_1_register_6::SCREENING_TYPE_1_REGISTER_6_SPEC>;
#[doc = "screening type 1 register 6, same as screening_type_1_register_0"]
pub mod screening_type_1_register_6;
#[doc = "SCREENING_TYPE_1_REGISTER_7 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_7_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_7 =
    crate::Reg<screening_type_1_register_7::SCREENING_TYPE_1_REGISTER_7_SPEC>;
#[doc = "screening type 1 register 7, same as screening_type_1_register_0"]
pub mod screening_type_1_register_7;
#[doc = "SCREENING_TYPE_1_REGISTER_8 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_8_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_8 =
    crate::Reg<screening_type_1_register_8::SCREENING_TYPE_1_REGISTER_8_SPEC>;
#[doc = "screening type 1 register 8, same as screening_type_1_register_0"]
pub mod screening_type_1_register_8;
#[doc = "SCREENING_TYPE_1_REGISTER_9 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_9_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_9 =
    crate::Reg<screening_type_1_register_9::SCREENING_TYPE_1_REGISTER_9_SPEC>;
#[doc = "screening type 1 register 9, same as screening_type_1_register_0"]
pub mod screening_type_1_register_9;
#[doc = "SCREENING_TYPE_1_REGISTER_10 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_10_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_10 =
    crate::Reg<screening_type_1_register_10::SCREENING_TYPE_1_REGISTER_10_SPEC>;
#[doc = "screening type 1 register 10, same as screening_type_1_register_0"]
pub mod screening_type_1_register_10;
#[doc = "SCREENING_TYPE_1_REGISTER_11 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_11_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_11 =
    crate::Reg<screening_type_1_register_11::SCREENING_TYPE_1_REGISTER_11_SPEC>;
#[doc = "screening type 1 register 11, same as screening_type_1_register_0"]
pub mod screening_type_1_register_11;
#[doc = "SCREENING_TYPE_1_REGISTER_12 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_12_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_12 =
    crate::Reg<screening_type_1_register_12::SCREENING_TYPE_1_REGISTER_12_SPEC>;
#[doc = "screening type 1 register 12, same as screening_type_1_register_0"]
pub mod screening_type_1_register_12;
#[doc = "SCREENING_TYPE_1_REGISTER_13 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_13_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_13 =
    crate::Reg<screening_type_1_register_13::SCREENING_TYPE_1_REGISTER_13_SPEC>;
#[doc = "screening type 1 register 13, same as screening_type_1_register_0"]
pub mod screening_type_1_register_13;
#[doc = "SCREENING_TYPE_1_REGISTER_14 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_14_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_14 =
    crate::Reg<screening_type_1_register_14::SCREENING_TYPE_1_REGISTER_14_SPEC>;
#[doc = "screening type 1 register 14, same as screening_type_1_register_0"]
pub mod screening_type_1_register_14;
#[doc = "SCREENING_TYPE_1_REGISTER_15 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_1_REGISTER_15_SPEC>`"]
pub type SCREENING_TYPE_1_REGISTER_15 =
    crate::Reg<screening_type_1_register_15::SCREENING_TYPE_1_REGISTER_15_SPEC>;
#[doc = "screening type 1 register 15, same as screening_type_1_register_0"]
pub mod screening_type_1_register_15;
#[doc = "SCREENING_TYPE_2_REGISTER_0 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_0_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_0 =
    crate::Reg<screening_type_2_register_0::SCREENING_TYPE_2_REGISTER_0_SPEC>;
#[doc = "Screener Type 2 match registers operate independently of screener type 1 registers and offer additional match capabilities, extending the capabilities into vendor specific protocols."]
pub mod screening_type_2_register_0;
#[doc = "SCREENING_TYPE_2_REGISTER_1 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_1_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_1 =
    crate::Reg<screening_type_2_register_1::SCREENING_TYPE_2_REGISTER_1_SPEC>;
#[doc = "screening type 2 register 1, same as screening_type_2_register_0"]
pub mod screening_type_2_register_1;
#[doc = "SCREENING_TYPE_2_REGISTER_2 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_2_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_2 =
    crate::Reg<screening_type_2_register_2::SCREENING_TYPE_2_REGISTER_2_SPEC>;
#[doc = "screening type 2 register 2, same as screening_type_2_register_0"]
pub mod screening_type_2_register_2;
#[doc = "SCREENING_TYPE_2_REGISTER_3 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_3_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_3 =
    crate::Reg<screening_type_2_register_3::SCREENING_TYPE_2_REGISTER_3_SPEC>;
#[doc = "screening type 2 register 3, same as screening_type_2_register_0"]
pub mod screening_type_2_register_3;
#[doc = "SCREENING_TYPE_2_REGISTER_4 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_4_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_4 =
    crate::Reg<screening_type_2_register_4::SCREENING_TYPE_2_REGISTER_4_SPEC>;
#[doc = "screening type 2 register 4, same as screening_type_2_register_0"]
pub mod screening_type_2_register_4;
#[doc = "SCREENING_TYPE_2_REGISTER_5 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_5_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_5 =
    crate::Reg<screening_type_2_register_5::SCREENING_TYPE_2_REGISTER_5_SPEC>;
#[doc = "screening type 2 register 5, same as screening_type_2_register_0"]
pub mod screening_type_2_register_5;
#[doc = "SCREENING_TYPE_2_REGISTER_6 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_6_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_6 =
    crate::Reg<screening_type_2_register_6::SCREENING_TYPE_2_REGISTER_6_SPEC>;
#[doc = "screening type 2 register 6, same as screening_type_2_register_0"]
pub mod screening_type_2_register_6;
#[doc = "SCREENING_TYPE_2_REGISTER_7 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_7_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_7 =
    crate::Reg<screening_type_2_register_7::SCREENING_TYPE_2_REGISTER_7_SPEC>;
#[doc = "screening type 2 register 7, same as screening_type_2_register_0"]
pub mod screening_type_2_register_7;
#[doc = "SCREENING_TYPE_2_REGISTER_8 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_8_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_8 =
    crate::Reg<screening_type_2_register_8::SCREENING_TYPE_2_REGISTER_8_SPEC>;
#[doc = "screening type 2 register 8, same as screening_type_2_register_0"]
pub mod screening_type_2_register_8;
#[doc = "SCREENING_TYPE_2_REGISTER_9 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_9_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_9 =
    crate::Reg<screening_type_2_register_9::SCREENING_TYPE_2_REGISTER_9_SPEC>;
#[doc = "screening type 2 register 9, same as screening_type_2_register_0"]
pub mod screening_type_2_register_9;
#[doc = "SCREENING_TYPE_2_REGISTER_10 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_10_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_10 =
    crate::Reg<screening_type_2_register_10::SCREENING_TYPE_2_REGISTER_10_SPEC>;
#[doc = "screening type 2 register 10, same as screening_type_2_register_0"]
pub mod screening_type_2_register_10;
#[doc = "SCREENING_TYPE_2_REGISTER_11 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_11_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_11 =
    crate::Reg<screening_type_2_register_11::SCREENING_TYPE_2_REGISTER_11_SPEC>;
#[doc = "screening type 2 register 11, same as screening_type_2_register_0"]
pub mod screening_type_2_register_11;
#[doc = "SCREENING_TYPE_2_REGISTER_12 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_12_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_12 =
    crate::Reg<screening_type_2_register_12::SCREENING_TYPE_2_REGISTER_12_SPEC>;
#[doc = "screening type 2 register 12, same as screening_type_2_register_0"]
pub mod screening_type_2_register_12;
#[doc = "SCREENING_TYPE_2_REGISTER_13 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_13_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_13 =
    crate::Reg<screening_type_2_register_13::SCREENING_TYPE_2_REGISTER_13_SPEC>;
#[doc = "screening type 2 register 13, same as screening_type_2_register_0"]
pub mod screening_type_2_register_13;
#[doc = "SCREENING_TYPE_2_REGISTER_14 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_14_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_14 =
    crate::Reg<screening_type_2_register_14::SCREENING_TYPE_2_REGISTER_14_SPEC>;
#[doc = "screening type 2 register 14, same as screening_type_2_register_0"]
pub mod screening_type_2_register_14;
#[doc = "SCREENING_TYPE_2_REGISTER_15 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_REGISTER_15_SPEC>`"]
pub type SCREENING_TYPE_2_REGISTER_15 =
    crate::Reg<screening_type_2_register_15::SCREENING_TYPE_2_REGISTER_15_SPEC>;
#[doc = "screening type 2 register 15, same as screening_type_2_register_0"]
pub mod screening_type_2_register_15;
#[doc = "TX_SCHED_CTRL (rw) register accessor: an alias for `Reg<TX_SCHED_CTRL_SPEC>`"]
pub type TX_SCHED_CTRL = crate::Reg<tx_sched_ctrl::TX_SCHED_CTRL_SPEC>;
#[doc = "This register controls the transmit scheduling algorithm the user can select for each active transmit queue. By default all queues are initialized to fixed priority, with the top indexed queue having overall priority"]
pub mod tx_sched_ctrl;
#[doc = "BW_RATE_LIMIT_Q0TO3 (rw) register accessor: an alias for `Reg<BW_RATE_LIMIT_Q0TO3_SPEC>`"]
pub type BW_RATE_LIMIT_Q0TO3 = crate::Reg<bw_rate_limit_q0to3::BW_RATE_LIMIT_Q0TO3_SPEC>;
#[doc = "This register holds the DWRR weighting value or the ETS bandwidth percentage value used by the transmit scheduler for queues 0 to 3."]
pub mod bw_rate_limit_q0to3;
#[doc = "BW_RATE_LIMIT_Q4TO7 (rw) register accessor: an alias for `Reg<BW_RATE_LIMIT_Q4TO7_SPEC>`"]
pub type BW_RATE_LIMIT_Q4TO7 = crate::Reg<bw_rate_limit_q4to7::BW_RATE_LIMIT_Q4TO7_SPEC>;
#[doc = "Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
pub mod bw_rate_limit_q4to7;
#[doc = "BW_RATE_LIMIT_Q8TO11 (r) register accessor: an alias for `Reg<BW_RATE_LIMIT_Q8TO11_SPEC>`"]
pub type BW_RATE_LIMIT_Q8TO11 = crate::Reg<bw_rate_limit_q8to11::BW_RATE_LIMIT_Q8TO11_SPEC>;
#[doc = "Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
pub mod bw_rate_limit_q8to11;
#[doc = "BW_RATE_LIMIT_Q12TO15 (r) register accessor: an alias for `Reg<BW_RATE_LIMIT_Q12TO15_SPEC>`"]
pub type BW_RATE_LIMIT_Q12TO15 = crate::Reg<bw_rate_limit_q12to15::BW_RATE_LIMIT_Q12TO15_SPEC>;
#[doc = "Not presents. MXETH has only 3 queues. Access to the register returns AHB error."]
pub mod bw_rate_limit_q12to15;
#[doc = "TX_Q_SEG_ALLOC_Q0TO7 (rw) register accessor: an alias for `Reg<TX_Q_SEG_ALLOC_Q0TO7_SPEC>`"]
pub type TX_Q_SEG_ALLOC_Q0TO7 = crate::Reg<tx_q_seg_alloc_q0to7::TX_Q_SEG_ALLOC_Q0TO7_SPEC>;
#[doc = "This register allows the user to distribute the Transmit SRAM used by the DMA across the priority queues, for queues 0 to 7. The SRAM itself is split into a number of evenly sized segments (this is defined in the verilog configuration defs file - for the configuration used to generate this register description, the total number of segments was set to '16'). Those segments can then be freely distributed across the active queues, in powers of 2. I.e. a value of 0 would mean 1 segment has been allocated to the queue. A value of 1 would mean 2 segments, a value of 2 means 4 segments and so on. The reset values of these registers are defined in the configuration defs file."]
pub mod tx_q_seg_alloc_q0to7;
#[doc = "TX_Q_SEG_ALLOC_Q8TO15 (r) register accessor: an alias for `Reg<TX_Q_SEG_ALLOC_Q8TO15_SPEC>`"]
pub type TX_Q_SEG_ALLOC_Q8TO15 = crate::Reg<tx_q_seg_alloc_q8to15::TX_Q_SEG_ALLOC_Q8TO15_SPEC>;
#[doc = "Not presents. Access to the register returns AHB error."]
pub mod tx_q_seg_alloc_q8to15;
#[doc = "RECEIVE_Q8_PTR (r) register accessor: an alias for `Reg<RECEIVE_Q8_PTR_SPEC>`"]
pub type RECEIVE_Q8_PTR = crate::Reg<receive_q8_ptr::RECEIVE_Q8_PTR_SPEC>;
#[doc = "receive_q8_ptr to receive_q15_ptr doesn't present. Access to the register returns AHB error."]
pub mod receive_q8_ptr;
#[doc = "RECEIVE_Q15_PTR (r) register accessor: an alias for `Reg<RECEIVE_Q15_PTR_SPEC>`"]
pub type RECEIVE_Q15_PTR = crate::Reg<receive_q15_ptr::RECEIVE_Q15_PTR_SPEC>;
#[doc = "Not presents."]
pub mod receive_q15_ptr;
#[doc = "DMA_RXBUF_SIZE_Q8 (r) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q8_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q8 = crate::Reg<dma_rxbuf_size_q8::DMA_RXBUF_SIZE_Q8_SPEC>;
#[doc = "dma_rxbuf_size_q8 to dma_rxbuf_size_q15 doesn't present. Access to the register returns AHB error."]
pub mod dma_rxbuf_size_q8;
#[doc = "DMA_RXBUF_SIZE_Q15 (r) register accessor: an alias for `Reg<DMA_RXBUF_SIZE_Q15_SPEC>`"]
pub type DMA_RXBUF_SIZE_Q15 = crate::Reg<dma_rxbuf_size_q15::DMA_RXBUF_SIZE_Q15_SPEC>;
#[doc = "Not presents."]
pub mod dma_rxbuf_size_q15;
#[doc = "INT_Q1_ENABLE (w) register accessor: an alias for `Reg<INT_Q1_ENABLE_SPEC>`"]
pub type INT_Q1_ENABLE = crate::Reg<int_q1_enable::INT_Q1_ENABLE_SPEC>;
#[doc = "At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
pub mod int_q1_enable;
#[doc = "INT_Q2_ENABLE (w) register accessor: an alias for `Reg<INT_Q2_ENABLE_SPEC>`"]
pub type INT_Q2_ENABLE = crate::Reg<int_q2_enable::INT_Q2_ENABLE_SPEC>;
#[doc = "At reset all interrupts are disabled. Writing a one to the relevant bit location enables the required interrupt. This register is write only and when read will return zero."]
pub mod int_q2_enable;
#[doc = "INT_Q3_ENABLE (r) register accessor: an alias for `Reg<INT_Q3_ENABLE_SPEC>`"]
pub type INT_Q3_ENABLE = crate::Reg<int_q3_enable::INT_Q3_ENABLE_SPEC>;
#[doc = "int_q3_enable to int_q7_enable doesn't present. Access to the register returns AHB error."]
pub mod int_q3_enable;
#[doc = "INT_Q7_ENABLE (r) register accessor: an alias for `Reg<INT_Q7_ENABLE_SPEC>`"]
pub type INT_Q7_ENABLE = crate::Reg<int_q7_enable::INT_Q7_ENABLE_SPEC>;
#[doc = "Not presents."]
pub mod int_q7_enable;
#[doc = "INT_Q1_DISABLE (w) register accessor: an alias for `Reg<INT_Q1_DISABLE_SPEC>`"]
pub type INT_Q1_DISABLE = crate::Reg<int_q1_disable::INT_Q1_DISABLE_SPEC>;
#[doc = "Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
pub mod int_q1_disable;
#[doc = "INT_Q2_DISABLE (w) register accessor: an alias for `Reg<INT_Q2_DISABLE_SPEC>`"]
pub type INT_Q2_DISABLE = crate::Reg<int_q2_disable::INT_Q2_DISABLE_SPEC>;
#[doc = "Writing a 1 to the relevant bit location disables that particular interrupt. This register is write only and when read will return zero."]
pub mod int_q2_disable;
#[doc = "INT_Q3_DISABLE (r) register accessor: an alias for `Reg<INT_Q3_DISABLE_SPEC>`"]
pub type INT_Q3_DISABLE = crate::Reg<int_q3_disable::INT_Q3_DISABLE_SPEC>;
#[doc = "int_q3_disable to int_q7_disable doesn't present. Access to the register returns AHB error."]
pub mod int_q3_disable;
#[doc = "INT_Q7_DISABLE (r) register accessor: an alias for `Reg<INT_Q7_DISABLE_SPEC>`"]
pub type INT_Q7_DISABLE = crate::Reg<int_q7_disable::INT_Q7_DISABLE_SPEC>;
#[doc = "Not presents."]
pub mod int_q7_disable;
#[doc = "INT_Q1_MASK (r) register accessor: an alias for `Reg<INT_Q1_MASK_SPEC>`"]
pub type INT_Q1_MASK = crate::Reg<int_q1_mask::INT_Q1_MASK_SPEC>;
#[doc = "The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
pub mod int_q1_mask;
#[doc = "INT_Q2_MASK (r) register accessor: an alias for `Reg<INT_Q2_MASK_SPEC>`"]
pub type INT_Q2_MASK = crate::Reg<int_q2_mask::INT_Q2_MASK_SPEC>;
#[doc = "The interrupt mask register is a read only register indicating which interrupts are masked. All bits are set at reset and can be reset individually by writing to the interrupt enable register or set individually by writing to the interrupt disable register. Having separate address locations for enable and disable saves the need for performing a read modify write when updating the interrupt mask register. For test purposes there is a write-only function to this register that allows the bits in the interrupt status register to be set or cleared, regardless of the state of the mask register."]
pub mod int_q2_mask;
#[doc = "INT_Q3_MASK (r) register accessor: an alias for `Reg<INT_Q3_MASK_SPEC>`"]
pub type INT_Q3_MASK = crate::Reg<int_q3_mask::INT_Q3_MASK_SPEC>;
#[doc = "int_q3_mask to int_q7_mask doesn't present. Access to the register returns AHB error."]
pub mod int_q3_mask;
#[doc = "INT_Q7_MASK (r) register accessor: an alias for `Reg<INT_Q7_MASK_SPEC>`"]
pub type INT_Q7_MASK = crate::Reg<int_q7_mask::INT_Q7_MASK_SPEC>;
#[doc = "Not presents."]
pub mod int_q7_mask;
#[doc = "INT_Q8_ENABLE (r) register accessor: an alias for `Reg<INT_Q8_ENABLE_SPEC>`"]
pub type INT_Q8_ENABLE = crate::Reg<int_q8_enable::INT_Q8_ENABLE_SPEC>;
#[doc = "int_q8_enable to int_q15_enable doesn't present. Access to the register returns AHB error."]
pub mod int_q8_enable;
#[doc = "INT_Q15_ENABLE (r) register accessor: an alias for `Reg<INT_Q15_ENABLE_SPEC>`"]
pub type INT_Q15_ENABLE = crate::Reg<int_q15_enable::INT_Q15_ENABLE_SPEC>;
#[doc = "Not presents."]
pub mod int_q15_enable;
#[doc = "INT_Q8_DISABLE (r) register accessor: an alias for `Reg<INT_Q8_DISABLE_SPEC>`"]
pub type INT_Q8_DISABLE = crate::Reg<int_q8_disable::INT_Q8_DISABLE_SPEC>;
#[doc = "int_q8_disable to int_q15_disable doesn't present. Access to the register returns AHB error."]
pub mod int_q8_disable;
#[doc = "INT_Q15_DISABLE (r) register accessor: an alias for `Reg<INT_Q15_DISABLE_SPEC>`"]
pub type INT_Q15_DISABLE = crate::Reg<int_q15_disable::INT_Q15_DISABLE_SPEC>;
#[doc = "Not presents."]
pub mod int_q15_disable;
#[doc = "INT_Q8_MASK (r) register accessor: an alias for `Reg<INT_Q8_MASK_SPEC>`"]
pub type INT_Q8_MASK = crate::Reg<int_q8_mask::INT_Q8_MASK_SPEC>;
#[doc = "int_q8_mask to int_q15_mask doesn't present. Access to the register returns AHB error."]
pub mod int_q8_mask;
#[doc = "INT_Q15_MASK (r) register accessor: an alias for `Reg<INT_Q15_MASK_SPEC>`"]
pub type INT_Q15_MASK = crate::Reg<int_q15_mask::INT_Q15_MASK_SPEC>;
#[doc = "Not presents."]
pub mod int_q15_mask;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_0 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_0_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_0 =
    crate::Reg<screening_type_2_ethertype_reg_0::SCREENING_TYPE_2_ETHERTYPE_REG_0_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_0;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_1 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_1_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_1 =
    crate::Reg<screening_type_2_ethertype_reg_1::SCREENING_TYPE_2_ETHERTYPE_REG_1_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_1;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_2 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_2 =
    crate::Reg<screening_type_2_ethertype_reg_2::SCREENING_TYPE_2_ETHERTYPE_REG_2_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_2;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_3 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_3_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_3 =
    crate::Reg<screening_type_2_ethertype_reg_3::SCREENING_TYPE_2_ETHERTYPE_REG_3_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_3;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_4 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_4_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_4 =
    crate::Reg<screening_type_2_ethertype_reg_4::SCREENING_TYPE_2_ETHERTYPE_REG_4_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_4;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_5 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_5_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_5 =
    crate::Reg<screening_type_2_ethertype_reg_5::SCREENING_TYPE_2_ETHERTYPE_REG_5_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_5;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_6 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_6_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_6 =
    crate::Reg<screening_type_2_ethertype_reg_6::SCREENING_TYPE_2_ETHERTYPE_REG_6_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_6;
#[doc = "SCREENING_TYPE_2_ETHERTYPE_REG_7 (rw) register accessor: an alias for `Reg<SCREENING_TYPE_2_ETHERTYPE_REG_7_SPEC>`"]
pub type SCREENING_TYPE_2_ETHERTYPE_REG_7 =
    crate::Reg<screening_type_2_ethertype_reg_7::SCREENING_TYPE_2_ETHERTYPE_REG_7_SPEC>;
#[doc = "Ethertype Register"]
pub mod screening_type_2_ethertype_reg_7;
#[doc = "TYPE2_COMPARE_0_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_0_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_0_WORD_0 = crate::Reg<type2_compare_0_word_0::TYPE2_COMPARE_0_WORD_0_SPEC>;
#[doc = "'Compare A, B and C fields of the screener type 2 match register are pointers to a pool of up to 32 compare registers. If enabled the compare is true if the data at the OFFSET into the frame, ANDed with the MASK Value if the mask is enabled, is equal to the COMPARE Value. Either a 16 bit comparison or a 32 bit comparison is done. This selection is made via the associated compare word1 register bit 9. If a 16 bit comparison is selected, then a 16 bit mask is also available to the user to select which bits should be compared. If the 32 bit compare option is selected, then no mask is available. The byte at the OFFSET number of bytes from the index start is compared thru bits 7 to 0 of the configured VALUE. The byte at the OFFSET number of bytes + 1 from the index start is compared thru bits 15 to 8 of the configured VALUE and so on. The OFFSET can be configured to be from 0 to 127 bytes from either the start of the frame, the byte following the therType field (last EtherType in the header if the frame is VLAN tagged), the byte following the IP header (IPv4 or IPv6) or from the byte following the start of the TCP/UDP header. The required number of Type 2 screening registers up to a maximum of 32 is configurable in the gem defines file and have been allocated APB address space between 0x700 and 0x7fc. Note. when using RX Partial Store and Forward mode and priority queues, the frame offset must be less than the Partial Store and Forward watermark. If the offset is higher than the watermark value it's not possible to identify the priority queue before the frame is sent to the AMBA interface, and an incorrect priority queue may be used. '"]
pub mod type2_compare_0_word_0;
#[doc = "TYPE2_COMPARE_0_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_0_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_0_WORD_1 = crate::Reg<type2_compare_0_word_1::TYPE2_COMPARE_0_WORD_1_SPEC>;
#[doc = "'Type2 Compare Word 1'"]
pub mod type2_compare_0_word_1;
#[doc = "TYPE2_COMPARE_1_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_1_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_1_WORD_0 = crate::Reg<type2_compare_1_word_0::TYPE2_COMPARE_1_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_1_word_0;
#[doc = "TYPE2_COMPARE_1_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_1_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_1_WORD_1 = crate::Reg<type2_compare_1_word_1::TYPE2_COMPARE_1_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_1_word_1;
#[doc = "TYPE2_COMPARE_2_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_2_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_2_WORD_0 = crate::Reg<type2_compare_2_word_0::TYPE2_COMPARE_2_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_2_word_0;
#[doc = "TYPE2_COMPARE_2_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_2_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_2_WORD_1 = crate::Reg<type2_compare_2_word_1::TYPE2_COMPARE_2_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_2_word_1;
#[doc = "TYPE2_COMPARE_3_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_3_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_3_WORD_0 = crate::Reg<type2_compare_3_word_0::TYPE2_COMPARE_3_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_3_word_0;
#[doc = "TYPE2_COMPARE_3_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_3_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_3_WORD_1 = crate::Reg<type2_compare_3_word_1::TYPE2_COMPARE_3_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_3_word_1;
#[doc = "TYPE2_COMPARE_4_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_4_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_4_WORD_0 = crate::Reg<type2_compare_4_word_0::TYPE2_COMPARE_4_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_4_word_0;
#[doc = "TYPE2_COMPARE_4_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_4_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_4_WORD_1 = crate::Reg<type2_compare_4_word_1::TYPE2_COMPARE_4_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_4_word_1;
#[doc = "TYPE2_COMPARE_5_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_5_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_5_WORD_0 = crate::Reg<type2_compare_5_word_0::TYPE2_COMPARE_5_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_5_word_0;
#[doc = "TYPE2_COMPARE_5_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_5_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_5_WORD_1 = crate::Reg<type2_compare_5_word_1::TYPE2_COMPARE_5_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_5_word_1;
#[doc = "TYPE2_COMPARE_6_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_6_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_6_WORD_0 = crate::Reg<type2_compare_6_word_0::TYPE2_COMPARE_6_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_6_word_0;
#[doc = "TYPE2_COMPARE_6_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_6_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_6_WORD_1 = crate::Reg<type2_compare_6_word_1::TYPE2_COMPARE_6_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_6_word_1;
#[doc = "TYPE2_COMPARE_7_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_7_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_7_WORD_0 = crate::Reg<type2_compare_7_word_0::TYPE2_COMPARE_7_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_7_word_0;
#[doc = "TYPE2_COMPARE_7_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_7_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_7_WORD_1 = crate::Reg<type2_compare_7_word_1::TYPE2_COMPARE_7_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_7_word_1;
#[doc = "TYPE2_COMPARE_8_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_8_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_8_WORD_0 = crate::Reg<type2_compare_8_word_0::TYPE2_COMPARE_8_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_8_word_0;
#[doc = "TYPE2_COMPARE_8_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_8_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_8_WORD_1 = crate::Reg<type2_compare_8_word_1::TYPE2_COMPARE_8_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_8_word_1;
#[doc = "TYPE2_COMPARE_9_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_9_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_9_WORD_0 = crate::Reg<type2_compare_9_word_0::TYPE2_COMPARE_9_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_9_word_0;
#[doc = "TYPE2_COMPARE_9_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_9_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_9_WORD_1 = crate::Reg<type2_compare_9_word_1::TYPE2_COMPARE_9_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_9_word_1;
#[doc = "TYPE2_COMPARE_10_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_10_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_10_WORD_0 =
    crate::Reg<type2_compare_10_word_0::TYPE2_COMPARE_10_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_10_word_0;
#[doc = "TYPE2_COMPARE_10_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_10_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_10_WORD_1 =
    crate::Reg<type2_compare_10_word_1::TYPE2_COMPARE_10_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_10_word_1;
#[doc = "TYPE2_COMPARE_11_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_11_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_11_WORD_0 =
    crate::Reg<type2_compare_11_word_0::TYPE2_COMPARE_11_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_11_word_0;
#[doc = "TYPE2_COMPARE_11_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_11_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_11_WORD_1 =
    crate::Reg<type2_compare_11_word_1::TYPE2_COMPARE_11_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_11_word_1;
#[doc = "TYPE2_COMPARE_12_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_12_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_12_WORD_0 =
    crate::Reg<type2_compare_12_word_0::TYPE2_COMPARE_12_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_12_word_0;
#[doc = "TYPE2_COMPARE_12_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_12_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_12_WORD_1 =
    crate::Reg<type2_compare_12_word_1::TYPE2_COMPARE_12_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_12_word_1;
#[doc = "TYPE2_COMPARE_13_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_13_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_13_WORD_0 =
    crate::Reg<type2_compare_13_word_0::TYPE2_COMPARE_13_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_13_word_0;
#[doc = "TYPE2_COMPARE_13_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_13_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_13_WORD_1 =
    crate::Reg<type2_compare_13_word_1::TYPE2_COMPARE_13_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_13_word_1;
#[doc = "TYPE2_COMPARE_14_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_14_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_14_WORD_0 =
    crate::Reg<type2_compare_14_word_0::TYPE2_COMPARE_14_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_14_word_0;
#[doc = "TYPE2_COMPARE_14_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_14_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_14_WORD_1 =
    crate::Reg<type2_compare_14_word_1::TYPE2_COMPARE_14_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_14_word_1;
#[doc = "TYPE2_COMPARE_15_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_15_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_15_WORD_0 =
    crate::Reg<type2_compare_15_word_0::TYPE2_COMPARE_15_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_15_word_0;
#[doc = "TYPE2_COMPARE_15_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_15_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_15_WORD_1 =
    crate::Reg<type2_compare_15_word_1::TYPE2_COMPARE_15_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_15_word_1;
#[doc = "TYPE2_COMPARE_16_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_16_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_16_WORD_0 =
    crate::Reg<type2_compare_16_word_0::TYPE2_COMPARE_16_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_16_word_0;
#[doc = "TYPE2_COMPARE_16_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_16_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_16_WORD_1 =
    crate::Reg<type2_compare_16_word_1::TYPE2_COMPARE_16_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_16_word_1;
#[doc = "TYPE2_COMPARE_17_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_17_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_17_WORD_0 =
    crate::Reg<type2_compare_17_word_0::TYPE2_COMPARE_17_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_17_word_0;
#[doc = "TYPE2_COMPARE_17_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_17_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_17_WORD_1 =
    crate::Reg<type2_compare_17_word_1::TYPE2_COMPARE_17_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_17_word_1;
#[doc = "TYPE2_COMPARE_18_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_18_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_18_WORD_0 =
    crate::Reg<type2_compare_18_word_0::TYPE2_COMPARE_18_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_18_word_0;
#[doc = "TYPE2_COMPARE_18_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_18_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_18_WORD_1 =
    crate::Reg<type2_compare_18_word_1::TYPE2_COMPARE_18_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_18_word_1;
#[doc = "TYPE2_COMPARE_19_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_19_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_19_WORD_0 =
    crate::Reg<type2_compare_19_word_0::TYPE2_COMPARE_19_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_19_word_0;
#[doc = "TYPE2_COMPARE_19_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_19_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_19_WORD_1 =
    crate::Reg<type2_compare_19_word_1::TYPE2_COMPARE_19_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_19_word_1;
#[doc = "TYPE2_COMPARE_20_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_20_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_20_WORD_0 =
    crate::Reg<type2_compare_20_word_0::TYPE2_COMPARE_20_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_20_word_0;
#[doc = "TYPE2_COMPARE_20_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_20_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_20_WORD_1 =
    crate::Reg<type2_compare_20_word_1::TYPE2_COMPARE_20_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_20_word_1;
#[doc = "TYPE2_COMPARE_21_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_21_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_21_WORD_0 =
    crate::Reg<type2_compare_21_word_0::TYPE2_COMPARE_21_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_21_word_0;
#[doc = "TYPE2_COMPARE_21_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_21_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_21_WORD_1 =
    crate::Reg<type2_compare_21_word_1::TYPE2_COMPARE_21_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_21_word_1;
#[doc = "TYPE2_COMPARE_22_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_22_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_22_WORD_0 =
    crate::Reg<type2_compare_22_word_0::TYPE2_COMPARE_22_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_22_word_0;
#[doc = "TYPE2_COMPARE_22_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_22_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_22_WORD_1 =
    crate::Reg<type2_compare_22_word_1::TYPE2_COMPARE_22_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_22_word_1;
#[doc = "TYPE2_COMPARE_23_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_23_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_23_WORD_0 =
    crate::Reg<type2_compare_23_word_0::TYPE2_COMPARE_23_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_23_word_0;
#[doc = "TYPE2_COMPARE_23_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_23_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_23_WORD_1 =
    crate::Reg<type2_compare_23_word_1::TYPE2_COMPARE_23_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_23_word_1;
#[doc = "TYPE2_COMPARE_24_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_24_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_24_WORD_0 =
    crate::Reg<type2_compare_24_word_0::TYPE2_COMPARE_24_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_24_word_0;
#[doc = "TYPE2_COMPARE_24_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_24_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_24_WORD_1 =
    crate::Reg<type2_compare_24_word_1::TYPE2_COMPARE_24_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_24_word_1;
#[doc = "TYPE2_COMPARE_25_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_25_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_25_WORD_0 =
    crate::Reg<type2_compare_25_word_0::TYPE2_COMPARE_25_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_25_word_0;
#[doc = "TYPE2_COMPARE_25_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_25_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_25_WORD_1 =
    crate::Reg<type2_compare_25_word_1::TYPE2_COMPARE_25_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_25_word_1;
#[doc = "TYPE2_COMPARE_26_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_26_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_26_WORD_0 =
    crate::Reg<type2_compare_26_word_0::TYPE2_COMPARE_26_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_26_word_0;
#[doc = "TYPE2_COMPARE_26_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_26_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_26_WORD_1 =
    crate::Reg<type2_compare_26_word_1::TYPE2_COMPARE_26_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_26_word_1;
#[doc = "TYPE2_COMPARE_27_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_27_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_27_WORD_0 =
    crate::Reg<type2_compare_27_word_0::TYPE2_COMPARE_27_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_27_word_0;
#[doc = "TYPE2_COMPARE_27_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_27_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_27_WORD_1 =
    crate::Reg<type2_compare_27_word_1::TYPE2_COMPARE_27_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_27_word_1;
#[doc = "TYPE2_COMPARE_28_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_28_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_28_WORD_0 =
    crate::Reg<type2_compare_28_word_0::TYPE2_COMPARE_28_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_28_word_0;
#[doc = "TYPE2_COMPARE_28_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_28_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_28_WORD_1 =
    crate::Reg<type2_compare_28_word_1::TYPE2_COMPARE_28_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_28_word_1;
#[doc = "TYPE2_COMPARE_29_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_29_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_29_WORD_0 =
    crate::Reg<type2_compare_29_word_0::TYPE2_COMPARE_29_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_29_word_0;
#[doc = "TYPE2_COMPARE_29_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_29_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_29_WORD_1 =
    crate::Reg<type2_compare_29_word_1::TYPE2_COMPARE_29_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_29_word_1;
#[doc = "TYPE2_COMPARE_30_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_30_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_30_WORD_0 =
    crate::Reg<type2_compare_30_word_0::TYPE2_COMPARE_30_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_30_word_0;
#[doc = "TYPE2_COMPARE_30_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_30_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_30_WORD_1 =
    crate::Reg<type2_compare_30_word_1::TYPE2_COMPARE_30_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_30_word_1;
#[doc = "TYPE2_COMPARE_31_WORD_0 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_31_WORD_0_SPEC>`"]
pub type TYPE2_COMPARE_31_WORD_0 =
    crate::Reg<type2_compare_31_word_0::TYPE2_COMPARE_31_WORD_0_SPEC>;
#[doc = "same as type2_compare_0_word_0"]
pub mod type2_compare_31_word_0;
#[doc = "TYPE2_COMPARE_31_WORD_1 (rw) register accessor: an alias for `Reg<TYPE2_COMPARE_31_WORD_1_SPEC>`"]
pub type TYPE2_COMPARE_31_WORD_1 =
    crate::Reg<type2_compare_31_word_1::TYPE2_COMPARE_31_WORD_1_SPEC>;
#[doc = "same as type2_compare_0_word_1"]
pub mod type2_compare_31_word_1;
