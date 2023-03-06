#[doc = "Register `FGINCTRLPANIC` reader"]
pub struct R(crate::R<FGINCTRLPANIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGINCTRLPANIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGINCTRLPANIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGINCTRLPANIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGINCTRLPANIC` writer"]
pub struct W(crate::W<FGINCTRLPANIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGINCTRLPANIC_SPEC>;
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
impl From<crate::W<FGINCTRLPANIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGINCTRLPANIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FGDMPANIC` reader - Frame Generator Display Mode."]
pub type FGDMPANIC_R = crate::FieldReader<u8, FGDMPANIC_A>;
#[doc = "Frame Generator Display Mode.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FGDMPANIC_A {
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
impl From<FGDMPANIC_A> for u8 {
    #[inline(always)]
    fn from(variant: FGDMPANIC_A) -> Self {
        variant as _
    }
}
impl FGDMPANIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FGDMPANIC_A {
        match self.bits {
            0 => FGDMPANIC_A::BLACK,
            1 => FGDMPANIC_A::CONSTCOL,
            2 => FGDMPANIC_A::PRIM,
            3 => FGDMPANIC_A::SEC,
            4 => FGDMPANIC_A::PRIM_ON_TOP,
            5 => FGDMPANIC_A::SEC_ON_TOP,
            6 => FGDMPANIC_A::TEST,
            7 => FGDMPANIC_A::BLEND,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BLACK`"]
    #[inline(always)]
    pub fn is_black(&self) -> bool {
        *self == FGDMPANIC_A::BLACK
    }
    #[doc = "Checks if the value of the field is `CONSTCOL`"]
    #[inline(always)]
    pub fn is_constcol(&self) -> bool {
        *self == FGDMPANIC_A::CONSTCOL
    }
    #[doc = "Checks if the value of the field is `PRIM`"]
    #[inline(always)]
    pub fn is_prim(&self) -> bool {
        *self == FGDMPANIC_A::PRIM
    }
    #[doc = "Checks if the value of the field is `SEC`"]
    #[inline(always)]
    pub fn is_sec(&self) -> bool {
        *self == FGDMPANIC_A::SEC
    }
    #[doc = "Checks if the value of the field is `PRIM_ON_TOP`"]
    #[inline(always)]
    pub fn is_prim_on_top(&self) -> bool {
        *self == FGDMPANIC_A::PRIM_ON_TOP
    }
    #[doc = "Checks if the value of the field is `SEC_ON_TOP`"]
    #[inline(always)]
    pub fn is_sec_on_top(&self) -> bool {
        *self == FGDMPANIC_A::SEC_ON_TOP
    }
    #[doc = "Checks if the value of the field is `TEST`"]
    #[inline(always)]
    pub fn is_test(&self) -> bool {
        *self == FGDMPANIC_A::TEST
    }
    #[doc = "Checks if the value of the field is `BLEND`"]
    #[inline(always)]
    pub fn is_blend(&self) -> bool {
        *self == FGDMPANIC_A::BLEND
    }
}
#[doc = "Field `FGDMPANIC` writer - Frame Generator Display Mode."]
pub type FGDMPANIC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FGINCTRLPANIC_SPEC, u8, FGDMPANIC_A, 3, O>;
impl<'a, const O: u8> FGDMPANIC_W<'a, O> {
    #[doc = "Black color background is shown on the whole screen area."]
    #[inline(always)]
    pub fn black(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::BLACK)
    }
    #[doc = "Programmable constant color is shown in the active area and black color in the blanking area."]
    #[inline(always)]
    pub fn constcol(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::CONSTCOL)
    }
    #[doc = "Primary input only is shown."]
    #[inline(always)]
    pub fn prim(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::PRIM)
    }
    #[doc = "Secondary input only is shown."]
    #[inline(always)]
    pub fn sec(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::SEC)
    }
    #[doc = "Both inputs overlaid with primary on top."]
    #[inline(always)]
    pub fn prim_on_top(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::PRIM_ON_TOP)
    }
    #[doc = "Both inputs overlaid with secondary on top."]
    #[inline(always)]
    pub fn sec_on_top(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::SEC_ON_TOP)
    }
    #[doc = "White color background with test pattern is shown in the active area and black color in the blanking area."]
    #[inline(always)]
    pub fn test(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::TEST)
    }
    #[doc = "Blending of the mode PRIM with the mode SEC."]
    #[inline(always)]
    pub fn blend(self) -> &'a mut W {
        self.variant(FGDMPANIC_A::BLEND)
    }
}
#[doc = "Field `ENPRIMALPHAPANIC` reader - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
pub type ENPRIMALPHAPANIC_R = crate::BitReader<bool>;
#[doc = "Field `ENPRIMALPHAPANIC` writer - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
pub type ENPRIMALPHAPANIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FGINCTRLPANIC_SPEC, bool, O>;
#[doc = "Field `ENSECALPHAPANIC` reader - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
pub type ENSECALPHAPANIC_R = crate::BitReader<bool>;
#[doc = "Field `ENSECALPHAPANIC` writer - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
pub type ENSECALPHAPANIC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FGINCTRLPANIC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Frame Generator Display Mode."]
    #[inline(always)]
    pub fn fgdmpanic(&self) -> FGDMPANIC_R {
        FGDMPANIC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
    #[inline(always)]
    pub fn enprimalphapanic(&self) -> ENPRIMALPHAPANIC_R {
        ENPRIMALPHAPANIC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
    #[inline(always)]
    pub fn ensecalphapanic(&self) -> ENSECALPHAPANIC_R {
        ENSECALPHAPANIC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Frame Generator Display Mode."]
    #[inline(always)]
    #[must_use]
    pub fn fgdmpanic(&mut self) -> FGDMPANIC_W<0> {
        FGDMPANIC_W::new(self)
    }
    #[doc = "Bit 3 - When enabled, alpha plane of primary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
    #[inline(always)]
    #[must_use]
    pub fn enprimalphapanic(&mut self) -> ENPRIMALPHAPANIC_W<3> {
        ENPRIMALPHAPANIC_W::new(self)
    }
    #[doc = "Bit 4 - When enabled, alpha plane of secondary channel is considered for screen composition. If the alpha component of the current pixel is 255, this pixel is opaque. Otherwise this pixel is transparent. Setting this field to 1 has no effect, when FgDmPanic == BLEND."]
    #[inline(always)]
    #[must_use]
    pub fn ensecalphapanic(&mut self) -> ENSECALPHAPANIC_W<4> {
        ENSECALPHAPANIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Input Control Panic Register (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fginctrlpanic](index.html) module"]
pub struct FGINCTRLPANIC_SPEC;
impl crate::RegisterSpec for FGINCTRLPANIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fginctrlpanic::R](R) reader structure"]
impl crate::Readable for FGINCTRLPANIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fginctrlpanic::W](W) writer structure"]
impl crate::Writable for FGINCTRLPANIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGINCTRLPANIC to value 0x06"]
impl crate::Resettable for FGINCTRLPANIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
