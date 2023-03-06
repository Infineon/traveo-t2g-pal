#[doc = "Register `LAYERPROPERTY3` reader"]
pub struct R(crate::R<LAYERPROPERTY3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY3` writer"]
pub struct W(crate::W<LAYERPROPERTY3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY3_SPEC>;
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
impl From<crate::W<LAYERPROPERTY3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILEMODE3` reader - See TileMode0."]
pub type TILEMODE3_R = crate::FieldReader<u8, TILEMODE3_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE3_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE3_A) -> Self {
        variant as _
    }
}
impl TILEMODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE3_A {
        match self.bits {
            0 => TILEMODE3_A::TILE_FILL_ZERO,
            1 => TILEMODE3_A::TILE_FILL_CONSTANT,
            2 => TILEMODE3_A::TILE_PAD,
            3 => TILEMODE3_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE3_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE3_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE3_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE3_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE3` writer - See TileMode0."]
pub type TILEMODE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY3_SPEC, u8, TILEMODE3_A, 2, O>;
impl<'a, const O: u8> TILEMODE3_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE3_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE3_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE3_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE3_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE3` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE3` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE3` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE3` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `ALPHAMASKENABLE3` reader - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASKENABLE3` writer - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE3` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE3` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE3` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE3` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE3` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE3` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `RGBALPHAMASKENABLE3` reader - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHAMASKENABLE3` writer - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE3` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE3` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB3` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB3_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB3` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE3` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE3_R = crate::FieldReader<u8, YUVCONVERSIONMODE3_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE3_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE3_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE3_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE3_A {
        match self.bits {
            0 => YUVCONVERSIONMODE3_A::OFF,
            1 => YUVCONVERSIONMODE3_A::ITU601,
            2 => YUVCONVERSIONMODE3_A::ITU601_FR,
            3 => YUVCONVERSIONMODE3_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE3_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE3_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE3_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE3_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE3` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE3_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY3_SPEC, u8, YUVCONVERSIONMODE3_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE3_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE3_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE3_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE3_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE3_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE3` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE3` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `CLIPWINDOWENABLE3` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE3` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE3` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE3_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE3` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode3(&self) -> TILEMODE3_R {
        TILEMODE3_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable3(&self) -> ALPHASRCENABLE3_R {
        ALPHASRCENABLE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable3(&self) -> ALPHACONSTENABLE3_R {
        ALPHACONSTENABLE3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    pub fn alphamaskenable3(&self) -> ALPHAMASKENABLE3_R {
        ALPHAMASKENABLE3_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable3(&self) -> ALPHATRANSENABLE3_R {
        ALPHATRANSENABLE3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable3(&self) -> RGBALPHASRCENABLE3_R {
        RGBALPHASRCENABLE3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable3(&self) -> RGBALPHACONSTENABLE3_R {
        RGBALPHACONSTENABLE3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    pub fn rgbalphamaskenable3(&self) -> RGBALPHAMASKENABLE3_R {
        RGBALPHAMASKENABLE3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable3(&self) -> RGBALPHATRANSENABLE3_R {
        RGBALPHATRANSENABLE3_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb3(&self) -> PREMULCONSTRGB3_R {
        PREMULCONSTRGB3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode3(&self) -> YUVCONVERSIONMODE3_R {
        YUVCONVERSIONMODE3_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable3(&self) -> GAMMAREMOVEENABLE3_R {
        GAMMAREMOVEENABLE3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable3(&self) -> CLIPWINDOWENABLE3_R {
        CLIPWINDOWENABLE3_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable3(&self) -> SOURCEBUFFERENABLE3_R {
        SOURCEBUFFERENABLE3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode3(&mut self) -> TILEMODE3_W<1> {
        TILEMODE3_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable3(&mut self) -> ALPHASRCENABLE3_W<3> {
        ALPHASRCENABLE3_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable3(&mut self) -> ALPHACONSTENABLE3_W<4> {
        ALPHACONSTENABLE3_W::new(self)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable3(&mut self) -> ALPHAMASKENABLE3_W<5> {
        ALPHAMASKENABLE3_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable3(&mut self) -> ALPHATRANSENABLE3_W<6> {
        ALPHATRANSENABLE3_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable3(&mut self) -> RGBALPHASRCENABLE3_W<7> {
        RGBALPHASRCENABLE3_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable3(&mut self) -> RGBALPHACONSTENABLE3_W<8> {
        RGBALPHACONSTENABLE3_W::new(self)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphamaskenable3(&mut self) -> RGBALPHAMASKENABLE3_W<9> {
        RGBALPHAMASKENABLE3_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable3(&mut self) -> RGBALPHATRANSENABLE3_W<10> {
        RGBALPHATRANSENABLE3_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb3(&mut self) -> PREMULCONSTRGB3_W<11> {
        PREMULCONSTRGB3_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode3(&mut self) -> YUVCONVERSIONMODE3_W<12> {
        YUVCONVERSIONMODE3_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable3(&mut self) -> GAMMAREMOVEENABLE3_W<14> {
        GAMMAREMOVEENABLE3_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable3(&mut self) -> CLIPWINDOWENABLE3_W<30> {
        CLIPWINDOWENABLE3_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable3(&mut self) -> SOURCEBUFFERENABLE3_W<31> {
        SOURCEBUFFERENABLE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty3](index.html) module"]
pub struct LAYERPROPERTY3_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty3::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty3::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY3 to value 0x08"]
impl crate::Resettable for LAYERPROPERTY3_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
