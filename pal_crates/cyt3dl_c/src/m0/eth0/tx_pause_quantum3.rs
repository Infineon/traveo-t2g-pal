#[doc = "Register `TX_PAUSE_QUANTUM3` reader"]
pub struct R(crate::R<TX_PAUSE_QUANTUM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_PAUSE_QUANTUM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_PAUSE_QUANTUM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_PAUSE_QUANTUM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_PAUSE_QUANTUM3` writer"]
pub struct W(crate::W<TX_PAUSE_QUANTUM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_PAUSE_QUANTUM3_SPEC>;
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
impl From<crate::W<TX_PAUSE_QUANTUM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_PAUSE_QUANTUM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUANTUM_P6` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTUM_P6_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTUM_P6` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
pub type QUANTUM_P6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_PAUSE_QUANTUM3_SPEC, u16, u16, 16, O>;
#[doc = "Field `QUANTUM_P7` reader - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTUM_P7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `QUANTUM_P7` writer - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
pub type QUANTUM_P7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TX_PAUSE_QUANTUM3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantum_p6(&self) -> QUANTUM_P6_R {
        QUANTUM_P6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantum_p7(&self) -> QUANTUM_P7_R {
        QUANTUM_P7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    #[must_use]
    pub fn quantum_p6(&mut self) -> QUANTUM_P6_W<0> {
        QUANTUM_P6_W::new(self)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    #[must_use]
    pub fn quantum_p7(&mut self) -> QUANTUM_P7_W<16> {
        QUANTUM_P7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Pause Quantum Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_pause_quantum3](index.html) module"]
pub struct TX_PAUSE_QUANTUM3_SPEC;
impl crate::RegisterSpec for TX_PAUSE_QUANTUM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_pause_quantum3::R](R) reader structure"]
impl crate::Readable for TX_PAUSE_QUANTUM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_pause_quantum3::W](W) writer structure"]
impl crate::Writable for TX_PAUSE_QUANTUM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_PAUSE_QUANTUM3 to value 0xffff_ffff"]
impl crate::Resettable for TX_PAUSE_QUANTUM3_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
