#[doc = "Register `WRITEADDRESS` reader"]
pub struct R(crate::R<WRITEADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITEADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITEADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITEADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITEADDRESS` writer"]
pub struct W(crate::W<WRITEADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITEADDRESS_SPEC>;
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
impl From<crate::W<WRITEADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITEADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITEADDRESS` reader - Last burst address that was written to the destination buffer."]
pub type WRITEADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRITEADDRESS` writer - Last burst address that was written to the destination buffer."]
pub type WRITEADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITEADDRESS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Last burst address that was written to the destination buffer."]
    #[inline(always)]
    pub fn writeaddress(&self) -> WRITEADDRESS_R {
        WRITEADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last burst address that was written to the destination buffer."]
    #[inline(always)]
    #[must_use]
    pub fn writeaddress(&mut self) -> WRITEADDRESS_W<0> {
        WRITEADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer synchronization.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writeaddress](index.html) module"]
pub struct WRITEADDRESS_SPEC;
impl crate::RegisterSpec for WRITEADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writeaddress::R](R) reader structure"]
impl crate::Readable for WRITEADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writeaddress::W](W) writer structure"]
impl crate::Writable for WRITEADDRESS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITEADDRESS to value 0"]
impl crate::Resettable for WRITEADDRESS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
