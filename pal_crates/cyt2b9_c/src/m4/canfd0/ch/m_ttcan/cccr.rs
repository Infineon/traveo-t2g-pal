#[doc = "Register `CCCR` reader"]
pub struct R(crate::R<CCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCCR` writer"]
pub struct W(crate::W<CCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCCR_SPEC>;
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
impl From<crate::W<CCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialization 0= Normal Operation 1= Initialization is started"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialization 0= Normal Operation 1= Initialization is started"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CCE` reader - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
pub type CCE_R = crate::BitReader<bool>;
#[doc = "Field `CCE` writer - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `ASM` reader - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
pub type ASM_R = crate::BitReader<bool>;
#[doc = "Field `ASM` writer - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
pub type ASM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
pub type CSA_R = crate::BitReader<bool>;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
pub type CSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `CSR` reader - Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type CSR_R = crate::BitReader<bool>;
#[doc = "Field `CSR` writer - Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
pub type CSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `MON_` reader - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
pub type MON__R = crate::BitReader<bool>;
#[doc = "Field `MON_` writer - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
pub type MON__W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
pub type DAR_R = crate::BitReader<bool>;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
pub type DAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `TEST` reader - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
pub type TEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `FDOE` reader - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
pub type FDOE_R = crate::BitReader<bool>;
#[doc = "Field `FDOE` writer - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
pub type FDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `BRSE` reader - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
pub type BRSE_R = crate::BitReader<bool>;
#[doc = "Field `BRSE` writer - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
pub type BRSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
pub type PXHD_R = crate::BitReader<bool>;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
pub type PXHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
pub type EFBI_R = crate::BitReader<bool>;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
pub type EFBI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `TXP` reader - Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
pub type TXP_R = crate::BitReader<bool>;
#[doc = "Field `TXP` writer - Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
pub type TXP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
#[doc = "Field `NISO` reader - Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
pub type NISO_R = crate::BitReader<bool>;
#[doc = "Field `NISO` writer - Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
pub type NISO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    pub fn asm(&self) -> ASM_R {
        ASM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
    #[inline(always)]
    pub fn csa(&self) -> CSA_R {
        CSA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    pub fn csr(&self) -> CSR_R {
        CSR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    pub fn mon_(&self) -> MON__R {
        MON__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    pub fn fdoe(&self) -> FDOE_R {
        FDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
    #[inline(always)]
    pub fn brse(&self) -> BRSE_R {
        BRSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
    #[inline(always)]
    pub fn pxhd(&self) -> PXHD_R {
        PXHD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    pub fn efbi(&self) -> EFBI_R {
        EFBI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
    #[inline(always)]
    pub fn niso(&self) -> NISO_R {
        NISO_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization 0= Normal Operation 1= Initialization is started"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Configuration Change Enable 0= The CPU has no write access to the protected configuration registers 1= The CPU has write access to the protected configuration registers (while CCCR.INIT = '1')"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<1> {
        CCE_W::new(self)
    }
    #[doc = "Bit 2 - Restricted Operation Mode Bit ASM can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. For a description of the Restricted Operation Mode see Section 3.1.5. 0= Normal CAN operation 1= Restricted Operation Mode active"]
    #[inline(always)]
    #[must_use]
    pub fn asm(&mut self) -> ASM_W<2> {
        ASM_W::new(self)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge 0= No clock stop acknowledged 1= M_TTCAN may be set in power down by stopping m_ttcan_hclk and m_ttcan_cclk"]
    #[inline(always)]
    #[must_use]
    pub fn csa(&mut self) -> CSA_W<3> {
        CSA_W::new(self)
    }
    #[doc = "Bit 4 - Clock Stop Request, not supported by M_TTCAN use CTL.STOP_REQ at the group level instead. 0= No clock stop is requested 1= Clock stop requested. When clock stop is requested, first INIT and then CSA will be set after all pending transfer requests have been completed and the CAN bus reached idle."]
    #[inline(always)]
    #[must_use]
    pub fn csr(&mut self) -> CSR_W<4> {
        CSR_W::new(self)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode Bit MON can only be set by the Host when both CCE and INIT are set to '1'. The bit can be reset by the Host at any time. 0= Bus Monitoring Mode is disabled 1= Bus Monitoring Mode is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn mon_(&mut self) -> MON__W<5> {
        MON__W::new(self)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission 0= Automatic retransmission of messages not transmitted successfully enabled 1= Automatic retransmission disabled"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DAR_W<6> {
        DAR_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable 0= Normal operation, register TEST holds reset values 1= Test Mode, write access to register TEST enabled"]
    #[inline(always)]
    #[must_use]
    pub fn test(&mut self) -> TEST_W<7> {
        TEST_W::new(self)
    }
    #[doc = "Bit 8 - FD Operation Enable 0= FD operation disabled 1= FD operation enabled"]
    #[inline(always)]
    #[must_use]
    pub fn fdoe(&mut self) -> FDOE_W<8> {
        FDOE_W::new(self)
    }
    #[doc = "Bit 9 - Bit Rate Switch Enable 0= Bit rate switching for transmissions disabled 1= Bit rate switching for transmissions enabled"]
    #[inline(always)]
    #[must_use]
    pub fn brse(&mut self) -> BRSE_W<9> {
        BRSE_W::new(self)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable 0= Protocol exception handling enabled 1= Protocol exception handling disabled"]
    #[inline(always)]
    #[must_use]
    pub fn pxhd(&mut self) -> PXHD_W<12> {
        PXHD_W::new(self)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration 0= Edge filtering disabled 1= Two consecutive dominant tq required to detect an edge for hard synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn efbi(&mut self) -> EFBI_W<13> {
        EFBI_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Pause If this bit is set, the M_TTCAN pauses for two CAN bit times before starting the next transmission after itself has successfully transmitted a frame (see Section 3.5). 0= Transmit pause disabled 1= Transmit pause enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txp(&mut self) -> TXP_W<14> {
        TXP_W::new(self)
    }
    #[doc = "Bit 15 - Non ISO Operation If this bit is set, the M_TTCAN uses the CAN FD frame format as specified by the Bosch CAN FD Specification V1.0. 0= CAN FD frame format according to ISO 11898-1:2015 1= CAN FD frame format according to Bosch CAN FD Specification V1.0 addressing the non-ISO CAN FD"]
    #[inline(always)]
    #[must_use]
    pub fn niso(&mut self) -> NISO_W<15> {
        NISO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cccr](index.html) module"]
pub struct CCCR_SPEC;
impl crate::RegisterSpec for CCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cccr::R](R) reader structure"]
impl crate::Readable for CCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cccr::W](W) writer structure"]
impl crate::Writable for CCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCCR to value 0x01"]
impl crate::Resettable for CCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
