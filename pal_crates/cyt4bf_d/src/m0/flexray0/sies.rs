#[doc = "Register `SIES` reader"]
pub struct R(crate::R<SIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIES` writer"]
pub struct W(crate::W<SIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIES_SPEC>;
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
impl From<crate::W<SIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WSTE` reader - Wakeup Status Interrupt Enable"]
pub type WSTE_R = crate::BitReader<bool>;
#[doc = "Field `WSTE` writer - Wakeup Status Interrupt Enable"]
pub type WSTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `CASE` reader - Collision Avoidance Symbol Interrupt Enable"]
pub type CASE_R = crate::BitReader<bool>;
#[doc = "Field `CASE` writer - Collision Avoidance Symbol Interrupt Enable"]
pub type CASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `CYCSE` reader - Cycle Start Interrupt Enable"]
pub type CYCSE_R = crate::BitReader<bool>;
#[doc = "Field `CYCSE` writer - Cycle Start Interrupt Enable"]
pub type CYCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TXIE_R = crate::BitReader<bool>;
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `RFNEE` reader - Receive FIFO Not Empty Interrupt Enable"]
pub type RFNEE_R = crate::BitReader<bool>;
#[doc = "Field `RFNEE` writer - Receive FIFO Not Empty Interrupt Enable"]
pub type RFNEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `RFCLE` reader - Receive FIFO Critical Level Interrupt Enable"]
pub type RFCLE_R = crate::BitReader<bool>;
#[doc = "Field `RFCLE` writer - Receive FIFO Critical Level Interrupt Enable"]
pub type RFCLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `NMVCE` reader - Network Management Vector Changed Interrupt Enable"]
pub type NMVCE_R = crate::BitReader<bool>;
#[doc = "Field `NMVCE` writer - Network Management Vector Changed Interrupt Enable"]
pub type NMVCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `TI0E` reader - Timer Interrupt 0 Enable"]
pub type TI0E_R = crate::BitReader<bool>;
#[doc = "Field `TI0E` writer - Timer Interrupt 0 Enable"]
pub type TI0E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `TI1E` reader - Timer Interrupt 1 Enable"]
pub type TI1E_R = crate::BitReader<bool>;
#[doc = "Field `TI1E` writer - Timer Interrupt 1 Enable"]
pub type TI1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `TIBCE` reader - Transfer Input Buffer Completed Interrupt Enable"]
pub type TIBCE_R = crate::BitReader<bool>;
#[doc = "Field `TIBCE` writer - Transfer Input Buffer Completed Interrupt Enable"]
pub type TIBCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `TOBCE` reader - Transfer Output Buffer Completed Interrupt Enable"]
pub type TOBCE_R = crate::BitReader<bool>;
#[doc = "Field `TOBCE` writer - Transfer Output Buffer Completed Interrupt Enable"]
pub type TOBCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `SWEE` reader - Stop Watch Event Interrupt Enable"]
pub type SWEE_R = crate::BitReader<bool>;
#[doc = "Field `SWEE` writer - Stop Watch Event Interrupt Enable"]
pub type SWEE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `SUCSE` reader - Startup Completed Successfully Interrupt Enable"]
pub type SUCSE_R = crate::BitReader<bool>;
#[doc = "Field `SUCSE` writer - Startup Completed Successfully Interrupt Enable"]
pub type SUCSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `MBSIE` reader - Message Buffer Status Interrupt Enable"]
pub type MBSIE_R = crate::BitReader<bool>;
#[doc = "Field `MBSIE` writer - Message Buffer Status Interrupt Enable"]
pub type MBSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `SDSE` reader - Start of Dynamic Segment Interrupt Enable"]
pub type SDSE_R = crate::BitReader<bool>;
#[doc = "Field `SDSE` writer - Start of Dynamic Segment Interrupt Enable"]
pub type SDSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `WUPAE` reader - Wakeup Pattern Channel A Interrupt Enable"]
pub type WUPAE_R = crate::BitReader<bool>;
#[doc = "Field `WUPAE` writer - Wakeup Pattern Channel A Interrupt Enable"]
pub type WUPAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `MTSAE` reader - MTS Received on Channel A Interrupt Enable"]
pub type MTSAE_R = crate::BitReader<bool>;
#[doc = "Field `MTSAE` writer - MTS Received on Channel A Interrupt Enable"]
pub type MTSAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `WUPBE` reader - Wakeup Pattern Channel B Interrupt Enable"]
pub type WUPBE_R = crate::BitReader<bool>;
#[doc = "Field `WUPBE` writer - Wakeup Pattern Channel B Interrupt Enable"]
pub type WUPBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
#[doc = "Field `MTSBE` reader - MTS Received on Channel B Interrupt Enable"]
pub type MTSBE_R = crate::BitReader<bool>;
#[doc = "Field `MTSBE` writer - MTS Received on Channel B Interrupt Enable"]
pub type MTSBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SIES_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup Status Interrupt Enable"]
    #[inline(always)]
    pub fn wste(&self) -> WSTE_R {
        WSTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol Interrupt Enable"]
    #[inline(always)]
    pub fn case(&self) -> CASE_R {
        CASE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt Enable"]
    #[inline(always)]
    pub fn cycse(&self) -> CYCSE_R {
        CYCSE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rfnee(&self) -> RFNEE_R {
        RFNEE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level Interrupt Enable"]
    #[inline(always)]
    pub fn rfcle(&self) -> RFCLE_R {
        RFCLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Network Management Vector Changed Interrupt Enable"]
    #[inline(always)]
    pub fn nmvce(&self) -> NMVCE_R {
        NMVCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 Enable"]
    #[inline(always)]
    pub fn ti0e(&self) -> TI0E_R {
        TI0E_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 Enable"]
    #[inline(always)]
    pub fn ti1e(&self) -> TI1E_R {
        TI1E_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tibce(&self) -> TIBCE_R {
        TIBCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed Interrupt Enable"]
    #[inline(always)]
    pub fn tobce(&self) -> TOBCE_R {
        TOBCE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&self) -> SWEE_R {
        SWEE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Startup Completed Successfully Interrupt Enable"]
    #[inline(always)]
    pub fn sucse(&self) -> SUCSE_R {
        SUCSE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt Enable"]
    #[inline(always)]
    pub fn mbsie(&self) -> MBSIE_R {
        MBSIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment Interrupt Enable"]
    #[inline(always)]
    pub fn sdse(&self) -> SDSE_R {
        SDSE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A Interrupt Enable"]
    #[inline(always)]
    pub fn wupae(&self) -> WUPAE_R {
        WUPAE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MTS Received on Channel A Interrupt Enable"]
    #[inline(always)]
    pub fn mtsae(&self) -> MTSAE_R {
        MTSAE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B Interrupt Enable"]
    #[inline(always)]
    pub fn wupbe(&self) -> WUPBE_R {
        WUPBE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTS Received on Channel B Interrupt Enable"]
    #[inline(always)]
    pub fn mtsbe(&self) -> MTSBE_R {
        MTSBE_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wste(&mut self) -> WSTE_W<0> {
        WSTE_W::new(self)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn case(&mut self) -> CASE_W<1> {
        CASE_W::new(self)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cycse(&mut self) -> CYCSE_W<2> {
        CYCSE_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<3> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 4 - Receive Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<4> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfnee(&mut self) -> RFNEE_W<5> {
        RFNEE_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfcle(&mut self) -> RFCLE_W<6> {
        RFCLE_W::new(self)
    }
    #[doc = "Bit 7 - Network Management Vector Changed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nmvce(&mut self) -> NMVCE_W<7> {
        NMVCE_W::new(self)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ti0e(&mut self) -> TI0E_W<8> {
        TI0E_W::new(self)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ti1e(&mut self) -> TI1E_W<9> {
        TI1E_W::new(self)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tibce(&mut self) -> TIBCE_W<10> {
        TIBCE_W::new(self)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tobce(&mut self) -> TOBCE_W<11> {
        TOBCE_W::new(self)
    }
    #[doc = "Bit 12 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swee(&mut self) -> SWEE_W<12> {
        SWEE_W::new(self)
    }
    #[doc = "Bit 13 - Startup Completed Successfully Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sucse(&mut self) -> SUCSE_W<13> {
        SUCSE_W::new(self)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mbsie(&mut self) -> MBSIE_W<14> {
        MBSIE_W::new(self)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdse(&mut self) -> SDSE_W<15> {
        SDSE_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupae(&mut self) -> WUPAE_W<16> {
        WUPAE_W::new(self)
    }
    #[doc = "Bit 17 - MTS Received on Channel A Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mtsae(&mut self) -> MTSAE_W<17> {
        MTSAE_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupbe(&mut self) -> WUPBE_W<24> {
        WUPBE_W::new(self)
    }
    #[doc = "Bit 25 - MTS Received on Channel B Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mtsbe(&mut self) -> MTSBE_W<25> {
        MTSBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Interrupt Enable Set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sies](index.html) module"]
pub struct SIES_SPEC;
impl crate::RegisterSpec for SIES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sies::R](R) reader structure"]
impl crate::Readable for SIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sies::W](W) writer structure"]
impl crate::Writable for SIES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIES to value 0"]
impl crate::Resettable for SIES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
