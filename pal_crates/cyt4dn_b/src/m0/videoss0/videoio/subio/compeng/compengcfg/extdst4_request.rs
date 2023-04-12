#[doc = "Register `EXTDST4_REQUEST` reader"]
pub struct R(crate::R<EXTDST4_REQUEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST4_REQUEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST4_REQUEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST4_REQUEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST4_REQUEST` writer"]
pub struct W(crate::W<EXTDST4_REQUEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST4_REQUEST_SPEC>;
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
impl From<crate::W<EXTDST4_REQUEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST4_REQUEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST4_SEL_SHDLDREQ` reader - Shadow load request flag for destination extdst4."]
pub type EXTDST4_SEL_SHDLDREQ_R = crate::BitReader<bool>;
#[doc = "Field `EXTDST4_SEL_SHDLDREQ` writer - Shadow load request flag for destination extdst4."]
pub type EXTDST4_SEL_SHDLDREQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST4_REQUEST_SPEC, bool, O>;
#[doc = "Field `EXTDST4_SHDLDREQ` reader - Vector of shadow load request flag of all sources for destination extdst4. Setting a bit has no effect if the source is currently in a different pipeline than the one of destination extdst4."]
pub type EXTDST4_SHDLDREQ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EXTDST4_SHDLDREQ` writer - Vector of shadow load request flag of all sources for destination extdst4. Setting a bit has no effect if the source is currently in a different pipeline than the one of destination extdst4."]
pub type EXTDST4_SHDLDREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST4_REQUEST_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bit 0 - Shadow load request flag for destination extdst4."]
    #[inline(always)]
    pub fn extdst4_sel_shdldreq(&self) -> EXTDST4_SEL_SHDLDREQ_R {
        EXTDST4_SEL_SHDLDREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:13 - Vector of shadow load request flag of all sources for destination extdst4. Setting a bit has no effect if the source is currently in a different pipeline than the one of destination extdst4."]
    #[inline(always)]
    pub fn extdst4_shdldreq(&self) -> EXTDST4_SHDLDREQ_R {
        EXTDST4_SHDLDREQ_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow load request flag for destination extdst4."]
    #[inline(always)]
    #[must_use]
    pub fn extdst4_sel_shdldreq(&mut self) -> EXTDST4_SEL_SHDLDREQ_W<0> {
        EXTDST4_SEL_SHDLDREQ_W::new(self)
    }
    #[doc = "Bits 1:13 - Vector of shadow load request flag of all sources for destination extdst4. Setting a bit has no effect if the source is currently in a different pipeline than the one of destination extdst4."]
    #[inline(always)]
    #[must_use]
    pub fn extdst4_shdldreq(&mut self) -> EXTDST4_SHDLDREQ_W<1> {
        EXTDST4_SHDLDREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ShadowLoadRequest register for endpoint extdst4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst4_request](index.html) module"]
pub struct EXTDST4_REQUEST_SPEC;
impl crate::RegisterSpec for EXTDST4_REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst4_request::R](R) reader structure"]
impl crate::Readable for EXTDST4_REQUEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst4_request::W](W) writer structure"]
impl crate::Writable for EXTDST4_REQUEST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST4_REQUEST to value 0"]
impl crate::Resettable for EXTDST4_REQUEST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
