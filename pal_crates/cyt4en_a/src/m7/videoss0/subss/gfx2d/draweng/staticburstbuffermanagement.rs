#[doc = "Register `STATICBURSTBUFFERMANAGEMENT` reader"]
pub struct R(crate::R<STATICBURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICBURSTBUFFERMANAGEMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICBURSTBUFFERMANAGEMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICBURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICBURSTBUFFERMANAGEMENT` writer"]
pub struct W(crate::W<STATICBURSTBUFFERMANAGEMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICBURSTBUFFERMANAGEMENT_SPEC>;
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
impl From<crate::W<STATICBURSTBUFFERMANAGEMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICBURSTBUFFERMANAGEMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETFETCHBURSTLENGTH` reader - Maximum burst length to be used on the AXI command fetch interface. Has to be 1 in context of VIDEOSSDDR."]
pub type SETFETCHBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETFETCHBURSTLENGTH` writer - Maximum burst length to be used on the AXI command fetch interface. Has to be 1 in context of VIDEOSSDDR."]
pub type SETFETCHBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICBURSTBUFFERMANAGEMENT_SPEC, u8, u8, 5, O>;
#[doc = "Field `SETSTOREBURSTLENGTH` reader - Set this to the burst length that should be used on the AXI alpha store interface. Please note that SetStoreBurstLength has to be smaller or equal to MaxStoreBurstLength. Only a power of two may be specified as burst length."]
pub type SETSTOREBURSTLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETSTOREBURSTLENGTH` writer - Set this to the burst length that should be used on the AXI alpha store interface. Please note that SetStoreBurstLength has to be smaller or equal to MaxStoreBurstLength. Only a power of two may be specified as burst length."]
pub type SETSTOREBURSTLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICBURSTBUFFERMANAGEMENT_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Maximum burst length to be used on the AXI command fetch interface. Has to be 1 in context of VIDEOSSDDR."]
    #[inline(always)]
    pub fn setfetchburstlength(&self) -> SETFETCHBURSTLENGTH_R {
        SETFETCHBURSTLENGTH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Set this to the burst length that should be used on the AXI alpha store interface. Please note that SetStoreBurstLength has to be smaller or equal to MaxStoreBurstLength. Only a power of two may be specified as burst length."]
    #[inline(always)]
    pub fn setstoreburstlength(&self) -> SETSTOREBURSTLENGTH_R {
        SETSTOREBURSTLENGTH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Maximum burst length to be used on the AXI command fetch interface. Has to be 1 in context of VIDEOSSDDR."]
    #[inline(always)]
    #[must_use]
    pub fn setfetchburstlength(&mut self) -> SETFETCHBURSTLENGTH_W<0> {
        SETFETCHBURSTLENGTH_W::new(self)
    }
    #[doc = "Bits 16:20 - Set this to the burst length that should be used on the AXI alpha store interface. Please note that SetStoreBurstLength has to be smaller or equal to MaxStoreBurstLength. Only a power of two may be specified as burst length."]
    #[inline(always)]
    #[must_use]
    pub fn setstoreburstlength(&mut self) -> SETSTOREBURSTLENGTH_W<16> {
        SETSTOREBURSTLENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst Buffer setup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticburstbuffermanagement](index.html) module"]
pub struct STATICBURSTBUFFERMANAGEMENT_SPEC;
impl crate::RegisterSpec for STATICBURSTBUFFERMANAGEMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticburstbuffermanagement::R](R) reader structure"]
impl crate::Readable for STATICBURSTBUFFERMANAGEMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticburstbuffermanagement::W](W) writer structure"]
impl crate::Writable for STATICBURSTBUFFERMANAGEMENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICBURSTBUFFERMANAGEMENT to value 0x0004_0004"]
impl crate::Resettable for STATICBURSTBUFFERMANAGEMENT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0004;
}
