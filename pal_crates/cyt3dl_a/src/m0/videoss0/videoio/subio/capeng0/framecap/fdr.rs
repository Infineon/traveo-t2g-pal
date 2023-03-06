#[doc = "Register `FDR` reader"]
pub struct R(crate::R<FDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDR` writer"]
pub struct W(crate::W<FDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDR_SPEC>;
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
impl From<crate::W<FDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIDTH` reader - Expected frame width of video frame (value is Width - 1). It could be configured after mode detection."]
pub type WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WIDTH` writer - Expected frame width of video frame (value is Width - 1). It could be configured after mode detection."]
pub type WIDTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u16, u16, 14, O>;
#[doc = "Field `HEIGHT` reader - Expected frame height of video frames and fields with id 0 (value is Height - 1). It could be configured after mode detection."]
pub type HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HEIGHT` writer - Expected frame height of video frames and fields with id 0 (value is Height - 1). It could be configured after mode detection."]
pub type HEIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDR_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Expected frame width of video frame (value is Width - 1). It could be configured after mode detection."]
    #[inline(always)]
    pub fn width(&self) -> WIDTH_R {
        WIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Expected frame height of video frames and fields with id 0 (value is Height - 1). It could be configured after mode detection."]
    #[inline(always)]
    pub fn height(&self) -> HEIGHT_R {
        HEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Expected frame width of video frame (value is Width - 1). It could be configured after mode detection."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WIDTH_W<0> {
        WIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Expected frame height of video frames and fields with id 0 (value is Height - 1). It could be configured after mode detection."]
    #[inline(always)]
    #[must_use]
    pub fn height(&mut self) -> HEIGHT_W<16> {
        HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame dimension (progressive frame and field 0).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdr](index.html) module"]
pub struct FDR_SPEC;
impl crate::RegisterSpec for FDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdr::R](R) reader structure"]
impl crate::Readable for FDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdr::W](W) writer structure"]
impl crate::Writable for FDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDR to value 0x00ef_013f"]
impl crate::Resettable for FDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ef_013f;
}
