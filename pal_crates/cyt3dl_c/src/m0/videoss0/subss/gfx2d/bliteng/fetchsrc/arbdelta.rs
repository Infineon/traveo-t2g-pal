#[doc = "Register `ARBDELTA` reader"]
pub struct R(crate::R<ARBDELTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARBDELTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARBDELTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARBDELTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARBDELTA` writer"]
pub struct W(crate::W<ARBDELTA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARBDELTA_SPEC>;
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
impl From<crate::W<ARBDELTA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARBDELTA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARBDELTAXX` reader - X coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAXX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARBDELTAXX` writer - X coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAXX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARBDELTA_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARBDELTAXY` reader - Y coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAXY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARBDELTAXY` writer - Y coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAXY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARBDELTA_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARBDELTAYX` reader - X coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAYX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARBDELTAYX` writer - X coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAYX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARBDELTA_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARBDELTAYY` reader - Y coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAYY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARBDELTAYY` writer - Y coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
pub type ARBDELTAYY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARBDELTA_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - X coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    pub fn arbdeltaxx(&self) -> ARBDELTAXX_R {
        ARBDELTAXX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Y coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    pub fn arbdeltaxy(&self) -> ARBDELTAXY_R {
        ARBDELTAXY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - X coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    pub fn arbdeltayx(&self) -> ARBDELTAYX_R {
        ARBDELTAYX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Y coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    pub fn arbdeltayy(&self) -> ARBDELTAYY_R {
        ARBDELTAYY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - X coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    #[must_use]
    pub fn arbdeltaxx(&mut self) -> ARBDELTAXX_W<0> {
        ARBDELTAXX_W::new(self)
    }
    #[doc = "Bits 8:15 - Y coordinate of vector between first and second sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    #[must_use]
    pub fn arbdeltaxy(&mut self) -> ARBDELTAXY_W<8> {
        ARBDELTAXY_W::new(self)
    }
    #[doc = "Bits 16:23 - X coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    #[must_use]
    pub fn arbdeltayx(&mut self) -> ARBDELTAYX_W<16> {
        ARBDELTAYX_W::new(self)
    }
    #[doc = "Bits 24:31 - Y coordinate of vector between start and first sample point. (format is signed fix-point 3.5)"]
    #[inline(always)]
    #[must_use]
    pub fn arbdeltayy(&mut self) -> ARBDELTAYY_W<24> {
        ARBDELTAYY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start values for delta incrementation of arbitrary warping.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [arbdelta](index.html) module"]
pub struct ARBDELTA_SPEC;
impl crate::RegisterSpec for ARBDELTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [arbdelta::R](R) reader structure"]
impl crate::Readable for ARBDELTA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [arbdelta::W](W) writer structure"]
impl crate::Writable for ARBDELTA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ARBDELTA to value 0"]
impl crate::Resettable for ARBDELTA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
