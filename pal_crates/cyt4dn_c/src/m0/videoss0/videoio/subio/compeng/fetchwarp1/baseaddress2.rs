#[doc = "Register `BASEADDRESS2` reader"]
pub struct R(crate::R<BASEADDRESS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASEADDRESS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASEADDRESS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASEADDRESS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASEADDRESS2` writer"]
pub struct W(crate::W<BASEADDRESS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASEADDRESS2_SPEC>;
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
impl From<crate::W<BASEADDRESS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASEADDRESS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDRESS2` reader - See BaseAddress0."]
pub type BASEADDRESS2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADDRESS2` writer - See BaseAddress0."]
pub type BASEADDRESS2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BASEADDRESS2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See BaseAddress0."]
    #[inline(always)]
    pub fn baseaddress2(&self) -> BASEADDRESS2_R {
        BASEADDRESS2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See BaseAddress0."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddress2(&mut self) -> BASEADDRESS2_W<0> {
        BASEADDRESS2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer base address of layer 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddress2](index.html) module"]
pub struct BASEADDRESS2_SPEC;
impl crate::RegisterSpec for BASEADDRESS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baseaddress2::R](R) reader structure"]
impl crate::Readable for BASEADDRESS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baseaddress2::W](W) writer structure"]
impl crate::Writable for BASEADDRESS2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASEADDRESS2 to value 0"]
impl crate::Resettable for BASEADDRESS2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
