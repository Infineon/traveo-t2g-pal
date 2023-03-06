#[doc = "Register `DST_FADE_CTL` reader"]
pub struct R(crate::R<DST_FADE_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_FADE_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_FADE_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_FADE_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST_FADE_CTL` writer"]
pub struct W(crate::W<DST_FADE_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_FADE_CTL_SPEC>;
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
impl From<crate::W<DST_FADE_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_FADE_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CODE` reader - Gain code in the range \\[0, 115\\]. See DST_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'."]
pub type CODE_R = crate::FieldReader<u8, CODE_A>;
#[doc = "Gain code in the range \\[0, 115\\]. See DST_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'.\n\nValue on reset: 115"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CODE_A {
    #[doc = "0: N/A"]
    CODE_OFF = 0,
    #[doc = "115: N/A"]
    CODE_SAME = 115,
}
impl From<CODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CODE_A) -> Self {
        variant as _
    }
}
impl CODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CODE_A> {
        match self.bits {
            0 => Some(CODE_A::CODE_OFF),
            115 => Some(CODE_A::CODE_SAME),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CODE_OFF`"]
    #[inline(always)]
    pub fn is_code_off(&self) -> bool {
        *self == CODE_A::CODE_OFF
    }
    #[doc = "Checks if the value of the field is `CODE_SAME`"]
    #[inline(always)]
    pub fn is_code_same(&self) -> bool {
        *self == CODE_A::CODE_SAME
    }
}
#[doc = "Field `CODE` writer - Gain code in the range \\[0, 115\\]. See DST_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'."]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DST_FADE_CTL_SPEC, u8, CODE_A, 7, O>;
impl<'a, const O: u8> CODE_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn code_off(self) -> &'a mut W {
        self.variant(CODE_A::CODE_OFF)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn code_same(self) -> &'a mut W {
        self.variant(CODE_A::CODE_SAME)
    }
}
impl R {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 115\\]. See DST_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'."]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Gain code in the range \\[0, 115\\]. See DST_GAIN_CTL.CODE\\[\\]. Note: HW modifies this field during 'fade in' and 'fade out'."]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<0> {
        CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination fade control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_fade_ctl](index.html) module"]
pub struct DST_FADE_CTL_SPEC;
impl crate::RegisterSpec for DST_FADE_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_fade_ctl::R](R) reader structure"]
impl crate::Readable for DST_FADE_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst_fade_ctl::W](W) writer structure"]
impl crate::Writable for DST_FADE_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DST_FADE_CTL to value 0x73"]
impl crate::Resettable for DST_FADE_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x73;
}
