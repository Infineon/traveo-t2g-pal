#[doc = "Register `INT2CONFIG` reader"]
pub struct R(crate::R<INT2CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT2CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT2CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT2CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT2CONFIG` writer"]
pub struct W(crate::W<INT2CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT2CONFIG_SPEC>;
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
impl From<crate::W<INT2CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT2CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT2COL` reader - Specifies on which column of the display raster the Int2 signal is triggered (1 .. Int2Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int2Col (1 .. Int2Col .. HTOTAL_MIN)."]
pub type INT2COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT2COL` writer - Specifies on which column of the display raster the Int2 signal is triggered (1 .. Int2Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int2Col (1 .. Int2Col .. HTOTAL_MIN)."]
pub type INT2COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT2CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT2HSEN` reader - When enabled, Int2Row setting is ignored so that the interrupt occurs every line at position given by Int2Col."]
pub type INT2HSEN_R = crate::BitReader<bool>;
#[doc = "Field `INT2HSEN` writer - When enabled, Int2Row setting is ignored so that the interrupt occurs every line at position given by Int2Col."]
pub type INT2HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT2CONFIG_SPEC, bool, O>;
#[doc = "Field `INT2ROW` reader - Specifies on which row of the display raster the Int2 signal is triggered (1 .. Int2Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int2Row (1 .. Int2Row .. VTOTAL_MIN)."]
pub type INT2ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT2ROW` writer - Specifies on which row of the display raster the Int2 signal is triggered (1 .. Int2Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int2Row (1 .. Int2Row .. VTOTAL_MIN)."]
pub type INT2ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT2CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT2EN` reader - Enables Int2."]
pub type INT2EN_R = crate::BitReader<bool>;
#[doc = "Field `INT2EN` writer - Enables Int2."]
pub type INT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT2CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int2 signal is triggered (1 .. Int2Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int2Col (1 .. Int2Col .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn int2col(&self) -> INT2COL_R {
        INT2COL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - When enabled, Int2Row setting is ignored so that the interrupt occurs every line at position given by Int2Col."]
    #[inline(always)]
    pub fn int2hsen(&self) -> INT2HSEN_R {
        INT2HSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int2 signal is triggered (1 .. Int2Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int2Row (1 .. Int2Row .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn int2row(&self) -> INT2ROW_R {
        INT2ROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables Int2."]
    #[inline(always)]
    pub fn int2en(&self) -> INT2EN_R {
        INT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int2 signal is triggered (1 .. Int2Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int2Col (1 .. Int2Col .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int2col(&mut self) -> INT2COL_W<0> {
        INT2COL_W::new(self)
    }
    #[doc = "Bit 15 - When enabled, Int2Row setting is ignored so that the interrupt occurs every line at position given by Int2Col."]
    #[inline(always)]
    #[must_use]
    pub fn int2hsen(&mut self) -> INT2HSEN_W<15> {
        INT2HSEN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int2 signal is triggered (1 .. Int2Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int2Row (1 .. Int2Row .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int2row(&mut self) -> INT2ROW_W<16> {
        INT2ROW_W::new(self)
    }
    #[doc = "Bit 31 - Enables Int2."]
    #[inline(always)]
    #[must_use]
    pub fn int2en(&mut self) -> INT2EN_W<31> {
        INT2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the Int2 interrupt signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int2config](index.html) module"]
pub struct INT2CONFIG_SPEC;
impl crate::RegisterSpec for INT2CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int2config::R](R) reader structure"]
impl crate::Readable for INT2CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int2config::W](W) writer structure"]
impl crate::Writable for INT2CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT2CONFIG to value 0"]
impl crate::Resettable for INT2CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
