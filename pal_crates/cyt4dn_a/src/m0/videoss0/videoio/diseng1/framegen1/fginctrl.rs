#[doc = "Register `FGINCTRL` reader"]
pub struct R(crate::R<FGINCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGINCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGINCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGINCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGINCTRL` writer"]
pub struct W(crate::W<FGINCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGINCTRL_SPEC>;
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
impl From<crate::W<FGINCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGINCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGDM` reader - Frame Generator Display Mode."]
pub type FGDM_R = crate::FieldReader<u8, FGDM_A>;
#[doc = "Frame Generator Display Mode.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FGDM_A {
    #[doc = "0: Black color background is shown on the whole screen area."]
    BLACK = 0,
    #[doc = "1: Programmable constant color is shown in the active area and black color in the blanking area."]
    CONSTCOL = 1,
    #[doc = "2: Primary input only is shown."]
    PRIM = 2,
    #[doc = "3: Secondary input only is shown."]
    SEC = 3,
    #[doc = "4: Both inputs overlaid with primary on top."]
    PRIM_ON_TOP = 4,
    #[doc = "5: Both inputs overlaid with secondary on top."]
    SEC_ON_TOP = 5,
    #[doc = "6: White color background with test pattern is shown in the active area and black color in the blanking area."]
    TEST = 6,
    #[doc = "7: Blending of the mode PRIM with the mode SEC."]
    BLEND = 7,
}
impl From<FGDM_A> for u8 {
    #[inline(always)]
    fn from(variant: FGDM_A) -> Self {
        variant as _
    }
}
impl FGDM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGDM_A {
        match self.bits {
            0 => FGDM_A::BLACK,
            1 => FGDM_A::CONSTCOL,
            2 => FGDM_A::PRIM,
            3 => FGDM_A::SEC,
            4 => FGDM_A::PRIM_ON_TOP,
            5 => FGDM_A::SEC_ON_TOP,
            6 => FGDM_A::TEST,
            7 => FGDM_A::BLEND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLACK`"]
    #[inline(always)]
    pub fn is_black(&self) -> bool {
        *self == FGDM_A::BLACK
    }
    #[doc = "Checks if the value of the field is `CONSTCOL`"]
    #[inline(always)]
    pub fn is_constcol(&self) -> bool {
        *self == FGDM_A::CONSTCOL
    }
    #[doc = "Checks if the value of the field is `PRIM`"]
    #[inline(always)]
    pub fn is_prim(&self) -> bool {
        *self == FGDM_A::PRIM
    }
    #[doc = "Checks if the value of the field is `SEC`"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == FGDM_A::SEC
    }
    #[doc = "Checks if the value of the field is `PRIM_ON_TOP`"]
    #[inline(always)]
    pub fn is_prim_on_top(&self) -> bool {
        *self == FGDM_A::PRIM_ON_TOP
    }
    #[doc = "Checks if the value of the field is `SEC_ON_TOP`"]
    #[inline(always)]
    pub fn is_sec_on_top(&self) -> bool {
        *self == FGDM_A::SEC_ON_TOP
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == FGDM_A::TEST
    }
    #[doc = "Checks if the value of the field is `BLEND`"]
    #[inline(always)]
    pub fn is_blend(&self) -> bool {
        *self == FGDM_A::BLEND
    }
}
#[doc = "Field `FGDM` writer - Frame Generator Display Mode."]
pub type FGDM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, FGINCTRL_SPEC, u8, FGDM_A, 3, O>;
impl<'a, const O: u8> FGDM_W<'a, O> {
    #[doc = "Black color background is shown on the whole screen area."]
    #[inline(always)]
    pub fn black(self) -> &'a mut W {
        self.variant(FGDM_A::BLACK)
    }
    #[doc = "Programmable constant color is shown in the active area and black color in the blanking area."]
    #[inline(always)]
    pub fn constcol(self) -> &'a mut W {
        self.variant(FGDM_A::CONSTCOL)
    }
    #[doc = "Primary input only is shown."]
    #[inline(always)]
    pub fn prim(self) -> &'a mut W {
        self.variant(FGDM_A::PRIM)
    }
    #[doc = "Secondary input only is shown."]
    #[inline(always)]
    pub fn sec(self) -> &'a mut W {
        self.variant(FGDM_A::SEC)
    }
    #[doc = "Both inputs overlaid with primary on top."]
    #[inline(always)]
    pub fn prim_on_top(self) -> &'a mut W {
        self.variant(FGDM_A::PRIM_ON_TOP)
    }
    #[doc = "Both inputs overlaid with secondary on top."]
    #[inline(always)]
    pub fn sec_on_top(self) -> &'a mut W {
        self.variant(FGDM_A::SEC_ON_TOP)
    }
    #[doc = "White color background with test pattern is shown in the active area and black color in the blanking area."]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(FGDM_A::TEST)
    }
    #[doc = "Blending of the mode PRIM with the mode SEC."]
    #[inline(always)]
    pub fn blend(self) -> &'a mut W {
        self.variant(FGDM_A::BLEND)
    }
}
#[doc = "Field `ENPRIMALPHA` reader - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
pub type ENPRIMALPHA_R = crate::BitReader<bool>;
#[doc = "Field `ENPRIMALPHA` writer - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
pub type ENPRIMALPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGINCTRL_SPEC, bool, O>;
#[doc = "Field `ENSECALPHA` reader - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
pub type ENSECALPHA_R = crate::BitReader<bool>;
#[doc = "Field `ENSECALPHA` writer - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
pub type ENSECALPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, FGINCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Frame Generator Display Mode."]
    #[inline(always)]
    pub fn fgdm(&self) -> FGDM_R {
        FGDM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
    #[inline(always)]
    pub fn enprimalpha(&self) -> ENPRIMALPHA_R {
        ENPRIMALPHA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
    #[inline(always)]
    pub fn ensecalpha(&self) -> ENSECALPHA_R {
        ENSECALPHA_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frame Generator Display Mode."]
    #[inline(always)]
    #[must_use]
    pub fn fgdm(&mut self) -> FGDM_W<0> {
        FGDM_W::new(self)
    }
    #[doc = "Bit 3 - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
    #[inline(always)]
    #[must_use]
    pub fn enprimalpha(&mut self) -> ENPRIMALPHA_W<3> {
        ENPRIMALPHA_W::new(self)
    }
    #[doc = "Bit 4 - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDm == BLEND."]
    #[inline(always)]
    #[must_use]
    pub fn ensecalpha(&mut self) -> ENSECALPHA_W<4> {
        ENSECALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Input Control Register (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fginctrl](index.html) module"]
pub struct FGINCTRL_SPEC;
impl crate::RegisterSpec for FGINCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fginctrl::R](R) reader structure"]
impl crate::Readable for FGINCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fginctrl::W](W) writer structure"]
impl crate::Writable for FGINCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGINCTRL to value 0x06"]
impl crate::Resettable for FGINCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
