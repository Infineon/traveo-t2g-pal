#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAM` reader - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
pub type TAM_R = crate::BitReader<bool>;
#[doc = "Field `TAM` writer - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
pub type TAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `TAT` reader - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
pub type TAT_R = crate::BitReader<bool>;
#[doc = "Field `TAT` writer - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
pub type TAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `CAM` reader - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
pub type CAM_R = crate::BitReader<bool>;
#[doc = "Field `CAM` writer - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
pub type CAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `CAT` reader - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
pub type CAT_R = crate::BitReader<bool>;
#[doc = "Field `CAT` writer - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
pub type CAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `LBCK` reader - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
pub type LBCK_R = crate::BitReader<bool>;
#[doc = "Field `LBCK` writer - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
pub type LBCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEST_SPEC, bool, O>;
#[doc = "Field `TX` reader - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
pub type TX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX` writer - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
pub type TX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEST_SPEC, u8, u8, 2, O>;
#[doc = "Field `RX` reader - Receive Pin Monitors the actual value of pin m_ttcan_rx 0= The CAN bus is dominant (m_ttcan_rx = '0') 1= The CAN bus is recessive (m_ttcan_rx = '1')"]
pub type RX_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn tam(&self) -> TAM_R {
        TAM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    pub fn tat(&self) -> TAT_R {
        TAT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    pub fn cam(&self) -> CAM_R {
        CAM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    pub fn cat(&self) -> CAT_R {
        CAT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    pub fn lbck(&self) -> LBCK_R {
        LBCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin Monitors the actual value of pin m_ttcan_rx 0= The CAN bus is dominant (m_ttcan_rx = '0') 1= The CAN bus is recessive (m_ttcan_rx = '1')"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ASC is not supported by M_TTCAN Test ASC Multiplexer Control Controls output pin m_ttcan_ascm in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_ascm controlled by FSE 1= Level at pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn tam(&mut self) -> TAM_W<0> {
        TAM_W::new(self)
    }
    #[doc = "Bit 1 - ASC is not supported by M_TTCAN Test ASC Transmit Control Controls output pin m_ttcan_asct in test mode, ORed with the signal from the FSE 0= Level at pin m_ttcan_asct controlled by FSE 1= Level at pin m_ttcan_asct = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn tat(&mut self) -> TAT_W<1> {
        TAT_W::new(self)
    }
    #[doc = "Bit 2 - ASC is not supported by M_TTCAN Check ASC Multiplexer Control Monitors level at output pin m_ttcan_ascm. 0= Output pin m_ttcan_ascm = '0' 1= Output pin m_ttcan_ascm = '1'"]
    #[inline(always)]
    #[must_use]
    pub fn cam(&mut self) -> CAM_W<2> {
        CAM_W::new(self)
    }
    #[doc = "Bit 3 - ASC is not supported by M_TTCAN Check ASC Transmit Control Monitors level at output pin m_ttcan_asct. 0= Output pin m_ttcan_asct = '0'"]
    #[inline(always)]
    #[must_use]
    pub fn cat(&mut self) -> CAT_W<3> {
        CAT_W::new(self)
    }
    #[doc = "Bit 4 - Loop Back Mode 0= Reset value, Loop Back Mode is disabled 1= Loop Back Mode is enabled (see Section 3.1.9, Test Modes)"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LBCK_W<4> {
        LBCK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 Reset value, m_ttcan_tx controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at pin m_ttcan_tx 10 Dominant ('0') level at pin m_ttcan_tx 11 Recessive ('1') at pin m_ttcan_tx"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<5> {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
