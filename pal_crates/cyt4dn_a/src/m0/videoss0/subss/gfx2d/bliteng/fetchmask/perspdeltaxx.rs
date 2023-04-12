#[doc = "Register `PERSPDELTAXX` reader"]
pub struct R(crate::R<PERSPDELTAXX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAXX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAXX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAXX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAXX` writer"]
pub struct W(crate::W<PERSPDELTAXX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAXX_SPEC>;
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
impl From<crate::W<PERSPDELTAXX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAXX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAXX` reader - Increment of homogenous X coordinate for horizontal step (X) in destination frame."]
pub type PERSPDELTAXX_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAXX` writer - Increment of homogenous X coordinate for horizontal step (X) in destination frame."]
pub type PERSPDELTAXX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAXX_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous X coordinate for horizontal step (X) in destination frame."]
    #[inline(always)]
    pub fn perspdeltaxx(&self) -> PERSPDELTAXX_R {
        PERSPDELTAXX_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous X coordinate for horizontal step (X) in destination frame."]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltaxx(&mut self) -> PERSPDELTAXX_W<0> {
        PERSPDELTAXX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaXX increment for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltaxx](index.html) module"]
pub struct PERSPDELTAXX_SPEC;
impl crate::RegisterSpec for PERSPDELTAXX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltaxx::R](R) reader structure"]
impl crate::Readable for PERSPDELTAXX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltaxx::W](W) writer structure"]
impl crate::Writable for PERSPDELTAXX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAXX to value 0x3f80_0000"]
impl crate::Resettable for PERSPDELTAXX_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f80_0000;
}
