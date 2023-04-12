#[doc = "Register `LAYERPROPERTY7` reader"]
pub struct R(crate::R<LAYERPROPERTY7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY7` writer"]
pub struct W(crate::W<LAYERPROPERTY7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY7_SPEC>;
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
impl From<crate::W<LAYERPROPERTY7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILEMODE7` reader - See TileMode0."]
pub type TILEMODE7_R = crate::FieldReader<u8, TILEMODE7_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE7_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE7_A) -> Self {
        variant as _
    }
}
impl TILEMODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE7_A {
        match self.bits {
            0 => TILEMODE7_A::TILE_FILL_ZERO,
            1 => TILEMODE7_A::TILE_FILL_CONSTANT,
            2 => TILEMODE7_A::TILE_PAD,
            3 => TILEMODE7_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE7_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE7_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE7_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE7_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE7` writer - See TileMode0."]
pub type TILEMODE7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY7_SPEC, u8, TILEMODE7_A, 2, O>;
impl<'a, const O: u8> TILEMODE7_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE7_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE7_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE7_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE7_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE7` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE7` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE7` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE7` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `ALPHAMASKENABLE7` reader - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASKENABLE7` writer - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE7` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE7` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE7` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE7` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE7` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE7` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `RGBALPHAMASKENABLE7` reader - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHAMASKENABLE7` writer - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE7` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE7` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB7` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB7_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB7` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE7` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE7_R = crate::FieldReader<u8, YUVCONVERSIONMODE7_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE7_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE7_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE7_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE7_A {
        match self.bits {
            0 => YUVCONVERSIONMODE7_A::OFF,
            1 => YUVCONVERSIONMODE7_A::ITU601,
            2 => YUVCONVERSIONMODE7_A::ITU601_FR,
            3 => YUVCONVERSIONMODE7_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE7_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE7_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE7_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE7_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE7` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE7_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY7_SPEC, u8, YUVCONVERSIONMODE7_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE7_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE7_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE7_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE7_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE7_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE7` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE7` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `CLIPWINDOWENABLE7` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE7` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE7` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE7_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE7` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY7_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode7(&self) -> TILEMODE7_R {
        TILEMODE7_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable7(&self) -> ALPHASRCENABLE7_R {
        ALPHASRCENABLE7_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable7(&self) -> ALPHACONSTENABLE7_R {
        ALPHACONSTENABLE7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    pub fn alphamaskenable7(&self) -> ALPHAMASKENABLE7_R {
        ALPHAMASKENABLE7_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable7(&self) -> ALPHATRANSENABLE7_R {
        ALPHATRANSENABLE7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable7(&self) -> RGBALPHASRCENABLE7_R {
        RGBALPHASRCENABLE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable7(&self) -> RGBALPHACONSTENABLE7_R {
        RGBALPHACONSTENABLE7_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    pub fn rgbalphamaskenable7(&self) -> RGBALPHAMASKENABLE7_R {
        RGBALPHAMASKENABLE7_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable7(&self) -> RGBALPHATRANSENABLE7_R {
        RGBALPHATRANSENABLE7_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb7(&self) -> PREMULCONSTRGB7_R {
        PREMULCONSTRGB7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode7(&self) -> YUVCONVERSIONMODE7_R {
        YUVCONVERSIONMODE7_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable7(&self) -> GAMMAREMOVEENABLE7_R {
        GAMMAREMOVEENABLE7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable7(&self) -> CLIPWINDOWENABLE7_R {
        CLIPWINDOWENABLE7_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable7(&self) -> SOURCEBUFFERENABLE7_R {
        SOURCEBUFFERENABLE7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode7(&mut self) -> TILEMODE7_W<1> {
        TILEMODE7_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable7(&mut self) -> ALPHASRCENABLE7_W<3> {
        ALPHASRCENABLE7_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable7(&mut self) -> ALPHACONSTENABLE7_W<4> {
        ALPHACONSTENABLE7_W::new(self)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable7(&mut self) -> ALPHAMASKENABLE7_W<5> {
        ALPHAMASKENABLE7_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable7(&mut self) -> ALPHATRANSENABLE7_W<6> {
        ALPHATRANSENABLE7_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable7(&mut self) -> RGBALPHASRCENABLE7_W<7> {
        RGBALPHASRCENABLE7_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable7(&mut self) -> RGBALPHACONSTENABLE7_W<8> {
        RGBALPHACONSTENABLE7_W::new(self)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphamaskenable7(&mut self) -> RGBALPHAMASKENABLE7_W<9> {
        RGBALPHAMASKENABLE7_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable7(&mut self) -> RGBALPHATRANSENABLE7_W<10> {
        RGBALPHATRANSENABLE7_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb7(&mut self) -> PREMULCONSTRGB7_W<11> {
        PREMULCONSTRGB7_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode7(&mut self) -> YUVCONVERSIONMODE7_W<12> {
        YUVCONVERSIONMODE7_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable7(&mut self) -> GAMMAREMOVEENABLE7_W<14> {
        GAMMAREMOVEENABLE7_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable7(&mut self) -> CLIPWINDOWENABLE7_W<30> {
        CLIPWINDOWENABLE7_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable7(&mut self) -> SOURCEBUFFERENABLE7_W<31> {
        SOURCEBUFFERENABLE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 7.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty7](index.html) module"]
pub struct LAYERPROPERTY7_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty7::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty7::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY7 to value 0x08"]
impl crate::Resettable for LAYERPROPERTY7_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
