#[doc = "Register `BASEADDRESS1` reader"]
pub struct R(crate::R<BASEADDRESS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASEADDRESS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASEADDRESS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASEADDRESS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASEADDRESS1` writer"]
pub struct W(crate::W<BASEADDRESS1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASEADDRESS1_SPEC>;
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
impl From<crate::W<BASEADDRESS1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASEADDRESS1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDRESS1` reader - See BaseAddress0."]
pub type BASEADDRESS1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADDRESS1` writer - See BaseAddress0."]
pub type BASEADDRESS1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BASEADDRESS1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - See BaseAddress0."]
    #[inline(always)]
    pub fn baseaddress1(&self) -> BASEADDRESS1_R {
        BASEADDRESS1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - See BaseAddress0."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddress1(&mut self) -> BASEADDRESS1_W<0> {
        BASEADDRESS1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer base address of layer 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddress1](index.html) module"]
pub struct BASEADDRESS1_SPEC;
impl crate::RegisterSpec for BASEADDRESS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baseaddress1::R](R) reader structure"]
impl crate::Readable for BASEADDRESS1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baseaddress1::W](W) writer structure"]
impl crate::Writable for BASEADDRESS1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASEADDRESS1 to value 0"]
impl crate::Resettable for BASEADDRESS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
