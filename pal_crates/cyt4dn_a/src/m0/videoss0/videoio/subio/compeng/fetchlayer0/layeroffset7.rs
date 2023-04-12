#[doc = "Register `LAYEROFFSET7` reader"]
pub struct R(crate::R<LAYEROFFSET7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET7` writer"]
pub struct W(crate::W<LAYEROFFSET7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET7_SPEC>;
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
impl From<crate::W<LAYEROFFSET7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET7` reader - Horizontal offset (X)."]
pub type LAYERXOFFSET7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET7` writer - Horizontal offset (X)."]
pub type LAYERXOFFSET7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET7_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET7` reader - Vertical offset (Y)."]
pub type LAYERYOFFSET7_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET7` writer - Vertical offset (Y)."]
pub type LAYERYOFFSET7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET7_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    pub fn layerxoffset7(&self) -> LAYERXOFFSET7_R {
        LAYERXOFFSET7_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    pub fn layeryoffset7(&self) -> LAYERYOFFSET7_R {
        LAYERYOFFSET7_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset7(&mut self) -> LAYERXOFFSET7_W<0> {
        LAYERXOFFSET7_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset7(&mut self) -> LAYERYOFFSET7_W<16> {
        LAYERYOFFSET7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 7 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset7](index.html) module"]
pub struct LAYEROFFSET7_SPEC;
impl crate::RegisterSpec for LAYEROFFSET7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset7::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset7::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET7 to value 0"]
impl crate::Resettable for LAYEROFFSET7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
