#[doc = "Register `DITHERCONTROL` reader"]
pub struct R(crate::R<DITHERCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DITHERCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DITHERCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DITHERCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DITHERCONTROL` writer"]
pub struct W(crate::W<DITHERCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DITHERCONTROL_SPEC>;
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
impl From<crate::W<DITHERCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DITHERCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE_RANGE_SELECT` reader - Mode which sets the reduction of component widths."]
pub type BLUE_RANGE_SELECT_R = crate::FieldReader<u8, BLUE_RANGE_SELECT_A>;
#[doc = "Mode which sets the reduction of component widths.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLUE_RANGE_SELECT_A {
    #[doc = "2: Reduces blue component width from 10 bit to 8bit."]
    BLUE_10TO8 = 2,
    #[doc = "3: Reduces blue component width from 10 bit to 7bit."]
    BLUE_10TO7 = 3,
    #[doc = "4: Reduces blue component width from 10 bit to 6bit."]
    BLUE_10TO6 = 4,
    #[doc = "5: Reduces blue component width from 10 bit to 5bit."]
    BLUE_10TO5 = 5,
}
impl From<BLUE_RANGE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BLUE_RANGE_SELECT_A) -> Self {
        variant as _
    }
}
impl BLUE_RANGE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLUE_RANGE_SELECT_A> {
        match self.bits {
            2 => Some(BLUE_RANGE_SELECT_A::BLUE_10TO8),
            3 => Some(BLUE_RANGE_SELECT_A::BLUE_10TO7),
            4 => Some(BLUE_RANGE_SELECT_A::BLUE_10TO6),
            5 => Some(BLUE_RANGE_SELECT_A::BLUE_10TO5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLUE_10TO8`"]
    #[inline(always)]
    pub fn is_blue_10to8(&self) -> bool {
        *self == BLUE_RANGE_SELECT_A::BLUE_10TO8
    }
    #[doc = "Checks if the value of the field is `BLUE_10TO7`"]
    #[inline(always)]
    pub fn is_blue_10to7(&self) -> bool {
        *self == BLUE_RANGE_SELECT_A::BLUE_10TO7
    }
    #[doc = "Checks if the value of the field is `BLUE_10TO6`"]
    #[inline(always)]
    pub fn is_blue_10to6(&self) -> bool {
        *self == BLUE_RANGE_SELECT_A::BLUE_10TO6
    }
    #[doc = "Checks if the value of the field is `BLUE_10TO5`"]
    #[inline(always)]
    pub fn is_blue_10to5(&self) -> bool {
        *self == BLUE_RANGE_SELECT_A::BLUE_10TO5
    }
}
#[doc = "Field `BLUE_RANGE_SELECT` writer - Mode which sets the reduction of component widths."]
pub type BLUE_RANGE_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DITHERCONTROL_SPEC, u8, BLUE_RANGE_SELECT_A, 3, O>;
impl<'a, const O: u8> BLUE_RANGE_SELECT_W<'a, O> {
    #[doc = "Reduces blue component width from 10 bit to 8bit."]
    #[inline(always)]
    pub fn blue_10to8(self) -> &'a mut W {
        self.variant(BLUE_RANGE_SELECT_A::BLUE_10TO8)
    }
    #[doc = "Reduces blue component width from 10 bit to 7bit."]
    #[inline(always)]
    pub fn blue_10to7(self) -> &'a mut W {
        self.variant(BLUE_RANGE_SELECT_A::BLUE_10TO7)
    }
    #[doc = "Reduces blue component width from 10 bit to 6bit."]
    #[inline(always)]
    pub fn blue_10to6(self) -> &'a mut W {
        self.variant(BLUE_RANGE_SELECT_A::BLUE_10TO6)
    }
    #[doc = "Reduces blue component width from 10 bit to 5bit."]
    #[inline(always)]
    pub fn blue_10to5(self) -> &'a mut W {
        self.variant(BLUE_RANGE_SELECT_A::BLUE_10TO5)
    }
}
#[doc = "Field `GREEN_RANGE_SELECT` reader - Mode which sets the reduction of component widths."]
pub type GREEN_RANGE_SELECT_R = crate::FieldReader<u8, GREEN_RANGE_SELECT_A>;
#[doc = "Mode which sets the reduction of component widths.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GREEN_RANGE_SELECT_A {
    #[doc = "2: Reduces green component width from 10 bit to 8bit."]
    GREEN_10TO8 = 2,
    #[doc = "3: Reduces green component width from 10 bit to 7bit."]
    GREEN_10TO7 = 3,
    #[doc = "4: Reduces green component width from 10 bit to 6bit."]
    GREEN_10TO6 = 4,
    #[doc = "5: Reduces green component width from 10 bit to 5bit."]
    GREEN_10TO5 = 5,
}
impl From<GREEN_RANGE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: GREEN_RANGE_SELECT_A) -> Self {
        variant as _
    }
}
impl GREEN_RANGE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GREEN_RANGE_SELECT_A> {
        match self.bits {
            2 => Some(GREEN_RANGE_SELECT_A::GREEN_10TO8),
            3 => Some(GREEN_RANGE_SELECT_A::GREEN_10TO7),
            4 => Some(GREEN_RANGE_SELECT_A::GREEN_10TO6),
            5 => Some(GREEN_RANGE_SELECT_A::GREEN_10TO5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GREEN_10TO8`"]
    #[inline(always)]
    pub fn is_green_10to8(&self) -> bool {
        *self == GREEN_RANGE_SELECT_A::GREEN_10TO8
    }
    #[doc = "Checks if the value of the field is `GREEN_10TO7`"]
    #[inline(always)]
    pub fn is_green_10to7(&self) -> bool {
        *self == GREEN_RANGE_SELECT_A::GREEN_10TO7
    }
    #[doc = "Checks if the value of the field is `GREEN_10TO6`"]
    #[inline(always)]
    pub fn is_green_10to6(&self) -> bool {
        *self == GREEN_RANGE_SELECT_A::GREEN_10TO6
    }
    #[doc = "Checks if the value of the field is `GREEN_10TO5`"]
    #[inline(always)]
    pub fn is_green_10to5(&self) -> bool {
        *self == GREEN_RANGE_SELECT_A::GREEN_10TO5
    }
}
#[doc = "Field `GREEN_RANGE_SELECT` writer - Mode which sets the reduction of component widths."]
pub type GREEN_RANGE_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DITHERCONTROL_SPEC, u8, GREEN_RANGE_SELECT_A, 3, O>;
impl<'a, const O: u8> GREEN_RANGE_SELECT_W<'a, O> {
    #[doc = "Reduces green component width from 10 bit to 8bit."]
    #[inline(always)]
    pub fn green_10to8(self) -> &'a mut W {
        self.variant(GREEN_RANGE_SELECT_A::GREEN_10TO8)
    }
    #[doc = "Reduces green component width from 10 bit to 7bit."]
    #[inline(always)]
    pub fn green_10to7(self) -> &'a mut W {
        self.variant(GREEN_RANGE_SELECT_A::GREEN_10TO7)
    }
    #[doc = "Reduces green component width from 10 bit to 6bit."]
    #[inline(always)]
    pub fn green_10to6(self) -> &'a mut W {
        self.variant(GREEN_RANGE_SELECT_A::GREEN_10TO6)
    }
    #[doc = "Reduces green component width from 10 bit to 5bit."]
    #[inline(always)]
    pub fn green_10to5(self) -> &'a mut W {
        self.variant(GREEN_RANGE_SELECT_A::GREEN_10TO5)
    }
}
#[doc = "Field `RED_RANGE_SELECT` reader - Mode which sets the reduction of component widths."]
pub type RED_RANGE_SELECT_R = crate::FieldReader<u8, RED_RANGE_SELECT_A>;
#[doc = "Mode which sets the reduction of component widths.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RED_RANGE_SELECT_A {
    #[doc = "2: Reduces red component width from 10 bit to 8bit."]
    RED_10TO8 = 2,
    #[doc = "3: Reduces red component width from 10 bit to 7bit."]
    RED_10TO7 = 3,
    #[doc = "4: Reduces red component width from 10 bit to 6bit."]
    RED_10TO6 = 4,
    #[doc = "5: Reduces red component width from 10 bit to 5bit."]
    RED_10TO5 = 5,
}
impl From<RED_RANGE_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RED_RANGE_SELECT_A) -> Self {
        variant as _
    }
}
impl RED_RANGE_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RED_RANGE_SELECT_A> {
        match self.bits {
            2 => Some(RED_RANGE_SELECT_A::RED_10TO8),
            3 => Some(RED_RANGE_SELECT_A::RED_10TO7),
            4 => Some(RED_RANGE_SELECT_A::RED_10TO6),
            5 => Some(RED_RANGE_SELECT_A::RED_10TO5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RED_10TO8`"]
    #[inline(always)]
    pub fn is_red_10to8(&self) -> bool {
        *self == RED_RANGE_SELECT_A::RED_10TO8
    }
    #[doc = "Checks if the value of the field is `RED_10TO7`"]
    #[inline(always)]
    pub fn is_red_10to7(&self) -> bool {
        *self == RED_RANGE_SELECT_A::RED_10TO7
    }
    #[doc = "Checks if the value of the field is `RED_10TO6`"]
    #[inline(always)]
    pub fn is_red_10to6(&self) -> bool {
        *self == RED_RANGE_SELECT_A::RED_10TO6
    }
    #[doc = "Checks if the value of the field is `RED_10TO5`"]
    #[inline(always)]
    pub fn is_red_10to5(&self) -> bool {
        *self == RED_RANGE_SELECT_A::RED_10TO5
    }
}
#[doc = "Field `RED_RANGE_SELECT` writer - Mode which sets the reduction of component widths."]
pub type RED_RANGE_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DITHERCONTROL_SPEC, u8, RED_RANGE_SELECT_A, 3, O>;
impl<'a, const O: u8> RED_RANGE_SELECT_W<'a, O> {
    #[doc = "Reduces red component width from 10 bit to 8bit."]
    #[inline(always)]
    pub fn red_10to8(self) -> &'a mut W {
        self.variant(RED_RANGE_SELECT_A::RED_10TO8)
    }
    #[doc = "Reduces red component width from 10 bit to 7bit."]
    #[inline(always)]
    pub fn red_10to7(self) -> &'a mut W {
        self.variant(RED_RANGE_SELECT_A::RED_10TO7)
    }
    #[doc = "Reduces red component width from 10 bit to 6bit."]
    #[inline(always)]
    pub fn red_10to6(self) -> &'a mut W {
        self.variant(RED_RANGE_SELECT_A::RED_10TO6)
    }
    #[doc = "Reduces red component width from 10 bit to 5bit."]
    #[inline(always)]
    pub fn red_10to5(self) -> &'a mut W {
        self.variant(RED_RANGE_SELECT_A::RED_10TO5)
    }
}
#[doc = "Field `OFFSET_SELECT` reader - Selects the method how the dither offset is calculated."]
pub type OFFSET_SELECT_R = crate::BitReader<OFFSET_SELECT_A>;
#[doc = "Selects the method how the dither offset is calculated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFSET_SELECT_A {
    #[doc = "0: Offset is a bayer matrix value, which is selected according to pixel frame position."]
    OFFS_SPATIAL = 0,
    #[doc = "1: Offset is the sum from a bayer matrix value, which is selected according to pixel frame position, and a value from a regular sequence, which changes each frame."]
    OFFS_TEMPORAL = 1,
}
impl From<OFFSET_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OFFSET_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFSET_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFSET_SELECT_A {
        match self.bits {
            false => OFFSET_SELECT_A::OFFS_SPATIAL,
            true => OFFSET_SELECT_A::OFFS_TEMPORAL,
        }
    }
    #[doc = "Checks if the value of the field is `OFFS_SPATIAL`"]
    #[inline(always)]
    pub fn is_offs_spatial(&self) -> bool {
        *self == OFFSET_SELECT_A::OFFS_SPATIAL
    }
    #[doc = "Checks if the value of the field is `OFFS_TEMPORAL`"]
    #[inline(always)]
    pub fn is_offs_temporal(&self) -> bool {
        *self == OFFSET_SELECT_A::OFFS_TEMPORAL
    }
}
#[doc = "Field `OFFSET_SELECT` writer - Selects the method how the dither offset is calculated."]
pub type OFFSET_SELECT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DITHERCONTROL_SPEC, OFFSET_SELECT_A, O>;
impl<'a, const O: u8> OFFSET_SELECT_W<'a, O> {
    #[doc = "Offset is a bayer matrix value, which is selected according to pixel frame position."]
    #[inline(always)]
    pub fn offs_spatial(self) -> &'a mut W {
        self.variant(OFFSET_SELECT_A::OFFS_SPATIAL)
    }
    #[doc = "Offset is the sum from a bayer matrix value, which is selected according to pixel frame position, and a value from a regular sequence, which changes each frame."]
    #[inline(always)]
    pub fn offs_temporal(self) -> &'a mut W {
        self.variant(OFFSET_SELECT_A::OFFS_TEMPORAL)
    }
}
#[doc = "Field `ALGO_SELECT` reader - The number of output colors that can virtually be displayed by dithering is slightly lower than the number of physical input colors. This field selects how the mapping is done."]
pub type ALGO_SELECT_R = crate::FieldReader<u8, ALGO_SELECT_A>;
#[doc = "The number of output colors that can virtually be displayed by dithering is slightly lower than the number of physical input colors. This field selects how the mapping is done.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALGO_SELECT_A {
    #[doc = "1: Best possible resolution for most dark colors. Adds a diminutive offset to overall image brightness."]
    NO_CORRECTION = 1,
    #[doc = "2: Preserves overall image brightness. Cannot resolve most dark and most bright colors. All codes in-between are distributed perfectly smooth."]
    BRIGHTNESS_CORRECTION = 2,
    #[doc = "3: Preserves overall image brightness. Best possible distribution of color codes over complete range."]
    CONTRAST_CORRECTION = 3,
}
impl From<ALGO_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ALGO_SELECT_A) -> Self {
        variant as _
    }
}
impl ALGO_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALGO_SELECT_A> {
        match self.bits {
            1 => Some(ALGO_SELECT_A::NO_CORRECTION),
            2 => Some(ALGO_SELECT_A::BRIGHTNESS_CORRECTION),
            3 => Some(ALGO_SELECT_A::CONTRAST_CORRECTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CORRECTION`"]
    #[inline(always)]
    pub fn is_no_correction(&self) -> bool {
        *self == ALGO_SELECT_A::NO_CORRECTION
    }
    #[doc = "Checks if the value of the field is `BRIGHTNESS_CORRECTION`"]
    #[inline(always)]
    pub fn is_brightness_correction(&self) -> bool {
        *self == ALGO_SELECT_A::BRIGHTNESS_CORRECTION
    }
    #[doc = "Checks if the value of the field is `CONTRAST_CORRECTION`"]
    #[inline(always)]
    pub fn is_contrast_correction(&self) -> bool {
        *self == ALGO_SELECT_A::CONTRAST_CORRECTION
    }
}
#[doc = "Field `ALGO_SELECT` writer - The number of output colors that can virtually be displayed by dithering is slightly lower than the number of physical input colors. This field selects how the mapping is done."]
pub type ALGO_SELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DITHERCONTROL_SPEC, u8, ALGO_SELECT_A, 2, O>;
impl<'a, const O: u8> ALGO_SELECT_W<'a, O> {
    #[doc = "Best possible resolution for most dark colors. Adds a diminutive offset to overall image brightness."]
    #[inline(always)]
    pub fn no_correction(self) -> &'a mut W {
        self.variant(ALGO_SELECT_A::NO_CORRECTION)
    }
    #[doc = "Preserves overall image brightness. Cannot resolve most dark and most bright colors. All codes in-between are distributed perfectly smooth."]
    #[inline(always)]
    pub fn brightness_correction(self) -> &'a mut W {
        self.variant(ALGO_SELECT_A::BRIGHTNESS_CORRECTION)
    }
    #[doc = "Preserves overall image brightness. Best possible distribution of color codes over complete range."]
    #[inline(always)]
    pub fn contrast_correction(self) -> &'a mut W {
        self.variant(ALGO_SELECT_A::CONTRAST_CORRECTION)
    }
}
#[doc = "Field `ALPHA_MODE` reader - Enables/disables that dithering can be switched by alpha bit."]
pub type ALPHA_MODE_R = crate::FieldReader<u8, ALPHA_MODE_A>;
#[doc = "Enables/disables that dithering can be switched by alpha bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALPHA_MODE_A {
    #[doc = "0: The alpha bit is not considered."]
    DISABLE = 0,
    #[doc = "1: Red, green and blue components are only dithered, if the alpha bit is 1."]
    ENABLE_BY1 = 1,
    #[doc = "2: Red, green and blue components are only dithered, if the alpha bit is 0."]
    ENABLE_BY0 = 2,
}
impl From<ALPHA_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ALPHA_MODE_A) -> Self {
        variant as _
    }
}
impl ALPHA_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALPHA_MODE_A> {
        match self.bits {
            0 => Some(ALPHA_MODE_A::DISABLE),
            1 => Some(ALPHA_MODE_A::ENABLE_BY1),
            2 => Some(ALPHA_MODE_A::ENABLE_BY0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALPHA_MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE_BY1`"]
    #[inline(always)]
    pub fn is_enable_by1(&self) -> bool {
        *self == ALPHA_MODE_A::ENABLE_BY1
    }
    #[doc = "Checks if the value of the field is `ENABLE_BY0`"]
    #[inline(always)]
    pub fn is_enable_by0(&self) -> bool {
        *self == ALPHA_MODE_A::ENABLE_BY0
    }
}
#[doc = "Field `ALPHA_MODE` writer - Enables/disables that dithering can be switched by alpha bit."]
pub type ALPHA_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DITHERCONTROL_SPEC, u8, ALPHA_MODE_A, 2, O>;
impl<'a, const O: u8> ALPHA_MODE_W<'a, O> {
    #[doc = "The alpha bit is not considered."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALPHA_MODE_A::DISABLE)
    }
    #[doc = "Red, green and blue components are only dithered, if the alpha bit is 1."]
    #[inline(always)]
    pub fn enable_by1(self) -> &'a mut W {
        self.variant(ALPHA_MODE_A::ENABLE_BY1)
    }
    #[doc = "Red, green and blue components are only dithered, if the alpha bit is 0."]
    #[inline(always)]
    pub fn enable_by0(self) -> &'a mut W {
        self.variant(ALPHA_MODE_A::ENABLE_BY0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    pub fn blue_range_select(&self) -> BLUE_RANGE_SELECT_R {
        BLUE_RANGE_SELECT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    pub fn green_range_select(&self) -> GREEN_RANGE_SELECT_R {
        GREEN_RANGE_SELECT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    pub fn red_range_select(&self) -> RED_RANGE_SELECT_R {
        RED_RANGE_SELECT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - Selects the method how the dither offset is calculated."]
    #[inline(always)]
    pub fn offset_select(&self) -> OFFSET_SELECT_R {
        OFFSET_SELECT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - The number of output colors that can virtually be displayed by dithering is slightly lower than the number of physical input colors. This field selects how the mapping is done."]
    #[inline(always)]
    pub fn algo_select(&self) -> ALGO_SELECT_R {
        ALGO_SELECT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables/disables that dithering can be switched by alpha bit."]
    #[inline(always)]
    pub fn alpha_mode(&self) -> ALPHA_MODE_R {
        ALPHA_MODE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    #[must_use]
    pub fn blue_range_select(&mut self) -> BLUE_RANGE_SELECT_W<0> {
        BLUE_RANGE_SELECT_W::new(self)
    }
    #[doc = "Bits 4:6 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    #[must_use]
    pub fn green_range_select(&mut self) -> GREEN_RANGE_SELECT_W<4> {
        GREEN_RANGE_SELECT_W::new(self)
    }
    #[doc = "Bits 8:10 - Mode which sets the reduction of component widths."]
    #[inline(always)]
    #[must_use]
    pub fn red_range_select(&mut self) -> RED_RANGE_SELECT_W<8> {
        RED_RANGE_SELECT_W::new(self)
    }
    #[doc = "Bit 16 - Selects the method how the dither offset is calculated."]
    #[inline(always)]
    #[must_use]
    pub fn offset_select(&mut self) -> OFFSET_SELECT_W<16> {
        OFFSET_SELECT_W::new(self)
    }
    #[doc = "Bits 20:21 - The number of output colors that can virtually be displayed by dithering is slightly lower than the number of physical input colors. This field selects how the mapping is done."]
    #[inline(always)]
    #[must_use]
    pub fn algo_select(&mut self) -> ALGO_SELECT_W<20> {
        ALGO_SELECT_W::new(self)
    }
    #[doc = "Bits 24:25 - Enables/disables that dithering can be switched by alpha bit."]
    #[inline(always)]
    #[must_use]
    pub fn alpha_mode(&mut self) -> ALPHA_MODE_W<24> {
        ALPHA_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither Unit processing control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dithercontrol](index.html) module"]
pub struct DITHERCONTROL_SPEC;
impl crate::RegisterSpec for DITHERCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dithercontrol::R](R) reader structure"]
impl crate::Readable for DITHERCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dithercontrol::W](W) writer structure"]
impl crate::Writable for DITHERCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DITHERCONTROL to value 0x0030_0222"]
impl crate::Resettable for DITHERCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_0222;
}
