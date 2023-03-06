#[doc = "Register `CROPSIZEX` reader"]
pub struct R(crate::R<CROPSIZEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CROPSIZEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CROPSIZEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CROPSIZEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CROPSIZEX` writer"]
pub struct W(crate::W<CROPSIZEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CROPSIZEX_SPEC>;
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
impl From<crate::W<CROPSIZEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CROPSIZEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CROPSIZEX` reader - Crop size for X direction"]
pub type CROPSIZEX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CROPSIZEX` writer - Crop size for X direction"]
pub type CROPSIZEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CROPSIZEX_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Crop size for X direction"]
    #[inline(always)]
    pub fn cropsizex(&self) -> CROPSIZEX_R {
        CROPSIZEX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Crop size for X direction"]
    #[inline(always)]
    #[must_use]
    pub fn cropsizex(&mut self) -> CROPSIZEX_W<0> {
        CROPSIZEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crop size for X direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cropsizex](index.html) module"]
pub struct CROPSIZEX_SPEC;
impl crate::RegisterSpec for CROPSIZEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cropsizex::R](R) reader structure"]
impl crate::Readable for CROPSIZEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cropsizex::W](W) writer structure"]
impl crate::Writable for CROPSIZEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CROPSIZEX to value 0"]
impl crate::Resettable for CROPSIZEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
