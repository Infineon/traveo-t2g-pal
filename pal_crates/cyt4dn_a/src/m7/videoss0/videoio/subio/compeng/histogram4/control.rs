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
#[doc = "Field `CLIP_EN` reader - Switch for clip window. If enabled all pixels outside the clip window are not taken into account."]
pub type CLIP_EN_R = crate::BitReader<CLIP_EN_A>;
#[doc = "Switch for clip window. If enabled all pixels outside the clip window are not taken into account.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLIP_EN_A {
    #[doc = "0: Clip window is switched off."]
    DISABLE = 0,
    #[doc = "1: Clip window is switched on."]
    ENABLE = 1,
}
impl From<CLIP_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CLIP_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CLIP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLIP_EN_A {
        match self.bits {
            false => CLIP_EN_A::DISABLE,
            true => CLIP_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CLIP_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CLIP_EN_A::ENABLE
    }
}
#[doc = "Field `CLIP_EN` writer - Switch for clip window. If enabled all pixels outside the clip window are not taken into account."]
pub type CLIP_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, CLIP_EN_A, O>;
impl<'a, const O: u8> CLIP_EN_W<'a, O> {
    #[doc = "Clip window is switched off."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CLIP_EN_A::DISABLE)
    }
    #[doc = "Clip window is switched on."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CLIP_EN_A::ENABLE)
    }
}
#[doc = "Field `COLOR_SPACE_MODE` reader - Assuming that YUV(YCrCb) color components according to BT.601 or BT.709 are provided at histogram input the appropriate conversion to RGB components in \\[0,255\\]
range can be selected here."]
pub type COLOR_SPACE_MODE_R = crate::FieldReader<u8, COLOR_SPACE_MODE_A>;
#[doc = "Assuming that YUV(YCrCb) color components according to BT.601 or BT.709 are provided at histogram input the appropriate conversion to RGB components in \\[0,255\\]
range can be selected here.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COLOR_SPACE_MODE_A {
    #[doc = "0: No color space conversion of input components is applied."]
    NO_CONV = 0,
    #[doc = "1: YUV input components as described in BT.601 are converted to RGB in \\[0,255\\]
range."]
    YUV2RGB_BT601 = 1,
    #[doc = "2: YUV input components as described in BT.709 are converted to RGB in \\[0,255\\]
range."]
    YUV2RGB_BT709 = 2,
}
impl From<COLOR_SPACE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: COLOR_SPACE_MODE_A) -> Self {
        variant as _
    }
}
impl COLOR_SPACE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COLOR_SPACE_MODE_A> {
        match self.bits {
            0 => Some(COLOR_SPACE_MODE_A::NO_CONV),
            1 => Some(COLOR_SPACE_MODE_A::YUV2RGB_BT601),
            2 => Some(COLOR_SPACE_MODE_A::YUV2RGB_BT709),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONV`"]
    #[inline(always)]
    pub fn is_no_conv(&self) -> bool {
        *self == COLOR_SPACE_MODE_A::NO_CONV
    }
    #[doc = "Checks if the value of the field is `YUV2RGB_BT601`"]
    #[inline(always)]
    pub fn is_yuv2rgb_bt601(&self) -> bool {
        *self == COLOR_SPACE_MODE_A::YUV2RGB_BT601
    }
    #[doc = "Checks if the value of the field is `YUV2RGB_BT709`"]
    #[inline(always)]
    pub fn is_yuv2rgb_bt709(&self) -> bool {
        *self == COLOR_SPACE_MODE_A::YUV2RGB_BT709
    }
}
#[doc = "Field `COLOR_SPACE_MODE` writer - Assuming that YUV(YCrCb) color components according to BT.601 or BT.709 are provided at histogram input the appropriate conversion to RGB components in \\[0,255\\]
range can be selected here."]
pub type COLOR_SPACE_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, COLOR_SPACE_MODE_A, 2, O>;
impl<'a, const O: u8> COLOR_SPACE_MODE_W<'a, O> {
    #[doc = "No color space conversion of input components is applied."]
    #[inline(always)]
    pub fn no_conv(self) -> &'a mut W {
        self.variant(COLOR_SPACE_MODE_A::NO_CONV)
    }
    #[doc = "YUV input components as described in BT.601 are converted to RGB in \\[0,255\\]
range."]
    #[inline(always)]
    pub fn yuv2rgb_bt601(self) -> &'a mut W {
        self.variant(COLOR_SPACE_MODE_A::YUV2RGB_BT601)
    }
    #[doc = "YUV input components as described in BT.709 are converted to RGB in \\[0,255\\]
range."]
    #[inline(always)]
    pub fn yuv2rgb_bt709(self) -> &'a mut W {
        self.variant(COLOR_SPACE_MODE_A::YUV2RGB_BT709)
    }
}
#[doc = "Field `LUM_MODE` reader - If a histogram for luma or luminance is required, the conversion formula for luma (Y component according to BT.601) or luminance (Y component according to BT.709) can be selected here. Luma or luminance calculation should only be applied if RGB components in range \\[0..255\\]
are delivered directly at histogram input or when YCrCb components are converted to RGB by the appropriate setting of register field color_space_mode. Luma or luminance counting is delivered in first (component0) histogram."]
pub type LUM_MODE_R = crate::FieldReader<u8, LUM_MODE_A>;
#[doc = "If a histogram for luma or luminance is required, the conversion formula for luma (Y component according to BT.601) or luminance (Y component according to BT.709) can be selected here. Luma or luminance calculation should only be applied if RGB components in range \\[0..255\\]
are delivered directly at histogram input or when YCrCb components are converted to RGB by the appropriate setting of register field color_space_mode. Luma or luminance counting is delivered in first (component0) histogram.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LUM_MODE_A {
    #[doc = "0: Component0 for first histogram is counted as it is."]
    NO_CONV = 0,
    #[doc = "1: Component0 for first histogram is replaced by luma, which is calculated from RGB input components according to BT.601."]
    LUMA = 1,
    #[doc = "2: Component0 for first histogram is replaced by luminance, which is calculated from RGB input components according to BT.709."]
    LUMINANCE = 2,
}
impl From<LUM_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LUM_MODE_A) -> Self {
        variant as _
    }
}
impl LUM_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LUM_MODE_A> {
        match self.bits {
            0 => Some(LUM_MODE_A::NO_CONV),
            1 => Some(LUM_MODE_A::LUMA),
            2 => Some(LUM_MODE_A::LUMINANCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONV`"]
    #[inline(always)]
    pub fn is_no_conv(&self) -> bool {
        *self == LUM_MODE_A::NO_CONV
    }
    #[doc = "Checks if the value of the field is `LUMA`"]
    #[inline(always)]
    pub fn is_luma(&self) -> bool {
        *self == LUM_MODE_A::LUMA
    }
    #[doc = "Checks if the value of the field is `LUMINANCE`"]
    #[inline(always)]
    pub fn is_luminance(&self) -> bool {
        *self == LUM_MODE_A::LUMINANCE
    }
}
#[doc = "Field `LUM_MODE` writer - If a histogram for luma or luminance is required, the conversion formula for luma (Y component according to BT.601) or luminance (Y component according to BT.709) can be selected here. Luma or luminance calculation should only be applied if RGB components in range \\[0..255\\]
are delivered directly at histogram input or when YCrCb components are converted to RGB by the appropriate setting of register field color_space_mode. Luma or luminance counting is delivered in first (component0) histogram."]
pub type LUM_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, LUM_MODE_A, 2, O>;
impl<'a, const O: u8> LUM_MODE_W<'a, O> {
    #[doc = "Component0 for first histogram is counted as it is."]
    #[inline(always)]
    pub fn no_conv(self) -> &'a mut W {
        self.variant(LUM_MODE_A::NO_CONV)
    }
    #[doc = "Component0 for first histogram is replaced by luma, which is calculated from RGB input components according to BT.601."]
    #[inline(always)]
    pub fn luma(self) -> &'a mut W {
        self.variant(LUM_MODE_A::LUMA)
    }
    #[doc = "Component0 for first histogram is replaced by luminance, which is calculated from RGB input components according to BT.709."]
    #[inline(always)]
    pub fn luminance(self) -> &'a mut W {
        self.variant(LUM_MODE_A::LUMINANCE)
    }
}
#[doc = "Field `ALPHAMASK` reader - Value 1 enables the alpha mask mode. In this mode only pixels with an alpha value larger than 0.5 are affecting the histogram bins."]
pub type ALPHAMASK_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK` writer - Value 1 enables the alpha mask mode. In this mode only pixels with an alpha value larger than 0.5 are affecting the histogram bins."]
pub type ALPHAMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `ALPHAINVERT` reader - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value smaller or equal 0.5 are affecting the histogram bins)."]
pub type ALPHAINVERT_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINVERT` writer - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value smaller or equal 0.5 are affecting the histogram bins)."]
pub type ALPHAINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Switch for clip window. If enabled all pixels outside the clip window are not taken into account."]
    #[inline(always)]
    pub fn clip_en(&self) -> CLIP_EN_R {
        CLIP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Assuming that YUV(YCrCb) color components according to BT.601 or BT.709 are provided at histogram input the appropriate conversion to RGB components in \\[0,255\\]
range can be selected here."]
    #[inline(always)]
    pub fn color_space_mode(&self) -> COLOR_SPACE_MODE_R {
        COLOR_SPACE_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - If a histogram for luma or luminance is required, the conversion formula for luma (Y component according to BT.601) or luminance (Y component according to BT.709) can be selected here. Luma or luminance calculation should only be applied if RGB components in range \\[0..255\\]
are delivered directly at histogram input or when YCrCb components are converted to RGB by the appropriate setting of register field color_space_mode. Luma or luminance counting is delivered in first (component0) histogram."]
    #[inline(always)]
    pub fn lum_mode(&self) -> LUM_MODE_R {
        LUM_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Value 1 enables the alpha mask mode. In this mode only pixels with an alpha value larger than 0.5 are affecting the histogram bins."]
    #[inline(always)]
    pub fn alphamask(&self) -> ALPHAMASK_R {
        ALPHAMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value smaller or equal 0.5 are affecting the histogram bins)."]
    #[inline(always)]
    pub fn alphainvert(&self) -> ALPHAINVERT_R {
        ALPHAINVERT_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Switch for clip window. If enabled all pixels outside the clip window are not taken into account."]
    #[inline(always)]
    #[must_use]
    pub fn clip_en(&mut self) -> CLIP_EN_W<0> {
        CLIP_EN_W::new(self)
    }
    #[doc = "Bits 4:5 - Assuming that YUV(YCrCb) color components according to BT.601 or BT.709 are provided at histogram input the appropriate conversion to RGB components in \\[0,255\\]
range can be selected here."]
    #[inline(always)]
    #[must_use]
    pub fn color_space_mode(&mut self) -> COLOR_SPACE_MODE_W<4> {
        COLOR_SPACE_MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - If a histogram for luma or luminance is required, the conversion formula for luma (Y component according to BT.601) or luminance (Y component according to BT.709) can be selected here. Luma or luminance calculation should only be applied if RGB components in range \\[0..255\\]
are delivered directly at histogram input or when YCrCb components are converted to RGB by the appropriate setting of register field color_space_mode. Luma or luminance counting is delivered in first (component0) histogram."]
    #[inline(always)]
    #[must_use]
    pub fn lum_mode(&mut self) -> LUM_MODE_W<8> {
        LUM_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Value 1 enables the alpha mask mode. In this mode only pixels with an alpha value larger than 0.5 are affecting the histogram bins."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask(&mut self) -> ALPHAMASK_W<12> {
        ALPHAMASK_W::new(self)
    }
    #[doc = "Bit 13 - Value 1 inverts the effect of the alpha mask mode when enabled (pixels with alpha value smaller or equal 0.5 are affecting the histogram bins)."]
    #[inline(always)]
    #[must_use]
    pub fn alphainvert(&mut self) -> ALPHAINVERT_W<13> {
        ALPHAINVERT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls some settings concerning input components and alpha mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
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
