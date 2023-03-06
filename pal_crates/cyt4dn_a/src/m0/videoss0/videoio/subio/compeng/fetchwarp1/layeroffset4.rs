#[doc = "Register `LAYEROFFSET4` reader"]
pub struct R(crate::R<LAYEROFFSET4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET4` writer"]
pub struct W(crate::W<LAYEROFFSET4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET4_SPEC>;
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
impl From<crate::W<LAYEROFFSET4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET4` reader - Horizontal offset (X)."]
pub type LAYERXOFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET4` writer - Horizontal offset (X)."]
pub type LAYERXOFFSET4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET4_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET4` reader - Vertical offset (Y)."]
pub type LAYERYOFFSET4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET4` writer - Vertical offset (Y)."]
pub type LAYERYOFFSET4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET4_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    pub fn layerxoffset4(&self) -> LAYERXOFFSET4_R {
        LAYERXOFFSET4_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    pub fn layeryoffset4(&self) -> LAYERYOFFSET4_R {
        LAYERYOFFSET4_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X)."]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset4(&mut self) -> LAYERXOFFSET4_W<0> {
        LAYERXOFFSET4_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y)."]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset4(&mut self) -> LAYERYOFFSET4_W<16> {
        LAYERYOFFSET4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 4 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset4](index.html) module"]
pub struct LAYEROFFSET4_SPEC;
impl crate::RegisterSpec for LAYEROFFSET4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset4::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset4::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET4 to value 0"]
impl crate::Resettable for LAYEROFFSET4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
