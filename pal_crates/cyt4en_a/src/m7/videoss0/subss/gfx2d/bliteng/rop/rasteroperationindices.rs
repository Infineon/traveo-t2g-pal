#[doc = "Register `RASTEROPERATIONINDICES` reader"]
pub struct R(crate::R<RASTEROPERATIONINDICES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RASTEROPERATIONINDICES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RASTEROPERATIONINDICES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RASTEROPERATIONINDICES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RASTEROPERATIONINDICES` writer"]
pub struct W(crate::W<RASTEROPERATIONINDICES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RASTEROPERATIONINDICES_SPEC>;
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
impl From<crate::W<RASTEROPERATIONINDICES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RASTEROPERATIONINDICES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPINDEXALPHA` reader - Alpha operation index"]
pub type OPINDEXALPHA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPINDEXALPHA` writer - Alpha operation index"]
pub type OPINDEXALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RASTEROPERATIONINDICES_SPEC, u8, u8, 8, O>;
#[doc = "Field `OPINDEXBLUE` reader - Blue operation index"]
pub type OPINDEXBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPINDEXBLUE` writer - Blue operation index"]
pub type OPINDEXBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RASTEROPERATIONINDICES_SPEC, u8, u8, 8, O>;
#[doc = "Field `OPINDEXGREEN` reader - Green operation index"]
pub type OPINDEXGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPINDEXGREEN` writer - Green operation index"]
pub type OPINDEXGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RASTEROPERATIONINDICES_SPEC, u8, u8, 8, O>;
#[doc = "Field `OPINDEXRED` reader - Red operation index"]
pub type OPINDEXRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPINDEXRED` writer - Red operation index"]
pub type OPINDEXRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RASTEROPERATIONINDICES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Alpha operation index"]
    #[inline(always)]
    pub fn opindexalpha(&self) -> OPINDEXALPHA_R {
        OPINDEXALPHA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Blue operation index"]
    #[inline(always)]
    pub fn opindexblue(&self) -> OPINDEXBLUE_R {
        OPINDEXBLUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green operation index"]
    #[inline(always)]
    pub fn opindexgreen(&self) -> OPINDEXGREEN_R {
        OPINDEXGREEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red operation index"]
    #[inline(always)]
    pub fn opindexred(&self) -> OPINDEXRED_R {
        OPINDEXRED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Alpha operation index"]
    #[inline(always)]
    #[must_use]
    pub fn opindexalpha(&mut self) -> OPINDEXALPHA_W<0> {
        OPINDEXALPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue operation index"]
    #[inline(always)]
    #[must_use]
    pub fn opindexblue(&mut self) -> OPINDEXBLUE_W<8> {
        OPINDEXBLUE_W::new(self)
    }
    #[doc = "Bits 16:23 - Green operation index"]
    #[inline(always)]
    #[must_use]
    pub fn opindexgreen(&mut self) -> OPINDEXGREEN_W<16> {
        OPINDEXGREEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Red operation index"]
    #[inline(always)]
    #[must_use]
    pub fn opindexred(&mut self) -> OPINDEXRED_W<24> {
        OPINDEXRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ROP operation indices\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rasteroperationindices](index.html) module"]
pub struct RASTEROPERATIONINDICES_SPEC;
impl crate::RegisterSpec for RASTEROPERATIONINDICES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rasteroperationindices::R](R) reader structure"]
impl crate::Readable for RASTEROPERATIONINDICES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rasteroperationindices::W](W) writer structure"]
impl crate::Writable for RASTEROPERATIONINDICES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RASTEROPERATIONINDICES to value 0"]
impl crate::Resettable for RASTEROPERATIONINDICES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
