#[doc = "Register `LAYEROFFSET5` reader"]
pub struct R(crate::R<LAYEROFFSET5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET5` writer"]
pub struct W(crate::W<LAYEROFFSET5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET5_SPEC>;
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
impl From<crate::W<LAYEROFFSET5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET5` reader - Horizontal offset (X)."]
pub type LAYERXOFFSET5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET5` writer - Horizontal offset (X)."]
pub type LAYERXOFFSET5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET5_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET5` reader - Vertical offset (Y)."]
pub type LAYERYOFFSET5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET5` writer - Vertical offset (Y)."]
pub type LAYERYOFFSET5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET5_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    pub fn layerxoffset5(&self) -> LAYERXOFFSET5_R {
        LAYERXOFFSET5_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    pub fn layeryoffset5(&self) -> LAYERYOFFSET5_R {
        LAYERYOFFSET5_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset5(&mut self) -> LAYERXOFFSET5_W<0> {
        LAYERXOFFSET5_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset5(&mut self) -> LAYERYOFFSET5_W<16> {
        LAYERYOFFSET5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 5 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset5](index.html) module"]
pub struct LAYEROFFSET5_SPEC;
impl crate::RegisterSpec for LAYEROFFSET5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset5::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset5::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET5 to value 0"]
impl crate::Resettable for LAYEROFFSET5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
