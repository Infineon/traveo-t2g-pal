#[doc = "Register `EVALLOWERRIGHT1` reader"]
pub struct R(crate::R<EVALLOWERRIGHT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALLOWERRIGHT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALLOWERRIGHT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALLOWERRIGHT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALLOWERRIGHT1` writer"]
pub struct W(crate::W<EVALLOWERRIGHT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALLOWERRIGHT1_SPEC>;
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
impl From<crate::W<EVALLOWERRIGHT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALLOWERRIGHT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALLOWERRIGHT1` reader - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALLOWERRIGHT1` writer - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT1_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALLOWERRIGHT1` reader - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALLOWERRIGHT1` writer - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT1_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    pub fn xevallowerright1(&self) -> XEVALLOWERRIGHT1_R {
        XEVALLOWERRIGHT1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    pub fn yevallowerright1(&self) -> YEVALLOWERRIGHT1_R {
        YEVALLOWERRIGHT1_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn xevallowerright1(&mut self) -> XEVALLOWERRIGHT1_W<0> {
        XEVALLOWERRIGHT1_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn yevallowerright1(&mut self) -> YEVALLOWERRIGHT1_W<16> {
        YEVALLOWERRIGHT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower right corner of evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evallowerright1](index.html) module"]
pub struct EVALLOWERRIGHT1_SPEC;
impl crate::RegisterSpec for EVALLOWERRIGHT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evallowerright1::R](R) reader structure"]
impl crate::Readable for EVALLOWERRIGHT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evallowerright1::W](W) writer structure"]
impl crate::Writable for EVALLOWERRIGHT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALLOWERRIGHT1 to value 0"]
impl crate::Resettable for EVALLOWERRIGHT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
