#[doc = "Register `INT1CONFIG` reader"]
pub struct R(crate::R<INT1CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT1CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT1CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT1CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT1CONFIG` writer"]
pub struct W(crate::W<INT1CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT1CONFIG_SPEC>;
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
impl From<crate::W<INT1CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT1CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT1COL` reader - Specifies on which column of the display raster the Int1 signal is triggered (1 .. Int1Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int1Col (1 .. Int1Col .. HTOTAL_MIN)."]
pub type INT1COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT1COL` writer - Specifies on which column of the display raster the Int1 signal is triggered (1 .. Int1Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int1Col (1 .. Int1Col .. HTOTAL_MIN)."]
pub type INT1COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT1CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT1HSEN` reader - When enabled, Int1Row setting is ignored so that the interrupt occurs every line at position given by Int1Col."]
pub type INT1HSEN_R = crate::BitReader<bool>;
#[doc = "Field `INT1HSEN` writer - When enabled, Int1Row setting is ignored so that the interrupt occurs every line at position given by Int1Col."]
pub type INT1HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT1CONFIG_SPEC, bool, O>;
#[doc = "Field `INT1ROW` reader - Specifies on which row of the display raster the Int1 signal is triggered (1 .. Int1Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int1Row (1 .. Int1Row .. VTOTAL_MIN)."]
pub type INT1ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT1ROW` writer - Specifies on which row of the display raster the Int1 signal is triggered (1 .. Int1Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int1Row (1 .. Int1Row .. VTOTAL_MIN)."]
pub type INT1ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT1CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT1EN` reader - Enables Int1 (irq\\[1\\])."]
pub type INT1EN_R = crate::BitReader<bool>;
#[doc = "Field `INT1EN` writer - Enables Int1 (irq\\[1\\])."]
pub type INT1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT1CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int1 signal is triggered (1 .. Int1Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int1Col (1 .. Int1Col .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn int1col(&self) -> INT1COL_R {
        INT1COL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - When enabled, Int1Row setting is ignored so that the interrupt occurs every line at position given by Int1Col."]
    #[inline(always)]
    pub fn int1hsen(&self) -> INT1HSEN_R {
        INT1HSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int1 signal is triggered (1 .. Int1Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int1Row (1 .. Int1Row .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn int1row(&self) -> INT1ROW_R {
        INT1ROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables Int1 (irq\\[1\\])."]
    #[inline(always)]
    pub fn int1en(&self) -> INT1EN_R {
        INT1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int1 signal is triggered (1 .. Int1Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int1Col (1 .. Int1Col .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int1col(&mut self) -> INT1COL_W<0> {
        INT1COL_W::new(self)
    }
    #[doc = "Bit 15 - When enabled, Int1Row setting is ignored so that the interrupt occurs every line at position given by Int1Col."]
    #[inline(always)]
    #[must_use]
    pub fn int1hsen(&mut self) -> INT1HSEN_W<15> {
        INT1HSEN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int1 signal is triggered (1 .. Int1Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int1Row (1 .. Int1Row .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int1row(&mut self) -> INT1ROW_W<16> {
        INT1ROW_W::new(self)
    }
    #[doc = "Bit 31 - Enables Int1 (irq\\[1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn int1en(&mut self) -> INT1EN_W<31> {
        INT1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the Int1 interrupt signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int1config](index.html) module"]
pub struct INT1CONFIG_SPEC;
impl crate::RegisterSpec for INT1CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int1config::R](R) reader structure"]
impl crate::Readable for INT1CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int1config::W](W) writer structure"]
impl crate::Writable for INT1CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT1CONFIG to value 0"]
impl crate::Resettable for INT1CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
