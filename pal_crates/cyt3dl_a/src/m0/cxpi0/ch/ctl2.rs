#[doc = "Register `CTL2` reader"]
pub struct R(crate::R<CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL2` writer"]
pub struct W(crate::W<CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL2_SPEC>;
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
impl From<crate::W<CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETRY` reader - Number of retries after arbitration lost. '0': No retries. .. '3': 3 retries. HW will immediately retry after arbitration lost i.e. after the message frame that won the arbitration is complete and fulfilled IFS. If SW wants to manage the retransmission then SW can program RETRY =0. In this case, HW will not retry after arbitration lost and will set TX_HEADER_ARB_LOST bit. SW needs to trigger HW to resend by programming the CMD fields again."]
pub type RETRY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RETRY` writer - Number of retries after arbitration lost. '0': No retries. .. '3': 3 retries. HW will immediately retry after arbitration lost i.e. after the message frame that won the arbitration is complete and fulfilled IFS. If SW wants to manage the retransmission then SW can program RETRY =0. In this case, HW will not retry after arbitration lost and will set TX_HEADER_ARB_LOST bit. SW needs to trigger HW to resend by programming the CMD fields again."]
pub type RETRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 2, O>;
#[doc = "Field `T_WAKEUP_LENGTH` reader - Specifies the wake up pulse low period in Tbits that is transmitted during Standby mode. '0': 1 bit period '1': 2 bit period .. '49': 50 bit period Any value above 49 is invalid. This field is only valid if TX_WAKE_PULSE is set to 1."]
pub type T_WAKEUP_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_WAKEUP_LENGTH` writer - Specifies the wake up pulse low period in Tbits that is transmitted during Standby mode. '0': 1 bit period '1': 2 bit period .. '49': 50 bit period Any value above 49 is invalid. This field is only valid if TX_WAKE_PULSE is set to 1."]
pub type T_WAKEUP_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TIMEOUT_LENGTH` reader - Timeout Length (in Tbits). Specifies the number of Tbits to exceed timeout between frame bytes within a message frame. CXPI spec states that the maximum allowed inter byte space (IBS) is 9Tbits. This field is valid only when TIMEOUT_SEL=1/2. '0' - 1Tbit '1' - 2Tbits .. '9' - 10Tbits Values >9 is invalid per CXPI spec. Note for NRZ mode, although there are propagation delay from transceiver to CXPI controller, the delay is cancelled out as the timeout is compared on the RX (for transmit case, HW waits for the feedback on RX)."]
pub type TIMEOUT_LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT_LENGTH` writer - Timeout Length (in Tbits). Specifies the number of Tbits to exceed timeout between frame bytes within a message frame. CXPI spec states that the maximum allowed inter byte space (IBS) is 9Tbits. This field is valid only when TIMEOUT_SEL=1/2. '0' - 1Tbit '1' - 2Tbits .. '9' - 10Tbits Values >9 is invalid per CXPI spec. Note for NRZ mode, although there are propagation delay from transceiver to CXPI controller, the delay is cancelled out as the timeout is compared on the RX (for transmit case, HW waits for the feedback on RX)."]
pub type TIMEOUT_LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TIMEOUT_SEL` reader - Timeout Select. '0' - Timeout check is disabled. HW clears timeout counter. '1' - Timeout check is enabled and HW will refer to TIMEOUT_LENGTH as number of Tbits allowed between header and response. '2' - Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time. '3' - invalid For '1', HW will restart/start timeout counter after transmitting/receiving header. HW will hold the counter if timeout until the next header is transmitted or received. Timeout will cause HW to stop transmission of the current message frame together with interrupt to SW. For receive, HW abort reception of the frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For '2', HW will re-start/start timeout counter after receiving any frame bytes within a message frame. HW will hold the counter if timeout until the IFS. Timeout will cause HW to stop transmission of the current message frame and notify SW with interrupt. For receive, HW will abort reception of the message frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For all cases, HW stops counting when it is out of a message frame such as IFS or CXPI bus IDLE. If the timeout counter > TIMEOUT_LENGTH, then it will set the INTR.TIMEOUT=1. Note that, TIMEOUT_SEL=1/2 also enables the count for IBS between receive header and transmit response on top of timeout check."]
pub type TIMEOUT_SEL_R = crate::FieldReader<u8, TIMEOUT_SEL_A>;
#[doc = "Timeout Select. '0' - Timeout check is disabled. HW clears timeout counter. '1' - Timeout check is enabled and HW will refer to TIMEOUT_LENGTH as number of Tbits allowed between header and response. '2' - Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time. '3' - invalid For '1', HW will restart/start timeout counter after transmitting/receiving header. HW will hold the counter if timeout until the next header is transmitted or received. Timeout will cause HW to stop transmission of the current message frame together with interrupt to SW. For receive, HW abort reception of the frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For '2', HW will re-start/start timeout counter after receiving any frame bytes within a message frame. HW will hold the counter if timeout until the IFS. Timeout will cause HW to stop transmission of the current message frame and notify SW with interrupt. For receive, HW will abort reception of the message frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For all cases, HW stops counting when it is out of a message frame such as IFS or CXPI bus IDLE. If the timeout counter > TIMEOUT_LENGTH, then it will set the INTR.TIMEOUT=1. Note that, TIMEOUT_SEL=1/2 also enables the count for IBS between receive header and transmit response on top of timeout check.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMEOUT_SEL_A {
    #[doc = "0: Timeout check is disabled."]
    TIMEOUT_DISABLED = 0,
    #[doc = "1: Timeout check is enabled and HW will refer to TIMEOUT_LENGHT as number of Tbits allowed between header and response."]
    TIMEOUT_CHECK_BTW_HDR_RSP = 1,
    #[doc = "2: Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time."]
    TIMEOUT_CHECK_BTW_HDR_HDR_RSP = 2,
}
impl From<TIMEOUT_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_SEL_A) -> Self {
        variant as _
    }
}
impl TIMEOUT_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMEOUT_SEL_A> {
        match self.bits {
            0 => Some(TIMEOUT_SEL_A::TIMEOUT_DISABLED),
            1 => Some(TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_RSP),
            2 => Some(TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_HDR_RSP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_DISABLED`"]
    #[inline(always)]
    pub fn is_timeout_disabled(&self) -> bool {
        *self == TIMEOUT_SEL_A::TIMEOUT_DISABLED
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_CHECK_BTW_HDR_RSP`"]
    #[inline(always)]
    pub fn is_timeout_check_btw_hdr_rsp(&self) -> bool {
        *self == TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_RSP
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_CHECK_BTW_HDR_HDR_RSP`"]
    #[inline(always)]
    pub fn is_timeout_check_btw_hdr_hdr_rsp(&self) -> bool {
        *self == TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_HDR_RSP
    }
}
#[doc = "Field `TIMEOUT_SEL` writer - Timeout Select. '0' - Timeout check is disabled. HW clears timeout counter. '1' - Timeout check is enabled and HW will refer to TIMEOUT_LENGTH as number of Tbits allowed between header and response. '2' - Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time. '3' - invalid For '1', HW will restart/start timeout counter after transmitting/receiving header. HW will hold the counter if timeout until the next header is transmitted or received. Timeout will cause HW to stop transmission of the current message frame together with interrupt to SW. For receive, HW abort reception of the frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For '2', HW will re-start/start timeout counter after receiving any frame bytes within a message frame. HW will hold the counter if timeout until the IFS. Timeout will cause HW to stop transmission of the current message frame and notify SW with interrupt. For receive, HW will abort reception of the message frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For all cases, HW stops counting when it is out of a message frame such as IFS or CXPI bus IDLE. If the timeout counter > TIMEOUT_LENGTH, then it will set the INTR.TIMEOUT=1. Note that, TIMEOUT_SEL=1/2 also enables the count for IBS between receive header and transmit response on top of timeout check."]
pub type TIMEOUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTL2_SPEC, u8, TIMEOUT_SEL_A, 2, O>;
impl<'a, const O: u8> TIMEOUT_SEL_W<'a, O> {
    #[doc = "Timeout check is disabled."]
    #[inline(always)]
    pub fn timeout_disabled(self) -> &'a mut W {
        self.variant(TIMEOUT_SEL_A::TIMEOUT_DISABLED)
    }
    #[doc = "Timeout check is enabled and HW will refer to TIMEOUT_LENGHT as number of Tbits allowed between header and response."]
    #[inline(always)]
    pub fn timeout_check_btw_hdr_rsp(self) -> &'a mut W {
        self.variant(TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_RSP)
    }
    #[doc = "Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time."]
    #[inline(always)]
    pub fn timeout_check_btw_hdr_hdr_rsp(self) -> &'a mut W {
        self.variant(TIMEOUT_SEL_A::TIMEOUT_CHECK_BTW_HDR_HDR_RSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Number of retries after arbitration lost. '0': No retries. .. '3': 3 retries. HW will immediately retry after arbitration lost i.e. after the message frame that won the arbitration is complete and fulfilled IFS. If SW wants to manage the retransmission then SW can program RETRY =0. In this case, HW will not retry after arbitration lost and will set TX_HEADER_ARB_LOST bit. SW needs to trigger HW to resend by programming the CMD fields again."]
    #[inline(always)]
    pub fn retry(&self) -> RETRY_R {
        RETRY_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:13 - Specifies the wake up pulse low period in Tbits that is transmitted during Standby mode. '0': 1 bit period '1': 2 bit period .. '49': 50 bit period Any value above 49 is invalid. This field is only valid if TX_WAKE_PULSE is set to 1."]
    #[inline(always)]
    pub fn t_wakeup_length(&self) -> T_WAKEUP_LENGTH_R {
        T_WAKEUP_LENGTH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Timeout Length (in Tbits). Specifies the number of Tbits to exceed timeout between frame bytes within a message frame. CXPI spec states that the maximum allowed inter byte space (IBS) is 9Tbits. This field is valid only when TIMEOUT_SEL=1/2. '0' - 1Tbit '1' - 2Tbits .. '9' - 10Tbits Values >9 is invalid per CXPI spec. Note for NRZ mode, although there are propagation delay from transceiver to CXPI controller, the delay is cancelled out as the timeout is compared on the RX (for transmit case, HW waits for the feedback on RX)."]
    #[inline(always)]
    pub fn timeout_length(&self) -> TIMEOUT_LENGTH_R {
        TIMEOUT_LENGTH_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 30:31 - Timeout Select. '0' - Timeout check is disabled. HW clears timeout counter. '1' - Timeout check is enabled and HW will refer to TIMEOUT_LENGTH as number of Tbits allowed between header and response. '2' - Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time. '3' - invalid For '1', HW will restart/start timeout counter after transmitting/receiving header. HW will hold the counter if timeout until the next header is transmitted or received. Timeout will cause HW to stop transmission of the current message frame together with interrupt to SW. For receive, HW abort reception of the frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For '2', HW will re-start/start timeout counter after receiving any frame bytes within a message frame. HW will hold the counter if timeout until the IFS. Timeout will cause HW to stop transmission of the current message frame and notify SW with interrupt. For receive, HW will abort reception of the message frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For all cases, HW stops counting when it is out of a message frame such as IFS or CXPI bus IDLE. If the timeout counter > TIMEOUT_LENGTH, then it will set the INTR.TIMEOUT=1. Note that, TIMEOUT_SEL=1/2 also enables the count for IBS between receive header and transmit response on top of timeout check."]
    #[inline(always)]
    pub fn timeout_sel(&self) -> TIMEOUT_SEL_R {
        TIMEOUT_SEL_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of retries after arbitration lost. '0': No retries. .. '3': 3 retries. HW will immediately retry after arbitration lost i.e. after the message frame that won the arbitration is complete and fulfilled IFS. If SW wants to manage the retransmission then SW can program RETRY =0. In this case, HW will not retry after arbitration lost and will set TX_HEADER_ARB_LOST bit. SW needs to trigger HW to resend by programming the CMD fields again."]
    #[inline(always)]
    #[must_use]
    pub fn retry(&mut self) -> RETRY_W<0> {
        RETRY_W::new(self)
    }
    #[doc = "Bits 8:13 - Specifies the wake up pulse low period in Tbits that is transmitted during Standby mode. '0': 1 bit period '1': 2 bit period .. '49': 50 bit period Any value above 49 is invalid. This field is only valid if TX_WAKE_PULSE is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn t_wakeup_length(&mut self) -> T_WAKEUP_LENGTH_W<8> {
        T_WAKEUP_LENGTH_W::new(self)
    }
    #[doc = "Bits 16:19 - Timeout Length (in Tbits). Specifies the number of Tbits to exceed timeout between frame bytes within a message frame. CXPI spec states that the maximum allowed inter byte space (IBS) is 9Tbits. This field is valid only when TIMEOUT_SEL=1/2. '0' - 1Tbit '1' - 2Tbits .. '9' - 10Tbits Values >9 is invalid per CXPI spec. Note for NRZ mode, although there are propagation delay from transceiver to CXPI controller, the delay is cancelled out as the timeout is compared on the RX (for transmit case, HW waits for the feedback on RX)."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_length(&mut self) -> TIMEOUT_LENGTH_W<16> {
        TIMEOUT_LENGTH_W::new(self)
    }
    #[doc = "Bits 30:31 - Timeout Select. '0' - Timeout check is disabled. HW clears timeout counter. '1' - Timeout check is enabled and HW will refer to TIMEOUT_LENGTH as number of Tbits allowed between header and response. '2' - Timeout check is enabled to check header-header, header-response, and header-header-response within a message frame to be space within TIMEOUT_LENGTH bit time. '3' - invalid For '1', HW will restart/start timeout counter after transmitting/receiving header. HW will hold the counter if timeout until the next header is transmitted or received. Timeout will cause HW to stop transmission of the current message frame together with interrupt to SW. For receive, HW abort reception of the frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For '2', HW will re-start/start timeout counter after receiving any frame bytes within a message frame. HW will hold the counter if timeout until the IFS. Timeout will cause HW to stop transmission of the current message frame and notify SW with interrupt. For receive, HW will abort reception of the message frame if timeout occurs while waiting for receiving response and notify SW with interrupt. For all cases, HW stops counting when it is out of a message frame such as IFS or CXPI bus IDLE. If the timeout counter > TIMEOUT_LENGTH, then it will set the INTR.TIMEOUT=1. Note that, TIMEOUT_SEL=1/2 also enables the count for IBS between receive header and transmit response on top of timeout check."]
    #[inline(always)]
    #[must_use]
    pub fn timeout_sel(&mut self) -> TIMEOUT_SEL_W<30> {
        TIMEOUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl2](index.html) module"]
pub struct CTL2_SPEC;
impl crate::RegisterSpec for CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl2::R](R) reader structure"]
impl crate::Readable for CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl2::W](W) writer structure"]
impl crate::Writable for CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
