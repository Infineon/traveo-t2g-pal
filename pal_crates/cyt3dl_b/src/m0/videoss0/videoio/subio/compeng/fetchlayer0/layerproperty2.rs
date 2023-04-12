#[doc = "Register `LAYERPROPERTY2` reader"]
pub struct R(crate::R<LAYERPROPERTY2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY2` writer"]
pub struct W(crate::W<LAYERPROPERTY2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY2_SPEC>;
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
impl From<crate::W<LAYERPROPERTY2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALETTEENABLE2` reader - See PaletteEnable0."]
pub type PALETTEENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `PALETTEENABLE2` writer - See PaletteEnable0."]
pub type PALETTEENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `TILEMODE2` reader - See TileMode0."]
pub type TILEMODE2_R = crate::FieldReader<u8, TILEMODE2_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE2_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE2_A) -> Self {
        variant as _
    }
}
impl TILEMODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE2_A {
        match self.bits {
            0 => TILEMODE2_A::TILE_FILL_ZERO,
            1 => TILEMODE2_A::TILE_FILL_CONSTANT,
            2 => TILEMODE2_A::TILE_PAD,
            3 => TILEMODE2_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE2_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE2_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE2_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE2_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE2` writer - See TileMode0."]
pub type TILEMODE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY2_SPEC, u8, TILEMODE2_A, 2, O>;
impl<'a, const O: u8> TILEMODE2_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE2_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE2_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE2_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE2_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE2` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE2` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE2` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE2` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE2` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE2` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE2` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE2` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE2` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE2` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE2` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE2` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB2` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB2_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB2` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE2` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE2_R = crate::FieldReader<u8, YUVCONVERSIONMODE2_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE2_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE2_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE2_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE2_A {
        match self.bits {
            0 => YUVCONVERSIONMODE2_A::OFF,
            1 => YUVCONVERSIONMODE2_A::ITU601,
            2 => YUVCONVERSIONMODE2_A::ITU601_FR,
            3 => YUVCONVERSIONMODE2_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE2_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE2_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE2_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE2_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE2` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE2_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY2_SPEC, u8, YUVCONVERSIONMODE2_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE2_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE2_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE2_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE2_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE2_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE2` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE2` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `PALETTEOFFSET2` reader - see PaletteOffset0."]
pub type PALETTEOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PALETTEOFFSET2` writer - see PaletteOffset0."]
pub type PALETTEOFFSET2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY2_SPEC, u16, u16, 11, O>;
#[doc = "Field `PALETTEIDWIDTH2` reader - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PALETTEIDWIDTH2` writer - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY2_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLIPWINDOWENABLE2` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE2` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE2` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE2_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE2` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    pub fn paletteenable2(&self) -> PALETTEENABLE2_R {
        PALETTEENABLE2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode2(&self) -> TILEMODE2_R {
        TILEMODE2_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable2(&self) -> ALPHASRCENABLE2_R {
        ALPHASRCENABLE2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable2(&self) -> ALPHACONSTENABLE2_R {
        ALPHACONSTENABLE2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable2(&self) -> ALPHATRANSENABLE2_R {
        ALPHATRANSENABLE2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable2(&self) -> RGBALPHASRCENABLE2_R {
        RGBALPHASRCENABLE2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable2(&self) -> RGBALPHACONSTENABLE2_R {
        RGBALPHACONSTENABLE2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable2(&self) -> RGBALPHATRANSENABLE2_R {
        RGBALPHATRANSENABLE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb2(&self) -> PREMULCONSTRGB2_R {
        PREMULCONSTRGB2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode2(&self) -> YUVCONVERSIONMODE2_R {
        YUVCONVERSIONMODE2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable2(&self) -> GAMMAREMOVEENABLE2_R {
        GAMMAREMOVEENABLE2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    pub fn paletteoffset2(&self) -> PALETTEOFFSET2_R {
        PALETTEOFFSET2_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    pub fn paletteidwidth2(&self) -> PALETTEIDWIDTH2_R {
        PALETTEIDWIDTH2_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable2(&self) -> CLIPWINDOWENABLE2_R {
        CLIPWINDOWENABLE2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable2(&self) -> SOURCEBUFFERENABLE2_R {
        SOURCEBUFFERENABLE2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteenable2(&mut self) -> PALETTEENABLE2_W<0> {
        PALETTEENABLE2_W::new(self)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode2(&mut self) -> TILEMODE2_W<1> {
        TILEMODE2_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable2(&mut self) -> ALPHASRCENABLE2_W<3> {
        ALPHASRCENABLE2_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable2(&mut self) -> ALPHACONSTENABLE2_W<4> {
        ALPHACONSTENABLE2_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable2(&mut self) -> ALPHATRANSENABLE2_W<6> {
        ALPHATRANSENABLE2_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable2(&mut self) -> RGBALPHASRCENABLE2_W<7> {
        RGBALPHASRCENABLE2_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable2(&mut self) -> RGBALPHACONSTENABLE2_W<8> {
        RGBALPHACONSTENABLE2_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable2(&mut self) -> RGBALPHATRANSENABLE2_W<10> {
        RGBALPHATRANSENABLE2_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb2(&mut self) -> PREMULCONSTRGB2_W<11> {
        PREMULCONSTRGB2_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode2(&mut self) -> YUVCONVERSIONMODE2_W<12> {
        YUVCONVERSIONMODE2_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable2(&mut self) -> GAMMAREMOVEENABLE2_W<14> {
        GAMMAREMOVEENABLE2_W::new(self)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteoffset2(&mut self) -> PALETTEOFFSET2_W<16> {
        PALETTEOFFSET2_W::new(self)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteidwidth2(&mut self) -> PALETTEIDWIDTH2_W<27> {
        PALETTEIDWIDTH2_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable2(&mut self) -> CLIPWINDOWENABLE2_W<30> {
        CLIPWINDOWENABLE2_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable2(&mut self) -> SOURCEBUFFERENABLE2_W<31> {
        SOURCEBUFFERENABLE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty2](index.html) module"]
pub struct LAYERPROPERTY2_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty2::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty2::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY2 to value 0x3800_0008"]
impl crate::Resettable for LAYERPROPERTY2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3800_0008;
}
