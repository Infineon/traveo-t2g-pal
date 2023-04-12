#[doc = "Register `READCHANNELBUFFER` reader"]
pub struct R(crate::R<READCHANNELBUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READCHANNELBUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READCHANNELBUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READCHANNELBUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READCHANNELBUFFER` writer"]
pub struct W(crate::W<READCHANNELBUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READCHANNELBUFFER_SPEC>;
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
impl From<crate::W<READCHANNELBUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READCHANNELBUFFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READCHANNELBASEADDRESS` reader - Base address of read buffer. Must be 32-bit aligned."]
pub type READCHANNELBASEADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `READCHANNELBASEADDRESS` writer - Base address of read buffer. Must be 32-bit aligned."]
pub type READCHANNELBASEADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READCHANNELBUFFER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address of read buffer. Must be 32-bit aligned."]
    #[inline(always)]
    pub fn readchannelbaseaddress(&self) -> READCHANNELBASEADDRESS_R {
        READCHANNELBASEADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of read buffer. Must be 32-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn readchannelbaseaddress(&mut self) -> READCHANNELBASEADDRESS_W<0> {
        READCHANNELBASEADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read Channel Buffer Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readchannelbuffer](index.html) module"]
pub struct READCHANNELBUFFER_SPEC;
impl crate::RegisterSpec for READCHANNELBUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readchannelbuffer::R](R) reader structure"]
impl crate::Readable for READCHANNELBUFFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readchannelbuffer::W](W) writer structure"]
impl crate::Writable for READCHANNELBUFFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READCHANNELBUFFER to value 0"]
impl crate::Resettable for READCHANNELBUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
