#[doc = "Register `ALPHABLENDFUNCTION` reader"]
pub struct R(crate::R<ALPHABLENDFUNCTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALPHABLENDFUNCTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALPHABLENDFUNCTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALPHABLENDFUNCTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALPHABLENDFUNCTION` writer"]
pub struct W(crate::W<ALPHABLENDFUNCTION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALPHABLENDFUNCTION_SPEC>;
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
impl From<crate::W<ALPHABLENDFUNCTION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALPHABLENDFUNCTION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDFUNCALPHASRC` reader - Alpha component source blend function"]
pub type BLENDFUNCALPHASRC_R = crate::FieldReader<u16, BLENDFUNCALPHASRC_A>;
#[doc = "Alpha component source blend function\n\nValue on reset: 768"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDFUNCALPHASRC_A {
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
impl From<BLENDFUNCALPHASRC_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDFUNCALPHASRC_A) -> Self {
        variant as _
    }
}
impl BLENDFUNCALPHASRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDFUNCALPHASRC_A> {
        match self.bits {
            0 => Some(BLENDFUNCALPHASRC_A::GL_ZERO),
            1 => Some(BLENDFUNCALPHASRC_A::GL_ONE),
            768 => Some(BLENDFUNCALPHASRC_A::GL_SRC_COLOR),
            769 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_COLOR),
            770 => Some(BLENDFUNCALPHASRC_A::GL_SRC_ALPHA),
            771 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_ALPHA),
            772 => Some(BLENDFUNCALPHASRC_A::GL_DST_ALPHA),
            773 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_ALPHA),
            774 => Some(BLENDFUNCALPHASRC_A::GL_DST_COLOR),
            775 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_COLOR),
            776 => Some(BLENDFUNCALPHASRC_A::GL_SRC_ALPHA_SATURATE),
            32769 => Some(BLENDFUNCALPHASRC_A::GL_CONSTANT_COLOR),
            32770 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_COLOR),
            32771 => Some(BLENDFUNCALPHASRC_A::GL_CONSTANT_ALPHA),
            32772 => Some(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_ALPHA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_ZERO`"]
    #[inline(always)]
    pub fn is_gl_zero(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ZERO
    }
    #[doc = "Checks if the value of the field is `GL_ONE`"]
    #[inline(always)]
    pub fn is_gl_one(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE
    }
    #[doc = "Checks if the value of the field is `GL_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_src_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_src_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_dst_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_dst_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA_SATURATE`"]
    #[inline(always)]
    pub fn is_gl_src_alpha_saturate(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_SRC_ALPHA_SATURATE
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_constant_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_color(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_constant_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_CONSTANT_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_alpha(&self) -> bool {
        *self == BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_ALPHA
    }
}
#[doc = "Field `BLENDFUNCALPHASRC` writer - Alpha component source blend function"]
pub type BLENDFUNCALPHASRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALPHABLENDFUNCTION_SPEC, u16, BLENDFUNCALPHASRC_A, 16, O>;
impl<'a, const O: u8> BLENDFUNCALPHASRC_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_zero(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ZERO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha_saturate(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_SRC_ALPHA_SATURATE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_CONSTANT_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHASRC_A::GL_ONE_MINUS_CONSTANT_ALPHA)
    }
}
#[doc = "Field `BLENDFUNCALPHADST` reader - Alpha component destination blend function"]
pub type BLENDFUNCALPHADST_R = crate::FieldReader<u16, BLENDFUNCALPHADST_A>;
#[doc = "Alpha component destination blend function\n\nValue on reset: 768"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDFUNCALPHADST_A {
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
impl From<BLENDFUNCALPHADST_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDFUNCALPHADST_A) -> Self {
        variant as _
    }
}
impl BLENDFUNCALPHADST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDFUNCALPHADST_A> {
        match self.bits {
            0 => Some(BLENDFUNCALPHADST_A::GL_ZERO),
            1 => Some(BLENDFUNCALPHADST_A::GL_ONE),
            768 => Some(BLENDFUNCALPHADST_A::GL_SRC_COLOR),
            769 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_COLOR),
            770 => Some(BLENDFUNCALPHADST_A::GL_SRC_ALPHA),
            771 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_ALPHA),
            772 => Some(BLENDFUNCALPHADST_A::GL_DST_ALPHA),
            773 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_ALPHA),
            774 => Some(BLENDFUNCALPHADST_A::GL_DST_COLOR),
            775 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_COLOR),
            776 => Some(BLENDFUNCALPHADST_A::GL_SRC_ALPHA_SATURATE),
            32769 => Some(BLENDFUNCALPHADST_A::GL_CONSTANT_COLOR),
            32770 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_COLOR),
            32771 => Some(BLENDFUNCALPHADST_A::GL_CONSTANT_ALPHA),
            32772 => Some(BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_ALPHA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_ZERO`"]
    #[inline(always)]
    pub fn is_gl_zero(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ZERO
    }
    #[doc = "Checks if the value of the field is `GL_ONE`"]
    #[inline(always)]
    pub fn is_gl_one(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE
    }
    #[doc = "Checks if the value of the field is `GL_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_src_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_src_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_SRC_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_src_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_dst_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_dst_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_DST_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_dst_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_SRC_ALPHA_SATURATE`"]
    #[inline(always)]
    pub fn is_gl_src_alpha_saturate(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_SRC_ALPHA_SATURATE
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_constant_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_COLOR`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_color(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_COLOR
    }
    #[doc = "Checks if the value of the field is `GL_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_constant_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_CONSTANT_ALPHA
    }
    #[doc = "Checks if the value of the field is `GL_ONE_MINUS_CONSTANT_ALPHA`"]
    #[inline(always)]
    pub fn is_gl_one_minus_constant_alpha(&self) -> bool {
        *self == BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_ALPHA
    }
}
#[doc = "Field `BLENDFUNCALPHADST` writer - Alpha component destination blend function"]
pub type BLENDFUNCALPHADST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ALPHABLENDFUNCTION_SPEC, u16, BLENDFUNCALPHADST_A, 16, O>;
impl<'a, const O: u8> BLENDFUNCALPHADST_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_zero(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ZERO)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_src_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_SRC_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_dst_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_DST_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_src_alpha_saturate(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_SRC_ALPHA_SATURATE)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_color(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_COLOR)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_CONSTANT_ALPHA)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_one_minus_constant_alpha(self) -> &'a mut W {
        self.variant(BLENDFUNCALPHADST_A::GL_ONE_MINUS_CONSTANT_ALPHA)
    }
}
impl R {
    #[doc = "Bits 0:15 - Alpha component source blend function"]
    #[inline(always)]
    pub fn blendfuncalphasrc(&self) -> BLENDFUNCALPHASRC_R {
        BLENDFUNCALPHASRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Alpha component destination blend function"]
    #[inline(always)]
    pub fn blendfuncalphadst(&self) -> BLENDFUNCALPHADST_R {
        BLENDFUNCALPHADST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Alpha component source blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfuncalphasrc(&mut self) -> BLENDFUNCALPHASRC_W<0> {
        BLENDFUNCALPHASRC_W::new(self)
    }
    #[doc = "Bits 16:31 - Alpha component destination blend function"]
    #[inline(always)]
    #[must_use]
    pub fn blendfuncalphadst(&mut self) -> BLENDFUNCALPHADST_W<16> {
        BLENDFUNCALPHADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL alpha blending factors\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alphablendfunction](index.html) module"]
pub struct ALPHABLENDFUNCTION_SPEC;
impl crate::RegisterSpec for ALPHABLENDFUNCTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alphablendfunction::R](R) reader structure"]
impl crate::Readable for ALPHABLENDFUNCTION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alphablendfunction::W](W) writer structure"]
impl crate::Writable for ALPHABLENDFUNCTION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALPHABLENDFUNCTION to value 0x0300_0300"]
impl crate::Resettable for ALPHABLENDFUNCTION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0300;
}
