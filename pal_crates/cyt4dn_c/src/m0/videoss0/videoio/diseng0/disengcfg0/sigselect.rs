#[doc = "Register `SIGSELECT` reader"]
pub struct R(crate::R<SIGSELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGSELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGSELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGSELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGSELECT` writer"]
pub struct W(crate::W<SIGSELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGSELECT_SPEC>;
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
impl From<crate::W<SIGSELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGSELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_SELECT` reader - Selects a source for Sig unit."]
pub type SIG_SELECT_R = crate::FieldReader<u8, SIG_SELECT_A>;
#[doc = "Selects a source for Sig unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SIG_SELECT_A {
    #[doc = "0: No source."]
    NONE = 0,
    #[doc = "1: Source is FrameGen output."]
    FRAMEGEN = 1,
    #[doc = "2: Source is GammaCor output."]
    GAMMACOR = 2,
    #[doc = "3: Source is Dither output."]
    DITHER = 3,
}
impl From<SIG_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SIG_SELECT_A) -> Self {
        variant as _
    }
}
impl SIG_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIG_SELECT_A {
        match self.bits {
            0 => SIG_SELECT_A::NONE,
            1 => SIG_SELECT_A::FRAMEGEN,
            2 => SIG_SELECT_A::GAMMACOR,
            3 => SIG_SELECT_A::DITHER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SIG_SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `FRAMEGEN`"]
    #[inline(always)]
    pub fn is_framegen(&self) -> bool {
        *self == SIG_SELECT_A::FRAMEGEN
    }
    #[doc = "Checks if the value of the field is `GAMMACOR`"]
    #[inline(always)]
    pub fn is_gammacor(&self) -> bool {
        *self == SIG_SELECT_A::GAMMACOR
    }
    #[doc = "Checks if the value of the field is `DITHER`"]
    #[inline(always)]
    pub fn is_dither(&self) -> bool {
        *self == SIG_SELECT_A::DITHER
    }
}
#[doc = "Field `SIG_SELECT` writer - Selects a source for Sig unit."]
pub type SIG_SELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SIGSELECT_SPEC, u8, SIG_SELECT_A, 2, O>;
impl<'a, const O: u8> SIG_SELECT_W<'a, O> {
    #[doc = "No source."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SIG_SELECT_A::NONE)
    }
    #[doc = "Source is FrameGen output."]
    #[inline(always)]
    pub fn framegen(self) -> &'a mut W {
        self.variant(SIG_SELECT_A::FRAMEGEN)
    }
    #[doc = "Source is GammaCor output."]
    #[inline(always)]
    pub fn gammacor(self) -> &'a mut W {
        self.variant(SIG_SELECT_A::GAMMACOR)
    }
    #[doc = "Source is Dither output."]
    #[inline(always)]
    pub fn dither(self) -> &'a mut W {
        self.variant(SIG_SELECT_A::DITHER)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects a source for Sig unit."]
    #[inline(always)]
    pub fn sig_select(&self) -> SIG_SELECT_R {
        SIG_SELECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects a source for Sig unit."]
    #[inline(always)]
    #[must_use]
    pub fn sig_select(&mut self) -> SIG_SELECT_W<0> {
        SIG_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tap selection for Signature.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigselect](index.html) module"]
pub struct SIGSELECT_SPEC;
impl crate::RegisterSpec for SIGSELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigselect::R](R) reader structure"]
impl crate::Readable for SIGSELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigselect::W](W) writer structure"]
impl crate::Writable for SIGSELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIGSELECT to value 0"]
impl crate::Resettable for SIGSELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
