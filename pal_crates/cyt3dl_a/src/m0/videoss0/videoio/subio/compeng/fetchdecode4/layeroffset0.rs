#[doc = "Register `LAYEROFFSET0` reader"]
pub struct R(crate::R<LAYEROFFSET0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LAYEROFFSET0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LAYEROFFSET0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LAYEROFFSET0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LAYEROFFSET0` writer"]
pub struct W(crate::W<LAYEROFFSET0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LAYEROFFSET0_SPEC>;
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
impl From<crate::W<LAYEROFFSET0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LAYEROFFSET0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LAYERXOFFSET0` reader - Horizontal offset (X). (format is signed integer)"]
pub type LAYERXOFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERXOFFSET0` writer - Horizontal offset (X). (format is signed integer)"]
pub type LAYERXOFFSET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET0_SPEC, u16, u16, 15, O>;
#[doc = "Field `LAYERYOFFSET0` reader - Vertical offset (Y). (format is signed integer)"]
pub type LAYERYOFFSET0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LAYERYOFFSET0` writer - Vertical offset (Y). (format is signed integer)"]
pub type LAYERYOFFSET0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LAYEROFFSET0_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - Horizontal offset (X). (format is signed integer)"]
    #[inline(always)]
    pub fn layerxoffset0(&self) -> LAYERXOFFSET0_R {
        LAYERXOFFSET0_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y). (format is signed integer)"]
    #[inline(always)]
    pub fn layeryoffset0(&self) -> LAYERYOFFSET0_R {
        LAYERYOFFSET0_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Horizontal offset (X). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn layerxoffset0(&mut self) -> LAYERXOFFSET0_W<0> {
        LAYERXOFFSET0_W::new(self)
    }
    #[doc = "Bits 16:30 - Vertical offset (Y). (format is signed integer)"]
    #[inline(always)]
    #[must_use]
    pub fn layeryoffset0(&mut self) -> LAYERYOFFSET0_W<16> {
        LAYERYOFFSET0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of layer 0 within the destination frame.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [layeroffset0](index.html) module"]
pub struct LAYEROFFSET0_SPEC;
impl crate::RegisterSpec for LAYEROFFSET0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [layeroffset0::R](R) reader structure"]
impl crate::Readable for LAYEROFFSET0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [layeroffset0::W](W) writer structure"]
impl crate::Writable for LAYEROFFSET0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LAYEROFFSET0 to value 0"]
impl crate::Resettable for LAYEROFFSET0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
