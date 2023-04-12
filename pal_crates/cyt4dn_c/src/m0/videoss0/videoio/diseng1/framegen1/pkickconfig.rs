#[doc = "Register `PKICKCONFIG` reader"]
pub struct R(crate::R<PKICKCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PKICKCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PKICKCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PKICKCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PKICKCONFIG` writer"]
pub struct W(crate::W<PKICKCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PKICKCONFIG_SPEC>;
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
impl From<crate::W<PKICKCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PKICKCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PKICKCOL` reader - Specifies on which column of the display raster the pkick signal is triggered (1 .. PKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining PKickCol (1 .. PKickCol .. HTOTAL_MIN)."]
pub type PKICKCOL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKICKCOL` writer - Specifies on which column of the display raster the pkick signal is triggered (1 .. PKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining PKickCol (1 .. PKickCol .. HTOTAL_MIN)."]
pub type PKICKCOL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PKICKCONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `PKICKINT0EN` reader - If enabled, maps the primary kick signal (pkick) on the interrupt pin int0. Overrides int0en."]
pub type PKICKINT0EN_R = crate::BitReader<bool>;
#[doc = "Field `PKICKINT0EN` writer - If enabled, maps the primary kick signal (pkick) on the interrupt pin int0. Overrides int0en."]
pub type PKICKINT0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKICKCONFIG_SPEC, bool, O>;
#[doc = "Field `PKICKROW` reader - Specifies on which row of the display raster the pkick signal is triggered (1 .. PKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining PKickRow (1 .. PKickRow .. VTOTAL_MIN)."]
pub type PKICKROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PKICKROW` writer - Specifies on which row of the display raster the pkick signal is triggered (1 .. PKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining PKickRow (1 .. PKickRow .. VTOTAL_MIN)."]
pub type PKICKROW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PKICKCONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `PKICKEN` reader - Enables pkick signal."]
pub type PKICKEN_R = crate::BitReader<bool>;
#[doc = "Field `PKICKEN` writer - Enables pkick signal."]
pub type PKICKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PKICKCONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the pkick signal is triggered (1 .. PKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining PKickCol (1 .. PKickCol .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn pkickcol(&self) -> PKICKCOL_R {
        PKICKCOL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - If enabled, maps the primary kick signal (pkick) on the interrupt pin int0. Overrides int0en."]
    #[inline(always)]
    pub fn pkickint0en(&self) -> PKICKINT0EN_R {
        PKICKINT0EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the pkick signal is triggered (1 .. PKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining PKickRow (1 .. PKickRow .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn pkickrow(&self) -> PKICKROW_R {
        PKICKROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables pkick signal."]
    #[inline(always)]
    pub fn pkicken(&self) -> PKICKEN_R {
        PKICKEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the pkick signal is triggered (1 .. PKickCol .. HTOTAL). Adaptive display timing must be taken into account when defining PKickCol (1 .. PKickCol .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn pkickcol(&mut self) -> PKICKCOL_W<0> {
        PKICKCOL_W::new(self)
    }
    #[doc = "Bit 15 - If enabled, maps the primary kick signal (pkick) on the interrupt pin int0. Overrides int0en."]
    #[inline(always)]
    #[must_use]
    pub fn pkickint0en(&mut self) -> PKICKINT0EN_W<15> {
        PKICKINT0EN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the pkick signal is triggered (1 .. PKickRow .. VTOTAL). Adaptive display timing must be taken into account when defining PKickRow (1 .. PKickRow .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn pkickrow(&mut self) -> PKICKROW_W<16> {
        PKICKROW_W::new(self)
    }
    #[doc = "Bit 31 - Enables pkick signal."]
    #[inline(always)]
    #[must_use]
    pub fn pkicken(&mut self) -> PKICKEN_W<31> {
        PKICKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the primary kick signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pkickconfig](index.html) module"]
pub struct PKICKCONFIG_SPEC;
impl crate::RegisterSpec for PKICKCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pkickconfig::R](R) reader structure"]
impl crate::Readable for PKICKCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pkickconfig::W](W) writer structure"]
impl crate::Writable for PKICKCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PKICKCONFIG to value 0x00f0_0140"]
impl crate::Resettable for PKICKCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f0_0140;
}
