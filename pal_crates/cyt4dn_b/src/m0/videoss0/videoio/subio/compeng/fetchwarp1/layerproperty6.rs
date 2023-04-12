#[doc = "Register `LAYERPROPERTY6` reader"]
pub struct R(crate::R<LAYERPROPERTY6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY6` writer"]
pub struct W(crate::W<LAYERPROPERTY6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY6_SPEC>;
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
impl From<crate::W<LAYERPROPERTY6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILEMODE6` reader - See TileMode0."]
pub type TILEMODE6_R = crate::FieldReader<u8, TILEMODE6_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE6_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE6_A) -> Self {
        variant as _
    }
}
impl TILEMODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE6_A {
        match self.bits {
            0 => TILEMODE6_A::TILE_FILL_ZERO,
            1 => TILEMODE6_A::TILE_FILL_CONSTANT,
            2 => TILEMODE6_A::TILE_PAD,
            3 => TILEMODE6_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE6_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE6_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE6_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE6_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE6` writer - See TileMode0."]
pub type TILEMODE6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY6_SPEC, u8, TILEMODE6_A, 2, O>;
impl<'a, const O: u8> TILEMODE6_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE6_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE6_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE6_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE6_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE6` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE6` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE6` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE6` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `ALPHAMASKENABLE6` reader - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASKENABLE6` writer - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE6` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE6` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE6` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE6` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE6` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE6` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `RGBALPHAMASKENABLE6` reader - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHAMASKENABLE6` writer - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE6` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE6` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB6` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB6_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB6` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE6` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE6_R = crate::FieldReader<u8, YUVCONVERSIONMODE6_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE6_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE6_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE6_A {
        match self.bits {
            0 => YUVCONVERSIONMODE6_A::OFF,
            1 => YUVCONVERSIONMODE6_A::ITU601,
            2 => YUVCONVERSIONMODE6_A::ITU601_FR,
            3 => YUVCONVERSIONMODE6_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE6_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE6_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE6_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE6_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE6` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE6_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY6_SPEC, u8, YUVCONVERSIONMODE6_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE6_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE6_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE6_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE6_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE6_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE6` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE6` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `CLIPWINDOWENABLE6` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE6` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE6` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE6_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE6` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY6_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode6(&self) -> TILEMODE6_R {
        TILEMODE6_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable6(&self) -> ALPHASRCENABLE6_R {
        ALPHASRCENABLE6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable6(&self) -> ALPHACONSTENABLE6_R {
        ALPHACONSTENABLE6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    pub fn alphamaskenable6(&self) -> ALPHAMASKENABLE6_R {
        ALPHAMASKENABLE6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable6(&self) -> ALPHATRANSENABLE6_R {
        ALPHATRANSENABLE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable6(&self) -> RGBALPHASRCENABLE6_R {
        RGBALPHASRCENABLE6_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable6(&self) -> RGBALPHACONSTENABLE6_R {
        RGBALPHACONSTENABLE6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    pub fn rgbalphamaskenable6(&self) -> RGBALPHAMASKENABLE6_R {
        RGBALPHAMASKENABLE6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable6(&self) -> RGBALPHATRANSENABLE6_R {
        RGBALPHATRANSENABLE6_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb6(&self) -> PREMULCONSTRGB6_R {
        PREMULCONSTRGB6_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode6(&self) -> YUVCONVERSIONMODE6_R {
        YUVCONVERSIONMODE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable6(&self) -> GAMMAREMOVEENABLE6_R {
        GAMMAREMOVEENABLE6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable6(&self) -> CLIPWINDOWENABLE6_R {
        CLIPWINDOWENABLE6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable6(&self) -> SOURCEBUFFERENABLE6_R {
        SOURCEBUFFERENABLE6_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode6(&mut self) -> TILEMODE6_W<1> {
        TILEMODE6_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable6(&mut self) -> ALPHASRCENABLE6_W<3> {
        ALPHASRCENABLE6_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable6(&mut self) -> ALPHACONSTENABLE6_W<4> {
        ALPHACONSTENABLE6_W::new(self)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable6(&mut self) -> ALPHAMASKENABLE6_W<5> {
        ALPHAMASKENABLE6_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable6(&mut self) -> ALPHATRANSENABLE6_W<6> {
        ALPHATRANSENABLE6_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable6(&mut self) -> RGBALPHASRCENABLE6_W<7> {
        RGBALPHASRCENABLE6_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable6(&mut self) -> RGBALPHACONSTENABLE6_W<8> {
        RGBALPHACONSTENABLE6_W::new(self)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphamaskenable6(&mut self) -> RGBALPHAMASKENABLE6_W<9> {
        RGBALPHAMASKENABLE6_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable6(&mut self) -> RGBALPHATRANSENABLE6_W<10> {
        RGBALPHATRANSENABLE6_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb6(&mut self) -> PREMULCONSTRGB6_W<11> {
        PREMULCONSTRGB6_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode6(&mut self) -> YUVCONVERSIONMODE6_W<12> {
        YUVCONVERSIONMODE6_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable6(&mut self) -> GAMMAREMOVEENABLE6_W<14> {
        GAMMAREMOVEENABLE6_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable6(&mut self) -> CLIPWINDOWENABLE6_W<30> {
        CLIPWINDOWENABLE6_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable6(&mut self) -> SOURCEBUFFERENABLE6_W<31> {
        SOURCEBUFFERENABLE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty6](index.html) module"]
pub struct LAYERPROPERTY6_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty6::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty6::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY6 to value 0x08"]
impl crate::Resettable for LAYERPROPERTY6_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
