#[doc = "Register `LAYEROFFSET3` reader"]
pub struct R(crate::R<LAYEROFFSET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET3` writer"]
pub struct W(crate::W<LAYEROFFSET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET3_SPEC>;
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
impl From<crate::W<LAYEROFFSET3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET3` reader - Horizontal offset (X)."]
pub type LAYERXOFFSET3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET3` writer - Horizontal offset (X)."]
pub type LAYERXOFFSET3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET3_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET3` reader - Vertical offset (Y)."]
pub type LAYERYOFFSET3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET3` writer - Vertical offset (Y)."]
pub type LAYERYOFFSET3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET3_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    pub fn layerxoffset3(&self) -> LAYERXOFFSET3_R {
        LAYERXOFFSET3_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    pub fn layeryoffset3(&self) -> LAYERYOFFSET3_R {
        LAYERYOFFSET3_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset3(&mut self) -> LAYERXOFFSET3_W<0> {
        LAYERXOFFSET3_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset3(&mut self) -> LAYERYOFFSET3_W<16> {
        LAYERYOFFSET3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 3 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset3](index.html) module"]
pub struct LAYEROFFSET3_SPEC;
impl crate::RegisterSpec for LAYEROFFSET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset3::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset3::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET3 to value 0"]
impl crate::Resettable for LAYEROFFSET3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
