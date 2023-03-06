#[doc = "Register `SCALERCONTROL` reader"]
pub struct R(crate::R<SCALERCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCALERCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCALERCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCALERCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCALERCONTROL` writer"]
pub struct W(crate::W<SCALERCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCALERCONTROL_SPEC>;
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
impl From<crate::W<SCALERCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCALERCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCALE_MODE_H` reader - Scale mode for horizontal scaling. (format is unsigned integer)"]
pub type SCALE_MODE_H_R = crate::BitReader<SCALE_MODE_H_A>;
#[doc = "Scale mode for horizontal scaling. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALE_MODE_H_A {
    #[doc = "0: Down-scaling (output width is less or equal input width)."]
    DOWNSCALE = 0,
    #[doc = "1: Up-scaling (output width greater or equal input width)"]
    UPSCALE = 1,
}
impl From<SCALE_MODE_H_A> for bool {
    #[inline(always)]
    fn from(variant: SCALE_MODE_H_A) -> Self {
        variant as u8 != 0
    }
}
impl SCALE_MODE_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALE_MODE_H_A {
        match self.bits {
            false => SCALE_MODE_H_A::DOWNSCALE,
            true => SCALE_MODE_H_A::UPSCALE,
        }
    }
    #[doc = "Checks if the value of the field is `DOWNSCALE`"]
    #[inline(always)]
    pub fn is_downscale(&self) -> bool {
        *self == SCALE_MODE_H_A::DOWNSCALE
    }
    #[doc = "Checks if the value of the field is `UPSCALE`"]
    #[inline(always)]
    pub fn is_upscale(&self) -> bool {
        *self == SCALE_MODE_H_A::UPSCALE
    }
}
#[doc = "Field `SCALE_MODE_H` writer - Scale mode for horizontal scaling. (format is unsigned integer)"]
pub type SCALE_MODE_H_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCALERCONTROL_SPEC, SCALE_MODE_H_A, O>;
impl<'a, const O: u8> SCALE_MODE_H_W<'a, O> {
    #[doc = "Down-scaling (output width is less or equal input width)."]
    #[inline(always)]
    pub fn downscale(self) -> &'a mut W {
        self.variant(SCALE_MODE_H_A::DOWNSCALE)
    }
    #[doc = "Up-scaling (output width greater or equal input width)"]
    #[inline(always)]
    pub fn upscale(self) -> &'a mut W {
        self.variant(SCALE_MODE_H_A::UPSCALE)
    }
}
#[doc = "Field `SCALE_MODE_V` reader - Scale mode for vertical scaling. (format is unsigned integer)"]
pub type SCALE_MODE_V_R = crate::BitReader<SCALE_MODE_V_A>;
#[doc = "Scale mode for vertical scaling. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCALE_MODE_V_A {
    #[doc = "0: Down-scaling (output height less or equal input height)."]
    DOWNSCALE = 0,
    #[doc = "1: Up-scaling (output height greater or equal input height)"]
    UPSCALE = 1,
}
impl From<SCALE_MODE_V_A> for bool {
    #[inline(always)]
    fn from(variant: SCALE_MODE_V_A) -> Self {
        variant as u8 != 0
    }
}
impl SCALE_MODE_V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCALE_MODE_V_A {
        match self.bits {
            false => SCALE_MODE_V_A::DOWNSCALE,
            true => SCALE_MODE_V_A::UPSCALE,
        }
    }
    #[doc = "Checks if the value of the field is `DOWNSCALE`"]
    #[inline(always)]
    pub fn is_downscale(&self) -> bool {
        *self == SCALE_MODE_V_A::DOWNSCALE
    }
    #[doc = "Checks if the value of the field is `UPSCALE`"]
    #[inline(always)]
    pub fn is_upscale(&self) -> bool {
        *self == SCALE_MODE_V_A::UPSCALE
    }
}
#[doc = "Field `SCALE_MODE_V` writer - Scale mode for vertical scaling. (format is unsigned integer)"]
pub type SCALE_MODE_V_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCALERCONTROL_SPEC, SCALE_MODE_V_A, O>;
impl<'a, const O: u8> SCALE_MODE_V_W<'a, O> {
    #[doc = "Down-scaling (output height less or equal input height)."]
    #[inline(always)]
    pub fn downscale(self) -> &'a mut W {
        self.variant(SCALE_MODE_V_A::DOWNSCALE)
    }
    #[doc = "Up-scaling (output height greater or equal input height)"]
    #[inline(always)]
    pub fn upscale(self) -> &'a mut W {
        self.variant(SCALE_MODE_V_A::UPSCALE)
    }
}
#[doc = "Field `FILTER_MODE_H` reader - Selects sampling algorithm for horizontal scaling. (format is unsigned integer)"]
pub type FILTER_MODE_H_R = crate::BitReader<FILTER_MODE_H_A>;
#[doc = "Selects sampling algorithm for horizontal scaling. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTER_MODE_H_A {
    #[doc = "0: Nearest filter (point-sampling)"]
    NEAREST = 0,
    #[doc = "1: Box filter (linear)"]
    LINEAR = 1,
}
impl From<FILTER_MODE_H_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_MODE_H_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTER_MODE_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_MODE_H_A {
        match self.bits {
            false => FILTER_MODE_H_A::NEAREST,
            true => FILTER_MODE_H_A::LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NEAREST`"]
    #[inline(always)]
    pub fn is_nearest(&self) -> bool {
        *self == FILTER_MODE_H_A::NEAREST
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == FILTER_MODE_H_A::LINEAR
    }
}
#[doc = "Field `FILTER_MODE_H` writer - Selects sampling algorithm for horizontal scaling. (format is unsigned integer)"]
pub type FILTER_MODE_H_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCALERCONTROL_SPEC, FILTER_MODE_H_A, O>;
impl<'a, const O: u8> FILTER_MODE_H_W<'a, O> {
    #[doc = "Nearest filter (point-sampling)"]
    #[inline(always)]
    pub fn nearest(self) -> &'a mut W {
        self.variant(FILTER_MODE_H_A::NEAREST)
    }
    #[doc = "Box filter (linear)"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(FILTER_MODE_H_A::LINEAR)
    }
}
#[doc = "Field `FILTER_MODE_V` reader - Selects sampling algorithm for vertical scaling. (format is unsigned integer)"]
pub type FILTER_MODE_V_R = crate::BitReader<FILTER_MODE_V_A>;
#[doc = "Selects sampling algorithm for vertical scaling. (format is unsigned integer)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTER_MODE_V_A {
    #[doc = "0: Nearest filter (point-sampling)"]
    NEAREST = 0,
    #[doc = "1: Box filter (linear)"]
    LINEAR = 1,
}
impl From<FILTER_MODE_V_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_MODE_V_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTER_MODE_V_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_MODE_V_A {
        match self.bits {
            false => FILTER_MODE_V_A::NEAREST,
            true => FILTER_MODE_V_A::LINEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NEAREST`"]
    #[inline(always)]
    pub fn is_nearest(&self) -> bool {
        *self == FILTER_MODE_V_A::NEAREST
    }
    #[doc = "Checks if the value of the field is `LINEAR`"]
    #[inline(always)]
    pub fn is_linear(&self) -> bool {
        *self == FILTER_MODE_V_A::LINEAR
    }
}
#[doc = "Field `FILTER_MODE_V` writer - Selects sampling algorithm for vertical scaling. (format is unsigned integer)"]
pub type FILTER_MODE_V_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SCALERCONTROL_SPEC, FILTER_MODE_V_A, O>;
impl<'a, const O: u8> FILTER_MODE_V_W<'a, O> {
    #[doc = "Nearest filter (point-sampling)"]
    #[inline(always)]
    pub fn nearest(self) -> &'a mut W {
        self.variant(FILTER_MODE_V_A::NEAREST)
    }
    #[doc = "Box filter (linear)"]
    #[inline(always)]
    pub fn linear(self) -> &'a mut W {
        self.variant(FILTER_MODE_V_A::LINEAR)
    }
}
#[doc = "Field `FIELD_MODE` reader - Controls generation of output field polarity for vertical scaling. (format is unsigned integer)"]
pub type FIELD_MODE_R = crate::FieldReader<u8, FIELD_MODE_A>;
#[doc = "Controls generation of output field polarity for vertical scaling. (format is unsigned integer)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIELD_MODE_A {
    #[doc = "0: Constant 0 indicates frame or top field."]
    ALWAYS0 = 0,
    #[doc = "1: Constant 1 indicates bottom field."]
    ALWAYS1 = 1,
    #[doc = "2: Output field polarity is taken from input field polarity."]
    INPUT = 2,
    #[doc = "3: Output field polarity toggles, starting with 0 after reset."]
    TOGGLE = 3,
}
impl From<FIELD_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FIELD_MODE_A) -> Self {
        variant as _
    }
}
impl FIELD_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELD_MODE_A {
        match self.bits {
            0 => FIELD_MODE_A::ALWAYS0,
            1 => FIELD_MODE_A::ALWAYS1,
            2 => FIELD_MODE_A::INPUT,
            3 => FIELD_MODE_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS0`"]
    #[inline(always)]
    pub fn is_always0(&self) -> bool {
        *self == FIELD_MODE_A::ALWAYS0
    }
    #[doc = "Checks if the value of the field is `ALWAYS1`"]
    #[inline(always)]
    pub fn is_always1(&self) -> bool {
        *self == FIELD_MODE_A::ALWAYS1
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == FIELD_MODE_A::INPUT
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == FIELD_MODE_A::TOGGLE
    }
}
#[doc = "Field `FIELD_MODE` writer - Controls generation of output field polarity for vertical scaling. (format is unsigned integer)"]
pub type FIELD_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SCALERCONTROL_SPEC, u8, FIELD_MODE_A, 2, O>;
impl<'a, const O: u8> FIELD_MODE_W<'a, O> {
    #[doc = "Constant 0 indicates frame or top field."]
    #[inline(always)]
    pub fn always0(self) -> &'a mut W {
        self.variant(FIELD_MODE_A::ALWAYS0)
    }
    #[doc = "Constant 1 indicates bottom field."]
    #[inline(always)]
    pub fn always1(self) -> &'a mut W {
        self.variant(FIELD_MODE_A::ALWAYS1)
    }
    #[doc = "Output field polarity is taken from input field polarity."]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(FIELD_MODE_A::INPUT)
    }
    #[doc = "Output field polarity toggles, starting with 0 after reset."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(FIELD_MODE_A::TOGGLE)
    }
}
#[doc = "Field `ALPHA_VALUE` reader - When 'mode' field is not 'NEUTRAL', then input alpha values are ignore and output alpha set to this constant value for all pixels. This restriction applies to GPScalerL derivative only. (format is unsigned integer)"]
pub type ALPHA_VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ALPHA_VALUE` writer - When 'mode' field is not 'NEUTRAL', then input alpha values are ignore and output alpha set to this constant value for all pixels. This restriction applies to GPScalerL derivative only. (format is unsigned integer)"]
pub type ALPHA_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCALERCONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Scale mode for horizontal scaling. (format is unsigned integer)"]
    #[inline(always)]
    pub fn scale_mode_h(&self) -> SCALE_MODE_H_R {
        SCALE_MODE_H_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Scale mode for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    pub fn scale_mode_v(&self) -> SCALE_MODE_V_R {
        SCALE_MODE_V_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects sampling algorithm for horizontal scaling. (format is unsigned integer)"]
    #[inline(always)]
    pub fn filter_mode_h(&self) -> FILTER_MODE_H_R {
        FILTER_MODE_H_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects sampling algorithm for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    pub fn filter_mode_v(&self) -> FILTER_MODE_V_R {
        FILTER_MODE_V_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Controls generation of output field polarity for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    pub fn field_mode(&self) -> FIELD_MODE_R {
        FIELD_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:27 - When 'mode' field is not 'NEUTRAL', then input alpha values are ignore and output alpha set to this constant value for all pixels. This restriction applies to GPScalerL derivative only. (format is unsigned integer)"]
    #[inline(always)]
    pub fn alpha_value(&self) -> ALPHA_VALUE_R {
        ALPHA_VALUE_R::new(((self.bits >> 20) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Scale mode for horizontal scaling. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn scale_mode_h(&mut self) -> SCALE_MODE_H_W<0> {
        SCALE_MODE_H_W::new(self)
    }
    #[doc = "Bit 4 - Scale mode for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn scale_mode_v(&mut self) -> SCALE_MODE_V_W<4> {
        SCALE_MODE_V_W::new(self)
    }
    #[doc = "Bit 8 - Selects sampling algorithm for horizontal scaling. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn filter_mode_h(&mut self) -> FILTER_MODE_H_W<8> {
        FILTER_MODE_H_W::new(self)
    }
    #[doc = "Bit 12 - Selects sampling algorithm for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn filter_mode_v(&mut self) -> FILTER_MODE_V_W<12> {
        FILTER_MODE_V_W::new(self)
    }
    #[doc = "Bits 16:17 - Controls generation of output field polarity for vertical scaling. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn field_mode(&mut self) -> FIELD_MODE_W<16> {
        FIELD_MODE_W::new(self)
    }
    #[doc = "Bits 20:27 - When 'mode' field is not 'NEUTRAL', then input alpha values are ignore and output alpha set to this constant value for all pixels. This restriction applies to GPScalerL derivative only. (format is unsigned integer)"]
    #[inline(always)]
    #[must_use]
    pub fn alpha_value(&mut self) -> ALPHA_VALUE_W<20> {
        ALPHA_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scaler operation control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scalercontrol](index.html) module"]
pub struct SCALERCONTROL_SPEC;
impl crate::RegisterSpec for SCALERCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scalercontrol::R](R) reader structure"]
impl crate::Readable for SCALERCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scalercontrol::W](W) writer structure"]
impl crate::Writable for SCALERCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCALERCONTROL to value 0x0002_0000"]
impl crate::Resettable for SCALERCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
