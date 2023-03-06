#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVS` reader - N/A"]
pub type OVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OVS` writer - N/A"]
pub type OVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `EC_AM_MODE` reader - This field specifies the clocking for the address matching (I2C) or slave selection detection logic (SPI) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_AM_MODE` writer - This field specifies the clocking for the address matching (I2C) or slave selection detection logic (SPI) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
pub type EC_AM_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EC_OP_MODE` reader - This field specifies the clocking for the SCB block '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EC_OP_MODE` writer - This field specifies the clocking for the SCB block '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
pub type EC_OP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EZ_MODE` reader - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames not separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `EZ_MODE` writer - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames not separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
pub type EZ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CMD_RESP_MODE` reader - N/A"]
pub type CMD_RESP_MODE_R = crate::BitReader<bool>;
#[doc = "Field `CMD_RESP_MODE` writer - N/A"]
pub type CMD_RESP_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MEM_WIDTH` reader - N/A"]
pub type MEM_WIDTH_R = crate::FieldReader<u8, MEM_WIDTH_A>;
#[doc = "N/A\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_WIDTH_A {
    #[doc = "0: N/A"]
    BYTE = 0,
    #[doc = "1: N/A"]
    HALFWORD = 1,
    #[doc = "2: N/A"]
    WORD = 2,
    #[doc = "3: N/A"]
    RSVD = 3,
}
impl From<MEM_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_WIDTH_A) -> Self {
        variant as _
    }
}
impl MEM_WIDTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM_WIDTH_A {
        match self.bits {
            0 => MEM_WIDTH_A::BYTE,
            1 => MEM_WIDTH_A::HALFWORD,
            2 => MEM_WIDTH_A::WORD,
            3 => MEM_WIDTH_A::RSVD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == MEM_WIDTH_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == MEM_WIDTH_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == MEM_WIDTH_A::WORD
    }
    #[doc = "Checks if the value of the field is `RSVD`"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == MEM_WIDTH_A::RSVD
    }
}
#[doc = "Field `MEM_WIDTH` writer - N/A"]
pub type MEM_WIDTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, MEM_WIDTH_A, 2, O>;
impl<'a, const O: u8> MEM_WIDTH_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(MEM_WIDTH_A::BYTE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(MEM_WIDTH_A::HALFWORD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(MEM_WIDTH_A::WORD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut W {
        self.variant(MEM_WIDTH_A::RSVD)
    }
}
#[doc = "Field `ADDR_ACCEPT` reader - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_R = crate::BitReader<bool>;
#[doc = "Field `ADDR_ACCEPT` writer - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
pub type ADDR_ACCEPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BLOCK` reader - Only used in externally clocked mode. If the externally clocked logic and the internal CPU accesses to EZ memory coincide/collide, this bit determines whether the CPU access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: INTR_TX.BLOCKED and INTR_RX.BLOCKED."]
pub type BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BLOCK` writer - Only used in externally clocked mode. If the externally clocked logic and the internal CPU accesses to EZ memory coincide/collide, this bit determines whether the CPU access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: INTR_TX.BLOCKED and INTR_RX.BLOCKED."]
pub type BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - N/A"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "N/A\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: N/A"]
    I2C = 0,
    #[doc = "1: N/A"]
    SPI = 1,
    #[doc = "2: N/A"]
    UART = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::I2C),
            1 => Some(MODE_A::SPI),
            2 => Some(MODE_A::UART),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == MODE_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == MODE_A::SPI
    }
    #[doc = "Checks if the value of the field is `UART`"]
    #[inline(always)]
    pub fn is_uart(&self) -> bool {
        *self == MODE_A::UART
    }
}
#[doc = "Field `MODE` writer - N/A"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(MODE_A::I2C)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(MODE_A::SPI)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn uart(self) -> &'a mut W {
        self.variant(MODE_A::UART)
    }
}
#[doc = "Field `EC_ACCESS` reader - EC_ACCESS is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 1. when waking up from DeepSleep power mode, and PLL is locked (clk_scb is at expected frequency), this filed should be set to 0."]
pub type EC_ACCESS_R = crate::BitReader<bool>;
#[doc = "Field `EC_ACCESS` writer - EC_ACCESS is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 1. when waking up from DeepSleep power mode, and PLL is locked (clk_scb is at expected frequency), this filed should be set to 0."]
pub type EC_ACCESS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - SCB block is enabled ('1') or not ('0'). The proper order in which to initialize SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL registers. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL register to enable SCB, select the specific operation mode and oversampling factor. Generally when this block is enabled, no control information should be changed. Changes should be made AFTER disabling this block, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the block is re-enabled. Note that disabling the block will cause re-initialization of the design and associated state is lost (e.g. FIFO content). Specific to SPI master case, when SCB is idle, below registers can be changed without disabling SCB block, TX_CTRL TX_FIFO_CTRL RX_CTRL RX_FIFO_CTRL SPI_CTRL.SSEL,"]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - SCB block is enabled ('1') or not ('0'). The proper order in which to initialize SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL registers. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL register to enable SCB, select the specific operation mode and oversampling factor. Generally when this block is enabled, no control information should be changed. Changes should be made AFTER disabling this block, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the block is re-enabled. Note that disabling the block will cause re-initialization of the design and associated state is lost (e.g. FIFO content). Specific to SPI master case, when SCB is idle, below registers can be changed without disabling SCB block, TX_CTRL TX_FIFO_CTRL RX_CTRL RX_FIFO_CTRL SPI_CTRL.SSEL,"]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This field specifies the clocking for the address matching (I2C) or slave selection detection logic (SPI) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_am_mode(&self) -> EC_AM_MODE_R {
        EC_AM_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This field specifies the clocking for the SCB block '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ec_op_mode(&self) -> EC_OP_MODE_R {
        EC_OP_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames not separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    pub fn ez_mode(&self) -> EZ_MODE_R {
        EZ_MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    pub fn cmd_resp_mode(&self) -> CMD_RESP_MODE_R {
        CMD_RESP_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 14:15 - N/A"]
    #[inline(always)]
    pub fn mem_width(&self) -> MEM_WIDTH_R {
        MEM_WIDTH_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    pub fn addr_accept(&self) -> ADDR_ACCEPT_R {
        ADDR_ACCEPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the internal CPU accesses to EZ memory coincide/collide, this bit determines whether the CPU access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: INTR_TX.BLOCKED and INTR_RX.BLOCKED."]
    #[inline(always)]
    pub fn block(&self) -> BLOCK_R {
        BLOCK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - EC_ACCESS is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 1. when waking up from DeepSleep power mode, and PLL is locked (clk_scb is at expected frequency), this filed should be set to 0."]
    #[inline(always)]
    pub fn ec_access(&self) -> EC_ACCESS_R {
        EC_ACCESS_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - SCB block is enabled ('1') or not ('0'). The proper order in which to initialize SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL registers. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL register to enable SCB, select the specific operation mode and oversampling factor. Generally when this block is enabled, no control information should be changed. Changes should be made AFTER disabling this block, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the block is re-enabled. Note that disabling the block will cause re-initialization of the design and associated state is lost (e.g. FIFO content). Specific to SPI master case, when SCB is idle, below registers can be changed without disabling SCB block, TX_CTRL TX_FIFO_CTRL RX_CTRL RX_FIFO_CTRL SPI_CTRL.SSEL,"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn ovs(&mut self) -> OVS_W<0> {
        OVS_W::new(self)
    }
    #[doc = "Bit 8 - This field specifies the clocking for the address matching (I2C) or slave selection detection logic (SPI) '0': Internally clocked mode '1': Externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. The clocking for the rest of the logic is determined by CTRL.EC_OP_MODE. Externally clocked mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_am_mode(&mut self) -> EC_AM_MODE_W<8> {
        EC_AM_MODE_W::new(self)
    }
    #[doc = "Bit 9 - This field specifies the clocking for the SCB block '0': Internally clocked mode '1': externally clocked mode In internally clocked mode, the serial interface protocols run off the SCB clock. In externally clocked mode, the serial interface protocols run off the clock as provided by the serial interface. Externally clocked operation mode is only used for synchronous serial interface protocols (SPI and I2C) in slave mode AND EZ mode. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported. The maximum SPI slave, EZ mode bitrate is 48 Mbps (transmission and IO delays outside the IP will degrade the effective bitrate). In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ec_op_mode(&mut self) -> EC_OP_MODE_W<9> {
        EC_OP_MODE_W::new(self)
    }
    #[doc = "Bit 10 - Non EZ mode ('0') or EZ mode ('1'). In EZ mode, a meta protocol is applied to the serial interface protocol. This meta protocol adds meaning to the data frames transferred by the serial interface protocol: a data frame can represent a memory address, a write memory data element or a read memory data element. EZ mode is only used for synchronous serial interface protocols: SPI and I2C. In SPI mode, only Motorola submode (all Motorola modes: 0, 1, 2, 3) is supported and the transmitter should use continuous data frames; i.e. data frames not separated by slave deselection. This mode is only applicable to slave functionality. In EZ mode, the slave can read from and write to an addressable memory structure of 32 bytes. In EZ mode, data frames should 8-bit in size and should be transmitted and received with the Most Significant Bit (MSB) first. In UART mode this field should be '0'."]
    #[inline(always)]
    #[must_use]
    pub fn ez_mode(&mut self) -> EZ_MODE_W<10> {
        EZ_MODE_W::new(self)
    }
    #[doc = "Bit 12 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_resp_mode(&mut self) -> CMD_RESP_MODE_W<12> {
        CMD_RESP_MODE_W::new(self)
    }
    #[doc = "Bits 14:15 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mem_width(&mut self) -> MEM_WIDTH_W<14> {
        MEM_WIDTH_W::new(self)
    }
    #[doc = "Bit 16 - Determines whether a received matching address is accepted in the RX FIFO ('1') or not ('0'). In I2C mode, this field is used to allow the slave to put the received slave address or general call address in the RX FIFO. Note that a received matching address is put in the RX FIFO when this bit is '1' for both I2C read and write transfers. In multi-processor UART receiver mode, this field is used to allow the receiver to put the received address in the RX FIFO. Note: non-matching addresses are never put in the RX FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn addr_accept(&mut self) -> ADDR_ACCEPT_W<16> {
        ADDR_ACCEPT_W::new(self)
    }
    #[doc = "Bit 17 - Only used in externally clocked mode. If the externally clocked logic and the internal CPU accesses to EZ memory coincide/collide, this bit determines whether the CPU access should block and result in bus wait states ('BLOCK is 1') or not (BLOCK is '0'). IF BLOCK is '0' and the accesses collide, CPU read operations return 0xffff:ffff and CPU write operations are ignored. Colliding accesses are registered as interrupt causes: INTR_TX.BLOCKED and INTR_RX.BLOCKED."]
    #[inline(always)]
    #[must_use]
    pub fn block(&mut self) -> BLOCK_W<17> {
        BLOCK_W::new(self)
    }
    #[doc = "Bits 24:25 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<24> {
        MODE_W::new(self)
    }
    #[doc = "Bit 28 - EC_ACCESS is used to enable I2CS_EC or SPIS_EC access to internal EZ memory. 1: enable clk_scb 0: disable clk_scb Before going to deepsleep this field should be set to 1. when waking up from DeepSleep power mode, and PLL is locked (clk_scb is at expected frequency), this filed should be set to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ec_access(&mut self) -> EC_ACCESS_W<28> {
        EC_ACCESS_W::new(self)
    }
    #[doc = "Bit 31 - SCB block is enabled ('1') or not ('0'). The proper order in which to initialize SCB is as follows: - Program protocol specific information using SPI_CTRL, UART_CTRL (and UART_TX_CTRL and UART_RX_CTRL) or I2C_CTRL registers. This includes selection of a submode, master/slave functionality and transmitter/receiver functionality when applicable. - Program generic transmitter (TX_CTRL) and receiver (RX_CTRL) information. This includes enabling of the transmitter and receiver functionality. - Program transmitter FIFO (TX_FIFO_CTRL) and receiver FIFO (RX_FIFO_CTRL) information. - Program CTRL register to enable SCB, select the specific operation mode and oversampling factor. Generally when this block is enabled, no control information should be changed. Changes should be made AFTER disabling this block, e.g. to modify the operation mode (from I2C to SPI) or to go from externally to internally clocked. The change takes effect after the block is re-enabled. Note that disabling the block will cause re-initialization of the design and associated state is lost (e.g. FIFO content). Specific to SPI master case, when SCB is idle, below registers can be changed without disabling SCB block, TX_CTRL TX_FIFO_CTRL RX_CTRL RX_FIFO_CTRL SPI_CTRL.SSEL,"]
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
#[doc = "Generic control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0300_400f"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_400f;
}
