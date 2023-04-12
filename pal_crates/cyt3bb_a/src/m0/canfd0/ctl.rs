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
#[doc = "Field `STOP_REQ` reader - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
pub type STOP_REQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP_REQ` writer - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
pub type STOP_REQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `MRAM_OFF` reader - MRAM off '0': Default MRAM on (with MRAM retained in DeepSleep). '1': Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
pub type MRAM_OFF_R = crate::BitReader<bool>;
#[doc = "Field `MRAM_OFF` writer - MRAM off '0': Default MRAM on (with MRAM retained in DeepSleep). '1': Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
pub type MRAM_OFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    pub fn stop_req(&self) -> STOP_REQ_R {
        STOP_REQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - MRAM off '0': Default MRAM on (with MRAM retained in DeepSleep). '1': Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    pub fn mram_off(&self) -> MRAM_OFF_R {
        MRAM_OFF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Stop Request for each TTCAN IP . The m_ttcan_clkstop_req of each TTCAN IP is directly driven by these bits."]
    #[inline(always)]
    #[must_use]
    pub fn stop_req(&mut self) -> STOP_REQ_W<0> {
        STOP_REQ_W::new(self)
    }
    #[doc = "Bit 31 - MRAM off '0': Default MRAM on (with MRAM retained in DeepSleep). '1': Switch MRAM off (not retained) to save power. Before setting this bit all the CAN channels have to be powered down using the STOP_REQ/ACK bits. When the MRAM is off any access attempt to it is considered an address error (as if MRAM_SIZE=0). After switching the MRAM on again software needs to allow for a certain power up time before MRAM can be used, i.e. before STOP_REQ can be de-asserted. The power up time is equivalent to the system SRAM power up time specified in the CPUSS.RAM_PWR_DELAY_CTL register. To meet S8 platform requirements, MRAM_OFF should be set to 0 prior to transitioning to Hibernate mode."]
    #[inline(always)]
    #[must_use]
    pub fn mram_off(&mut self) -> MRAM_OFF_W<31> {
        MRAM_OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global CAN control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
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
