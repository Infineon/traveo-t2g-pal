#[doc = "Register `INT3CONFIG` reader"]
pub struct R(crate::R<INT3CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT3CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT3CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT3CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT3CONFIG` writer"]
pub struct W(crate::W<INT3CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT3CONFIG_SPEC>;
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
impl From<crate::W<INT3CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT3CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT3COL` reader - Specifies on which column of the display raster the Int3 signal is triggered (1 .. Int3Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int3Col (1 .. Int3Col .. HTOTAL_MIN)."]
pub type INT3COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT3COL` writer - Specifies on which column of the display raster the Int3 signal is triggered (1 .. Int3Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int3Col (1 .. Int3Col .. HTOTAL_MIN)."]
pub type INT3COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT3CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT3HSEN` reader - When enabled, Int3Row setting is ignored so that the interrupt occurs every line at position given by Int3Col."]
pub type INT3HSEN_R = crate::BitReader<bool>;
#[doc = "Field `INT3HSEN` writer - When enabled, Int3Row setting is ignored so that the interrupt occurs every line at position given by Int3Col."]
pub type INT3HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT3CONFIG_SPEC, bool, O>;
#[doc = "Field `INT3ROW` reader - Specifies on which row of the display raster the Int3 signal is triggered (1 .. Int3Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int3Row (1 .. Int3Row .. VTOTAL_MIN)."]
pub type INT3ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT3ROW` writer - Specifies on which row of the display raster the Int3 signal is triggered (1 .. Int3Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int3Row (1 .. Int3Row .. VTOTAL_MIN)."]
pub type INT3ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT3CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT3EN` reader - Enables Int3."]
pub type INT3EN_R = crate::BitReader<bool>;
#[doc = "Field `INT3EN` writer - Enables Int3."]
pub type INT3EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT3CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int3 signal is triggered (1 .. Int3Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int3Col (1 .. Int3Col .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn int3col(&self) -> INT3COL_R {
        INT3COL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - When enabled, Int3Row setting is ignored so that the interrupt occurs every line at position given by Int3Col."]
    #[inline(always)]
    pub fn int3hsen(&self) -> INT3HSEN_R {
        INT3HSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int3 signal is triggered (1 .. Int3Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int3Row (1 .. Int3Row .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn int3row(&self) -> INT3ROW_R {
        INT3ROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables Int3."]
    #[inline(always)]
    pub fn int3en(&self) -> INT3EN_R {
        INT3EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int3 signal is triggered (1 .. Int3Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int3Col (1 .. Int3Col .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int3col(&mut self) -> INT3COL_W<0> {
        INT3COL_W::new(self)
    }
    #[doc = "Bit 15 - When enabled, Int3Row setting is ignored so that the interrupt occurs every line at position given by Int3Col."]
    #[inline(always)]
    #[must_use]
    pub fn int3hsen(&mut self) -> INT3HSEN_W<15> {
        INT3HSEN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int3 signal is triggered (1 .. Int3Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int3Row (1 .. Int3Row .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int3row(&mut self) -> INT3ROW_W<16> {
        INT3ROW_W::new(self)
    }
    #[doc = "Bit 31 - Enables Int3."]
    #[inline(always)]
    #[must_use]
    pub fn int3en(&mut self) -> INT3EN_W<31> {
        INT3EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the Int3 interrupt signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int3config](index.html) module"]
pub struct INT3CONFIG_SPEC;
impl crate::RegisterSpec for INT3CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int3config::R](R) reader structure"]
impl crate::Readable for INT3CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int3config::W](W) writer structure"]
impl crate::Writable for INT3CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT3CONFIG to value 0"]
impl crate::Resettable for INT3CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
