#[doc = "Register `LAYERPROPERTY5` reader"]
pub struct R(crate::R<LAYERPROPERTY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY5` writer"]
pub struct W(crate::W<LAYERPROPERTY5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY5_SPEC>;
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
impl From<crate::W<LAYERPROPERTY5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALETTEENABLE5` reader - See PaletteEnable0."]
pub type PALETTEENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `PALETTEENABLE5` writer - See PaletteEnable0."]
pub type PALETTEENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `TILEMODE5` reader - See TileMode0."]
pub type TILEMODE5_R = crate::FieldReader<u8, TILEMODE5_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE5_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE5_A) -> Self {
        variant as _
    }
}
impl TILEMODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE5_A {
        match self.bits {
            0 => TILEMODE5_A::TILE_FILL_ZERO,
            1 => TILEMODE5_A::TILE_FILL_CONSTANT,
            2 => TILEMODE5_A::TILE_PAD,
            3 => TILEMODE5_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE5_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE5_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE5_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE5_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE5` writer - See TileMode0."]
pub type TILEMODE5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY5_SPEC, u8, TILEMODE5_A, 2, O>;
impl<'a, const O: u8> TILEMODE5_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE5_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE5_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE5_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE5_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE5` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE5` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE5` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE5` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE5` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE5` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE5` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE5` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE5` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE5` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE5` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE5` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB5` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB5_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB5` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE5` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE5_R = crate::FieldReader<u8, YUVCONVERSIONMODE5_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE5_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE5_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE5_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE5_A {
        match self.bits {
            0 => YUVCONVERSIONMODE5_A::OFF,
            1 => YUVCONVERSIONMODE5_A::ITU601,
            2 => YUVCONVERSIONMODE5_A::ITU601_FR,
            3 => YUVCONVERSIONMODE5_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE5_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE5_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE5_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE5_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE5` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE5_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY5_SPEC, u8, YUVCONVERSIONMODE5_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE5_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE5_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE5_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE5_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE5_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE5` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE5` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `PALETTEOFFSET5` reader - see PaletteOffset0."]
pub type PALETTEOFFSET5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PALETTEOFFSET5` writer - see PaletteOffset0."]
pub type PALETTEOFFSET5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY5_SPEC, u16, u16, 11, O>;
#[doc = "Field `PALETTEIDWIDTH5` reader - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PALETTEIDWIDTH5` writer - see PaletteIdWidth0."]
pub type PALETTEIDWIDTH5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY5_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLIPWINDOWENABLE5` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE5` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE5` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE5_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE5` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    pub fn paletteenable5(&self) -> PALETTEENABLE5_R {
        PALETTEENABLE5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode5(&self) -> TILEMODE5_R {
        TILEMODE5_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable5(&self) -> ALPHASRCENABLE5_R {
        ALPHASRCENABLE5_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable5(&self) -> ALPHACONSTENABLE5_R {
        ALPHACONSTENABLE5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable5(&self) -> ALPHATRANSENABLE5_R {
        ALPHATRANSENABLE5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable5(&self) -> RGBALPHASRCENABLE5_R {
        RGBALPHASRCENABLE5_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable5(&self) -> RGBALPHACONSTENABLE5_R {
        RGBALPHACONSTENABLE5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable5(&self) -> RGBALPHATRANSENABLE5_R {
        RGBALPHATRANSENABLE5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb5(&self) -> PREMULCONSTRGB5_R {
        PREMULCONSTRGB5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode5(&self) -> YUVCONVERSIONMODE5_R {
        YUVCONVERSIONMODE5_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable5(&self) -> GAMMAREMOVEENABLE5_R {
        GAMMAREMOVEENABLE5_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    pub fn paletteoffset5(&self) -> PALETTEOFFSET5_R {
        PALETTEOFFSET5_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    pub fn paletteidwidth5(&self) -> PALETTEIDWIDTH5_R {
        PALETTEIDWIDTH5_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable5(&self) -> CLIPWINDOWENABLE5_R {
        CLIPWINDOWENABLE5_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable5(&self) -> SOURCEBUFFERENABLE5_R {
        SOURCEBUFFERENABLE5_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See PaletteEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteenable5(&mut self) -> PALETTEENABLE5_W<0> {
        PALETTEENABLE5_W::new(self)
    }
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode5(&mut self) -> TILEMODE5_W<1> {
        TILEMODE5_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable5(&mut self) -> ALPHASRCENABLE5_W<3> {
        ALPHASRCENABLE5_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable5(&mut self) -> ALPHACONSTENABLE5_W<4> {
        ALPHACONSTENABLE5_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable5(&mut self) -> ALPHATRANSENABLE5_W<6> {
        ALPHATRANSENABLE5_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable5(&mut self) -> RGBALPHASRCENABLE5_W<7> {
        RGBALPHASRCENABLE5_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable5(&mut self) -> RGBALPHACONSTENABLE5_W<8> {
        RGBALPHACONSTENABLE5_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable5(&mut self) -> RGBALPHATRANSENABLE5_W<10> {
        RGBALPHATRANSENABLE5_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb5(&mut self) -> PREMULCONSTRGB5_W<11> {
        PREMULCONSTRGB5_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode5(&mut self) -> YUVCONVERSIONMODE5_W<12> {
        YUVCONVERSIONMODE5_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable5(&mut self) -> GAMMAREMOVEENABLE5_W<14> {
        GAMMAREMOVEENABLE5_W::new(self)
    }
    #[doc = "Bits 16:26 - see PaletteOffset0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteoffset5(&mut self) -> PALETTEOFFSET5_W<16> {
        PALETTEOFFSET5_W::new(self)
    }
    #[doc = "Bits 27:29 - see PaletteIdWidth0."]
    #[inline(always)]
    #[must_use]
    pub fn paletteidwidth5(&mut self) -> PALETTEIDWIDTH5_W<27> {
        PALETTEIDWIDTH5_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable5(&mut self) -> CLIPWINDOWENABLE5_W<30> {
        CLIPWINDOWENABLE5_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable5(&mut self) -> SOURCEBUFFERENABLE5_W<31> {
        SOURCEBUFFERENABLE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty5](index.html) module"]
pub struct LAYERPROPERTY5_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty5::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty5::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY5 to value 0x3800_0008"]
impl crate::Resettable for LAYERPROPERTY5_SPEC {
    const RESET_VALUE: Self::Ux = 0x3800_0008;
}
