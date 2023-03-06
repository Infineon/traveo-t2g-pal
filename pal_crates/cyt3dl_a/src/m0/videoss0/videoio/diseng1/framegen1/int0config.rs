#[doc = "Register `INT0CONFIG` reader"]
pub struct R(crate::R<INT0CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT0CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT0CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT0CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT0CONFIG` writer"]
pub struct W(crate::W<INT0CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT0CONFIG_SPEC>;
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
impl From<crate::W<INT0CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT0CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT0COL` reader - Specifies on which column of the display raster the Int0 signal is triggered (1 .. Int0Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int0Col (1 .. Int0Col .. HTOTAL_MIN)."]
pub type INT0COL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT0COL` writer - Specifies on which column of the display raster the Int0 signal is triggered (1 .. Int0Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int0Col (1 .. Int0Col .. HTOTAL_MIN)."]
pub type INT0COL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT0CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT0HSEN` reader - When enabled, Int0Row setting is ignored so that the interrupt occurs every line at position given by Int0Col."]
pub type INT0HSEN_R = crate::BitReader<bool>;
#[doc = "Field `INT0HSEN` writer - When enabled, Int0Row setting is ignored so that the interrupt occurs every line at position given by Int0Col."]
pub type INT0HSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT0CONFIG_SPEC, bool, O>;
#[doc = "Field `INT0ROW` reader - Specifies on which row of the display raster the Int0 signal is triggered (1 .. Int0Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int0Row (1 .. Int0Row .. VTOTAL_MIN)."]
pub type INT0ROW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT0ROW` writer - Specifies on which row of the display raster the Int0 signal is triggered (1 .. Int0Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int0Row (1 .. Int0Row .. VTOTAL_MIN)."]
pub type INT0ROW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INT0CONFIG_SPEC, u16, u16, 14, O>;
#[doc = "Field `INT0EN` reader - Enables Int0."]
pub type INT0EN_R = crate::BitReader<bool>;
#[doc = "Field `INT0EN` writer - Enables Int0."]
pub type INT0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT0CONFIG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int0 signal is triggered (1 .. Int0Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int0Col (1 .. Int0Col .. HTOTAL_MIN)."]
    #[inline(always)]
    pub fn int0col(&self) -> INT0COL_R {
        INT0COL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - When enabled, Int0Row setting is ignored so that the interrupt occurs every line at position given by Int0Col."]
    #[inline(always)]
    pub fn int0hsen(&self) -> INT0HSEN_R {
        INT0HSEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int0 signal is triggered (1 .. Int0Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int0Row (1 .. Int0Row .. VTOTAL_MIN)."]
    #[inline(always)]
    pub fn int0row(&self) -> INT0ROW_R {
        INT0ROW_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - Enables Int0."]
    #[inline(always)]
    pub fn int0en(&self) -> INT0EN_R {
        INT0EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Specifies on which column of the display raster the Int0 signal is triggered (1 .. Int0Col .. HTOTAL). Adaptive display timing must be taken into account when defining Int0Col (1 .. Int0Col .. HTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int0col(&mut self) -> INT0COL_W<0> {
        INT0COL_W::new(self)
    }
    #[doc = "Bit 15 - When enabled, Int0Row setting is ignored so that the interrupt occurs every line at position given by Int0Col."]
    #[inline(always)]
    #[must_use]
    pub fn int0hsen(&mut self) -> INT0HSEN_W<15> {
        INT0HSEN_W::new(self)
    }
    #[doc = "Bits 16:29 - Specifies on which row of the display raster the Int0 signal is triggered (1 .. Int0Row .. VTOTAL). Adaptive display timing must be taken into account when defining Int0Row (1 .. Int0Row .. VTOTAL_MIN)."]
    #[inline(always)]
    #[must_use]
    pub fn int0row(&mut self) -> INT0ROW_W<16> {
        INT0ROW_W::new(self)
    }
    #[doc = "Bit 31 - Enables Int0."]
    #[inline(always)]
    #[must_use]
    pub fn int0en(&mut self) -> INT0EN_W<31> {
        INT0EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Coordinates of the trigger point for generation of the Int0 interrupt signal\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int0config](index.html) module"]
pub struct INT0CONFIG_SPEC;
impl crate::RegisterSpec for INT0CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int0config::R](R) reader structure"]
impl crate::Readable for INT0CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int0config::W](W) writer structure"]
impl crate::Writable for INT0CONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT0CONFIG to value 0"]
impl crate::Resettable for INT0CONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
