#[doc = "Register `CROPSIZEY` reader"]
pub struct R(crate::R<CROPSIZEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CROPSIZEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CROPSIZEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CROPSIZEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CROPSIZEY` writer"]
pub struct W(crate::W<CROPSIZEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CROPSIZEY_SPEC>;
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
impl From<crate::W<CROPSIZEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CROPSIZEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CROPSIZEY` reader - Crop size for Y direction"]
pub type CROPSIZEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CROPSIZEY` writer - Crop size for Y direction"]
pub type CROPSIZEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CROPSIZEY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Crop size for Y direction"]
    #[inline(always)]
    pub fn cropsizey(&self) -> CROPSIZEY_R {
        CROPSIZEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Crop size for Y direction"]
    #[inline(always)]
    #[must_use]
    pub fn cropsizey(&mut self) -> CROPSIZEY_W<0> {
        CROPSIZEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crop size for Y direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cropsizey](index.html) module"]
pub struct CROPSIZEY_SPEC;
impl crate::RegisterSpec for CROPSIZEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cropsizey::R](R) reader structure"]
impl crate::Readable for CROPSIZEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cropsizey::W](W) writer structure"]
impl crate::Writable for CROPSIZEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CROPSIZEY to value 0"]
impl crate::Resettable for CROPSIZEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
