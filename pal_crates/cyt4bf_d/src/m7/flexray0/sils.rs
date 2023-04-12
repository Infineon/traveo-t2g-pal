#[doc = "Register `SILS` reader"]
pub struct R(crate::R<SILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SILS` writer"]
pub struct W(crate::W<SILS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SILS_SPEC>;
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
impl From<crate::W<SILS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SILS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WSTL` reader - Wakeup Status Interrupt Line"]
pub type WSTL_R = crate::BitReader<bool>;
#[doc = "Field `WSTL` writer - Wakeup Status Interrupt Line"]
pub type WSTL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `CASL` reader - Collision Avoidance Symbol Interrupt Line"]
pub type CASL_R = crate::BitReader<bool>;
#[doc = "Field `CASL` writer - Collision Avoidance Symbol Interrupt Line"]
pub type CASL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `CYCSL` reader - Cycle Start Interrupt Line"]
pub type CYCSL_R = crate::BitReader<bool>;
#[doc = "Field `CYCSL` writer - Cycle Start Interrupt Line"]
pub type CYCSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `TXIL` reader - Transmit Interrupt Line"]
pub type TXIL_R = crate::BitReader<bool>;
#[doc = "Field `TXIL` writer - Transmit Interrupt Line"]
pub type TXIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `RXIL` reader - Receive Interrupt Line"]
pub type RXIL_R = crate::BitReader<bool>;
#[doc = "Field `RXIL` writer - Receive Interrupt Line"]
pub type RXIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `RFNEL` reader - Receive FIFO Not Empty Interrupt Line"]
pub type RFNEL_R = crate::BitReader<bool>;
#[doc = "Field `RFNEL` writer - Receive FIFO Not Empty Interrupt Line"]
pub type RFNEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `RFCLL` reader - Receive FIFO Critical Level Interrupt Line"]
pub type RFCLL_R = crate::BitReader<bool>;
#[doc = "Field `RFCLL` writer - Receive FIFO Critical Level Interrupt Line"]
pub type RFCLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `NMVCL` reader - Network Management Vector Changed Interrupt Line"]
pub type NMVCL_R = crate::BitReader<bool>;
#[doc = "Field `NMVCL` writer - Network Management Vector Changed Interrupt Line"]
pub type NMVCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `TI0L` reader - Timer Interrupt 0 Line"]
pub type TI0L_R = crate::BitReader<bool>;
#[doc = "Field `TI0L` writer - Timer Interrupt 0 Line"]
pub type TI0L_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `TI1L` reader - Timer Interrupt 1 Line"]
pub type TI1L_R = crate::BitReader<bool>;
#[doc = "Field `TI1L` writer - Timer Interrupt 1 Line"]
pub type TI1L_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `TIBCL` reader - Transfer Input Buffer Completed Interrupt Line"]
pub type TIBCL_R = crate::BitReader<bool>;
#[doc = "Field `TIBCL` writer - Transfer Input Buffer Completed Interrupt Line"]
pub type TIBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `TOBCL` reader - Transfer Output Buffer Completed Interrupt Line"]
pub type TOBCL_R = crate::BitReader<bool>;
#[doc = "Field `TOBCL` writer - Transfer Output Buffer Completed Interrupt Line"]
pub type TOBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `SWEL` reader - Stop Watch Event Interrupt Line"]
pub type SWEL_R = crate::BitReader<bool>;
#[doc = "Field `SWEL` writer - Stop Watch Event Interrupt Line"]
pub type SWEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `SUCSL` reader - Startup Completed Successfully Interrupt Line"]
pub type SUCSL_R = crate::BitReader<bool>;
#[doc = "Field `SUCSL` writer - Startup Completed Successfully Interrupt Line"]
pub type SUCSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `MBSIL` reader - Message Buffer Status Interrupt Line"]
pub type MBSIL_R = crate::BitReader<bool>;
#[doc = "Field `MBSIL` writer - Message Buffer Status Interrupt Line"]
pub type MBSIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `SDSL` reader - Start of Dynamic Segment Interrupt Line"]
pub type SDSL_R = crate::BitReader<bool>;
#[doc = "Field `SDSL` writer - Start of Dynamic Segment Interrupt Line"]
pub type SDSL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `WUPAL` reader - Wakeup Pattern Channel A Interrupt Line"]
pub type WUPAL_R = crate::BitReader<bool>;
#[doc = "Field `WUPAL` writer - Wakeup Pattern Channel A Interrupt Line"]
pub type WUPAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `MTSAL` reader - Media Access Test Symbol Channel A Interrupt Line"]
pub type MTSAL_R = crate::BitReader<bool>;
#[doc = "Field `MTSAL` writer - Media Access Test Symbol Channel A Interrupt Line"]
pub type MTSAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `WUPBL` reader - Wakeup Pattern Channel B Interrupt Line"]
pub type WUPBL_R = crate::BitReader<bool>;
#[doc = "Field `WUPBL` writer - Wakeup Pattern Channel B Interrupt Line"]
pub type WUPBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
#[doc = "Field `MTSBL` reader - Media Access Test Symbol Channel B Interrupt Line"]
pub type MTSBL_R = crate::BitReader<bool>;
#[doc = "Field `MTSBL` writer - Media Access Test Symbol Channel B Interrupt Line"]
pub type MTSBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SILS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Wakeup Status Interrupt Line"]
    #[inline(always)]
    pub fn wstl(&self) -> WSTL_R {
        WSTL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol Interrupt Line"]
    #[inline(always)]
    pub fn casl(&self) -> CASL_R {
        CASL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt Line"]
    #[inline(always)]
    pub fn cycsl(&self) -> CYCSL_R {
        CYCSL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Interrupt Line"]
    #[inline(always)]
    pub fn txil(&self) -> TXIL_R {
        TXIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Interrupt Line"]
    #[inline(always)]
    pub fn rxil(&self) -> RXIL_R {
        RXIL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty Interrupt Line"]
    #[inline(always)]
    pub fn rfnel(&self) -> RFNEL_R {
        RFNEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level Interrupt Line"]
    #[inline(always)]
    pub fn rfcll(&self) -> RFCLL_R {
        RFCLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Network Management Vector Changed Interrupt Line"]
    #[inline(always)]
    pub fn nmvcl(&self) -> NMVCL_R {
        NMVCL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 Line"]
    #[inline(always)]
    pub fn ti0l(&self) -> TI0L_R {
        TI0L_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 Line"]
    #[inline(always)]
    pub fn ti1l(&self) -> TI1L_R {
        TI1L_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed Interrupt Line"]
    #[inline(always)]
    pub fn tibcl(&self) -> TIBCL_R {
        TIBCL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed Interrupt Line"]
    #[inline(always)]
    pub fn tobcl(&self) -> TOBCL_R {
        TOBCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Startup Completed Successfully Interrupt Line"]
    #[inline(always)]
    pub fn sucsl(&self) -> SUCSL_R {
        SUCSL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt Line"]
    #[inline(always)]
    pub fn mbsil(&self) -> MBSIL_R {
        MBSIL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment Interrupt Line"]
    #[inline(always)]
    pub fn sdsl(&self) -> SDSL_R {
        SDSL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A Interrupt Line"]
    #[inline(always)]
    pub fn wupal(&self) -> WUPAL_R {
        WUPAL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Media Access Test Symbol Channel A Interrupt Line"]
    #[inline(always)]
    pub fn mtsal(&self) -> MTSAL_R {
        MTSAL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B Interrupt Line"]
    #[inline(always)]
    pub fn wupbl(&self) -> WUPBL_R {
        WUPBL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Media Access Test Symbol Channel B Interrupt Line"]
    #[inline(always)]
    pub fn mtsbl(&self) -> MTSBL_R {
        MTSBL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Status Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn wstl(&mut self) -> WSTL_W<0> {
        WSTL_W::new(self)
    }
    #[doc = "Bit 1 - Collision Avoidance Symbol Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn casl(&mut self) -> CASL_W<1> {
        CASL_W::new(self)
    }
    #[doc = "Bit 2 - Cycle Start Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn cycsl(&mut self) -> CYCSL_W<2> {
        CYCSL_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn txil(&mut self) -> TXIL_W<3> {
        TXIL_W::new(self)
    }
    #[doc = "Bit 4 - Receive Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rxil(&mut self) -> RXIL_W<4> {
        RXIL_W::new(self)
    }
    #[doc = "Bit 5 - Receive FIFO Not Empty Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rfnel(&mut self) -> RFNEL_W<5> {
        RFNEL_W::new(self)
    }
    #[doc = "Bit 6 - Receive FIFO Critical Level Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn rfcll(&mut self) -> RFCLL_W<6> {
        RFCLL_W::new(self)
    }
    #[doc = "Bit 7 - Network Management Vector Changed Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn nmvcl(&mut self) -> NMVCL_W<7> {
        NMVCL_W::new(self)
    }
    #[doc = "Bit 8 - Timer Interrupt 0 Line"]
    #[inline(always)]
    #[must_use]
    pub fn ti0l(&mut self) -> TI0L_W<8> {
        TI0L_W::new(self)
    }
    #[doc = "Bit 9 - Timer Interrupt 1 Line"]
    #[inline(always)]
    #[must_use]
    pub fn ti1l(&mut self) -> TI1L_W<9> {
        TI1L_W::new(self)
    }
    #[doc = "Bit 10 - Transfer Input Buffer Completed Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tibcl(&mut self) -> TIBCL_W<10> {
        TIBCL_W::new(self)
    }
    #[doc = "Bit 11 - Transfer Output Buffer Completed Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn tobcl(&mut self) -> TOBCL_W<11> {
        TOBCL_W::new(self)
    }
    #[doc = "Bit 12 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn swel(&mut self) -> SWEL_W<12> {
        SWEL_W::new(self)
    }
    #[doc = "Bit 13 - Startup Completed Successfully Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sucsl(&mut self) -> SUCSL_W<13> {
        SUCSL_W::new(self)
    }
    #[doc = "Bit 14 - Message Buffer Status Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn mbsil(&mut self) -> MBSIL_W<14> {
        MBSIL_W::new(self)
    }
    #[doc = "Bit 15 - Start of Dynamic Segment Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn sdsl(&mut self) -> SDSL_W<15> {
        SDSL_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup Pattern Channel A Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn wupal(&mut self) -> WUPAL_W<16> {
        WUPAL_W::new(self)
    }
    #[doc = "Bit 17 - Media Access Test Symbol Channel A Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn mtsal(&mut self) -> MTSAL_W<17> {
        MTSAL_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Pattern Channel B Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn wupbl(&mut self) -> WUPBL_W<24> {
        WUPBL_W::new(self)
    }
    #[doc = "Bit 25 - Media Access Test Symbol Channel B Interrupt Line"]
    #[inline(always)]
    #[must_use]
    pub fn mtsbl(&mut self) -> MTSBL_W<25> {
        MTSBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Interrupt Line Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sils](index.html) module"]
pub struct SILS_SPEC;
impl crate::RegisterSpec for SILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sils::R](R) reader structure"]
impl crate::Readable for SILS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sils::W](W) writer structure"]
impl crate::Writable for SILS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SILS to value 0x0303_ffff"]
impl crate::Resettable for SILS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0303_ffff;
}
