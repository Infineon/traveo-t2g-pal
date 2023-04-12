#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COLORDITHERENABLE` reader - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsRed/Green/Blue is smaller than 10 bits."]
pub type COLORDITHERENABLE_R = crate::BitReader<bool>;
#[doc = "Field `COLORDITHERENABLE` writer - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsRed/Green/Blue is smaller than 10 bits."]
pub type COLORDITHERENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHADITHERENABLE` reader - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsAlpha is smaller than 8 bits."]
pub type ALPHADITHERENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ALPHADITHERENABLE` writer - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsAlpha is smaller than 8 bits."]
pub type ALPHADITHERENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `DITHEROFFSET` reader - Dither offset that is additionally applied to the spatial offset, which is internally generated from the pixel position. Can be used by SW to generate image sequences with temporal dithering."]
pub type DITHEROFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DITHEROFFSET` writer - Dither offset that is additionally applied to the spatial offset, which is internally generated from the pixel position. Can be used by SW to generate image sequences with temporal dithering."]
pub type DITHEROFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `GAMMAAPPLYENABLE` reader - Enable gamma conversion stage to apply gamma function. This gamma function converts the pixel data from linear color space to non-linear color space before writing it to AXI."]
pub type GAMMAAPPLYENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAAPPLYENABLE` writer - Enable gamma conversion stage to apply gamma function. This gamma function converts the pixel data from linear color space to non-linear color space before writing it to AXI."]
pub type GAMMAAPPLYENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE` reader - Enables different kind of RGB to YUV conversions."]
pub type YUVCONVERSIONMODE_R = crate::FieldReader<u8, YUVCONVERSIONMODE_A>;
#[doc = "Enables different kind of RGB to YUV conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE_A {
    #[doc = "0: No conversion. Input data must be RGB."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE_A {
        match self.bits {
            0 => YUVCONVERSIONMODE_A::OFF,
            1 => YUVCONVERSIONMODE_A::ITU601,
            2 => YUVCONVERSIONMODE_A::ITU601_FR,
            3 => YUVCONVERSIONMODE_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE` writer - Enables different kind of RGB to YUV conversions."]
pub type YUVCONVERSIONMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, YUVCONVERSIONMODE_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE_W<'a, O> {
    #[doc = "No conversion. Input data must be RGB."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE_A::ITU709)
    }
}
#[doc = "Field `RASTERMODE` reader - Selects a method for destination buffer data from input pixels."]
pub type RASTERMODE_R = crate::FieldReader<u8, RASTERMODE_A>;
#[doc = "Selects a method for destination buffer data from input pixels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RASTERMODE_A {
    #[doc = "0: RGBA or YUV 4:4:4 pixel buffer."]
    NORMAL = 0,
    #[doc = "1: Packed YUV 4:2:2 pixel buffer. Effect is that U samples are written for pixels with even and V samples for odd column index only. So BitsPerPixel must be set to the size that a pair of YU or YV has in memory (most typically 16 bits). All correlated widths and horizontal offsets must be even."]
    YUV422 = 1,
    #[doc = "2: RLAD compressed bit stream."]
    ENCODE = 2,
}
impl From<RASTERMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RASTERMODE_A) -> Self {
        variant as _
    }
}
impl RASTERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RASTERMODE_A> {
        match self.bits {
            0 => Some(RASTERMODE_A::NORMAL),
            1 => Some(RASTERMODE_A::YUV422),
            2 => Some(RASTERMODE_A::ENCODE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == RASTERMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `YUV422`"]
    #[inline(always)]
    pub fn is_yuv422(&self) -> bool {
        *self == RASTERMODE_A::YUV422
    }
    #[doc = "Checks if the value of the field is `ENCODE`"]
    #[inline(always)]
    pub fn is_encode(&self) -> bool {
        *self == RASTERMODE_A::ENCODE
    }
}
#[doc = "Field `RASTERMODE` writer - Selects a method for destination buffer data from input pixels."]
pub type RASTERMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, RASTERMODE_A, 2, O>;
impl<'a, const O: u8> RASTERMODE_W<'a, O> {
    #[doc = "RGBA or YUV 4:4:4 pixel buffer."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RASTERMODE_A::NORMAL)
    }
    #[doc = "Packed YUV 4:2:2 pixel buffer. Effect is that U samples are written for pixels with even and V samples for odd column index only. So BitsPerPixel must be set to the size that a pair of YU or YV has in memory (most typically 16 bits). All correlated widths and horizontal offsets must be even."]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(RASTERMODE_A::YUV422)
    }
    #[doc = "RLAD compressed bit stream."]
    #[inline(always)]
    pub fn encode(self) -> &'a mut W {
        self.variant(RASTERMODE_A::ENCODE)
    }
}
#[doc = "Field `YUV422DOWNSAMPLINGMODE` reader - Selects a method for horizontal down-sampling when RasterMode is YUV422."]
pub type YUV422DOWNSAMPLINGMODE_R = crate::FieldReader<u8, YUV422DOWNSAMPLINGMODE_A>;
#[doc = "Selects a method for horizontal down-sampling when RasterMode is YUV422.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUV422DOWNSAMPLINGMODE_A {
    #[doc = "0: Nearest mode. Discards all odd samples, outputs even samples."]
    NEAREST = 0,
    #[doc = "1: Linear coaligned mode. 3 nearest UV samples are combined in linear filter to get one output sample."]
    COALIGNED = 1,
    #[doc = "2: Linear interspersed mode. 2 nearest UV samples are combined in linear filter to get one output sample."]
    INTERSPERSED = 2,
}
impl From<YUV422DOWNSAMPLINGMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: YUV422DOWNSAMPLINGMODE_A) -> Self {
        variant as _
    }
}
impl YUV422DOWNSAMPLINGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<YUV422DOWNSAMPLINGMODE_A> {
        match self.bits {
            0 => Some(YUV422DOWNSAMPLINGMODE_A::NEAREST),
            1 => Some(YUV422DOWNSAMPLINGMODE_A::COALIGNED),
            2 => Some(YUV422DOWNSAMPLINGMODE_A::INTERSPERSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEAREST`"]
    #[inline(always)]
    pub fn is_nearest(&self) -> bool {
        *self == YUV422DOWNSAMPLINGMODE_A::NEAREST
    }
    #[doc = "Checks if the value of the field is `COALIGNED`"]
    #[inline(always)]
    pub fn is_coaligned(&self) -> bool {
        *self == YUV422DOWNSAMPLINGMODE_A::COALIGNED
    }
    #[doc = "Checks if the value of the field is `INTERSPERSED`"]
    #[inline(always)]
    pub fn is_interspersed(&self) -> bool {
        *self == YUV422DOWNSAMPLINGMODE_A::INTERSPERSED
    }
}
#[doc = "Field `YUV422DOWNSAMPLINGMODE` writer - Selects a method for horizontal down-sampling when RasterMode is YUV422."]
pub type YUV422DOWNSAMPLINGMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, YUV422DOWNSAMPLINGMODE_A, 2, O>;
impl<'a, const O: u8> YUV422DOWNSAMPLINGMODE_W<'a, O> {
    #[doc = "Nearest mode. Discards all odd samples, outputs even samples."]
    #[inline(always)]
    pub fn nearest(self) -> &'a mut W {
        self.variant(YUV422DOWNSAMPLINGMODE_A::NEAREST)
    }
    #[doc = "Linear coaligned mode. 3 nearest UV samples are combined in linear filter to get one output sample."]
    #[inline(always)]
    pub fn coaligned(self) -> &'a mut W {
        self.variant(YUV422DOWNSAMPLINGMODE_A::COALIGNED)
    }
    #[doc = "Linear interspersed mode. 2 nearest UV samples are combined in linear filter to get one output sample."]
    #[inline(always)]
    pub fn interspersed(self) -> &'a mut W {
        self.variant(YUV422DOWNSAMPLINGMODE_A::INTERSPERSED)
    }
}
impl R {
    #[doc = "Bit 0 - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsRed/Green/Blue is smaller than 10 bits."]
    #[inline(always)]
    pub fn colorditherenable(&self) -> COLORDITHERENABLE_R {
        COLORDITHERENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsAlpha is smaller than 8 bits."]
    #[inline(always)]
    pub fn alphaditherenable(&self) -> ALPHADITHERENABLE_R {
        ALPHADITHERENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Dither offset that is additionally applied to the spatial offset, which is internally generated from the pixel position. Can be used by SW to generate image sequences with temporal dithering."]
    #[inline(always)]
    pub fn ditheroffset(&self) -> DITHEROFFSET_R {
        DITHEROFFSET_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Enable gamma conversion stage to apply gamma function. This gamma function converts the pixel data from linear color space to non-linear color space before writing it to AXI."]
    #[inline(always)]
    pub fn gammaapplyenable(&self) -> GAMMAAPPLYENABLE_R {
        GAMMAAPPLYENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Enables different kind of RGB to YUV conversions."]
    #[inline(always)]
    pub fn yuvconversionmode(&self) -> YUVCONVERSIONMODE_R {
        YUVCONVERSIONMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Selects a method for destination buffer data from input pixels."]
    #[inline(always)]
    pub fn rastermode(&self) -> RASTERMODE_R {
        RASTERMODE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Selects a method for horizontal down-sampling when RasterMode is YUV422."]
    #[inline(always)]
    pub fn yuv422downsamplingmode(&self) -> YUV422DOWNSAMPLINGMODE_R {
        YUV422DOWNSAMPLINGMODE_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsRed/Green/Blue is smaller than 10 bits."]
    #[inline(always)]
    #[must_use]
    pub fn colorditherenable(&mut self) -> COLORDITHERENABLE_W<0> {
        COLORDITHERENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Controls whether spatial dithering (value 1) or LSB truncation (value 0) is used when ComponentBitsAlpha is smaller than 8 bits."]
    #[inline(always)]
    #[must_use]
    pub fn alphaditherenable(&mut self) -> ALPHADITHERENABLE_W<1> {
        ALPHADITHERENABLE_W::new(self)
    }
    #[doc = "Bits 4:7 - Dither offset that is additionally applied to the spatial offset, which is internally generated from the pixel position. Can be used by SW to generate image sequences with temporal dithering."]
    #[inline(always)]
    #[must_use]
    pub fn ditheroffset(&mut self) -> DITHEROFFSET_W<4> {
        DITHEROFFSET_W::new(self)
    }
    #[doc = "Bit 12 - Enable gamma conversion stage to apply gamma function. This gamma function converts the pixel data from linear color space to non-linear color space before writing it to AXI."]
    #[inline(always)]
    #[must_use]
    pub fn gammaapplyenable(&mut self) -> GAMMAAPPLYENABLE_W<12> {
        GAMMAAPPLYENABLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Enables different kind of RGB to YUV conversions."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode(&mut self) -> YUVCONVERSIONMODE_W<16> {
        YUVCONVERSIONMODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Selects a method for destination buffer data from input pixels."]
    #[inline(always)]
    #[must_use]
    pub fn rastermode(&mut self) -> RASTERMODE_W<18> {
        RASTERMODE_W::new(self)
    }
    #[doc = "Bits 20:21 - Selects a method for horizontal down-sampling when RasterMode is YUV422."]
    #[inline(always)]
    #[must_use]
    pub fn yuv422downsamplingmode(&mut self) -> YUV422DOWNSAMPLINGMODE_W<20> {
        YUV422DOWNSAMPLINGMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Store unit dynamic control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
