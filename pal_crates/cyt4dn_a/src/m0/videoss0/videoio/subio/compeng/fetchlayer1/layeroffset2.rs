#[doc = "Register `LAYEROFFSET2` reader"]
pub struct R(crate::R<LAYEROFFSET2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET2` writer"]
pub struct W(crate::W<LAYEROFFSET2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET2_SPEC>;
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
impl From<crate::W<LAYEROFFSET2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET2` reader - Horizontal offset (X)."]
pub type LAYERXOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET2` writer - Horizontal offset (X)."]
pub type LAYERXOFFSET2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET2_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET2` reader - Vertical offset (Y)."]
pub type LAYERYOFFSET2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET2` writer - Vertical offset (Y)."]
pub type LAYERYOFFSET2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET2_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    pub fn layerxoffset2(&self) -> LAYERXOFFSET2_R {
        LAYERXOFFSET2_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    pub fn layeryoffset2(&self) -> LAYERYOFFSET2_R {
        LAYERYOFFSET2_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset2(&mut self) -> LAYERXOFFSET2_W<0> {
        LAYERXOFFSET2_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset2(&mut self) -> LAYERYOFFSET2_W<16> {
        LAYERYOFFSET2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 2 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset2](index.html) module"]
pub struct LAYEROFFSET2_SPEC;
impl crate::RegisterSpec for LAYEROFFSET2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset2::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset2::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET2 to value 0"]
impl crate::Resettable for LAYEROFFSET2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
