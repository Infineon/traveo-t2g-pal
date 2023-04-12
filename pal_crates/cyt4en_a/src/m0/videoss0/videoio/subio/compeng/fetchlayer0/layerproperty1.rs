#[doc = "Register `LAYERPROPERTY1` reader"]
pub struct R(crate::R<LAYERPROPERTY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY1` writer"]
pub struct W(crate::W<LAYERPROPERTY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY1_SPEC>;
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
impl From<crate::W<LAYERPROPERTY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALETTEENABLE1` reader - See PaletteEnable0."]
pub type PALETTEENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `PALETTEENABLE1` writer - See PaletteEnable0."]
pub type PALETTEENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `TILEMODE1` reader - See TileMode0."]
pub type TILEMODE1_R = crate::FieldReader<u8, TILEMODE1_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE1_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE1_A) -> Self {
        variant as _
    }
}
impl TILEMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE1_A {
        match self.bits {
            0 => TILEMODE1_A::TILE_FILL_ZERO,
            1 => TILEMODE1_A::TILE_FILL_CONSTANT,
            2 => TILEMODE1_A::TILE_PAD,
            3 => TILEMODE1_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE1_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE1_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE1_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE1_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE1` writer - See TileMode0."]
pub type TILEMODE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY1_SPEC, u8, TILEMODE1_A, 2, O>;
impl<'a, const O: u8> TILEMODE1_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE1_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE1_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE1_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE1_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE1` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE1` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE1` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE1` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE1` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE1` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE1` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE1` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE1` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE1` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE1` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE1` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB1` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB1_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB1` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE1` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE1_R = crate::FieldReader<u8, YUVCONVERSIONMODE1_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE1_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE1_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE1_A {
        match self.bits {
            0 => YUVCONVERSIONMODE1_A::OFF,
            1 => YUVCONVERSIONMODE1_A::ITU601,
            2 => YUVCONVERSIONMODE1_A::ITU601_FR,
            3 => YUVCONVERSIONMODE1_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE1_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE1_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE1_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE1_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE1` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE1_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY1_SPEC, u8, YUVCONVERSIONMODE1_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE1_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE1_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE1_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE1_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE1_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE1` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE1` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `PALETTEOFFSET1` reader - see PaletteOffset0."]
pub type PALETTEOFFSET1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PALETTEOFFSET1` writer - see PaletteOffset0."]
pub type PALETTEOFFSET1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY1_SPEC, u16, u16, 11, O>;
#[doc = "Field `PALETTEIDWIDTH1` reader - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PALETTEIDWIDTH1` writer - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY1_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLIPWINDOWENABLE1` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE1` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE1` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE1_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE1` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    pub fn paletteenable1(&self) -> PALETTEENABLE1_R {
        PALETTEENABLE1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode1(&self) -> TILEMODE1_R {
        TILEMODE1_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable1(&self) -> ALPHASRCENABLE1_R {
        ALPHASRCENABLE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable1(&self) -> ALPHACONSTENABLE1_R {
        ALPHACONSTENABLE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable1(&self) -> ALPHATRANSENABLE1_R {
        ALPHATRANSENABLE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable1(&self) -> RGBALPHASRCENABLE1_R {
        RGBALPHASRCENABLE1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable1(&self) -> RGBALPHACONSTENABLE1_R {
        RGBALPHACONSTENABLE1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable1(&self) -> RGBALPHATRANSENABLE1_R {
        RGBALPHATRANSENABLE1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb1(&self) -> PREMULCONSTRGB1_R {
        PREMULCONSTRGB1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode1(&self) -> YUVCONVERSIONMODE1_R {
        YUVCONVERSIONMODE1_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable1(&self) -> GAMMAREMOVEENABLE1_R {
        GAMMAREMOVEENABLE1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    pub fn paletteoffset1(&self) -> PALETTEOFFSET1_R {
        PALETTEOFFSET1_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    pub fn paletteidwidth1(&self) -> PALETTEIDWIDTH1_R {
        PALETTEIDWIDTH1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable1(&self) -> CLIPWINDOWENABLE1_R {
        CLIPWINDOWENABLE1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable1(&self) -> SOURCEBUFFERENABLE1_R {
        SOURCEBUFFERENABLE1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteenable1(&mut self) -> PALETTEENABLE1_W<0> {
        PALETTEENABLE1_W::new(self)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode1(&mut self) -> TILEMODE1_W<1> {
        TILEMODE1_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable1(&mut self) -> ALPHASRCENABLE1_W<3> {
        ALPHASRCENABLE1_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable1(&mut self) -> ALPHACONSTENABLE1_W<4> {
        ALPHACONSTENABLE1_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable1(&mut self) -> ALPHATRANSENABLE1_W<6> {
        ALPHATRANSENABLE1_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable1(&mut self) -> RGBALPHASRCENABLE1_W<7> {
        RGBALPHASRCENABLE1_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable1(&mut self) -> RGBALPHACONSTENABLE1_W<8> {
        RGBALPHACONSTENABLE1_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable1(&mut self) -> RGBALPHATRANSENABLE1_W<10> {
        RGBALPHATRANSENABLE1_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb1(&mut self) -> PREMULCONSTRGB1_W<11> {
        PREMULCONSTRGB1_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode1(&mut self) -> YUVCONVERSIONMODE1_W<12> {
        YUVCONVERSIONMODE1_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable1(&mut self) -> GAMMAREMOVEENABLE1_W<14> {
        GAMMAREMOVEENABLE1_W::new(self)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteoffset1(&mut self) -> PALETTEOFFSET1_W<16> {
        PALETTEOFFSET1_W::new(self)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteidwidth1(&mut self) -> PALETTEIDWIDTH1_W<27> {
        PALETTEIDWIDTH1_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable1(&mut self) -> CLIPWINDOWENABLE1_W<30> {
        CLIPWINDOWENABLE1_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable1(&mut self) -> SOURCEBUFFERENABLE1_W<31> {
        SOURCEBUFFERENABLE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty1](index.html) module"]
pub struct LAYERPROPERTY1_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty1::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty1::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY1 to value 0x3800_0008"]
impl crate::Resettable for LAYERPROPERTY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3800_0008;
}
