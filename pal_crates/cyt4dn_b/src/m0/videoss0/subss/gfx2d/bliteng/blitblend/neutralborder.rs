#[doc = "Register `NEUTRALBORDER` reader"]
pub struct R(crate::R<NEUTRALBORDER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEUTRALBORDER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEUTRALBORDER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEUTRALBORDER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NEUTRALBORDER` writer"]
pub struct W(crate::W<NEUTRALBORDER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NEUTRALBORDER_SPEC>;
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
impl From<crate::W<NEUTRALBORDER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NEUTRALBORDER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NEUTRALBORDERMODE` reader - Chooses whether to bypass primary or secondary input pixels"]
pub type NEUTRALBORDERMODE_R = crate::BitReader<NEUTRALBORDERMODE_A>;
#[doc = "Chooses whether to bypass primary or secondary input pixels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEUTRALBORDERMODE_A {
    #[doc = "0: Bypasses primary pixel"]
    PRIMARY = 0,
    #[doc = "1: Bypasses secondary pixel"]
    SECONDARY = 1,
}
impl From<NEUTRALBORDERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: NEUTRALBORDERMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl NEUTRALBORDERMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEUTRALBORDERMODE_A {
        match self.bits {
            false => NEUTRALBORDERMODE_A::PRIMARY,
            true => NEUTRALBORDERMODE_A::SECONDARY,
        }
    }
    #[doc = "Checks if the value of the field is `PRIMARY`"]
    #[inline(always)]
    pub fn is_primary(&self) -> bool {
        *self == NEUTRALBORDERMODE_A::PRIMARY
    }
    #[doc = "Checks if the value of the field is `SECONDARY`"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == NEUTRALBORDERMODE_A::SECONDARY
    }
}
#[doc = "Field `NEUTRALBORDERMODE` writer - Chooses whether to bypass primary or secondary input pixels"]
pub type NEUTRALBORDERMODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, NEUTRALBORDER_SPEC, NEUTRALBORDERMODE_A, O>;
impl<'a, const O: u8> NEUTRALBORDERMODE_W<'a, O> {
    #[doc = "Bypasses primary pixel"]
    #[inline(always)]
    pub fn primary(self) -> &'a mut W {
        self.variant(NEUTRALBORDERMODE_A::PRIMARY)
    }
    #[doc = "Bypasses secondary pixel"]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut W {
        self.variant(NEUTRALBORDERMODE_A::SECONDARY)
    }
}
#[doc = "Field `NEUTRALBORDERLEFT` reader - Number of neutral left border pixels"]
pub type NEUTRALBORDERLEFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEUTRALBORDERLEFT` writer - Number of neutral left border pixels"]
pub type NEUTRALBORDERLEFT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NEUTRALBORDER_SPEC, u8, u8, 3, O>;
#[doc = "Field `NEUTRALBORDERRIGHT` reader - Number of neutral right border pixels"]
pub type NEUTRALBORDERRIGHT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEUTRALBORDERRIGHT` writer - Number of neutral right border pixels"]
pub type NEUTRALBORDERRIGHT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, NEUTRALBORDER_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Chooses whether to bypass primary or secondary input pixels"]
    #[inline(always)]
    pub fn neutralbordermode(&self) -> NEUTRALBORDERMODE_R {
        NEUTRALBORDERMODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Number of neutral left border pixels"]
    #[inline(always)]
    pub fn neutralborderleft(&self) -> NEUTRALBORDERLEFT_R {
        NEUTRALBORDERLEFT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Number of neutral right border pixels"]
    #[inline(always)]
    pub fn neutralborderright(&self) -> NEUTRALBORDERRIGHT_R {
        NEUTRALBORDERRIGHT_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Chooses whether to bypass primary or secondary input pixels"]
    #[inline(always)]
    #[must_use]
    pub fn neutralbordermode(&mut self) -> NEUTRALBORDERMODE_W<0> {
        NEUTRALBORDERMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Number of neutral left border pixels"]
    #[inline(always)]
    #[must_use]
    pub fn neutralborderleft(&mut self) -> NEUTRALBORDERLEFT_W<8> {
        NEUTRALBORDERLEFT_W::new(self)
    }
    #[doc = "Bits 12:14 - Number of neutral right border pixels"]
    #[inline(always)]
    #[must_use]
    pub fn neutralborderright(&mut self) -> NEUTRALBORDERRIGHT_W<12> {
        NEUTRALBORDERRIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Neutral border setup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [neutralborder](index.html) module"]
pub struct NEUTRALBORDER_SPEC;
impl crate::RegisterSpec for NEUTRALBORDER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [neutralborder::R](R) reader structure"]
impl crate::Readable for NEUTRALBORDER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [neutralborder::W](W) writer structure"]
impl crate::Writable for NEUTRALBORDER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NEUTRALBORDER to value 0"]
impl crate::Resettable for NEUTRALBORDER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
