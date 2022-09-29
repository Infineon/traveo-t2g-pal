#[doc = "Register `CTL0` reader"]
pub struct R(crate::R<CTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL0` writer"]
pub struct W(crate::W<CTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL0_SPEC>;
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
impl From<crate::W<CTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Mode of operation: '0': NRZ mode. '1': PWM mode."]
pub type MODE_R = crate::BitReader<MODE_A>;
#[doc = "Mode of operation: '0': NRZ mode. '1': PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE_A {
    #[doc = "0: NRZ mode"]
    NRZ = 0,
    #[doc = "1: PWM mode"]
    PWM = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::NRZ,
            true => MODE_A::PWM,
        }
    }
    #[doc = "Checks if the value of the field is `NRZ`"]
    #[inline(always)]
    pub fn is_nrz(&self) -> bool {
        *self == MODE_A::NRZ
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE_A::PWM
    }
}
#[doc = "Field `MODE` writer - Mode of operation: '0': NRZ mode. '1': PWM mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, MODE_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "NRZ mode"]
    #[inline(always)]
    pub fn nrz(self) -> &'a mut W {
        self.variant(MODE_A::NRZ)
    }
    #[doc = "PWM mode"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE_A::PWM)
    }
}
#[doc = "Field `AUTO_EN` reader - CXPI transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
pub type AUTO_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_EN` writer - CXPI transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
pub type AUTO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `RXPIDZERO_CHECK_EN` reader - Receive PID Zero Check Enable. 0 - No action if received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1. 1 - If received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1, HW (slave) does not clear CMD.RX_HEADER and will anticipate receiving header again (CMD.TX_HEADER=0). If CMD.TX_HEADER=1 in the same scenario, then HW (slave) clears CMD.RX_HEADER upon receiving the header follow by transmit PID. This mode is useful for case where polling method is used and CXPI controller is configured as slave. This would reduce dependency on SW to react to the header received within IBS=1."]
pub type RXPIDZERO_CHECK_EN_R = crate::BitReader<bool>;
#[doc = "Field `RXPIDZERO_CHECK_EN` writer - Receive PID Zero Check Enable. 0 - No action if received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1. 1 - If received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1, HW (slave) does not clear CMD.RX_HEADER and will anticipate receiving header again (CMD.TX_HEADER=0). If CMD.TX_HEADER=1 in the same scenario, then HW (slave) clears CMD.RX_HEADER upon receiving the header follow by transmit PID. This mode is useful for case where polling method is used and CXPI controller is configured as slave. This would reduce dependency on SW to react to the header received within IBS=1."]
pub type RXPIDZERO_CHECK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `FILTER_EN` reader - RX filter enable (for 'cxpi_rx_in') '0': No filter '1': Median 3 (default value) operates on the last three 'cxpi_rx_in' values. The sequences '000', '001', '010', and '100' result in a filtered value '0'. The sequences '111', '110', '101', and '011' result in a filtered value '1'."]
pub type FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_EN` writer - RX filter enable (for 'cxpi_rx_in') '0': No filter '1': Median 3 (default value) operates on the last three 'cxpi_rx_in' values. The sequences '000', '001', '010', and '100' result in a filtered value '0'. The sequences '111', '110', '101', and '011' result in a filtered value '1'."]
pub type FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, bool, O>;
#[doc = "Field `IFS` reader - Inter Frame Space in bit periods: '0'' Invalid. ... '10': 10 bit periods ... '31': 31 bit periods Values of &lt;10 are not allowed. This field is used for transmission/reception for adding waiting inter frame space. Note that after a valid transaction (after CRC), HW waits for 10 bits as EOF. Hence, by setting CTL0.IFS=0xA means that HW will wait for 10bits before transmitting a new transaction (not including the EOF). Note: 0 is not allowed when IFS_WAIT=1. SW needs to ensure it has program valid values before it can set IFS_WAIT=1. If IFS_WAIT=1 after timeout occurs, the value of CTL0.IFS would need to consider the timeout count i.e. (IFS needed - CTL2.TIMEOUT_LENGTH-1) to get the total idle time. For example if TIMEOUT_LENGTH=9 and IFS needed is 20, then CTL0.IFS is set to 10."]
pub type IFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFS` writer - Inter Frame Space in bit periods: '0'' Invalid. ... '10': 10 bit periods ... '31': 31 bit periods Values of &lt;10 are not allowed. This field is used for transmission/reception for adding waiting inter frame space. Note that after a valid transaction (after CRC), HW waits for 10 bits as EOF. Hence, by setting CTL0.IFS=0xA means that HW will wait for 10bits before transmitting a new transaction (not including the EOF). Note: 0 is not allowed when IFS_WAIT=1. SW needs to ensure it has program valid values before it can set IFS_WAIT=1. If IFS_WAIT=1 after timeout occurs, the value of CTL0.IFS would need to consider the timeout count i.e. (IFS needed - CTL2.TIMEOUT_LENGTH-1) to get the total idle time. For example if TIMEOUT_LENGTH=9 and IFS needed is 20, then CTL0.IFS is set to 10."]
pub type IFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 5, O>;
#[doc = "Field `IBS` reader - Inter Byte Space in bit periods: '0' No offset. '1' 1 IBS is inserted per every byte frame. ... '9' 9 IBS are inserted per every byte frame. Values >9 are invalid per spec. This field is used to control number of IBS after every byte frame when transmitting message frame. Note that this field is the minimum IBS inserted for every byte frame as the SW may require some time to prepare the response when it receives the PID. When receiving, this field is ignored with the exception of receiving header and transmitting response. For receiving header and transmitting response, SW can enable IBS insertion by setting TIMEOUT_SEL=1/2 prior to setting CMD.RX_HEADER=1 and CMD.RX_RESPONSE=1. If the received header corresponds to transmit response, SW clears CMD.RX_RESPONSE =0 and sets TX FIFO and CMD.TX_RESPONSE=1. HW waits for minimum IBS before transmit response (if timeout has not occurred yet). It is prohibit to program IBS>TIMEOUT_LENGTH. This field should not be changed during inflight transaction including EOF."]
pub type IBS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBS` writer - Inter Byte Space in bit periods: '0' No offset. '1' 1 IBS is inserted per every byte frame. ... '9' 9 IBS are inserted per every byte frame. Values >9 are invalid per spec. This field is used to control number of IBS after every byte frame when transmitting message frame. Note that this field is the minimum IBS inserted for every byte frame as the SW may require some time to prepare the response when it receives the PID. When receiving, this field is ignored with the exception of receiving header and transmitting response. For receiving header and transmitting response, SW can enable IBS insertion by setting TIMEOUT_SEL=1/2 prior to setting CMD.RX_HEADER=1 and CMD.RX_RESPONSE=1. If the received header corresponds to transmit response, SW clears CMD.RX_RESPONSE =0 and sets TX FIFO and CMD.TX_RESPONSE=1. HW waits for minimum IBS before transmit response (if timeout has not occurred yet). It is prohibit to program IBS>TIMEOUT_LENGTH. This field should not be changed during inflight transaction including EOF."]
pub type IBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `BIT_ERROR_IGNORE` reader - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: This field does NOT effect the reporting of the bit error through INTR/STATUS.TX_BIT_ERROR; i.e. bit errors are always reported. Note: This field must not be set to '1' when it is NRZ mode. This is due to delay in transceiver will cause the transmitter behavior undefined when error occurs."]
pub type BIT_ERROR_IGNORE_R = crate::BitReader<BIT_ERROR_IGNORE_A>;
#[doc = "Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: This field does NOT effect the reporting of the bit error through INTR/STATUS.TX_BIT_ERROR; i.e. bit errors are always reported. Note: This field must not be set to '1' when it is NRZ mode. This is due to delay in transceiver will cause the transmitter behavior undefined when error occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIT_ERROR_IGNORE_A {
    #[doc = "0: Message transfer is aborted"]
    ABORT_TX_MSG = 0,
    #[doc = "1: Message transfer is NOT aborted"]
    CONT_TX_MSG = 1,
}
impl From<BIT_ERROR_IGNORE_A> for bool {
    #[inline(always)]
    fn from(variant: BIT_ERROR_IGNORE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIT_ERROR_IGNORE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT_ERROR_IGNORE_A {
        match self.bits {
            false => BIT_ERROR_IGNORE_A::ABORT_TX_MSG,
            true => BIT_ERROR_IGNORE_A::CONT_TX_MSG,
        }
    }
    #[doc = "Checks if the value of the field is `ABORT_TX_MSG`"]
    #[inline(always)]
    pub fn is_abort_tx_msg(&self) -> bool {
        *self == BIT_ERROR_IGNORE_A::ABORT_TX_MSG
    }
    #[doc = "Checks if the value of the field is `CONT_TX_MSG`"]
    #[inline(always)]
    pub fn is_cont_tx_msg(&self) -> bool {
        *self == BIT_ERROR_IGNORE_A::CONT_TX_MSG
    }
}
#[doc = "Field `BIT_ERROR_IGNORE` writer - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: This field does NOT effect the reporting of the bit error through INTR/STATUS.TX_BIT_ERROR; i.e. bit errors are always reported. Note: This field must not be set to '1' when it is NRZ mode. This is due to delay in transceiver will cause the transmitter behavior undefined when error occurs."]
pub type BIT_ERROR_IGNORE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTL0_SPEC, BIT_ERROR_IGNORE_A, O>;
impl<'a, const O: u8> BIT_ERROR_IGNORE_W<'a, O> {
    #[doc = "Message transfer is aborted"]
    #[inline(always)]
    pub fn abort_tx_msg(self) -> &'a mut W {
        self.variant(BIT_ERROR_IGNORE_A::ABORT_TX_MSG)
    }
    #[doc = "Message transfer is NOT aborted"]
    #[inline(always)]
    pub fn cont_tx_msg(self) -> &'a mut W {
        self.variant(BIT_ERROR_IGNORE_A::CONT_TX_MSG)
    }
}
#[doc = "Field `MASTER` reader - CXPI master mode. '0': Indicates CXPI as slave node. '1': Indicates CXPI as master node. This bit is only valid if ENABLED=1. SW needs to set only 1 node as master within the same CXPI cluster. SW needs to set this bit either at the same time as ENABLED or before ENABLED is set. If SW needs to change the controller to different mode, it needs to make sure that HW is quiescent before doing so."]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "CXPI master mode. '0': Indicates CXPI as slave node. '1': Indicates CXPI as master node. This bit is only valid if ENABLED=1. SW needs to set only 1 node as master within the same CXPI cluster. SW needs to set this bit either at the same time as ENABLED or before ENABLED is set. If SW needs to change the controller to different mode, it needs to make sure that HW is quiescent before doing so.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode"]
    SLAVE_MODE = 0,
    #[doc = "1: Master mode"]
    MASTER_MODE = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE_MODE,
            true => MASTER_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MASTER_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MASTER_A::MASTER_MODE
    }
}
#[doc = "Field `MASTER` writer - CXPI master mode. '0': Indicates CXPI as slave node. '1': Indicates CXPI as master node. This bit is only valid if ENABLED=1. SW needs to set only 1 node as master within the same CXPI cluster. SW needs to set this bit either at the same time as ENABLED or before ENABLED is set. If SW needs to change the controller to different mode, it needs to make sure that HW is quiescent before doing so."]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE_MODE)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_MODE)
    }
}
#[doc = "Field `ENABLED` reader - Channel enable: '0': Disabled. If a channel is disabled, CMD, STATUS, INTR MMIO registers will have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<ENABLED_A>;
#[doc = "Channel enable: '0': Disabled. If a channel is disabled, CMD, STATUS, INTR MMIO registers will have their fields reset to their default value. '1': Enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLED_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
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
#[doc = "Field `ENABLED` writer - Channel enable: '0': Disabled. If a channel is disabled, CMD, STATUS, INTR MMIO registers will have their fields reset to their default value. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL0_SPEC, ENABLED_A, O>;
impl<'a, const O: u8> ENABLED_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLED_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLED_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Mode of operation: '0': NRZ mode. '1': PWM mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CXPI transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
    #[inline(always)]
    pub fn auto_en(&self) -> AUTO_EN_R {
        AUTO_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive PID Zero Check Enable. 0 - No action if received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1. 1 - If received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1, HW (slave) does not clear CMD.RX_HEADER and will anticipate receiving header again (CMD.TX_HEADER=0). If CMD.TX_HEADER=1 in the same scenario, then HW (slave) clears CMD.RX_HEADER upon receiving the header follow by transmit PID. This mode is useful for case where polling method is used and CXPI controller is configured as slave. This would reduce dependency on SW to react to the header received within IBS=1."]
    #[inline(always)]
    pub fn rxpidzero_check_en(&self) -> RXPIDZERO_CHECK_EN_R {
        RXPIDZERO_CHECK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX filter enable (for 'cxpi_rx_in') '0': No filter '1': Median 3 (default value) operates on the last three 'cxpi_rx_in' values. The sequences '000', '001', '010', and '100' result in a filtered value '0'. The sequences '111', '110', '101', and '011' result in a filtered value '1'."]
    #[inline(always)]
    pub fn filter_en(&self) -> FILTER_EN_R {
        FILTER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Inter Frame Space in bit periods: '0'' Invalid. ... '10': 10 bit periods ... '31': 31 bit periods Values of &lt;10 are not allowed. This field is used for transmission/reception for adding waiting inter frame space. Note that after a valid transaction (after CRC), HW waits for 10 bits as EOF. Hence, by setting CTL0.IFS=0xA means that HW will wait for 10bits before transmitting a new transaction (not including the EOF). Note: 0 is not allowed when IFS_WAIT=1. SW needs to ensure it has program valid values before it can set IFS_WAIT=1. If IFS_WAIT=1 after timeout occurs, the value of CTL0.IFS would need to consider the timeout count i.e. (IFS needed - CTL2.TIMEOUT_LENGTH-1) to get the total idle time. For example if TIMEOUT_LENGTH=9 and IFS needed is 20, then CTL0.IFS is set to 10."]
    #[inline(always)]
    pub fn ifs(&self) -> IFS_R {
        IFS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - Inter Byte Space in bit periods: '0' No offset. '1' 1 IBS is inserted per every byte frame. ... '9' 9 IBS are inserted per every byte frame. Values >9 are invalid per spec. This field is used to control number of IBS after every byte frame when transmitting message frame. Note that this field is the minimum IBS inserted for every byte frame as the SW may require some time to prepare the response when it receives the PID. When receiving, this field is ignored with the exception of receiving header and transmitting response. For receiving header and transmitting response, SW can enable IBS insertion by setting TIMEOUT_SEL=1/2 prior to setting CMD.RX_HEADER=1 and CMD.RX_RESPONSE=1. If the received header corresponds to transmit response, SW clears CMD.RX_RESPONSE =0 and sets TX FIFO and CMD.TX_RESPONSE=1. HW waits for minimum IBS before transmit response (if timeout has not occurred yet). It is prohibit to program IBS>TIMEOUT_LENGTH. This field should not be changed during inflight transaction including EOF."]
    #[inline(always)]
    pub fn ibs(&self) -> IBS_R {
        IBS_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: This field does NOT effect the reporting of the bit error through INTR/STATUS.TX_BIT_ERROR; i.e. bit errors are always reported. Note: This field must not be set to '1' when it is NRZ mode. This is due to delay in transceiver will cause the transmitter behavior undefined when error occurs."]
    #[inline(always)]
    pub fn bit_error_ignore(&self) -> BIT_ERROR_IGNORE_R {
        BIT_ERROR_IGNORE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - CXPI master mode. '0': Indicates CXPI as slave node. '1': Indicates CXPI as master node. This bit is only valid if ENABLED=1. SW needs to set only 1 node as master within the same CXPI cluster. SW needs to set this bit either at the same time as ENABLED or before ENABLED is set. If SW needs to change the controller to different mode, it needs to make sure that HW is quiescent before doing so."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. If a channel is disabled, CMD, STATUS, INTR MMIO registers will have their fields reset to their default value. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mode of operation: '0': NRZ mode. '1': PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - CXPI transceiver auto enable: '0': Disabled. '1': Enabled. The TX_RX_STATUS.EN_OUT field is controlled by HW."]
    #[inline(always)]
    #[must_use]
    pub fn auto_en(&mut self) -> AUTO_EN_W<4> {
        AUTO_EN_W::new(self)
    }
    #[doc = "Bit 7 - Receive PID Zero Check Enable. 0 - No action if received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1. 1 - If received PID\\[6:0\\]
= 0 and PID\\[7\\]=1'b1, HW (slave) does not clear CMD.RX_HEADER and will anticipate receiving header again (CMD.TX_HEADER=0). If CMD.TX_HEADER=1 in the same scenario, then HW (slave) clears CMD.RX_HEADER upon receiving the header follow by transmit PID. This mode is useful for case where polling method is used and CXPI controller is configured as slave. This would reduce dependency on SW to react to the header received within IBS=1."]
    #[inline(always)]
    #[must_use]
    pub fn rxpidzero_check_en(&mut self) -> RXPIDZERO_CHECK_EN_W<7> {
        RXPIDZERO_CHECK_EN_W::new(self)
    }
    #[doc = "Bit 8 - RX filter enable (for 'cxpi_rx_in') '0': No filter '1': Median 3 (default value) operates on the last three 'cxpi_rx_in' values. The sequences '000', '001', '010', and '100' result in a filtered value '0'. The sequences '111', '110', '101', and '011' result in a filtered value '1'."]
    #[inline(always)]
    #[must_use]
    pub fn filter_en(&mut self) -> FILTER_EN_W<8> {
        FILTER_EN_W::new(self)
    }
    #[doc = "Bits 16:20 - Inter Frame Space in bit periods: '0'' Invalid. ... '10': 10 bit periods ... '31': 31 bit periods Values of &lt;10 are not allowed. This field is used for transmission/reception for adding waiting inter frame space. Note that after a valid transaction (after CRC), HW waits for 10 bits as EOF. Hence, by setting CTL0.IFS=0xA means that HW will wait for 10bits before transmitting a new transaction (not including the EOF). Note: 0 is not allowed when IFS_WAIT=1. SW needs to ensure it has program valid values before it can set IFS_WAIT=1. If IFS_WAIT=1 after timeout occurs, the value of CTL0.IFS would need to consider the timeout count i.e. (IFS needed - CTL2.TIMEOUT_LENGTH-1) to get the total idle time. For example if TIMEOUT_LENGTH=9 and IFS needed is 20, then CTL0.IFS is set to 10."]
    #[inline(always)]
    #[must_use]
    pub fn ifs(&mut self) -> IFS_W<16> {
        IFS_W::new(self)
    }
    #[doc = "Bits 21:24 - Inter Byte Space in bit periods: '0' No offset. '1' 1 IBS is inserted per every byte frame. ... '9' 9 IBS are inserted per every byte frame. Values >9 are invalid per spec. This field is used to control number of IBS after every byte frame when transmitting message frame. Note that this field is the minimum IBS inserted for every byte frame as the SW may require some time to prepare the response when it receives the PID. When receiving, this field is ignored with the exception of receiving header and transmitting response. For receiving header and transmitting response, SW can enable IBS insertion by setting TIMEOUT_SEL=1/2 prior to setting CMD.RX_HEADER=1 and CMD.RX_RESPONSE=1. If the received header corresponds to transmit response, SW clears CMD.RX_RESPONSE =0 and sets TX FIFO and CMD.TX_RESPONSE=1. HW waits for minimum IBS before transmit response (if timeout has not occurred yet). It is prohibit to program IBS>TIMEOUT_LENGTH. This field should not be changed during inflight transaction including EOF."]
    #[inline(always)]
    #[must_use]
    pub fn ibs(&mut self) -> IBS_W<21> {
        IBS_W::new(self)
    }
    #[doc = "Bit 27 - Specifies behavior on a detected bit error during header or response transmission: '0': Message transfer is aborted. '1': Message transfer is NOT aborted. Note: This field does NOT effect the reporting of the bit error through INTR/STATUS.TX_BIT_ERROR; i.e. bit errors are always reported. Note: This field must not be set to '1' when it is NRZ mode. This is due to delay in transceiver will cause the transmitter behavior undefined when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn bit_error_ignore(&mut self) -> BIT_ERROR_IGNORE_W<27> {
        BIT_ERROR_IGNORE_W::new(self)
    }
    #[doc = "Bit 30 - CXPI master mode. '0': Indicates CXPI as slave node. '1': Indicates CXPI as master node. This bit is only valid if ENABLED=1. SW needs to set only 1 node as master within the same CXPI cluster. SW needs to set this bit either at the same time as ENABLED or before ENABLED is set. If SW needs to change the controller to different mode, it needs to make sure that HW is quiescent before doing so."]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<30> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 31 - Channel enable: '0': Disabled. If a channel is disabled, CMD, STATUS, INTR MMIO registers will have their fields reset to their default value. '1': Enabled."]
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
#[doc = "Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl0](index.html) module"]
pub struct CTL0_SPEC;
impl crate::RegisterSpec for CTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl0::R](R) reader structure"]
impl crate::Readable for CTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl0::W](W) writer structure"]
impl crate::Writable for CTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL0 to value 0x10"]
impl crate::Resettable for CTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
