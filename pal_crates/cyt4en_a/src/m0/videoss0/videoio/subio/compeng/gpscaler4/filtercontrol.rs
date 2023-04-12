#[doc = "Register `FILTERCONTROL` reader"]
pub struct R(crate::R<FILTERCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTERCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTERCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTERCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTERCONTROL` writer"]
pub struct W(crate::W<FILTERCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTERCONTROL_SPEC>;
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
impl From<crate::W<FILTERCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTERCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAPS_H` reader - Defines the horizontal size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter width - 1)/2 (format is unsigned integer)"]
pub type TAPS_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAPS_H` writer - Defines the horizontal size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter width - 1)/2 (format is unsigned integer)"]
pub type TAPS_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTERCONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `TAPS_V` reader - Defines the vertical size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter height - 1)/2 (format is unsigned integer)"]
pub type TAPS_V_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAPS_V` writer - Defines the vertical size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter height - 1)/2 (format is unsigned integer)"]
pub type TAPS_V_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTERCONTROL_SPEC, u8, u8, 3, O>;
#[doc = "Field `BYPASS_MODE_R` reader - Bypass configuration for red color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_R_R = crate::BitReader<BYPASS_MODE_R_A>;
#[doc = "Bypass configuration for red color components of every pixel. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_MODE_R_A {
    #[doc = "0: Red color is filtered."]
    NEUTRAL = 0,
    #[doc = "1: Red color is not filtered, but is written to the line buffer."]
    ACTIVE = 1,
}
impl From<BYPASS_MODE_R_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_MODE_R_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_MODE_R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_MODE_R_A {
        match self.bits {
            false => BYPASS_MODE_R_A::NEUTRAL,
            true => BYPASS_MODE_R_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == BYPASS_MODE_R_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BYPASS_MODE_R_A::ACTIVE
    }
}
#[doc = "Field `BYPASS_MODE_R` writer - Bypass configuration for red color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, BYPASS_MODE_R_A, O>;
impl<'a, const O: u8> BYPASS_MODE_R_W<'a, O> {
    #[doc = "Red color is filtered."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(BYPASS_MODE_R_A::NEUTRAL)
    }
    #[doc = "Red color is not filtered, but is written to the line buffer."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BYPASS_MODE_R_A::ACTIVE)
    }
}
#[doc = "Field `BYPASS_MODE_G` reader - Bypass configuration for green color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_G_R = crate::BitReader<BYPASS_MODE_G_A>;
#[doc = "Bypass configuration for green color components of every pixel. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_MODE_G_A {
    #[doc = "0: Green color is filtered."]
    NEUTRAL = 0,
    #[doc = "1: Green color is not filtered, but is written to the line buffer."]
    ACTIVE = 1,
}
impl From<BYPASS_MODE_G_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_MODE_G_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_MODE_G_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_MODE_G_A {
        match self.bits {
            false => BYPASS_MODE_G_A::NEUTRAL,
            true => BYPASS_MODE_G_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == BYPASS_MODE_G_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BYPASS_MODE_G_A::ACTIVE
    }
}
#[doc = "Field `BYPASS_MODE_G` writer - Bypass configuration for green color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_G_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, BYPASS_MODE_G_A, O>;
impl<'a, const O: u8> BYPASS_MODE_G_W<'a, O> {
    #[doc = "Green color is filtered."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(BYPASS_MODE_G_A::NEUTRAL)
    }
    #[doc = "Green color is not filtered, but is written to the line buffer."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BYPASS_MODE_G_A::ACTIVE)
    }
}
#[doc = "Field `BYPASS_MODE_B` reader - Bypass configuration for blue color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_B_R = crate::BitReader<BYPASS_MODE_B_A>;
#[doc = "Bypass configuration for blue color components of every pixel. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_MODE_B_A {
    #[doc = "0: Blue color is filtered."]
    NEUTRAL = 0,
    #[doc = "1: Blue color is not filtered, but is written to the line buffer."]
    ACTIVE = 1,
}
impl From<BYPASS_MODE_B_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_MODE_B_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_MODE_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_MODE_B_A {
        match self.bits {
            false => BYPASS_MODE_B_A::NEUTRAL,
            true => BYPASS_MODE_B_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == BYPASS_MODE_B_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BYPASS_MODE_B_A::ACTIVE
    }
}
#[doc = "Field `BYPASS_MODE_B` writer - Bypass configuration for blue color components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, BYPASS_MODE_B_A, O>;
impl<'a, const O: u8> BYPASS_MODE_B_W<'a, O> {
    #[doc = "Blue color is filtered."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(BYPASS_MODE_B_A::NEUTRAL)
    }
    #[doc = "Blue color is not filtered, but is written to the line buffer."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BYPASS_MODE_B_A::ACTIVE)
    }
}
#[doc = "Field `BYPASS_MODE_A` reader - Bypass configuration for alpha components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_A_R = crate::BitReader<BYPASS_MODE_A_A>;
#[doc = "Bypass configuration for alpha components of every pixel. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPASS_MODE_A_A {
    #[doc = "0: Alpha is filtered."]
    NEUTRAL = 0,
    #[doc = "1: Alpha is not filtered, but is written to the line buffer."]
    ACTIVE = 1,
}
impl From<BYPASS_MODE_A_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_MODE_A_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPASS_MODE_A_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_MODE_A_A {
        match self.bits {
            false => BYPASS_MODE_A_A::NEUTRAL,
            true => BYPASS_MODE_A_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == BYPASS_MODE_A_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BYPASS_MODE_A_A::ACTIVE
    }
}
#[doc = "Field `BYPASS_MODE_A` writer - Bypass configuration for alpha components of every pixel. (format is unsigned integer)"]
pub type BYPASS_MODE_A_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, BYPASS_MODE_A_A, O>;
impl<'a, const O: u8> BYPASS_MODE_A_W<'a, O> {
    #[doc = "Alpha is filtered."]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(BYPASS_MODE_A_A::NEUTRAL)
    }
    #[doc = "Alpha is not filtered, but is written to the line buffer."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BYPASS_MODE_A_A::ACTIVE)
    }
}
#[doc = "Field `EXPONENT_FIR` reader - FIR product sum is divided by 2**FIR_exponent and rounded. (format is unsigned integer)"]
pub type EXPONENT_FIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXPONENT_FIR` writer - FIR product sum is divided by 2**FIR_exponent and rounded. (format is unsigned integer)"]
pub type EXPONENT_FIR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FILTERCONTROL_SPEC, u8, u8, 4, O>;
#[doc = "Field `TILE_MODE` reader - Selects the tile mode for pixels outside the source frame. In order to enable tiling tiling_at_begin_h or tiling_at_begin_v and the corresponding output frame dimension have to be set. (format is unsigned integer)"]
pub type TILE_MODE_R = crate::FieldReader<u8, TILE_MODE_A>;
#[doc = "Selects the tile mode for pixels outside the source frame. In order to enable tiling tiling_at_begin_h or tiling_at_begin_v and the corresponding output frame dimension have to be set. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TILE_MODE_A {
    #[doc = "0: Use zero value."]
    TILE_FILL_ZERO = 0,
    #[doc = "1: Use constant color value from the field tile_color."]
    TILE_FILL_CONSTANT = 1,
    #[doc = "2: Use closest pixel from source frame."]
    TILE_PAD = 2,
    #[doc = "3: Use closest pixel from source buffer but zero for alpha component."]
    TILE_PAD_ZERO = 3,
}
impl From<TILE_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: TILE_MODE_A) -> Self {
        variant as _
    }
}
impl TILE_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TILE_MODE_A {
        match self.bits {
            0 => TILE_MODE_A::TILE_FILL_ZERO,
            1 => TILE_MODE_A::TILE_FILL_CONSTANT,
            2 => TILE_MODE_A::TILE_PAD,
            3 => TILE_MODE_A::TILE_PAD_ZERO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_ZERO`"]
    #[inline(always)]
    pub fn is_tile_fill_zero(&self) -> bool {
        *self == TILE_MODE_A::TILE_FILL_ZERO
    }
    #[doc = "Checks if the value of the field is `TILE_FILL_CONSTANT`"]
    #[inline(always)]
    pub fn is_tile_fill_constant(&self) -> bool {
        *self == TILE_MODE_A::TILE_FILL_CONSTANT
    }
    #[doc = "Checks if the value of the field is `TILE_PAD`"]
    #[inline(always)]
    pub fn is_tile_pad(&self) -> bool {
        *self == TILE_MODE_A::TILE_PAD
    }
    #[doc = "Checks if the value of the field is `TILE_PAD_ZERO`"]
    #[inline(always)]
    pub fn is_tile_pad_zero(&self) -> bool {
        *self == TILE_MODE_A::TILE_PAD_ZERO
    }
}
#[doc = "Field `TILE_MODE` writer - Selects the tile mode for pixels outside the source frame. In order to enable tiling tiling_at_begin_h or tiling_at_begin_v and the corresponding output frame dimension have to be set. (format is unsigned integer)"]
pub type TILE_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FILTERCONTROL_SPEC, u8, TILE_MODE_A, 2, O>;
impl<'a, const O: u8> TILE_MODE_W<'a, O> {
    #[doc = "Use zero value."]
    #[inline(always)]
    pub fn tile_fill_zero(self) -> &'a mut W {
        self.variant(TILE_MODE_A::TILE_FILL_ZERO)
    }
    #[doc = "Use constant color value from the field tile_color."]
    #[inline(always)]
    pub fn tile_fill_constant(self) -> &'a mut W {
        self.variant(TILE_MODE_A::TILE_FILL_CONSTANT)
    }
    #[doc = "Use closest pixel from source frame."]
    #[inline(always)]
    pub fn tile_pad(self) -> &'a mut W {
        self.variant(TILE_MODE_A::TILE_PAD)
    }
    #[doc = "Use closest pixel from source buffer but zero for alpha component."]
    #[inline(always)]
    pub fn tile_pad_zero(self) -> &'a mut W {
        self.variant(TILE_MODE_A::TILE_PAD_ZERO)
    }
}
#[doc = "Field `TILING_AT_BEGIN_H` reader - If set to true, this bit enables tiling at the begin of the slice for horizontal direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
pub type TILING_AT_BEGIN_H_R = crate::BitReader<bool>;
#[doc = "Field `TILING_AT_BEGIN_H` writer - If set to true, this bit enables tiling at the begin of the slice for horizontal direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
pub type TILING_AT_BEGIN_H_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, bool, O>;
#[doc = "Field `TILING_AT_BEGIN_V` reader - If set to true, this bit enables tiling at the begin of the slice for vertical direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
pub type TILING_AT_BEGIN_V_R = crate::BitReader<bool>;
#[doc = "Field `TILING_AT_BEGIN_V` writer - If set to true, this bit enables tiling at the begin of the slice for vertical direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
pub type TILING_AT_BEGIN_V_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FILTERCONTROL_SPEC, bool, O>;
#[doc = "Field `TILE_COLOR_ALPHA` reader - Sets the alpha value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_ALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TILE_COLOR_ALPHA` writer - Sets the alpha value for pixels outside of the slice. (format is unsigned integer)"]
pub type TILE_COLOR_ALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FILTERCONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:2 - Defines the horizontal size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter width - 1)/2 (format is unsigned integer)"]
    #[inline(always)]
    pub fn taps_h(&self) -> TAPS_H_R {
        TAPS_H_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Defines the vertical size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter height - 1)/2 (format is unsigned integer)"]
    #[inline(always)]
    pub fn taps_v(&self) -> TAPS_V_R {
        TAPS_V_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Bypass configuration for red color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    pub fn bypass_mode_r(&self) -> BYPASS_MODE_R_R {
        BYPASS_MODE_R_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bypass configuration for green color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    pub fn bypass_mode_g(&self) -> BYPASS_MODE_G_R {
        BYPASS_MODE_G_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bypass configuration for blue color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    pub fn bypass_mode_b(&self) -> BYPASS_MODE_B_R {
        BYPASS_MODE_B_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bypass configuration for alpha components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    pub fn bypass_mode_a(&self) -> BYPASS_MODE_A_R {
        BYPASS_MODE_A_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - FIR product sum is divided by 2**FIR_exponent and rounded. (format is unsigned integer)"]
    #[inline(always)]
    pub fn exponent_fir(&self) -> EXPONENT_FIR_R {
        EXPONENT_FIR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Selects the tile mode for pixels outside the source frame. In order to enable tiling tiling_at_begin_h or tiling_at_begin_v and the corresponding output frame dimension have to be set. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tile_mode(&self) -> TILE_MODE_R {
        TILE_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - If set to true, this bit enables tiling at the begin of the slice for horizontal direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tiling_at_begin_h(&self) -> TILING_AT_BEGIN_H_R {
        TILING_AT_BEGIN_H_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - If set to true, this bit enables tiling at the begin of the slice for vertical direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tiling_at_begin_v(&self) -> TILING_AT_BEGIN_V_R {
        TILING_AT_BEGIN_V_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Sets the alpha value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    pub fn tile_color_alpha(&self) -> TILE_COLOR_ALPHA_R {
        TILE_COLOR_ALPHA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Defines the horizontal size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter width - 1)/2 (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn taps_h(&mut self) -> TAPS_H_W<0> {
        TAPS_H_W::new(self)
    }
    #[doc = "Bits 4:6 - Defines the vertical size of the FIR filter between 1 and 15 with increment of 2. Only odd filter width is supported. To program this field following formula has to be used: (filter height - 1)/2 (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn taps_v(&mut self) -> TAPS_V_W<4> {
        TAPS_V_W::new(self)
    }
    #[doc = "Bit 8 - Bypass configuration for red color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode_r(&mut self) -> BYPASS_MODE_R_W<8> {
        BYPASS_MODE_R_W::new(self)
    }
    #[doc = "Bit 9 - Bypass configuration for green color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode_g(&mut self) -> BYPASS_MODE_G_W<9> {
        BYPASS_MODE_G_W::new(self)
    }
    #[doc = "Bit 10 - Bypass configuration for blue color components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode_b(&mut self) -> BYPASS_MODE_B_W<10> {
        BYPASS_MODE_B_W::new(self)
    }
    #[doc = "Bit 11 - Bypass configuration for alpha components of every pixel. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_mode_a(&mut self) -> BYPASS_MODE_A_W<11> {
        BYPASS_MODE_A_W::new(self)
    }
    #[doc = "Bits 12:15 - FIR product sum is divided by 2**FIR_exponent and rounded. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn exponent_fir(&mut self) -> EXPONENT_FIR_W<12> {
        EXPONENT_FIR_W::new(self)
    }
    #[doc = "Bits 16:17 - Selects the tile mode for pixels outside the source frame. In order to enable tiling tiling_at_begin_h or tiling_at_begin_v and the corresponding output frame dimension have to be set. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tile_mode(&mut self) -> TILE_MODE_W<16> {
        TILE_MODE_W::new(self)
    }
    #[doc = "Bit 20 - If set to true, this bit enables tiling at the begin of the slice for horizontal direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tiling_at_begin_h(&mut self) -> TILING_AT_BEGIN_H_W<20> {
        TILING_AT_BEGIN_H_W::new(self)
    }
    #[doc = "Bit 21 - If set to true, this bit enables tiling at the begin of the slice for vertical direction. Tiling at the end occurs automatically by setting the corresponding output size. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tiling_at_begin_v(&mut self) -> TILING_AT_BEGIN_V_W<21> {
        TILING_AT_BEGIN_V_W::new(self)
    }
    #[doc = "Bits 24:31 - Sets the alpha value for pixels outside of the slice. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn tile_color_alpha(&mut self) -> TILE_COLOR_ALPHA_W<24> {
        TILE_COLOR_ALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter operation control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filtercontrol](index.html) module"]
pub struct FILTERCONTROL_SPEC;
impl crate::RegisterSpec for FILTERCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filtercontrol::R](R) reader structure"]
impl crate::Readable for FILTERCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filtercontrol::W](W) writer structure"]
impl crate::Writable for FILTERCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTERCONTROL to value 0"]
impl crate::Resettable for FILTERCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
