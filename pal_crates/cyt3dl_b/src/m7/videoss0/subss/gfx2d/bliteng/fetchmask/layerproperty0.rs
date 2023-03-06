#[doc = "Register `LAYERPROPERTY0` reader"]
pub struct R(crate::R<LAYERPROPERTY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY0` writer"]
pub struct W(crate::W<LAYERPROPERTY0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY0_SPEC>;
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
impl From<crate::W<LAYERPROPERTY0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PALETTEENABLE0` reader - Enables (value = 1) a color palette with 8 bits input and 24 bits output. Lower bits of the lookup index are read from memory (PaletteIdWidth0), upper bits are set to index of this layer. Palette output is extended by upper bits of index word read from memory (e.g. to store alpha together with index). Result is mapped to color channels according to ColorComponentBits/Shift settings."]
pub type PALETTEENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `PALETTEENABLE0` writer - Enables (value = 1) a color palette with 8 bits input and 24 bits output. Lower bits of the lookup index are read from memory (PaletteIdWidth0), upper bits are set to index of this layer. Palette output is extended by upper bits of index word read from memory (e.g. to store alpha together with index). Result is mapped to color channels according to ColorComponentBits/Shift settings."]
pub type PALETTEENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `TILEMODE0` reader - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
pub type TILEMODE0_R = crate::FieldReader<u8, TILEMODE0_A>;
#[doc = "Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE0_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE0_A) -> Self {
        variant as _
    }
}
impl TILEMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE0_A {
        match self.bits {
            0 => TILEMODE0_A::TILE_FILL_ZERO,
            1 => TILEMODE0_A::TILE_FILL_CONSTANT,
            2 => TILEMODE0_A::TILE_PAD,
            3 => TILEMODE0_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE0_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE0_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE0_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE0_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE0` writer - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
pub type TILEMODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY0_SPEC, u8, TILEMODE0_A, 2, O>;
impl<'a, const O: u8> TILEMODE0_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE0_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE0` reader - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
pub type ALPHASRCENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE0` writer - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
pub type ALPHASRCENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE0` reader - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
pub type ALPHACONSTENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE0` writer - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
pub type ALPHACONSTENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `ALPHAMASKENABLE0` reader - Value 1 enables mask alpha for computing the output alpha. When disabled mask alpha is set to 1."]
pub type ALPHAMASKENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASKENABLE0` writer - Value 1 enables mask alpha for computing the output alpha. When disabled mask alpha is set to 1."]
pub type ALPHAMASKENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE0` reader - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
pub type ALPHATRANSENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE0` writer - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
pub type ALPHATRANSENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE0` reader - Value 1 enables source alpha (stored together with color component in the source buffer) for RGB pre-multiply. When disabled source alpha is set to 1."]
pub type RGBALPHASRCENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE0` writer - Value 1 enables source alpha (stored together with color component in the source buffer) for RGB pre-multiply. When disabled source alpha is set to 1."]
pub type RGBALPHASRCENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE0` reader - Value 1 enables constant alpha (ConstAlpha fields) for RGB pre-multiply. When disabled constant alpha is set to 1."]
pub type RGBALPHACONSTENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE0` writer - Value 1 enables constant alpha (ConstAlpha fields) for RGB pre-multiply. When disabled constant alpha is set to 1."]
pub type RGBALPHACONSTENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `RGBALPHAMASKENABLE0` reader - Value 1 enables mask alpha (read by another Fetch unit from a separate alpha layer) for RGB pre-multiply. When disabled mask alpha is set to 1. Alpha mask input must be enabled for this field to have effect."]
pub type RGBALPHAMASKENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHAMASKENABLE0` writer - Value 1 enables mask alpha (read by another Fetch unit from a separate alpha layer) for RGB pre-multiply. When disabled mask alpha is set to 1. Alpha mask input must be enabled for this field to have effect."]
pub type RGBALPHAMASKENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE0` reader - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
pub type RGBALPHATRANSENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE0` writer - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
pub type RGBALPHATRANSENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB0` reader - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/Mask/TransEnable have no effect then."]
pub type PREMULCONSTRGB0_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB0` writer - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/Mask/TransEnable have no effect then."]
pub type PREMULCONSTRGB0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE0` reader - Enables different kind of YUV to RGB conversions."]
pub type YUVCONVERSIONMODE0_R = crate::FieldReader<u8, YUVCONVERSIONMODE0_A>;
#[doc = "Enables different kind of YUV to RGB conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE0_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE0_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE0_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE0_A {
        match self.bits {
            0 => YUVCONVERSIONMODE0_A::OFF,
            1 => YUVCONVERSIONMODE0_A::ITU601,
            2 => YUVCONVERSIONMODE0_A::ITU601_FR,
            3 => YUVCONVERSIONMODE0_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE0_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE0_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE0_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE0_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE0` writer - Enables different kind of YUV to RGB conversions."]
pub type YUVCONVERSIONMODE0_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY0_SPEC, u8, YUVCONVERSIONMODE0_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE0_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE0_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE0_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE0_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE0_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE0` reader - Value 1 enables the stage to remove a gamma from RGB components."]
pub type GAMMAREMOVEENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE0` writer - Value 1 enables the stage to remove a gamma from RGB components."]
pub type GAMMAREMOVEENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `PALETTEOFFSET0` reader - Palette may contain multiple areas. This offset shows to the starting position of the currently used area."]
pub type PALETTEOFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PALETTEOFFSET0` writer - Palette may contain multiple areas. This offset shows to the starting position of the currently used area."]
pub type PALETTEOFFSET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY0_SPEC, u16, u16, 11, O>;
#[doc = "Field `PALETTEIDWIDTH0` reader - Number minus one of least significant bits of pixel data read from the source buffer that are used as index value for color palette look-up."]
pub type PALETTEIDWIDTH0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PALETTEIDWIDTH0` writer - Number minus one of least significant bits of pixel data read from the source buffer that are used as index value for color palette look-up."]
pub type PALETTEIDWIDTH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYERPROPERTY0_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLIPWINDOWENABLE0` reader - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
pub type CLIPWINDOWENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE0` writer - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
pub type CLIPWINDOWENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE0` reader - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
pub type SOURCEBUFFERENABLE0_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE0` writer - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
pub type SOURCEBUFFERENABLE0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables (value = 1) a color palette with 8 bits input and 24 bits output. Lower bits of the lookup index are read from memory (PaletteIdWidth0), upper bits are set to index of this layer. Palette output is extended by upper bits of index word read from memory (e.g. to store alpha together with index). Result is mapped to color channels according to ColorComponentBits/Shift settings."]
    #[inline(always)]
    pub fn paletteenable0(&self) -> PALETTEENABLE0_R {
        PALETTEENABLE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
    #[inline(always)]
    pub fn tilemode0(&self) -> TILEMODE0_R {
        TILEMODE0_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
    #[inline(always)]
    pub fn alphasrcenable0(&self) -> ALPHASRCENABLE0_R {
        ALPHASRCENABLE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
    #[inline(always)]
    pub fn alphaconstenable0(&self) -> ALPHACONSTENABLE0_R {
        ALPHACONSTENABLE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Value 1 enables mask alpha for computing the output alpha. When disabled mask alpha is set to 1."]
    #[inline(always)]
    pub fn alphamaskenable0(&self) -> ALPHAMASKENABLE0_R {
        ALPHAMASKENABLE0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    pub fn alphatransenable0(&self) -> ALPHATRANSENABLE0_R {
        ALPHATRANSENABLE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Value 1 enables source alpha (stored together with color component in the source buffer) for RGB pre-multiply. When disabled source alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphasrcenable0(&self) -> RGBALPHASRCENABLE0_R {
        RGBALPHASRCENABLE0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Value 1 enables constant alpha (ConstAlpha fields) for RGB pre-multiply. When disabled constant alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphaconstenable0(&self) -> RGBALPHACONSTENABLE0_R {
        RGBALPHACONSTENABLE0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Value 1 enables mask alpha (read by another Fetch unit from a separate alpha layer) for RGB pre-multiply. When disabled mask alpha is set to 1. Alpha mask input must be enabled for this field to have effect."]
    #[inline(always)]
    pub fn rgbalphamaskenable0(&self) -> RGBALPHAMASKENABLE0_R {
        RGBALPHAMASKENABLE0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    pub fn rgbalphatransenable0(&self) -> RGBALPHATRANSENABLE0_R {
        RGBALPHATRANSENABLE0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/Mask/TransEnable have no effect then."]
    #[inline(always)]
    pub fn premulconstrgb0(&self) -> PREMULCONSTRGB0_R {
        PREMULCONSTRGB0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Enables different kind of YUV to RGB conversions."]
    #[inline(always)]
    pub fn yuvconversionmode0(&self) -> YUVCONVERSIONMODE0_R {
        YUVCONVERSIONMODE0_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Value 1 enables the stage to remove a gamma from RGB components."]
    #[inline(always)]
    pub fn gammaremoveenable0(&self) -> GAMMAREMOVEENABLE0_R {
        GAMMAREMOVEENABLE0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Palette may contain multiple areas. This offset shows to the starting position of the currently used area."]
    #[inline(always)]
    pub fn paletteoffset0(&self) -> PALETTEOFFSET0_R {
        PALETTEOFFSET0_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bits 27:29 - Number minus one of least significant bits of pixel data read from the source buffer that are used as index value for color palette look-up."]
    #[inline(always)]
    pub fn paletteidwidth0(&self) -> PALETTEIDWIDTH0_R {
        PALETTEIDWIDTH0_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
    #[inline(always)]
    pub fn clipwindowenable0(&self) -> CLIPWINDOWENABLE0_R {
        CLIPWINDOWENABLE0_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
    #[inline(always)]
    pub fn sourcebufferenable0(&self) -> SOURCEBUFFERENABLE0_R {
        SOURCEBUFFERENABLE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables (value = 1) a color palette with 8 bits input and 24 bits output. Lower bits of the lookup index are read from memory (PaletteIdWidth0), upper bits are set to index of this layer. Palette output is extended by upper bits of index word read from memory (e.g. to store alpha together with index). Result is mapped to color channels according to ColorComponentBits/Shift settings."]
    #[inline(always)]
    #[must_use]
    pub fn paletteenable0(&mut self) -> PALETTEENABLE0_W<0> {
        PALETTEENABLE0_W::new(self)
    }
    #[doc = "Bits 1:2 - Select the tile mode for pixels outside the source buffer. Clip color (0,0,0,0) takes precedence if a pixel becomes subject to both tiling and clipping."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode0(&mut self) -> TILEMODE0_W<1> {
        TILEMODE0_W::new(self)
    }
    #[doc = "Bit 3 - Value 1 enables source alpha for computing the output alpha. When disabled source alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable0(&mut self) -> ALPHASRCENABLE0_W<3> {
        ALPHASRCENABLE0_W::new(self)
    }
    #[doc = "Bit 4 - Value 1 enables constant alpha for computing the output alpha. When disabled constant alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable0(&mut self) -> ALPHACONSTENABLE0_W<4> {
        ALPHACONSTENABLE0_W::new(self)
    }
    #[doc = "Bit 5 - Value 1 enables mask alpha for computing the output alpha. When disabled mask alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable0(&mut self) -> ALPHAMASKENABLE0_W<5> {
        ALPHAMASKENABLE0_W::new(self)
    }
    #[doc = "Bit 6 - Value 1 enables transparent alpha for computing the output alpha. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable0(&mut self) -> ALPHATRANSENABLE0_W<6> {
        ALPHATRANSENABLE0_W::new(self)
    }
    #[doc = "Bit 7 - Value 1 enables source alpha (stored together with color component in the source buffer) for RGB pre-multiply. When disabled source alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable0(&mut self) -> RGBALPHASRCENABLE0_W<7> {
        RGBALPHASRCENABLE0_W::new(self)
    }
    #[doc = "Bit 8 - Value 1 enables constant alpha (ConstAlpha fields) for RGB pre-multiply. When disabled constant alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable0(&mut self) -> RGBALPHACONSTENABLE0_W<8> {
        RGBALPHACONSTENABLE0_W::new(self)
    }
    #[doc = "Bit 9 - Value 1 enables mask alpha (read by another Fetch unit from a separate alpha layer) for RGB pre-multiply. When disabled mask alpha is set to 1. Alpha mask input must be enabled for this field to have effect."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphamaskenable0(&mut self) -> RGBALPHAMASKENABLE0_W<9> {
        RGBALPHAMASKENABLE0_W::new(self)
    }
    #[doc = "Bit 10 - Value 1 enables transparent alpha (0 or 1 depending on RGB matching ConstantColor setting) for RGB pre-multiply. When disabled transparent alpha is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable0(&mut self) -> RGBALPHATRANSENABLE0_W<10> {
        RGBALPHATRANSENABLE0_W::new(self)
    }
    #[doc = "Bit 11 - When enabled (value 1) the values given by ConstantRed/Green/Blue are used instead of alpha for source RGB pre-multiply. Settings RGBAlphaSrc/Const/Mask/TransEnable have no effect then."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb0(&mut self) -> PREMULCONSTRGB0_W<11> {
        PREMULCONSTRGB0_W::new(self)
    }
    #[doc = "Bits 12:13 - Enables different kind of YUV to RGB conversions."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode0(&mut self) -> YUVCONVERSIONMODE0_W<12> {
        YUVCONVERSIONMODE0_W::new(self)
    }
    #[doc = "Bit 14 - Value 1 enables the stage to remove a gamma from RGB components."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable0(&mut self) -> GAMMAREMOVEENABLE0_W<14> {
        GAMMAREMOVEENABLE0_W::new(self)
    }
    #[doc = "Bits 16:26 - Palette may contain multiple areas. This offset shows to the starting position of the currently used area."]
    #[inline(always)]
    #[must_use]
    pub fn paletteoffset0(&mut self) -> PALETTEOFFSET0_W<16> {
        PALETTEOFFSET0_W::new(self)
    }
    #[doc = "Bits 27:29 - Number minus one of least significant bits of pixel data read from the source buffer that are used as index value for color palette look-up."]
    #[inline(always)]
    #[must_use]
    pub fn paletteidwidth0(&mut self) -> PALETTEIDWIDTH0_W<27> {
        PALETTEIDWIDTH0_W::new(self)
    }
    #[doc = "Bit 30 - Value 1 enables the clip window for this layer. Pixels outside the clip window get the clip color, pixels inside the source or tiling color."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable0(&mut self) -> CLIPWINDOWENABLE0_W<30> {
        CLIPWINDOWENABLE0_W::new(self)
    }
    #[doc = "Bit 31 - Value 1 enables the source buffer for this layer. When disabled the tiling color is used only (TileMode TILE_PAD not allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable0(&mut self) -> SOURCEBUFFERENABLE0_W<31> {
        SOURCEBUFFERENABLE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty0](index.html) module"]
pub struct LAYERPROPERTY0_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty0::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty0::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY0 to value 0xb800_0008"]
impl crate::Resettable for LAYERPROPERTY0_SPEC {
    const RESET_VALUE: Self::Ux = 0xb800_0008;
}
