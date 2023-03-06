#[doc = "Register `PERSPDELTAYY` reader"]
pub struct R(crate::R<PERSPDELTAYY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERSPDELTAYY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERSPDELTAYY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERSPDELTAYY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERSPDELTAYY` writer"]
pub struct W(crate::W<PERSPDELTAYY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERSPDELTAYY_SPEC>;
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
impl From<crate::W<PERSPDELTAYY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERSPDELTAYY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERSPDELTAYY` reader - Increment of homogenous Y coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAYY_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERSPDELTAYY` writer - Increment of homogenous Y coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
pub type PERSPDELTAYY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PERSPDELTAYY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Increment of homogenous Y coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    pub fn perspdeltayy(&self) -> PERSPDELTAYY_R {
        PERSPDELTAYY_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Increment of homogenous Y coordinate for vertical step (Y) in destination frame. (format is floating-point 1e8m23)"]
    #[inline(always)]
    #[must_use]
    pub fn perspdeltayy(&mut self) -> PERSPDELTAYY_W<0> {
        PERSPDELTAYY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DeltaYY increment for affine/perspective warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perspdeltayy](index.html) module"]
pub struct PERSPDELTAYY_SPEC;
impl crate::RegisterSpec for PERSPDELTAYY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perspdeltayy::R](R) reader structure"]
impl crate::Readable for PERSPDELTAYY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perspdeltayy::W](W) writer structure"]
impl crate::Writable for PERSPDELTAYY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERSPDELTAYY to value 0x3f80_0000"]
impl crate::Resettable for PERSPDELTAYY_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f80_0000;
}
