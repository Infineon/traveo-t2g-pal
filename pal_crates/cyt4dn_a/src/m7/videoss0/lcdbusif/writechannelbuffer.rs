#[doc = "Register `WRITECHANNELBUFFER` reader"]
pub struct R(crate::R<WRITECHANNELBUFFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRITECHANNELBUFFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRITECHANNELBUFFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRITECHANNELBUFFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WRITECHANNELBUFFER` writer"]
pub struct W(crate::W<WRITECHANNELBUFFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECHANNELBUFFER_SPEC>;
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
impl From<crate::W<WRITECHANNELBUFFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECHANNELBUFFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITECHANNELBASEADDRESS` reader - Base address of write buffer. Must be 32-bit aligned."]
pub type WRITECHANNELBASEADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `WRITECHANNELBASEADDRESS` writer - Base address of write buffer. Must be 32-bit aligned."]
pub type WRITECHANNELBASEADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WRITECHANNELBUFFER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Base address of write buffer. Must be 32-bit aligned."]
    #[inline(always)]
    pub fn writechannelbaseaddress(&self) -> WRITECHANNELBASEADDRESS_R {
        WRITECHANNELBASEADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of write buffer. Must be 32-bit aligned."]
    #[inline(always)]
    #[must_use]
    pub fn writechannelbaseaddress(&mut self) -> WRITECHANNELBASEADDRESS_W<0> {
        WRITECHANNELBASEADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Channel Buffer Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writechannelbuffer](index.html) module"]
pub struct WRITECHANNELBUFFER_SPEC;
impl crate::RegisterSpec for WRITECHANNELBUFFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [writechannelbuffer::R](R) reader structure"]
impl crate::Readable for WRITECHANNELBUFFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [writechannelbuffer::W](W) writer structure"]
impl crate::Writable for WRITECHANNELBUFFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WRITECHANNELBUFFER to value 0"]
impl crate::Resettable for WRITECHANNELBUFFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
