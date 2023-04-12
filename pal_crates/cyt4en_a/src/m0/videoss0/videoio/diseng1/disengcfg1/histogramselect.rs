#[doc = "Register `HISTOGRAMSELECT` reader"]
pub struct R(crate::R<HISTOGRAMSELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HISTOGRAMSELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HISTOGRAMSELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HISTOGRAMSELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HISTOGRAMSELECT` writer"]
pub struct W(crate::W<HISTOGRAMSELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HISTOGRAMSELECT_SPEC>;
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
impl From<crate::W<HISTOGRAMSELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HISTOGRAMSELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HISTOGRAM_SELECT` reader - Selects a source for Histogram unit."]
pub type HISTOGRAM_SELECT_R = crate::FieldReader<u8, HISTOGRAM_SELECT_A>;
#[doc = "Selects a source for Histogram unit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HISTOGRAM_SELECT_A {
    #[doc = "0: No source."]
    NONE = 0,
    #[doc = "1: Source is FrameGen output."]
    FRAMEGEN = 1,
    #[doc = "2: Source is GammaCor output."]
    GAMMACOR = 2,
    #[doc = "3: Source is Dither output."]
    DITHER = 3,
}
impl From<HISTOGRAM_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: HISTOGRAM_SELECT_A) -> Self {
        variant as _
    }
}
impl HISTOGRAM_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HISTOGRAM_SELECT_A {
        match self.bits {
            0 => HISTOGRAM_SELECT_A::NONE,
            1 => HISTOGRAM_SELECT_A::FRAMEGEN,
            2 => HISTOGRAM_SELECT_A::GAMMACOR,
            3 => HISTOGRAM_SELECT_A::DITHER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == HISTOGRAM_SELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `FRAMEGEN`"]
    #[inline(always)]
    pub fn is_framegen(&self) -> bool {
        *self == HISTOGRAM_SELECT_A::FRAMEGEN
    }
    #[doc = "Checks if the value of the field is `GAMMACOR`"]
    #[inline(always)]
    pub fn is_gammacor(&self) -> bool {
        *self == HISTOGRAM_SELECT_A::GAMMACOR
    }
    #[doc = "Checks if the value of the field is `DITHER`"]
    #[inline(always)]
    pub fn is_dither(&self) -> bool {
        *self == HISTOGRAM_SELECT_A::DITHER
    }
}
#[doc = "Field `HISTOGRAM_SELECT` writer - Selects a source for Histogram unit."]
pub type HISTOGRAM_SELECT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HISTOGRAMSELECT_SPEC, u8, HISTOGRAM_SELECT_A, 2, O>;
impl<'a, const O: u8> HISTOGRAM_SELECT_W<'a, O> {
    #[doc = "No source."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(HISTOGRAM_SELECT_A::NONE)
    }
    #[doc = "Source is FrameGen output."]
    #[inline(always)]
    pub fn framegen(self) -> &'a mut W {
        self.variant(HISTOGRAM_SELECT_A::FRAMEGEN)
    }
    #[doc = "Source is GammaCor output."]
    #[inline(always)]
    pub fn gammacor(self) -> &'a mut W {
        self.variant(HISTOGRAM_SELECT_A::GAMMACOR)
    }
    #[doc = "Source is Dither output."]
    #[inline(always)]
    pub fn dither(self) -> &'a mut W {
        self.variant(HISTOGRAM_SELECT_A::DITHER)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects a source for Histogram unit."]
    #[inline(always)]
    pub fn histogram_select(&self) -> HISTOGRAM_SELECT_R {
        HISTOGRAM_SELECT_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects a source for Histogram unit."]
    #[inline(always)]
    #[must_use]
    pub fn histogram_select(&mut self) -> HISTOGRAM_SELECT_W<0> {
        HISTOGRAM_SELECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tap selection for Histogram.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [histogramselect](index.html) module"]
pub struct HISTOGRAMSELECT_SPEC;
impl crate::RegisterSpec for HISTOGRAMSELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [histogramselect::R](R) reader structure"]
impl crate::Readable for HISTOGRAMSELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [histogramselect::W](W) writer structure"]
impl crate::Writable for HISTOGRAMSELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HISTOGRAMSELECT to value 0"]
impl crate::Resettable for HISTOGRAMSELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
