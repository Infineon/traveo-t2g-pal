#[doc = "Register `TX_PFC_PAUSE` reader"]
pub struct R(crate::R<TX_PFC_PAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PFC_PAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PFC_PAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PFC_PAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PFC_PAUSE` writer"]
pub struct W(crate::W<TX_PFC_PAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PFC_PAUSE_SPEC>;
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
impl From<crate::W<TX_PFC_PAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PFC_PAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTOR_ENABLE` reader - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTOR_ENABLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTOR_ENABLE` writer - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
pub type VECTOR_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_PFC_PAUSE_SPEC, u8, u8, 8, O>;
#[doc = "Field `VECTOR` reader - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VECTOR` writer - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
pub type VECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_PFC_PAUSE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    pub fn vector_enable(&self) -> VECTOR_ENABLE_R {
        VECTOR_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    pub fn vector(&self) -> VECTOR_R {
        VECTOR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Priority Vector Enable. If bit 17 of the network control register is written with a one then the priority enable vector of the PFC priority based pause frame will be set equal to the value stored in this register \\[7:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn vector_enable(&mut self) -> VECTOR_ENABLE_W<0> {
        VECTOR_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:15 - Priority Vector Pause Size. If bit 17 of the network control register is written with a one then for each entry equal to zero in the Transmit PFC Pause Register\\[15:8\\], the PFC pause frame's pause quantum field associated with that entry will be taken from the transmit pause quantum register. For each entry equal to one in the Transmit PFC Pause Register \\[15:8\\], the pause quantum associated with that entry will be zero."]
    #[inline(always)]
    #[must_use]
    pub fn vector(&mut self) -> VECTOR_W<8> {
        VECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pfc_pause](index.html) module"]
pub struct TX_PFC_PAUSE_SPEC;
impl crate::RegisterSpec for TX_PFC_PAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pfc_pause::R](R) reader structure"]
impl crate::Readable for TX_PFC_PAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pfc_pause::W](W) writer structure"]
impl crate::Writable for TX_PFC_PAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PFC_PAUSE to value 0"]
impl crate::Resettable for TX_PFC_PAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
