#[doc = "Register `EVALLOWERRIGHT2` reader"]
pub struct R(crate::R<EVALLOWERRIGHT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALLOWERRIGHT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALLOWERRIGHT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALLOWERRIGHT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALLOWERRIGHT2` writer"]
pub struct W(crate::W<EVALLOWERRIGHT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALLOWERRIGHT2_SPEC>;
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
impl From<crate::W<EVALLOWERRIGHT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALLOWERRIGHT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XEVALLOWERRIGHT2` reader - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XEVALLOWERRIGHT2` writer - See XEvalLowerRight0."]
pub type XEVALLOWERRIGHT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT2_SPEC, u16, u16, 14, O>;
#[doc = "Field `YEVALLOWERRIGHT2` reader - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YEVALLOWERRIGHT2` writer - See YEvalLowerRight0."]
pub type YEVALLOWERRIGHT2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVALLOWERRIGHT2_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    pub fn xevallowerright2(&self) -> XEVALLOWERRIGHT2_R {
        XEVALLOWERRIGHT2_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    pub fn yevallowerright2(&self) -> YEVALLOWERRIGHT2_R {
        YEVALLOWERRIGHT2_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - See XEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn xevallowerright2(&mut self) -> XEVALLOWERRIGHT2_W<0> {
        XEVALLOWERRIGHT2_W::new(self)
    }
    #[doc = "Bits 16:29 - See YEvalLowerRight0."]
    #[inline(always)]
    #[must_use]
    pub fn yevallowerright2(&mut self) -> YEVALLOWERRIGHT2_W<16> {
        YEVALLOWERRIGHT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower right corner of evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evallowerright2](index.html) module"]
pub struct EVALLOWERRIGHT2_SPEC;
impl crate::RegisterSpec for EVALLOWERRIGHT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evallowerright2::R](R) reader structure"]
impl crate::Readable for EVALLOWERRIGHT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evallowerright2::W](W) writer structure"]
impl crate::Writable for EVALLOWERRIGHT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALLOWERRIGHT2 to value 0"]
impl crate::Resettable for EVALLOWERRIGHT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
