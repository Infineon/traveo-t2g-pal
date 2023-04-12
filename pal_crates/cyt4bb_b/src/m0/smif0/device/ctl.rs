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
#[doc = "Field `WR_EN` reader - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_R = crate::BitReader<bool>;
#[doc = "Field `WR_EN` writer - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
pub type WR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CRYPTO_EN` reader - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_EN` writer - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
pub type CRYPTO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `DATA_SEL` reader - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_SEL` writer - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
pub type DATA_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `MERGE_TIMEOUT` reader - Continuous transfer merge timeout in clk_mem cycles. This limits the standby time of the memory interface, i.e. the time the memory device is selected but no data is transferred. The counting of the merge timeout period is done in the XIP block using clk_mem cycles. It starts when the last TX or RX byte is transferred to or from the data FIFOs. After this period the memory device is deselected. A later transfer, even from a continuous address, starts with the overhead phases (command, address, mode, dummy cycles)."]
pub type MERGE_TIMEOUT_R = crate::FieldReader<u8, MERGE_TIMEOUT_A>;
#[doc = "Continuous transfer merge timeout in clk_mem cycles. This limits the standby time of the memory interface, i.e. the time the memory device is selected but no data is transferred. The counting of the merge timeout period is done in the XIP block using clk_mem cycles. It starts when the last TX or RX byte is transferred to or from the data FIFOs. After this period the memory device is deselected. A later transfer, even from a continuous address, starts with the overhead phases (command, address, mode, dummy cycles).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MERGE_TIMEOUT_A {
    #[doc = "0: Timeout after 1 clk_mem cycle. That means transfers will only be merged if the continuous transfer request is already available when the previous transfer is finished."]
    _1_CYCLE = 0,
    #[doc = "1: Timeout after 2^4 = 16 clk_mem cycles. At a clk_mem frequency of 200MHz this means 80ns."]
    _16_CYCLES = 1,
    #[doc = "2: Timeout after 2^8 = 256 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~1.3us."]
    _256_CYCLES = 2,
    #[doc = "3: Timeout after 2^12 = 4096 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~20us."]
    _4096_CYCLES = 3,
    #[doc = "4: Timeout after 2^16 = 65536 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~330us."]
    _65536_CYCLES = 4,
    #[doc = "5: N/A"]
    RSVD5 = 5,
    #[doc = "6: N/A"]
    RSVD6 = 6,
    #[doc = "7: N/A"]
    RSVD7 = 7,
}
impl From<MERGE_TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: MERGE_TIMEOUT_A) -> Self {
        variant as _
    }
}
impl MERGE_TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MERGE_TIMEOUT_A {
        match self.bits {
            0 => MERGE_TIMEOUT_A::_1_CYCLE,
            1 => MERGE_TIMEOUT_A::_16_CYCLES,
            2 => MERGE_TIMEOUT_A::_256_CYCLES,
            3 => MERGE_TIMEOUT_A::_4096_CYCLES,
            4 => MERGE_TIMEOUT_A::_65536_CYCLES,
            5 => MERGE_TIMEOUT_A::RSVD5,
            6 => MERGE_TIMEOUT_A::RSVD6,
            7 => MERGE_TIMEOUT_A::RSVD7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1_CYCLE`"]
    #[inline(always)]
    pub fn is_1_cycle(&self) -> bool {
        *self == MERGE_TIMEOUT_A::_1_CYCLE
    }
    #[doc = "Checks if the value of the field is `_16_CYCLES`"]
    #[inline(always)]
    pub fn is_16_cycles(&self) -> bool {
        *self == MERGE_TIMEOUT_A::_16_CYCLES
    }
    #[doc = "Checks if the value of the field is `_256_CYCLES`"]
    #[inline(always)]
    pub fn is_256_cycles(&self) -> bool {
        *self == MERGE_TIMEOUT_A::_256_CYCLES
    }
    #[doc = "Checks if the value of the field is `_4096_CYCLES`"]
    #[inline(always)]
    pub fn is_4096_cycles(&self) -> bool {
        *self == MERGE_TIMEOUT_A::_4096_CYCLES
    }
    #[doc = "Checks if the value of the field is `_65536_CYCLES`"]
    #[inline(always)]
    pub fn is_65536_cycles(&self) -> bool {
        *self == MERGE_TIMEOUT_A::_65536_CYCLES
    }
    #[doc = "Checks if the value of the field is `RSVD5`"]
    #[inline(always)]
    pub fn is_rsvd5(&self) -> bool {
        *self == MERGE_TIMEOUT_A::RSVD5
    }
    #[doc = "Checks if the value of the field is `RSVD6`"]
    #[inline(always)]
    pub fn is_rsvd6(&self) -> bool {
        *self == MERGE_TIMEOUT_A::RSVD6
    }
    #[doc = "Checks if the value of the field is `RSVD7`"]
    #[inline(always)]
    pub fn is_rsvd7(&self) -> bool {
        *self == MERGE_TIMEOUT_A::RSVD7
    }
}
#[doc = "Field `MERGE_TIMEOUT` writer - Continuous transfer merge timeout in clk_mem cycles. This limits the standby time of the memory interface, i.e. the time the memory device is selected but no data is transferred. The counting of the merge timeout period is done in the XIP block using clk_mem cycles. It starts when the last TX or RX byte is transferred to or from the data FIFOs. After this period the memory device is deselected. A later transfer, even from a continuous address, starts with the overhead phases (command, address, mode, dummy cycles)."]
pub type MERGE_TIMEOUT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, MERGE_TIMEOUT_A, 3, O>;
impl<'a, const O: u8> MERGE_TIMEOUT_W<'a, O> {
    #[doc = "Timeout after 1 clk_mem cycle. That means transfers will only be merged if the continuous transfer request is already available when the previous transfer is finished."]
    #[inline(always)]
    pub fn _1_cycle(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::_1_CYCLE)
    }
    #[doc = "Timeout after 2^4 = 16 clk_mem cycles. At a clk_mem frequency of 200MHz this means 80ns."]
    #[inline(always)]
    pub fn _16_cycles(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::_16_CYCLES)
    }
    #[doc = "Timeout after 2^8 = 256 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~1.3us."]
    #[inline(always)]
    pub fn _256_cycles(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::_256_CYCLES)
    }
    #[doc = "Timeout after 2^12 = 4096 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~20us."]
    #[inline(always)]
    pub fn _4096_cycles(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::_4096_CYCLES)
    }
    #[doc = "Timeout after 2^16 = 65536 clk_mem cycles. At a clk_mem frequency of 200MHz this means ~330us."]
    #[inline(always)]
    pub fn _65536_cycles(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::_65536_CYCLES)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd5(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::RSVD5)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd6(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::RSVD6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd7(self) -> &'a mut W {
        self.variant(MERGE_TIMEOUT_A::RSVD7)
    }
}
#[doc = "Field `MERGE_EN` reader - Continous transfer merge enable: '0': Disabled. No merging of transfers is done. Longest possible memory transfer is 16 Byte. '1': Enabled. Merging of continous transfers is done. This skips the overhead (command, address, mode, dummy cycles) for a continuous (linear sequential) transfer."]
pub type MERGE_EN_R = crate::BitReader<bool>;
#[doc = "Field `MERGE_EN` writer - Continous transfer merge enable: '0': Disabled. No merging of transfers is done. Longest possible memory transfer is 16 Byte. '1': Enabled. Merging of continous transfers is done. This skips the overhead (command, address, mode, dummy cycles) for a continuous (linear sequential) transfer."]
pub type MERGE_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `TOTAL_TIMEOUT` reader - Total transfer timeout in clk_mem cycles. The counting of the total timout period is done in the XIP block using clk_mem cycles. It starts when the first command of a new (not merged) transaction is transferred to the TX command FIFO causing the interface logic to select the memory. After this period the memory device is deselected. This feature is needed for RAM devices requiring refresh cycles. The value needs to be derived from the RAMs maximum transaction length time (tCMS) minus the time of transferring 2x16byte data block (data granularity of the XIP ports is 16byte, 1 16byte block transfer outstanding, another available for merging). If the result in negative, MERGE_EN must be set to '0'. Example: RAM device tCMS = 4us, interface clock frequency = 100MHz: total transfer timeout is 4us - 2x16x10ns = 3680ns. With clk_mem frequency of 200MHz the TOTAL_TIMEOUT value is 3680/5 = 736. Note: In the unlikely case that the total transfer timeout is used (usually for RAM devices) while the page boundary crossing latency generation is enabled via RD_BOUND_CTL.PRESENT (usually for FLASH devices) the additional time needs to taken into account."]
pub type TOTAL_TIMEOUT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOTAL_TIMEOUT` writer - Total transfer timeout in clk_mem cycles. The counting of the total timout period is done in the XIP block using clk_mem cycles. It starts when the first command of a new (not merged) transaction is transferred to the TX command FIFO causing the interface logic to select the memory. After this period the memory device is deselected. This feature is needed for RAM devices requiring refresh cycles. The value needs to be derived from the RAMs maximum transaction length time (tCMS) minus the time of transferring 2x16byte data block (data granularity of the XIP ports is 16byte, 1 16byte block transfer outstanding, another available for merging). If the result in negative, MERGE_EN must be set to '0'. Example: RAM device tCMS = 4us, interface clock frequency = 100MHz: total transfer timeout is 4us - 2x16x10ns = 3680ns. With clk_mem frequency of 200MHz the TOTAL_TIMEOUT value is 3680/5 = 736. Note: In the unlikely case that the total transfer timeout is used (usually for RAM devices) while the page boundary crossing latency generation is enabled via RD_BOUND_CTL.PRESENT (usually for FLASH devices) the additional time needs to taken into account."]
pub type TOTAL_TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u16, u16, 14, O>;
#[doc = "Field `TOTAL_TIMEOUT_EN` reader - Total transfer timeout enable. '0': Disabled. There is no limit for the total transfer time. The continuous transfer merge timeout for limiting the idle time of the memory interface still applies. '1': Enabled. The maximum total transfer time is limited by field TOTAL_TIMEOUT."]
pub type TOTAL_TIMEOUT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TOTAL_TIMEOUT_EN` writer - Total transfer timeout enable. '0': Disabled. There is no limit for the total transfer time. The continuous transfer merge timeout for limiting the idle time of the memory interface still applies. '1': Enabled. The maximum total transfer time is limited by field TOTAL_TIMEOUT."]
pub type TOTAL_TIMEOUT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Device enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    pub fn wr_en(&self) -> WR_EN_R {
        WR_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    pub fn crypto_en(&self) -> CRYPTO_EN_R {
        CRYPTO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Continuous transfer merge timeout in clk_mem cycles. This limits the standby time of the memory interface, i.e. the time the memory device is selected but no data is transferred. The counting of the merge timeout period is done in the XIP block using clk_mem cycles. It starts when the last TX or RX byte is transferred to or from the data FIFOs. After this period the memory device is deselected. A later transfer, even from a continuous address, starts with the overhead phases (command, address, mode, dummy cycles)."]
    #[inline(always)]
    pub fn merge_timeout(&self) -> MERGE_TIMEOUT_R {
        MERGE_TIMEOUT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Continous transfer merge enable: '0': Disabled. No merging of transfers is done. Longest possible memory transfer is 16 Byte. '1': Enabled. Merging of continous transfers is done. This skips the overhead (command, address, mode, dummy cycles) for a continuous (linear sequential) transfer."]
    #[inline(always)]
    pub fn merge_en(&self) -> MERGE_EN_R {
        MERGE_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Total transfer timeout in clk_mem cycles. The counting of the total timout period is done in the XIP block using clk_mem cycles. It starts when the first command of a new (not merged) transaction is transferred to the TX command FIFO causing the interface logic to select the memory. After this period the memory device is deselected. This feature is needed for RAM devices requiring refresh cycles. The value needs to be derived from the RAMs maximum transaction length time (tCMS) minus the time of transferring 2x16byte data block (data granularity of the XIP ports is 16byte, 1 16byte block transfer outstanding, another available for merging). If the result in negative, MERGE_EN must be set to '0'. Example: RAM device tCMS = 4us, interface clock frequency = 100MHz: total transfer timeout is 4us - 2x16x10ns = 3680ns. With clk_mem frequency of 200MHz the TOTAL_TIMEOUT value is 3680/5 = 736. Note: In the unlikely case that the total transfer timeout is used (usually for RAM devices) while the page boundary crossing latency generation is enabled via RD_BOUND_CTL.PRESENT (usually for FLASH devices) the additional time needs to taken into account."]
    #[inline(always)]
    pub fn total_timeout(&self) -> TOTAL_TIMEOUT_R {
        TOTAL_TIMEOUT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Total transfer timeout enable. '0': Disabled. There is no limit for the total transfer time. The continuous transfer merge timeout for limiting the idle time of the memory interface still applies. '1': Enabled. The maximum total transfer time is limited by field TOTAL_TIMEOUT."]
    #[inline(always)]
    pub fn total_timeout_en(&self) -> TOTAL_TIMEOUT_EN_R {
        TOTAL_TIMEOUT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write enable: '0': write transfers are not allowed to this device. An attempt to write to this device results in an AHB-Lite bus error. '1': write transfers are allowed to this device."]
    #[inline(always)]
    #[must_use]
    pub fn wr_en(&mut self) -> WR_EN_W<0> {
        WR_EN_W::new(self)
    }
    #[doc = "Bit 4 - Cryptography on read/write accesses: '0': disabled. '1': enabled."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_en(&mut self) -> CRYPTO_EN_W<4> {
        CRYPTO_EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Specifies the connection of the IP's data lines (spi_data\\[0\\], ..., spi_data\\[7\\]) to the device's data lines (SI/IO0, SO/IO1, IO2, IO3, IO4, IO5, IO6, IO7): '0': spi_data\\[0\\]
= IO0, spi_data\\[1\\]
= IO1, ..., spi_data\\[7\\]
= IO7. This value is allowed for single, dual, quad, dual quad and octal SPI modes. This value must be used for the first device in dual quad SPI mode. This value must be used for octal SPI mode. '1': spi_data\\[2\\]
= IO0, spi_data\\[3\\]
= IO1. This value is only allowed for single and dual SPI modes. '2': spi_data\\[4\\]
= IO0, spi_data\\[5\\]
= IO1, ..., spi_data\\[7\\]
= IO3. This value is only allowed for single, dual, quad and dual quad SPI modes. In dual quad SPI mode, this value must be used for the second device. '3': spi_data\\[6\\]
= IO0, spi_data\\[7\\]
= IO1. This value is only allowed for single and dual SPI modes."]
    #[inline(always)]
    #[must_use]
    pub fn data_sel(&mut self) -> DATA_SEL_W<8> {
        DATA_SEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Continuous transfer merge timeout in clk_mem cycles. This limits the standby time of the memory interface, i.e. the time the memory device is selected but no data is transferred. The counting of the merge timeout period is done in the XIP block using clk_mem cycles. It starts when the last TX or RX byte is transferred to or from the data FIFOs. After this period the memory device is deselected. A later transfer, even from a continuous address, starts with the overhead phases (command, address, mode, dummy cycles)."]
    #[inline(always)]
    #[must_use]
    pub fn merge_timeout(&mut self) -> MERGE_TIMEOUT_W<12> {
        MERGE_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 15 - Continous transfer merge enable: '0': Disabled. No merging of transfers is done. Longest possible memory transfer is 16 Byte. '1': Enabled. Merging of continous transfers is done. This skips the overhead (command, address, mode, dummy cycles) for a continuous (linear sequential) transfer."]
    #[inline(always)]
    #[must_use]
    pub fn merge_en(&mut self) -> MERGE_EN_W<15> {
        MERGE_EN_W::new(self)
    }
    #[doc = "Bits 16:29 - Total transfer timeout in clk_mem cycles. The counting of the total timout period is done in the XIP block using clk_mem cycles. It starts when the first command of a new (not merged) transaction is transferred to the TX command FIFO causing the interface logic to select the memory. After this period the memory device is deselected. This feature is needed for RAM devices requiring refresh cycles. The value needs to be derived from the RAMs maximum transaction length time (tCMS) minus the time of transferring 2x16byte data block (data granularity of the XIP ports is 16byte, 1 16byte block transfer outstanding, another available for merging). If the result in negative, MERGE_EN must be set to '0'. Example: RAM device tCMS = 4us, interface clock frequency = 100MHz: total transfer timeout is 4us - 2x16x10ns = 3680ns. With clk_mem frequency of 200MHz the TOTAL_TIMEOUT value is 3680/5 = 736. Note: In the unlikely case that the total transfer timeout is used (usually for RAM devices) while the page boundary crossing latency generation is enabled via RD_BOUND_CTL.PRESENT (usually for FLASH devices) the additional time needs to taken into account."]
    #[inline(always)]
    #[must_use]
    pub fn total_timeout(&mut self) -> TOTAL_TIMEOUT_W<16> {
        TOTAL_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 30 - Total transfer timeout enable. '0': Disabled. There is no limit for the total transfer time. The continuous transfer merge timeout for limiting the idle time of the memory interface still applies. '1': Enabled. The maximum total transfer time is limited by field TOTAL_TIMEOUT."]
    #[inline(always)]
    #[must_use]
    pub fn total_timeout_en(&mut self) -> TOTAL_TIMEOUT_EN_W<30> {
        TOTAL_TIMEOUT_EN_W::new(self)
    }
    #[doc = "Bit 31 - Device enable: '0': Disabled. '1': Enabled."]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
