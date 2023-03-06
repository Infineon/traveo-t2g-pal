#[doc = "Register `LAYERPROPERTY4` reader"]
pub struct R(crate::R<LAYERPROPERTY4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYERPROPERTY4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYERPROPERTY4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYERPROPERTY4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYERPROPERTY4` writer"]
pub struct W(crate::W<LAYERPROPERTY4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYERPROPERTY4_SPEC>;
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
impl From<crate::W<LAYERPROPERTY4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYERPROPERTY4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TILEMODE4` reader - See TileMode0."]
pub type TILEMODE4_R = crate::FieldReader<u8, TILEMODE4_A>;
#[doc = "See TileMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILEMODE4_A {
    #[doc = "0: Use zero value"]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color register value"]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    TILE_PAD_ZERO = 3,
}
impl From<TILEMODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: TILEMODE4_A) -> Self {
        variant as _
    }
}
impl TILEMODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILEMODE4_A {
        match self.bits {
            0 => TILEMODE4_A::TILE_FILL_ZERO,
            1 => TILEMODE4_A::TILE_FILL_CONSTANT,
            2 => TILEMODE4_A::TILE_PAD,
            3 => TILEMODE4_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILEMODE4_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILEMODE4_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILEMODE4_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILEMODE4_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILEMODE4` writer - See TileMode0."]
pub type TILEMODE4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY4_SPEC, u8, TILEMODE4_A, 2, O>;
impl<'a, const O: u8> TILEMODE4_W<'a, O> {
    #[doc = "Use zero value"]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILEMODE4_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color register value"]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILEMODE4_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source buffer. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILEMODE4_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component. Must not be used for DECODE or YUV422 operations or when SourceBufferEnable is 0."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILEMODE4_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `ALPHASRCENABLE4` reader - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHASRCENABLE4` writer - See AlphaSrcSelect0."]
pub type ALPHASRCENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `ALPHACONSTENABLE4` reader - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHACONSTENABLE4` writer - See AlphaConstSelect0."]
pub type ALPHACONSTENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `ALPHAMASKENABLE4` reader - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASKENABLE4` writer - See AlphaMaskSelect0."]
pub type ALPHAMASKENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `ALPHATRANSENABLE4` reader - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `ALPHATRANSENABLE4` writer - See AlphaTransSelect0."]
pub type ALPHATRANSENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `RGBALPHASRCENABLE4` reader - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHASRCENABLE4` writer - See RGBAlphaSrcSelect0."]
pub type RGBALPHASRCENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `RGBALPHACONSTENABLE4` reader - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHACONSTENABLE4` writer - See RGBAlphaConstSelect0."]
pub type RGBALPHACONSTENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `RGBALPHAMASKENABLE4` reader - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHAMASKENABLE4` writer - See RGBAlphaMaskSelect0."]
pub type RGBALPHAMASKENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `RGBALPHATRANSENABLE4` reader - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `RGBALPHATRANSENABLE4` writer - See RGBAlphaTransSelect0."]
pub type RGBALPHATRANSENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `PREMULCONSTRGB4` reader - See PremulConstRGB0."]
pub type PREMULCONSTRGB4_R = crate::BitReader<bool>;
#[doc = "Field `PREMULCONSTRGB4` writer - See PremulConstRGB0."]
pub type PREMULCONSTRGB4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `YUVCONVERSIONMODE4` reader - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE4_R = crate::FieldReader<u8, YUVCONVERSIONMODE4_A>;
#[doc = "See YUVConversionMode0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum YUVCONVERSIONMODE4_A {
    #[doc = "0: No conversion."]
    OFF = 0,
    #[doc = "1: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU601 = 1,
    #[doc = "2: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    ITU601_FR = 2,
    #[doc = "3: Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    ITU709 = 3,
}
impl From<YUVCONVERSIONMODE4_A> for u8 {
    #[inline(always)]
    fn from(variant: YUVCONVERSIONMODE4_A) -> Self {
        variant as _
    }
}
impl YUVCONVERSIONMODE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> YUVCONVERSIONMODE4_A {
        match self.bits {
            0 => YUVCONVERSIONMODE4_A::OFF,
            1 => YUVCONVERSIONMODE4_A::ITU601,
            2 => YUVCONVERSIONMODE4_A::ITU601_FR,
            3 => YUVCONVERSIONMODE4_A::ITU709,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == YUVCONVERSIONMODE4_A::OFF
    }
    #[doc = "Checks if the value of the field is `ITU601`"]
    #[inline(always)]
    pub fn is_itu601(&self) -> bool {
        *self == YUVCONVERSIONMODE4_A::ITU601
    }
    #[doc = "Checks if the value of the field is `ITU601_FR`"]
    #[inline(always)]
    pub fn is_itu601_fr(&self) -> bool {
        *self == YUVCONVERSIONMODE4_A::ITU601_FR
    }
    #[doc = "Checks if the value of the field is `ITU709`"]
    #[inline(always)]
    pub fn is_itu709(&self) -> bool {
        *self == YUVCONVERSIONMODE4_A::ITU709
    }
}
#[doc = "Field `YUVCONVERSIONMODE4` writer - See YUVConversionMode0."]
pub type YUVCONVERSIONMODE4_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, LAYERPROPERTY4_SPEC, u8, YUVCONVERSIONMODE4_A, 2, O>;
impl<'a, const O: u8> YUVCONVERSIONMODE4_W<'a, O> {
    #[doc = "No conversion."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE4_A::OFF)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6 (standard definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu601(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE4_A::ITU601)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.601-6, but assuming full range YUV inputs (0..255). Most typically used for computer graphics (e.g. for JPEG encoding)."]
    #[inline(always)]
    pub fn itu601_fr(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE4_A::ITU601_FR)
    }
    #[doc = "Conversion from YCbCr (YUV) to RGB according to ITU recommendation BT.709-5 part 2 (high definition TV). Input range is 16..235 for Y and 16..240 for U/V."]
    #[inline(always)]
    pub fn itu709(self) -> &'a mut W {
        self.variant(YUVCONVERSIONMODE4_A::ITU709)
    }
}
#[doc = "Field `GAMMAREMOVEENABLE4` reader - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `GAMMAREMOVEENABLE4` writer - See GammaRemoveEnable0."]
pub type GAMMAREMOVEENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `CLIPWINDOWENABLE4` reader - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `CLIPWINDOWENABLE4` writer - See ClipWindowEnable0."]
pub type CLIPWINDOWENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
#[doc = "Field `SOURCEBUFFERENABLE4` reader - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE4_R = crate::BitReader<bool>;
#[doc = "Field `SOURCEBUFFERENABLE4` writer - See SourceBufferEnable0."]
pub type SOURCEBUFFERENABLE4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, LAYERPROPERTY4_SPEC, bool, O>;
impl R {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    pub fn tilemode4(&self) -> TILEMODE4_R {
        TILEMODE4_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    pub fn alphasrcenable4(&self) -> ALPHASRCENABLE4_R {
        ALPHASRCENABLE4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    pub fn alphaconstenable4(&self) -> ALPHACONSTENABLE4_R {
        ALPHACONSTENABLE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    pub fn alphamaskenable4(&self) -> ALPHAMASKENABLE4_R {
        ALPHAMASKENABLE4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    pub fn alphatransenable4(&self) -> ALPHATRANSENABLE4_R {
        ALPHATRANSENABLE4_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    pub fn rgbalphasrcenable4(&self) -> RGBALPHASRCENABLE4_R {
        RGBALPHASRCENABLE4_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    pub fn rgbalphaconstenable4(&self) -> RGBALPHACONSTENABLE4_R {
        RGBALPHACONSTENABLE4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    pub fn rgbalphamaskenable4(&self) -> RGBALPHAMASKENABLE4_R {
        RGBALPHAMASKENABLE4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    pub fn rgbalphatransenable4(&self) -> RGBALPHATRANSENABLE4_R {
        RGBALPHATRANSENABLE4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    pub fn premulconstrgb4(&self) -> PREMULCONSTRGB4_R {
        PREMULCONSTRGB4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    pub fn yuvconversionmode4(&self) -> YUVCONVERSIONMODE4_R {
        YUVCONVERSIONMODE4_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    pub fn gammaremoveenable4(&self) -> GAMMAREMOVEENABLE4_R {
        GAMMAREMOVEENABLE4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    pub fn clipwindowenable4(&self) -> CLIPWINDOWENABLE4_R {
        CLIPWINDOWENABLE4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    pub fn sourcebufferenable4(&self) -> SOURCEBUFFERENABLE4_R {
        SOURCEBUFFERENABLE4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:2 - See TileMode0."]
    #[inline(always)]
    #[must_use]
    pub fn tilemode4(&mut self) -> TILEMODE4_W<1> {
        TILEMODE4_W::new(self)
    }
    #[doc = "Bit 3 - See AlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphasrcenable4(&mut self) -> ALPHASRCENABLE4_W<3> {
        ALPHASRCENABLE4_W::new(self)
    }
    #[doc = "Bit 4 - See AlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphaconstenable4(&mut self) -> ALPHACONSTENABLE4_W<4> {
        ALPHACONSTENABLE4_W::new(self)
    }
    #[doc = "Bit 5 - See AlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamaskenable4(&mut self) -> ALPHAMASKENABLE4_W<5> {
        ALPHAMASKENABLE4_W::new(self)
    }
    #[doc = "Bit 6 - See AlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn alphatransenable4(&mut self) -> ALPHATRANSENABLE4_W<6> {
        ALPHATRANSENABLE4_W::new(self)
    }
    #[doc = "Bit 7 - See RGBAlphaSrcSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphasrcenable4(&mut self) -> RGBALPHASRCENABLE4_W<7> {
        RGBALPHASRCENABLE4_W::new(self)
    }
    #[doc = "Bit 8 - See RGBAlphaConstSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphaconstenable4(&mut self) -> RGBALPHACONSTENABLE4_W<8> {
        RGBALPHACONSTENABLE4_W::new(self)
    }
    #[doc = "Bit 9 - See RGBAlphaMaskSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphamaskenable4(&mut self) -> RGBALPHAMASKENABLE4_W<9> {
        RGBALPHAMASKENABLE4_W::new(self)
    }
    #[doc = "Bit 10 - See RGBAlphaTransSelect0."]
    #[inline(always)]
    #[must_use]
    pub fn rgbalphatransenable4(&mut self) -> RGBALPHATRANSENABLE4_W<10> {
        RGBALPHATRANSENABLE4_W::new(self)
    }
    #[doc = "Bit 11 - See PremulConstRGB0."]
    #[inline(always)]
    #[must_use]
    pub fn premulconstrgb4(&mut self) -> PREMULCONSTRGB4_W<11> {
        PREMULCONSTRGB4_W::new(self)
    }
    #[doc = "Bits 12:13 - See YUVConversionMode0."]
    #[inline(always)]
    #[must_use]
    pub fn yuvconversionmode4(&mut self) -> YUVCONVERSIONMODE4_W<12> {
        YUVCONVERSIONMODE4_W::new(self)
    }
    #[doc = "Bit 14 - See GammaRemoveEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn gammaremoveenable4(&mut self) -> GAMMAREMOVEENABLE4_W<14> {
        GAMMAREMOVEENABLE4_W::new(self)
    }
    #[doc = "Bit 30 - See ClipWindowEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn clipwindowenable4(&mut self) -> CLIPWINDOWENABLE4_W<30> {
        CLIPWINDOWENABLE4_W::new(self)
    }
    #[doc = "Bit 31 - See SourceBufferEnable0."]
    #[inline(always)]
    #[must_use]
    pub fn sourcebufferenable4(&mut self) -> SOURCEBUFFERENABLE4_W<31> {
        SOURCEBUFFERENABLE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common properties of layer 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layerproperty4](index.html) module"]
pub struct LAYERPROPERTY4_SPEC;
impl crate::RegisterSpec for LAYERPROPERTY4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layerproperty4::R](R) reader structure"]
impl crate::Readable for LAYERPROPERTY4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layerproperty4::W](W) writer structure"]
impl crate::Writable for LAYERPROPERTY4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYERPROPERTY4 to value 0x08"]
impl crate::Resettable for LAYERPROPERTY4_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
