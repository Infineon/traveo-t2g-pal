#[doc = "Register `GTUC3` reader"]
pub struct R(crate::R<GTUC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC3` writer"]
pub struct W(crate::W<GTUC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC3_SPEC>;
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
impl From<crate::W<GTUC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UIOA` reader - Microtick Initial Offset Channel A (pMicroInitialOffset\\[A\\]) Configures the number of microticks between the actual time reference point on channel A and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[A\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
pub type UIOA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UIOA` writer - Microtick Initial Offset Channel A (pMicroInitialOffset\\[A\\]) Configures the number of microticks between the actual time reference point on channel A and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[A\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
pub type UIOA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC3_SPEC, u8, u8, 8, O>;
#[doc = "Field `UIOB` reader - Microtick Initial Offset Channel B (pMicroInitialOffset\\[B\\]) Configures the number of microticks between the actual time reference point on channel B and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[B\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
pub type UIOB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UIOB` writer - Microtick Initial Offset Channel B (pMicroInitialOffset\\[B\\]) Configures the number of microticks between the actual time reference point on channel B and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[B\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
pub type UIOB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC3_SPEC, u8, u8, 8, O>;
#[doc = "Field `MIOA` reader - Macrotick Initial Offset Channel A (pMacroInitialOffset\\[A\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
pub type MIOA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIOA` writer - Macrotick Initial Offset Channel A (pMacroInitialOffset\\[A\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
pub type MIOA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC3_SPEC, u8, u8, 7, O>;
#[doc = "Field `MIOB` reader - Macrotick Initial Offset Channel B (pMacroInitialOffset\\[B\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
pub type MIOB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIOB` writer - Macrotick Initial Offset Channel B (pMacroInitialOffset\\[B\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
pub type MIOB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC3_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:7 - Microtick Initial Offset Channel A (pMicroInitialOffset\\[A\\]) Configures the number of microticks between the actual time reference point on channel A and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[A\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
    #[inline(always)]
    pub fn uioa(&self) -> UIOA_R {
        UIOA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Microtick Initial Offset Channel B (pMicroInitialOffset\\[B\\]) Configures the number of microticks between the actual time reference point on channel B and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[B\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
    #[inline(always)]
    pub fn uiob(&self) -> UIOB_R {
        UIOB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:22 - Macrotick Initial Offset Channel A (pMacroInitialOffset\\[A\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
    #[inline(always)]
    pub fn mioa(&self) -> MIOA_R {
        MIOA_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Macrotick Initial Offset Channel B (pMacroInitialOffset\\[B\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
    #[inline(always)]
    pub fn miob(&self) -> MIOB_R {
        MIOB_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Microtick Initial Offset Channel A (pMicroInitialOffset\\[A\\]) Configures the number of microticks between the actual time reference point on channel A and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[A\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
    #[inline(always)]
    #[must_use]
    pub fn uioa(&mut self) -> UIOA_W<0> {
        UIOA_W::new(self)
    }
    #[doc = "Bits 8:15 - Microtick Initial Offset Channel B (pMicroInitialOffset\\[B\\]) Configures the number of microticks between the actual time reference point on channel B and the subsequent macrotick boundary of the secondary time reference point. The parameter depends on pDelayCompensation\\[B\\]
and therefore has to be set for each channel independently. Valid values are 0 to 240 uT."]
    #[inline(always)]
    #[must_use]
    pub fn uiob(&mut self) -> UIOB_W<8> {
        UIOB_W::new(self)
    }
    #[doc = "Bits 16:22 - Macrotick Initial Offset Channel A (pMacroInitialOffset\\[A\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
    #[inline(always)]
    #[must_use]
    pub fn mioa(&mut self) -> MIOA_W<16> {
        MIOA_W::new(self)
    }
    #[doc = "Bits 24:30 - Macrotick Initial Offset Channel B (pMacroInitialOffset\\[B\\]) Configures the number of macroticks between the static slot boundary and the subsequent macrotick boundary of the secondary time reference point based on the nominal macrotick duration. Must be identical in all nodes of a cluster. Valid values are 2 to 72 MT."]
    #[inline(always)]
    #[must_use]
    pub fn miob(&mut self) -> MIOB_W<24> {
        MIOB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc3](index.html) module"]
pub struct GTUC3_SPEC;
impl crate::RegisterSpec for GTUC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc3::R](R) reader structure"]
impl crate::Readable for GTUC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc3::W](W) writer structure"]
impl crate::Writable for GTUC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC3 to value 0x0202_0000"]
impl crate::Resettable for GTUC3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_0000;
}
