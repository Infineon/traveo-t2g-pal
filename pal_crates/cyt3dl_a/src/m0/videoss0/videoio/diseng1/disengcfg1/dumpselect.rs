#[doc = "Register `DUMPSELECT` reader"]
pub struct R(crate::R<DUMPSELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUMPSELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUMPSELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUMPSELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUMPSELECT` writer"]
pub struct W(crate::W<DUMPSELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUMPSELECT_SPEC>;
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
impl From<crate::W<DUMPSELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUMPSELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUMP_SELECT` reader - Selects a source for FrameDump unit."]
pub type DUMP_SELECT_R = crate::FieldReader<u8, DUMP_SELECT_A>;
#[doc = "Selects a source for FrameDump unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUMP_SELECT_A {
    #[doc = "0: No source."]
    NONE = 0,
    #[doc = "1: Source is FrameGen output."]
    FRAMEGEN = 1,
    #[doc = "2: Source is GammaCor output."]
    GAMMACOR = 2,
    #[doc = "3: Source is Dither output."]
    DITHER = 3,
}
impl From<DUMP_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DUMP_SELECT_A) -> Self {
        variant as _
    }
}
impl DUMP_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUMP_SELECT_A {
        match self.bits {
            0 => DUMP_SELECT_A::NONE,
            1 => DUMP_SELECT_A::FRAMEGEN,
            2 => DUMP_SELECT_A::GAMMACOR,
            3 => DUMP_SELECT_A::DITHER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DUMP_SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `FRAMEGEN`"]
    #[inline(always)]
    pub fn is_framegen(&self) -> bool {
        *self == DUMP_SELECT_A::FRAMEGEN
    }
    #[doc = "Checks if the value of the field is `GAMMACOR`"]
    #[inline(always)]
    pub fn is_gammacor(&self) -> bool {
        *self == DUMP_SELECT_A::GAMMACOR
    }
    #[doc = "Checks if the value of the field is `DITHER`"]
    #[inline(always)]
    pub fn is_dither(&self) -> bool {
        *self == DUMP_SELECT_A::DITHER
    }
}
#[doc = "Field `DUMP_SELECT` writer - Selects a source for FrameDump unit."]
pub type DUMP_SELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DUMPSELECT_SPEC, u8, DUMP_SELECT_A, 2, O>;
impl<'a, const O: u8> DUMP_SELECT_W<'a, O> {
    #[doc = "No source."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DUMP_SELECT_A::NONE)
    }
    #[doc = "Source is FrameGen output."]
    #[inline(always)]
    pub fn framegen(self) -> &'a mut W {
        self.variant(DUMP_SELECT_A::FRAMEGEN)
    }
    #[doc = "Source is GammaCor output."]
    #[inline(always)]
    pub fn gammacor(self) -> &'a mut W {
        self.variant(DUMP_SELECT_A::GAMMACOR)
    }
    #[doc = "Source is Dither output."]
    #[inline(always)]
    pub fn dither(self) -> &'a mut W {
        self.variant(DUMP_SELECT_A::DITHER)
    }
}
impl R {
    #[doc = "Bits 8:9 - Selects a source for FrameDump unit."]
    #[inline(always)]
    pub fn dump_select(&self) -> DUMP_SELECT_R {
        DUMP_SELECT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Selects a source for FrameDump unit."]
    #[inline(always)]
    #[must_use]
    pub fn dump_select(&mut self) -> DUMP_SELECT_W<8> {
        DUMP_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tap selection for Frame Dump.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dumpselect](index.html) module"]
pub struct DUMPSELECT_SPEC;
impl crate::RegisterSpec for DUMPSELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dumpselect::R](R) reader structure"]
impl crate::Readable for DUMPSELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dumpselect::W](W) writer structure"]
impl crate::Writable for DUMPSELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DUMPSELECT to value 0"]
impl crate::Resettable for DUMPSELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
