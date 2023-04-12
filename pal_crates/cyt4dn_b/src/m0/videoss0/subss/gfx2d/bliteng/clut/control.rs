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
#[doc = "Field `MODE` reader - Operation mode for color lookup table"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Operation mode for color lookup table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: module in neutral mode, input data is bypassed to the output"]
    NEUTRAL = 0,
    #[doc = "1: module in color lookup mode (LUT holds a 10bit color value lookup value for each input color)"]
    LUT = 1,
    #[doc = "2: module in 10bit color index table mode (LUT holds a 3x10bit color value, indexed with the red input color)"]
    INDEX_10BIT = 2,
    #[doc = "3: module in RGBA color index table mode (LUT holds a 3x8bit color value and a 6bit alpha value, indexed with the red input color)"]
    INDEX_RGBA = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::NEUTRAL,
            1 => MODE_A::LUT,
            2 => MODE_A::INDEX_10BIT,
            3 => MODE_A::INDEX_RGBA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEUTRAL`"]
    #[inline(always)]
    pub fn is_neutral(&self) -> bool {
        *self == MODE_A::NEUTRAL
    }
    #[doc = "Checks if the value of the field is `LUT`"]
    #[inline(always)]
    pub fn is_lut(&self) -> bool {
        *self == MODE_A::LUT
    }
    #[doc = "Checks if the value of the field is `INDEX_10BIT`"]
    #[inline(always)]
    pub fn is_index_10bit(&self) -> bool {
        *self == MODE_A::INDEX_10BIT
    }
    #[doc = "Checks if the value of the field is `INDEX_RGBA`"]
    #[inline(always)]
    pub fn is_index_rgba(&self) -> bool {
        *self == MODE_A::INDEX_RGBA
    }
}
#[doc = "Field `MODE` writer - Operation mode for color lookup table"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CONTROL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "module in neutral mode, input data is bypassed to the output"]
    #[inline(always)]
    pub fn neutral(self) -> &'a mut W {
        self.variant(MODE_A::NEUTRAL)
    }
    #[doc = "module in color lookup mode (LUT holds a 10bit color value lookup value for each input color)"]
    #[inline(always)]
    pub fn lut(self) -> &'a mut W {
        self.variant(MODE_A::LUT)
    }
    #[doc = "module in 10bit color index table mode (LUT holds a 3x10bit color value, indexed with the red input color)"]
    #[inline(always)]
    pub fn index_10bit(self) -> &'a mut W {
        self.variant(MODE_A::INDEX_10BIT)
    }
    #[doc = "module in RGBA color index table mode (LUT holds a 3x8bit color value and a 6bit alpha value, indexed with the red input color)"]
    #[inline(always)]
    pub fn index_rgba(self) -> &'a mut W {
        self.variant(MODE_A::INDEX_RGBA)
    }
}
#[doc = "Field `COL_8BIT` reader - Color (red, green, blue) output bitwidth select"]
pub type COL_8BIT_R = crate::BitReader<COL_8BIT_A>;
#[doc = "Color (red, green, blue) output bitwidth select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COL_8BIT_A {
    #[doc = "0: color is 10bit output"]
    DISABLE = 0,
    #[doc = "1: color is 8bit output (dithering of internal 10bit value)"]
    ENABLE = 1,
}
impl From<COL_8BIT_A> for bool {
    #[inline(always)]
    fn from(variant: COL_8BIT_A) -> Self {
        variant as u8 != 0
    }
}
impl COL_8BIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COL_8BIT_A {
        match self.bits {
            false => COL_8BIT_A::DISABLE,
            true => COL_8BIT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == COL_8BIT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == COL_8BIT_A::ENABLE
    }
}
#[doc = "Field `COL_8BIT` writer - Color (red, green, blue) output bitwidth select"]
pub type COL_8BIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, COL_8BIT_A, O>;
impl<'a, const O: u8> COL_8BIT_W<'a, O> {
    #[doc = "color is 10bit output"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(COL_8BIT_A::DISABLE)
    }
    #[doc = "color is 8bit output (dithering of internal 10bit value)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(COL_8BIT_A::ENABLE)
    }
}
#[doc = "Field `ALPHAMASK` reader - Enables the alpha mask mode. This mode disables lookup for all pixels with an alpha component smaller or greater/equal than 128. They are bypassed unchanged."]
pub type ALPHAMASK_R = crate::BitReader<ALPHAMASK_A>;
#[doc = "Enables the alpha mask mode. This mode disables lookup for all pixels with an alpha component smaller or greater/equal than 128. They are bypassed unchanged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALPHAMASK_A {
    #[doc = "0: Alpha mask mode disabled"]
    DISABLE = 0,
    #[doc = "1: Alpha mask mode enabled"]
    ENABLE = 1,
}
impl From<ALPHAMASK_A> for bool {
    #[inline(always)]
    fn from(variant: ALPHAMASK_A) -> Self {
        variant as u8 != 0
    }
}
impl ALPHAMASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHAMASK_A {
        match self.bits {
            false => ALPHAMASK_A::DISABLE,
            true => ALPHAMASK_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ALPHAMASK_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ALPHAMASK_A::ENABLE
    }
}
#[doc = "Field `ALPHAMASK` writer - Enables the alpha mask mode. This mode disables lookup for all pixels with an alpha component smaller or greater/equal than 128. They are bypassed unchanged."]
pub type ALPHAMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, ALPHAMASK_A, O>;
impl<'a, const O: u8> ALPHAMASK_W<'a, O> {
    #[doc = "Alpha mask mode disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ALPHAMASK_A::DISABLE)
    }
    #[doc = "Alpha mask mode enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ALPHAMASK_A::ENABLE)
    }
}
#[doc = "Field `ALPHAINVERT` reader - Chooses whether to disable lookup for alpha components smaller or greater/equal than 128. For this field to have an effect AlphaMask must be set to ENABLE."]
pub type ALPHAINVERT_R = crate::BitReader<ALPHAINVERT_A>;
#[doc = "Chooses whether to disable lookup for alpha components smaller or greater/equal than 128. For this field to have an effect AlphaMask must be set to ENABLE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALPHAINVERT_A {
    #[doc = "0: Disable computation for alpha smaller than 128"]
    NORMAL = 0,
    #[doc = "1: Disable computation for alpha greater than or equal to 128"]
    INVERT = 1,
}
impl From<ALPHAINVERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALPHAINVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALPHAINVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALPHAINVERT_A {
        match self.bits {
            false => ALPHAINVERT_A::NORMAL,
            true => ALPHAINVERT_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ALPHAINVERT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        *self == ALPHAINVERT_A::INVERT
    }
}
#[doc = "Field `ALPHAINVERT` writer - Chooses whether to disable lookup for alpha components smaller or greater/equal than 128. For this field to have an effect AlphaMask must be set to ENABLE."]
pub type ALPHAINVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, ALPHAINVERT_A, O>;
impl<'a, const O: u8> ALPHAINVERT_W<'a, O> {
    #[doc = "Disable computation for alpha smaller than 128"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ALPHAINVERT_A::NORMAL)
    }
    #[doc = "Disable computation for alpha greater than or equal to 128"]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(ALPHAINVERT_A::INVERT)
    }
}
#[doc = "Field `IDX_BITS` reader - Number of msb bits of the red color input used for the LUT index input"]
pub type IDX_BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDX_BITS` writer - Number of msb bits of the red color input used for the LUT index input"]
pub type IDX_BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONTROL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Operation mode for color lookup table"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Color (red, green, blue) output bitwidth select"]
    #[inline(always)]
    pub fn col_8bit(&self) -> COL_8BIT_R {
        COL_8BIT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the alpha mask mode. This mode disables lookup for all pixels with an alpha component smaller or greater/equal than 128. They are bypassed unchanged."]
    #[inline(always)]
    pub fn alphamask(&self) -> ALPHAMASK_R {
        ALPHAMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Chooses whether to disable lookup for alpha components smaller or greater/equal than 128. For this field to have an effect AlphaMask must be set to ENABLE."]
    #[inline(always)]
    pub fn alphainvert(&self) -> ALPHAINVERT_R {
        ALPHAINVERT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of msb bits of the red color input used for the LUT index input"]
    #[inline(always)]
    pub fn idx_bits(&self) -> IDX_BITS_R {
        IDX_BITS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation mode for color lookup table"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Color (red, green, blue) output bitwidth select"]
    #[inline(always)]
    #[must_use]
    pub fn col_8bit(&mut self) -> COL_8BIT_W<4> {
        COL_8BIT_W::new(self)
    }
    #[doc = "Bit 5 - Enables the alpha mask mode. This mode disables lookup for all pixels with an alpha component smaller or greater/equal than 128. They are bypassed unchanged."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask(&mut self) -> ALPHAMASK_W<5> {
        ALPHAMASK_W::new(self)
    }
    #[doc = "Bit 6 - Chooses whether to disable lookup for alpha components smaller or greater/equal than 128. For this field to have an effect AlphaMask must be set to ENABLE."]
    #[inline(always)]
    #[must_use]
    pub fn alphainvert(&mut self) -> ALPHAINVERT_W<6> {
        ALPHAINVERT_W::new(self)
    }
    #[doc = "Bits 8:11 - Number of msb bits of the red color input used for the LUT index input"]
    #[inline(always)]
    #[must_use]
    pub fn idx_bits(&mut self) -> IDX_BITS_W<8> {
        IDX_BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLUT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
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
#[doc = "`reset()` method sets CONTROL to value 0x0800"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
