#[doc = "Register `SKICKCONFIG` reader"]
pub struct R(crate::R<SKICKCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SKICKCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SKICKCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SKICKCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SKICKCONFIG` writer"]
pub struct W(crate::W<SKICKCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SKICKCONFIG_SPEC>;
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
impl From<crate::W<SKICKCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SKICKCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SKICKCOL` reader - Specifies on which column of the display raster the skick signal is triggered (1 .. SKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining SKickCol (1 .. SKickCol .. HTOTAL_MIN)."]
pub type SKICKCOL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SKICKCOL` writer - Specifies on which column of the display raster the skick signal is triggered (1 .. SKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining SKickCol (1 .. SKickCol .. HTOTAL_MIN)."]
pub type SKICKCOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SKICKCONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `SKICKINT1EN` reader - If enabled, maps the secondary kick signal (skick) on the interrupt pin int1. Overrides int1en."]
pub type SKICKINT1EN_R = crate::BitReader<bool>;
#[doc = "Field `SKICKINT1EN` writer - If enabled, maps the secondary kick signal (skick) on the interrupt pin int1. Overrides int1en."]
pub type SKICKINT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SKICKCONFIG_SPEC, bool, O>;
#[doc = "Field `SKICKROW` reader - Specifies on which row of the display raster the skick signal is triggered (1 .. SKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining SKickRow (1 .. SKickRow .. VTOTAL_MIN)."]
pub type SKICKROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SKICKROW` writer - Specifies on which row of the display raster the skick signal is triggered (1 .. SKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining SKickRow (1 .. SKickRow .. VTOTAL_MIN)."]
pub type SKICKROW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SKICKCONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `SKICKTRIG` reader - Select source for skick generation."]
pub type SKICKTRIG_R = crate::BitReader<SKICKTRIG_A>;
#[doc = "Select source for skick generation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKICKTRIG_A {
    #[doc = "0: Use internal skick signal, trigger point defined by SKickRow and SKickCol."]
    INTERNAL = 0,
    #[doc = "1: Use external skick input as trigger."]
    EXTERNAL = 1,
}
impl From<SKICKTRIG_A> for bool {
    #[inline(always)]
    fn from(variant: SKICKTRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl SKICKTRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKICKTRIG_A {
        match self.bits {
            false => SKICKTRIG_A::INTERNAL,
            true => SKICKTRIG_A::EXTERNAL,
        }
    }
    #[doc = "Checks if the value of the field is `INTERNAL`"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SKICKTRIG_A::INTERNAL
    }
    #[doc = "Checks if the value of the field is `EXTERNAL`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SKICKTRIG_A::EXTERNAL
    }
}
#[doc = "Field `SKICKTRIG` writer - Select source for skick generation."]
pub type SKICKTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SKICKCONFIG_SPEC, SKICKTRIG_A, O>;
impl<'a, const O: u8> SKICKTRIG_W<'a, O> {
    #[doc = "Use internal skick signal, trigger point defined by SKickRow and SKickCol."]
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SKICKTRIG_A::INTERNAL)
    }
    #[doc = "Use external skick input as trigger."]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SKICKTRIG_A::EXTERNAL)
    }
}
#[doc = "Field `SKICKEN` reader - Enables generation of internal skick signal."]
pub type SKICKEN_R = crate::BitReader<bool>;
#[doc = "Field `SKICKEN` writer - Enables generation of internal skick signal."]
pub type SKICKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SKICKCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the skick signal is triggered (1 .. SKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining SKickCol (1 .. SKickCol .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn skickcol(&self) -> SKICKCOL_R {
        SKICKCOL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - If enabled, maps the secondary kick signal (skick) on the interrupt pin int1. Overrides int1en."]
    #[inline(always)]
    pub fn skickint1en(&self) -> SKICKINT1EN_R {
        SKICKINT1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the skick signal is triggered (1 .. SKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining SKickRow (1 .. SKickRow .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn skickrow(&self) -> SKICKROW_R {
        SKICKROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - Select source for skick generation."]
    #[inline(always)]
    pub fn skicktrig(&self) -> SKICKTRIG_R {
        SKICKTRIG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables generation of internal skick signal."]
    #[inline(always)]
    pub fn skicken(&self) -> SKICKEN_R {
        SKICKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the skick signal is triggered (1 .. SKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining SKickCol (1 .. SKickCol .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn skickcol(&mut self) -> SKICKCOL_W<0> {
        SKICKCOL_W::new(self)
    }
    #[doc = "Bit 15 - If enabled, maps the secondary kick signal (skick) on the interrupt pin int1. Overrides int1en."]
    #[inline(always)]
    #[must_use]
    pub fn skickint1en(&mut self) -> SKICKINT1EN_W<15> {
        SKICKINT1EN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the skick signal is triggered (1 .. SKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining SKickRow (1 .. SKickRow .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn skickrow(&mut self) -> SKICKROW_W<16> {
        SKICKROW_W::new(self)
    }
    #[doc = "Bit 30 - Select source for skick generation."]
    #[inline(always)]
    #[must_use]
    pub fn skicktrig(&mut self) -> SKICKTRIG_W<30> {
        SKICKTRIG_W::new(self)
    }
    #[doc = "Bit 31 - Enables generation of internal skick signal."]
    #[inline(always)]
    #[must_use]
    pub fn skicken(&mut self) -> SKICKEN_W<31> {
        SKICKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the secondary kick signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [skickconfig](index.html) module"]
pub struct SKICKCONFIG_SPEC;
impl crate::RegisterSpec for SKICKCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [skickconfig::R](R) reader structure"]
impl crate::Readable for SKICKCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [skickconfig::W](W) writer structure"]
impl crate::Writable for SKICKCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SKICKCONFIG to value 0x00f0_0140"]
impl crate::Resettable for SKICKCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_0140;
}
