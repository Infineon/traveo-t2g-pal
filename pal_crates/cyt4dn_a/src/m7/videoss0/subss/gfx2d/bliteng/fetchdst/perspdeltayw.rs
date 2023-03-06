#[doc = "Register `PERSPDELTAYW` reader"]
pub struct R(crate::R<PERSPDELTAYW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAYW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAYW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAYW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAYW` writer"]
pub struct W(crate::W<PERSPDELTAYW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAYW_SPEC>;
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
impl From<crate::W<PERSPDELTAYW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAYW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAYW` reader - Increment of homogenous W coordinate for vertical step (Y) in destination frame."]
pub type PERSPDELTAYW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAYW` writer - Increment of homogenous W coordinate for vertical step (Y) in destination frame."]
pub type PERSPDELTAYW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAYW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous W coordinate for vertical step (Y) in destination frame."]
    #[inline(always)]
    pub fn perspdeltayw(&self) -> PERSPDELTAYW_R {
        PERSPDELTAYW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous W coordinate for vertical step (Y) in destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltayw(&mut self) -> PERSPDELTAYW_W<0> {
        PERSPDELTAYW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaWY increment for perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltayw](index.html) module"]
pub struct PERSPDELTAYW_SPEC;
impl crate::RegisterSpec for PERSPDELTAYW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltayw::R](R) reader structure"]
impl crate::Readable for PERSPDELTAYW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltayw::W](W) writer structure"]
impl crate::Writable for PERSPDELTAYW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAYW to value 0"]
impl crate::Resettable for PERSPDELTAYW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
