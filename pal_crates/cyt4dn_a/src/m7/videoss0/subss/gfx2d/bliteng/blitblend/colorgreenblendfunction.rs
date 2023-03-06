#[doc = "Register `COLORGREENBLENDFUNCTION` reader"]
pub struct R(crate::R<COLORGREENBLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COLORGREENBLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COLORGREENBLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COLORGREENBLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COLORGREENBLENDFUNCTION` writer"]
pub struct W(crate::W<COLORGREENBLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COLORGREENBLENDFUNCTION_SPEC>;
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
impl From<crate::W<COLORGREENBLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COLORGREENBLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCCOLORGREENSRC` reader - Green component source blend function"]
pub type BLENDFUNCCOLORGREENSRC_R = crate::FieldReader<u16, BLENDFUNCCOLORGREENSRC_A>;
#[doc = "Green component source blend function\n\nValue on reset: 768"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDFUNCCOLORGREENSRC_A {
    #[doc = "0: N/A"]
    GL_ZERO = 0,
    #[doc = "1: N/A"]
    GL_ONE = 1,
    #[doc = "768: N/A"]
    GL_SRC_COLOR = 768,
    #[doc = "769: N/A"]
    GL_ONE_MINUS_SRC_COLOR = 769,
    #[doc = "770: N/A"]
    GL_SRC_ALPHA = 770,
    #[doc = "771: N/A"]
    GL_ONE_MINUS_SRC_ALPHA = 771,
    #[doc = "772: N/A"]
    GL_DST_ALPHA = 772,
    #[doc = "773: N/A"]
    GL_ONE_MINUS_DST_ALPHA = 773,
    #[doc = "774: N/A"]
    GL_DST_COLOR = 774,
    #[doc = "775: N/A"]
    GL_ONE_MINUS_DST_COLOR = 775,
    #[doc = "776: N/A"]
    GL_SRC_ALPHA_SATURATE = 776,
    #[doc = "32769: N/A"]
    GL_CONSTANT_COLOR = 32769,
    #[doc = "32770: N/A"]
    GL_ONE_MINUS_CONSTANT_COLOR = 32770,
    #[doc = "32771: N/A"]
    GL_CONSTANT_ALPHA = 32771,
    #[doc = "32772: N/A"]
    GL_ONE_MINUS_CONSTANT_ALPHA = 32772,
}
impl From<BLENDFUNCCOLORGREENSRC_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDFUNCCOLORGREENSRC_A) -> Self {
        variant as _
    }
}
impl BLENDFUNCCOLORGREENSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDFUNCCOLORGREENSRC_A> {
        match self.bits {
            0 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ZERO),
            1 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE),
            768 => Some(BLENDFUNCCOLORGREENSRC_A::GL_SRC_COLOR),
            769 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_COLOR),
            770 => Some(BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA),
            771 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_ALPHA),
            772 => Some(BLENDFUNCCOLORGREENSRC_A::GL_DST_ALPHA),
            773 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_ALPHA),
            774 => Some(BLENDFUNCCOLORGREENSRC_A::GL_DST_COLOR),
            775 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_COLOR),
            776 => Some(BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA_SATURATE),
            32769 => Some(BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_COLOR),
            32770 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_COLOR),
            32771 => Some(BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_ALPHA),
            32772 => Some(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_ALPHA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_ZERO`"]
    #[inline(always)]
    pub fn is_gl_zero(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ZERO
    }
    #[doc = "Checks if the value of the field is `GL_ONE`"]
    #[inline(always)]
    pub fn is_gl_one(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE
    }
    #[doc = "Checks if the value of the field is `GL_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_src_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_src_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_dst_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_dst_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA_SATURATE`"]
    #[inline(always)]
    pub fn is_gl_src_alpha_saturate(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA_SATURATE
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_constant_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_constant_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_ALPHA
    }
}
#[doc = "Field `BLENDFUNCCOLORGREENSRC` writer - Green component source blend function"]
pub type BLENDFUNCCOLORGREENSRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORGREENBLENDFUNCTION_SPEC, u16, BLENDFUNCCOLORGREENSRC_A, 16, O>;
impl<'a, const O: u8> BLENDFUNCCOLORGREENSRC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_zero(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ZERO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha_saturate(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_SRC_ALPHA_SATURATE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_CONSTANT_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENSRC_A::GL_ONE_MINUS_CONSTANT_ALPHA)
    }
}
#[doc = "Field `BLENDFUNCCOLORGREENDST` reader - Green component destination blend function"]
pub type BLENDFUNCCOLORGREENDST_R = crate::FieldReader<u16, BLENDFUNCCOLORGREENDST_A>;
#[doc = "Green component destination blend function\n\nValue on reset: 768"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDFUNCCOLORGREENDST_A {
    #[doc = "0: N/A"]
    GL_ZERO = 0,
    #[doc = "1: N/A"]
    GL_ONE = 1,
    #[doc = "768: N/A"]
    GL_SRC_COLOR = 768,
    #[doc = "769: N/A"]
    GL_ONE_MINUS_SRC_COLOR = 769,
    #[doc = "770: N/A"]
    GL_SRC_ALPHA = 770,
    #[doc = "771: N/A"]
    GL_ONE_MINUS_SRC_ALPHA = 771,
    #[doc = "772: N/A"]
    GL_DST_ALPHA = 772,
    #[doc = "773: N/A"]
    GL_ONE_MINUS_DST_ALPHA = 773,
    #[doc = "774: N/A"]
    GL_DST_COLOR = 774,
    #[doc = "775: N/A"]
    GL_ONE_MINUS_DST_COLOR = 775,
    #[doc = "776: N/A"]
    GL_SRC_ALPHA_SATURATE = 776,
    #[doc = "32769: N/A"]
    GL_CONSTANT_COLOR = 32769,
    #[doc = "32770: N/A"]
    GL_ONE_MINUS_CONSTANT_COLOR = 32770,
    #[doc = "32771: N/A"]
    GL_CONSTANT_ALPHA = 32771,
    #[doc = "32772: N/A"]
    GL_ONE_MINUS_CONSTANT_ALPHA = 32772,
}
impl From<BLENDFUNCCOLORGREENDST_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDFUNCCOLORGREENDST_A) -> Self {
        variant as _
    }
}
impl BLENDFUNCCOLORGREENDST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDFUNCCOLORGREENDST_A> {
        match self.bits {
            0 => Some(BLENDFUNCCOLORGREENDST_A::GL_ZERO),
            1 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE),
            768 => Some(BLENDFUNCCOLORGREENDST_A::GL_SRC_COLOR),
            769 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_COLOR),
            770 => Some(BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA),
            771 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_ALPHA),
            772 => Some(BLENDFUNCCOLORGREENDST_A::GL_DST_ALPHA),
            773 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_ALPHA),
            774 => Some(BLENDFUNCCOLORGREENDST_A::GL_DST_COLOR),
            775 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_COLOR),
            776 => Some(BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA_SATURATE),
            32769 => Some(BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_COLOR),
            32770 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_COLOR),
            32771 => Some(BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_ALPHA),
            32772 => Some(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_ALPHA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_ZERO`"]
    #[inline(always)]
    pub fn is_gl_zero(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ZERO
    }
    #[doc = "Checks if the value of the field is `GL_ONE`"]
    #[inline(always)]
    pub fn is_gl_one(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE
    }
    #[doc = "Checks if the value of the field is `GL_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_src_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_src_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_dst_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_dst_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA_SATURATE`"]
    #[inline(always)]
    pub fn is_gl_src_alpha_saturate(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA_SATURATE
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_constant_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_color(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_constant_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_alpha(&self) -> bool {
        *self == BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_ALPHA
    }
}
#[doc = "Field `BLENDFUNCCOLORGREENDST` writer - Green component destination blend function"]
pub type BLENDFUNCCOLORGREENDST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COLORGREENBLENDFUNCTION_SPEC, u16, BLENDFUNCCOLORGREENDST_A, 16, O>;
impl<'a, const O: u8> BLENDFUNCCOLORGREENDST_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_zero(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ZERO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha_saturate(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_SRC_ALPHA_SATURATE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_CONSTANT_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCCOLORGREENDST_A::GL_ONE_MINUS_CONSTANT_ALPHA)
    }
}
impl R {
    #[doc = "Bits 0:15 - Green component source blend function"]
    #[inline(always)]
    pub fn blendfunccolorgreensrc(&self) -> BLENDFUNCCOLORGREENSRC_R {
        BLENDFUNCCOLORGREENSRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Green component destination blend function"]
    #[inline(always)]
    pub fn blendfunccolorgreendst(&self) -> BLENDFUNCCOLORGREENDST_R {
        BLENDFUNCCOLORGREENDST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Green component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorgreensrc(&mut self) -> BLENDFUNCCOLORGREENSRC_W<0> {
        BLENDFUNCCOLORGREENSRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Green component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfunccolorgreendst(&mut self) -> BLENDFUNCCOLORGREENDST_W<16> {
        BLENDFUNCCOLORGREENDST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL RGB blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [colorgreenblendfunction](index.html) module"]
pub struct COLORGREENBLENDFUNCTION_SPEC;
impl crate::RegisterSpec for COLORGREENBLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [colorgreenblendfunction::R](R) reader structure"]
impl crate::Readable for COLORGREENBLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [colorgreenblendfunction::W](W) writer structure"]
impl crate::Writable for COLORGREENBLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COLORGREENBLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for COLORGREENBLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
