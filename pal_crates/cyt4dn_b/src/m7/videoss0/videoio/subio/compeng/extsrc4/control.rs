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
#[doc = "Field `CLIPWINDOWENABLE` reader - Value 1 enables the clip window feature."]
pub type CLIPWINDOWENABLE_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE` writer - Value 1 enables the clip window feature."]
pub type CLIPWINDOWENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `RASTERMODE` reader - Selects a method how generate output samples from input stream."]
pub type RASTERMODE_R = crate::BitReader<RASTERMODE_A>;
#[doc = "Selects a method how generate output samples from input stream.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RASTERMODE_A {
    #[doc = "0: Input stream is RGB(A) or YUV 4:4:4."]
    NORMAL = 0,
    #[doc = "1: Input stream is packed YUV 4:2:2."]
    YUV422 = 1,
}
impl From<RASTERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RASTERMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl RASTERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RASTERMODE_A {
        match self.bits {
            false => RASTERMODE_A::NORMAL,
            true => RASTERMODE_A::YUV422,
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
}
#[doc = "Field `RASTERMODE` writer - Selects a method how generate output samples from input stream."]
pub type RASTERMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, RASTERMODE_A, O>;
impl<'a, const O: u8> RASTERMODE_W<'a, O> {
    #[doc = "Input stream is RGB(A) or YUV 4:4:4."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(RASTERMODE_A::NORMAL)
    }
    #[doc = "Input stream is packed YUV 4:2:2."]
    #[inline(always)]
    pub fn yuv422(self) -> &'a mut W {
        self.variant(RASTERMODE_A::YUV422)
    }
}
#[doc = "Field `YUV422UPSAMPLINGMODE` reader - Selects a method for horizontal up-sampling when RasterMode is YUV422."]
pub type YUV422UPSAMPLINGMODE_R = crate::BitReader<YUV422UPSAMPLINGMODE_A>;
#[doc = "Selects a method for horizontal up-sampling when RasterMode is YUV422.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YUV422UPSAMPLINGMODE_A {
    #[doc = "0: Replicate mode for interspersed samples (UV samples between Y samples)."]
    REPLICATE = 0,
    #[doc = "1: Interpolate mode for coaligned samples (UV samples at Y sample positions)."]
    INTERPOLATE = 1,
}
impl From<YUV422UPSAMPLINGMODE_A> for bool {
    #[inline(always)]
    fn from(variant: YUV422UPSAMPLINGMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl YUV422UPSAMPLINGMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUV422UPSAMPLINGMODE_A {
        match self.bits {
            false => YUV422UPSAMPLINGMODE_A::REPLICATE,
            true => YUV422UPSAMPLINGMODE_A::INTERPOLATE,
        }
    }
    #[doc = "Checks if the value of the field is `REPLICATE`"]
    #[inline(always)]
    pub fn is_replicate(&self) -> bool {
        *self == YUV422UPSAMPLINGMODE_A::REPLICATE
    }
    #[doc = "Checks if the value of the field is `INTERPOLATE`"]
    #[inline(always)]
    pub fn is_interpolate(&self) -> bool {
        *self == YUV422UPSAMPLINGMODE_A::INTERPOLATE
    }
}
#[doc = "Field `YUV422UPSAMPLINGMODE` writer - Selects a method for horizontal up-sampling when RasterMode is YUV422."]
pub type YUV422UPSAMPLINGMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONTROL_SPEC, YUV422UPSAMPLINGMODE_A, O>;
impl<'a, const O: u8> YUV422UPSAMPLINGMODE_W<'a, O> {
    #[doc = "Replicate mode for interspersed samples (UV samples between Y samples)."]
    #[inline(always)]
    pub fn replicate(self) -> &'a mut W {
        self.variant(YUV422UPSAMPLINGMODE_A::REPLICATE)
    }
    #[doc = "Interpolate mode for coaligned samples (UV samples at Y sample positions)."]
    #[inline(always)]
    pub fn interpolate(self) -> &'a mut W {
        self.variant(YUV422UPSAMPLINGMODE_A::INTERPOLATE)
    }
}
#[doc = "Field `YUVCONVERSIONMODE` reader - Enables different kind of YUV to RGB conversions."]
pub type YUVCONVERSIONMODE_R = crate::FieldReader<u8, YUVCONVERSIONMODE_A>;
#[doc = "Enables different kind of YUV to RGB conversions.\n\nValue on reset: 0"]
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
#[doc = "Field `YUVCONVERSIONMODE` writer - Enables different kind of YUV to RGB conversions."]
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
#[doc = "Field `RGBALPHASRCENABLE` reader - Value 1 enables source alpha (extracted from input stream) for RGB pre-multiply. When disabled source alpha is set to 1."]
pub type RGBALPHASRCENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE` writer - Value 1 enables source alpha (extracted from input stream) for RGB pre-multiply. When disabled source alpha is set to 1."]
pub type RGBALPHASRCENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE` reader - Value 1 enables constant alpha (ConstAlpha field) for RGB pre-multiply. When disabled constant alpha is set to 1."]
pub type RGBALPHACONSTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE` writer - Value 1 enables constant alpha (ConstAlpha field) for RGB pre-multiply. When disabled constant alpha is set to 1."]
pub type RGBALPHACONSTENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE` reader - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
pub type RGBALPHATRANSENABLE_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE` writer - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
pub type RGBALPHATRANSENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB` reader - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/TransEnable have no effect then."]
pub type PREMULCONSTRGB_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB` writer - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/TransEnable have no effect then."]
pub type PREMULCONSTRGB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHASRCENABLE` reader - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
pub type ALPHASRCENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE` writer - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
pub type ALPHASRCENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE` reader - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
pub type ALPHACONSTENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE` writer - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
pub type ALPHACONSTENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE` reader - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
pub type ALPHATRANSENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE` writer - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
pub type ALPHATRANSENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `GAMMAREMOVEENABLE` reader - Value 1 enables the stage to remove a gamma from RGB components."]
pub type GAMMAREMOVEENABLE_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE` writer - Value 1 enables the stage to remove a gamma from RGB components."]
pub type GAMMAREMOVEENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Value 1 enables the clip window feature."]
    #[inline(always)]
    pub fn clipwindowenable(&self) -> CLIPWINDOWENABLE_R {
        CLIPWINDOWENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Selects a method how generate output samples from input stream."]
    #[inline(always)]
    pub fn rastermode(&self) -> RASTERMODE_R {
        RASTERMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects a method for horizontal up-sampling when RasterMode is YUV422."]
    #[inline(always)]
    pub fn yuv422upsamplingmode(&self) -> YUV422UPSAMPLINGMODE_R {
        YUV422UPSAMPLINGMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Enables different kind of YUV to RGB conversions."]
    #[inline(always)]
    pub fn yuvconversionmode(&self) -> YUVCONVERSIONMODE_R {
        YUVCONVERSIONMODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Value 1 enables source alpha (extracted from input stream) for RGB pre-multiply. When disabled source alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphasrcenable(&self) -> RGBALPHASRCENABLE_R {
        RGBALPHASRCENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Value 1 enables constant alpha (ConstAlpha field) for RGB pre-multiply. When disabled constant alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphaconstenable(&self) -> RGBALPHACONSTENABLE_R {
        RGBALPHACONSTENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphatransenable(&self) -> RGBALPHATRANSENABLE_R {
        RGBALPHATRANSENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/TransEnable have no effect then."]
    #[inline(always)]
    pub fn premulconstrgb(&self) -> PREMULCONSTRGB_R {
        PREMULCONSTRGB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
    #[inline(always)]
    pub fn alphasrcenable(&self) -> ALPHASRCENABLE_R {
        ALPHASRCENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
    #[inline(always)]
    pub fn alphaconstenable(&self) -> ALPHACONSTENABLE_R {
        ALPHACONSTENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    pub fn alphatransenable(&self) -> ALPHATRANSENABLE_R {
        ALPHATRANSENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Value 1 enables the stage to remove a gamma from RGB components."]
    #[inline(always)]
    pub fn gammaremoveenable(&self) -> GAMMAREMOVEENABLE_R {
        GAMMAREMOVEENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Value 1 enables the clip window feature."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable(&mut self) -> CLIPWINDOWENABLE_W<0> {
        CLIPWINDOWENABLE_W::new(self)
    }
    #[doc = "Bit 4 - Selects a method how generate output samples from input stream."]
    #[inline(always)]
    #[must_use]
    pub fn rastermode(&mut self) -> RASTERMODE_W<4> {
        RASTERMODE_W::new(self)
    }
    #[doc = "Bit 5 - Selects a method for horizontal up-sampling when RasterMode is YUV422."]
    #[inline(always)]
    #[must_use]
    pub fn yuv422upsamplingmode(&mut self) -> YUV422UPSAMPLINGMODE_W<5> {
        YUV422UPSAMPLINGMODE_W::new(self)
    }
    #[doc = "Bits 6:7 - Enables different kind of YUV to RGB conversions."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode(&mut self) -> YUVCONVERSIONMODE_W<6> {
        YUVCONVERSIONMODE_W::new(self)
    }
    #[doc = "Bit 8 - Value 1 enables source alpha (extracted from input stream) for RGB pre-multiply. When disabled source alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable(&mut self) -> RGBALPHASRCENABLE_W<8> {
        RGBALPHASRCENABLE_W::new(self)
    }
    #[doc = "Bit 9 - Value 1 enables constant alpha (ConstAlpha field) for RGB pre-multiply. When disabled constant alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable(&mut self) -> RGBALPHACONSTENABLE_W<9> {
        RGBALPHACONSTENABLE_W::new(self)
    }
    #[doc = "Bit 10 - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable(&mut self) -> RGBALPHATRANSENABLE_W<10> {
        RGBALPHATRANSENABLE_W::new(self)
    }
    #[doc = "Bit 11 - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/TransEnable have no effect then."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb(&mut self) -> PREMULCONSTRGB_W<11> {
        PREMULCONSTRGB_W::new(self)
    }
    #[doc = "Bit 12 - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable(&mut self) -> ALPHASRCENABLE_W<12> {
        ALPHASRCENABLE_W::new(self)
    }
    #[doc = "Bit 13 - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable(&mut self) -> ALPHACONSTENABLE_W<13> {
        ALPHACONSTENABLE_W::new(self)
    }
    #[doc = "Bit 14 - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable(&mut self) -> ALPHATRANSENABLE_W<14> {
        ALPHATRANSENABLE_W::new(self)
    }
    #[doc = "Bit 16 - Value 1 enables the stage to remove a gamma from RGB components."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable(&mut self) -> GAMMAREMOVEENABLE_W<16> {
        GAMMAREMOVEENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
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
#[doc = "`reset()` method sets CONTROL to value 0x1000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
