#[doc = "Register `OUTPUTDIMENSION` reader"]
pub struct R(crate::R<OUTPUTDIMENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTPUTDIMENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTPUTDIMENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTPUTDIMENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTPUTDIMENSION` writer"]
pub struct W(crate::W<OUTPUTDIMENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTPUTDIMENSION_SPEC>;
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
impl From<crate::W<OUTPUTDIMENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTPUTDIMENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT_WIDTH` reader - Number of output pixel per input line. This number is equal to the width of the output frame minus one. (format is unsigned integer)"]
pub type OUTPUT_WIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTPUT_WIDTH` writer - Number of output pixel per input line. This number is equal to the width of the output frame minus one. (format is unsigned integer)"]
pub type OUTPUT_WIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIMENSION_SPEC, u16, u16, 14, O>;
#[doc = "Field `OUTPUT_HEIGHT` reader - Number of output lines per input frame or slice. This number is equal to the height of the output frame minus one. (format is unsigned integer)"]
pub type OUTPUT_HEIGHT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OUTPUT_HEIGHT` writer - Number of output lines per input frame or slice. This number is equal to the height of the output frame minus one. (format is unsigned integer)"]
pub type OUTPUT_HEIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OUTPUTDIMENSION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Number of output pixel per input line. This number is equal to the width of the output frame minus one. (format is unsigned integer)"]
    #[inline(always)]
    pub fn output_width(&self) -> OUTPUT_WIDTH_R {
        OUTPUT_WIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Number of output lines per input frame or slice. This number is equal to the height of the output frame minus one. (format is unsigned integer)"]
    #[inline(always)]
    pub fn output_height(&self) -> OUTPUT_HEIGHT_R {
        OUTPUT_HEIGHT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Number of output pixel per input line. This number is equal to the width of the output frame minus one. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn output_width(&mut self) -> OUTPUT_WIDTH_W<0> {
        OUTPUT_WIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Number of output lines per input frame or slice. This number is equal to the height of the output frame minus one. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn output_height(&mut self) -> OUTPUT_HEIGHT_W<16> {
        OUTPUT_HEIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dimensions of the output frame or fame slice. This values are required for both FIR filter and scaler, since their output may have other dimension than input. The dimension of scaler's output is defined by the scale factor. The dimension of FIR filter's output depends on the tiling at the begin and the end of the slice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outputdimension](index.html) module"]
pub struct OUTPUTDIMENSION_SPEC;
impl crate::RegisterSpec for OUTPUTDIMENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outputdimension::R](R) reader structure"]
impl crate::Readable for OUTPUTDIMENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outputdimension::W](W) writer structure"]
impl crate::Writable for OUTPUTDIMENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUTPUTDIMENSION to value 0"]
impl crate::Resettable for OUTPUTDIMENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
