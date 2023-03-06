#[doc = "Register `BLENDMODE1` reader"]
pub struct R(crate::R<BLENDMODE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLENDMODE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLENDMODE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLENDMODE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLENDMODE1` writer"]
pub struct W(crate::W<BLENDMODE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLENDMODE1_SPEC>;
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
impl From<crate::W<BLENDMODE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLENDMODE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDMODECOLORRED` reader - Red component blend mode"]
pub type BLENDMODECOLORRED_R = crate::FieldReader<u16, BLENDMODECOLORRED_A>;
#[doc = "Red component blend mode\n\nValue on reset: 32774"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDMODECOLORRED_A {
    #[doc = "32774: N/A"]
    GL_FUNC_ADD = 32774,
    #[doc = "32775: N/A"]
    GL_MIN = 32775,
    #[doc = "32776: N/A"]
    GL_MAX = 32776,
    #[doc = "32778: N/A"]
    GL_FUNC_SUBTRACT = 32778,
    #[doc = "32779: N/A"]
    GL_FUNC_REVERSE_SUBTRACT = 32779,
    #[doc = "8192: N/A"]
    VG_BLEND_SRC = 8192,
    #[doc = "8193: N/A"]
    VG_BLEND_SRC_OVER = 8193,
    #[doc = "8194: N/A"]
    VG_BLEND_DST_OVER = 8194,
    #[doc = "8195: N/A"]
    VG_BLEND_SRC_IN = 8195,
    #[doc = "8196: N/A"]
    VG_BLEND_DST_IN = 8196,
    #[doc = "8197: N/A"]
    VG_BLEND_MULTIPLY = 8197,
    #[doc = "8198: N/A"]
    VG_BLEND_SCREEN = 8198,
    #[doc = "8199: N/A"]
    VG_BLEND_DARKEN = 8199,
    #[doc = "8200: N/A"]
    VG_BLEND_LIGHTEN = 8200,
    #[doc = "8201: N/A"]
    VG_BLEND_ADDITIVE = 8201,
}
impl From<BLENDMODECOLORRED_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDMODECOLORRED_A) -> Self {
        variant as _
    }
}
impl BLENDMODECOLORRED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDMODECOLORRED_A> {
        match self.bits {
            32774 => Some(BLENDMODECOLORRED_A::GL_FUNC_ADD),
            32775 => Some(BLENDMODECOLORRED_A::GL_MIN),
            32776 => Some(BLENDMODECOLORRED_A::GL_MAX),
            32778 => Some(BLENDMODECOLORRED_A::GL_FUNC_SUBTRACT),
            32779 => Some(BLENDMODECOLORRED_A::GL_FUNC_REVERSE_SUBTRACT),
            8192 => Some(BLENDMODECOLORRED_A::VG_BLEND_SRC),
            8193 => Some(BLENDMODECOLORRED_A::VG_BLEND_SRC_OVER),
            8194 => Some(BLENDMODECOLORRED_A::VG_BLEND_DST_OVER),
            8195 => Some(BLENDMODECOLORRED_A::VG_BLEND_SRC_IN),
            8196 => Some(BLENDMODECOLORRED_A::VG_BLEND_DST_IN),
            8197 => Some(BLENDMODECOLORRED_A::VG_BLEND_MULTIPLY),
            8198 => Some(BLENDMODECOLORRED_A::VG_BLEND_SCREEN),
            8199 => Some(BLENDMODECOLORRED_A::VG_BLEND_DARKEN),
            8200 => Some(BLENDMODECOLORRED_A::VG_BLEND_LIGHTEN),
            8201 => Some(BLENDMODECOLORRED_A::VG_BLEND_ADDITIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_ADD`"]
    #[inline(always)]
    pub fn is_gl_func_add(&self) -> bool {
        *self == BLENDMODECOLORRED_A::GL_FUNC_ADD
    }
    #[doc = "Checks if the value of the field is `GL_MIN`"]
    #[inline(always)]
    pub fn is_gl_min(&self) -> bool {
        *self == BLENDMODECOLORRED_A::GL_MIN
    }
    #[doc = "Checks if the value of the field is `GL_MAX`"]
    #[inline(always)]
    pub fn is_gl_max(&self) -> bool {
        *self == BLENDMODECOLORRED_A::GL_MAX
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_SUBTRACT`"]
    #[inline(always)]
    pub fn is_gl_func_subtract(&self) -> bool {
        *self == BLENDMODECOLORRED_A::GL_FUNC_SUBTRACT
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_REVERSE_SUBTRACT`"]
    #[inline(always)]
    pub fn is_gl_func_reverse_subtract(&self) -> bool {
        *self == BLENDMODECOLORRED_A::GL_FUNC_REVERSE_SUBTRACT
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC`"]
    #[inline(always)]
    pub fn is_vg_blend_src(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_SRC
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC_OVER`"]
    #[inline(always)]
    pub fn is_vg_blend_src_over(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_SRC_OVER
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DST_OVER`"]
    #[inline(always)]
    pub fn is_vg_blend_dst_over(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_DST_OVER
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC_IN`"]
    #[inline(always)]
    pub fn is_vg_blend_src_in(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_SRC_IN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DST_IN`"]
    #[inline(always)]
    pub fn is_vg_blend_dst_in(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_DST_IN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_MULTIPLY`"]
    #[inline(always)]
    pub fn is_vg_blend_multiply(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_MULTIPLY
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SCREEN`"]
    #[inline(always)]
    pub fn is_vg_blend_screen(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_SCREEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DARKEN`"]
    #[inline(always)]
    pub fn is_vg_blend_darken(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_DARKEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_LIGHTEN`"]
    #[inline(always)]
    pub fn is_vg_blend_lighten(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_LIGHTEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_ADDITIVE`"]
    #[inline(always)]
    pub fn is_vg_blend_additive(&self) -> bool {
        *self == BLENDMODECOLORRED_A::VG_BLEND_ADDITIVE
    }
}
#[doc = "Field `BLENDMODECOLORRED` writer - Red component blend mode"]
pub type BLENDMODECOLORRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE1_SPEC, u16, BLENDMODECOLORRED_A, 16, O>;
impl<'a, const O: u8> BLENDMODECOLORRED_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_add(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::GL_FUNC_ADD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_min(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::GL_MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_max(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::GL_MAX)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_subtract(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::GL_FUNC_SUBTRACT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_reverse_subtract(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::GL_FUNC_REVERSE_SUBTRACT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_SRC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src_over(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_SRC_OVER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_dst_over(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_DST_OVER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src_in(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_SRC_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_dst_in(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_DST_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_multiply(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_MULTIPLY)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_screen(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_SCREEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_darken(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_DARKEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_lighten(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_LIGHTEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_additive(self) -> &'a mut W {
        self.variant(BLENDMODECOLORRED_A::VG_BLEND_ADDITIVE)
    }
}
#[doc = "Field `BLENDMODECOLORGREEN` reader - Green component blend mode"]
pub type BLENDMODECOLORGREEN_R = crate::FieldReader<u16, BLENDMODECOLORGREEN_A>;
#[doc = "Green component blend mode\n\nValue on reset: 32774"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLENDMODECOLORGREEN_A {
    #[doc = "32774: N/A"]
    GL_FUNC_ADD = 32774,
    #[doc = "32775: N/A"]
    GL_MIN = 32775,
    #[doc = "32776: N/A"]
    GL_MAX = 32776,
    #[doc = "32778: N/A"]
    GL_FUNC_SUBTRACT = 32778,
    #[doc = "32779: N/A"]
    GL_FUNC_REVERSE_SUBTRACT = 32779,
    #[doc = "8192: N/A"]
    VG_BLEND_SRC = 8192,
    #[doc = "8193: N/A"]
    VG_BLEND_SRC_OVER = 8193,
    #[doc = "8194: N/A"]
    VG_BLEND_DST_OVER = 8194,
    #[doc = "8195: N/A"]
    VG_BLEND_SRC_IN = 8195,
    #[doc = "8196: N/A"]
    VG_BLEND_DST_IN = 8196,
    #[doc = "8197: N/A"]
    VG_BLEND_MULTIPLY = 8197,
    #[doc = "8198: N/A"]
    VG_BLEND_SCREEN = 8198,
    #[doc = "8199: N/A"]
    VG_BLEND_DARKEN = 8199,
    #[doc = "8200: N/A"]
    VG_BLEND_LIGHTEN = 8200,
    #[doc = "8201: N/A"]
    VG_BLEND_ADDITIVE = 8201,
}
impl From<BLENDMODECOLORGREEN_A> for u16 {
    #[inline(always)]
    fn from(variant: BLENDMODECOLORGREEN_A) -> Self {
        variant as _
    }
}
impl BLENDMODECOLORGREEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLENDMODECOLORGREEN_A> {
        match self.bits {
            32774 => Some(BLENDMODECOLORGREEN_A::GL_FUNC_ADD),
            32775 => Some(BLENDMODECOLORGREEN_A::GL_MIN),
            32776 => Some(BLENDMODECOLORGREEN_A::GL_MAX),
            32778 => Some(BLENDMODECOLORGREEN_A::GL_FUNC_SUBTRACT),
            32779 => Some(BLENDMODECOLORGREEN_A::GL_FUNC_REVERSE_SUBTRACT),
            8192 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_SRC),
            8193 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_SRC_OVER),
            8194 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_DST_OVER),
            8195 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_SRC_IN),
            8196 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_DST_IN),
            8197 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_MULTIPLY),
            8198 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_SCREEN),
            8199 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_DARKEN),
            8200 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_LIGHTEN),
            8201 => Some(BLENDMODECOLORGREEN_A::VG_BLEND_ADDITIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_ADD`"]
    #[inline(always)]
    pub fn is_gl_func_add(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::GL_FUNC_ADD
    }
    #[doc = "Checks if the value of the field is `GL_MIN`"]
    #[inline(always)]
    pub fn is_gl_min(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::GL_MIN
    }
    #[doc = "Checks if the value of the field is `GL_MAX`"]
    #[inline(always)]
    pub fn is_gl_max(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::GL_MAX
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_SUBTRACT`"]
    #[inline(always)]
    pub fn is_gl_func_subtract(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::GL_FUNC_SUBTRACT
    }
    #[doc = "Checks if the value of the field is `GL_FUNC_REVERSE_SUBTRACT`"]
    #[inline(always)]
    pub fn is_gl_func_reverse_subtract(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::GL_FUNC_REVERSE_SUBTRACT
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC`"]
    #[inline(always)]
    pub fn is_vg_blend_src(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_SRC
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC_OVER`"]
    #[inline(always)]
    pub fn is_vg_blend_src_over(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_SRC_OVER
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DST_OVER`"]
    #[inline(always)]
    pub fn is_vg_blend_dst_over(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_DST_OVER
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SRC_IN`"]
    #[inline(always)]
    pub fn is_vg_blend_src_in(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_SRC_IN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DST_IN`"]
    #[inline(always)]
    pub fn is_vg_blend_dst_in(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_DST_IN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_MULTIPLY`"]
    #[inline(always)]
    pub fn is_vg_blend_multiply(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_MULTIPLY
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_SCREEN`"]
    #[inline(always)]
    pub fn is_vg_blend_screen(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_SCREEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_DARKEN`"]
    #[inline(always)]
    pub fn is_vg_blend_darken(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_DARKEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_LIGHTEN`"]
    #[inline(always)]
    pub fn is_vg_blend_lighten(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_LIGHTEN
    }
    #[doc = "Checks if the value of the field is `VG_BLEND_ADDITIVE`"]
    #[inline(always)]
    pub fn is_vg_blend_additive(&self) -> bool {
        *self == BLENDMODECOLORGREEN_A::VG_BLEND_ADDITIVE
    }
}
#[doc = "Field `BLENDMODECOLORGREEN` writer - Green component blend mode"]
pub type BLENDMODECOLORGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE1_SPEC, u16, BLENDMODECOLORGREEN_A, 16, O>;
impl<'a, const O: u8> BLENDMODECOLORGREEN_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_add(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::GL_FUNC_ADD)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_min(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::GL_MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_max(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::GL_MAX)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_subtract(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::GL_FUNC_SUBTRACT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn gl_func_reverse_subtract(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::GL_FUNC_REVERSE_SUBTRACT)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_SRC)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src_over(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_SRC_OVER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_dst_over(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_DST_OVER)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_src_in(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_SRC_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_dst_in(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_DST_IN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_multiply(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_MULTIPLY)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_screen(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_SCREEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_darken(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_DARKEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_lighten(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_LIGHTEN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vg_blend_additive(self) -> &'a mut W {
        self.variant(BLENDMODECOLORGREEN_A::VG_BLEND_ADDITIVE)
    }
}
impl R {
    #[doc = "Bits 0:15 - Red component blend mode"]
    #[inline(always)]
    pub fn blendmodecolorred(&self) -> BLENDMODECOLORRED_R {
        BLENDMODECOLORRED_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Green component blend mode"]
    #[inline(always)]
    pub fn blendmodecolorgreen(&self) -> BLENDMODECOLORGREEN_R {
        BLENDMODECOLORGREEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Red component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodecolorred(&mut self) -> BLENDMODECOLORRED_W<0> {
        BLENDMODECOLORRED_W::new(self)
    }
    #[doc = "Bits 16:31 - Green component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodecolorgreen(&mut self) -> BLENDMODECOLORGREEN_W<16> {
        BLENDMODECOLORGREEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL and Open VG blending modes for colors red and green\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blendmode1](index.html) module"]
pub struct BLENDMODE1_SPEC;
impl crate::RegisterSpec for BLENDMODE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blendmode1::R](R) reader structure"]
impl crate::Readable for BLENDMODE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blendmode1::W](W) writer structure"]
impl crate::Writable for BLENDMODE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLENDMODE1 to value 0x8006_8006"]
impl crate::Resettable for BLENDMODE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8006_8006;
}
