#[doc = "Register `DESTINATIONBUFFER` reader"]
pub struct R(crate::R<DESTINATIONBUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONBUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONBUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONBUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONBUFFER` writer"]
pub struct W(crate::W<DESTINATIONBUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONBUFFER_SPEC>;
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
impl From<crate::W<DESTINATIONBUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONBUFFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSTBASEADDRESS` reader - Byte aligned start address of the destination buffer."]
pub type DSTBASEADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSTBASEADDRESS` writer - Byte aligned start address of the destination buffer."]
pub type DSTBASEADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Byte aligned start address of the destination buffer."]
    #[inline(always)]
    pub fn dstbaseaddress(&self) -> DSTBASEADDRESS_R {
        DSTBASEADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Byte aligned start address of the destination buffer."]
    #[inline(always)]
    #[must_use]
    pub fn dstbaseaddress(&mut self) -> DSTBASEADDRESS_W<0> {
        DSTBASEADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer base address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationbuffer](index.html) module"]
pub struct DESTINATIONBUFFER_SPEC;
impl crate::RegisterSpec for DESTINATIONBUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationbuffer::R](R) reader structure"]
impl crate::Readable for DESTINATIONBUFFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationbuffer::W](W) writer structure"]
impl crate::Writable for DESTINATIONBUFFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONBUFFER to value 0"]
impl crate::Resettable for DESTINATIONBUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
