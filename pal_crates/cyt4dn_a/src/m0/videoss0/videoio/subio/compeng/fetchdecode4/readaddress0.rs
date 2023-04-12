#[doc = "Register `READADDRESS0` reader"]
pub struct R(crate::R<READADDRESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READADDRESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READADDRESS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READADDRESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `READADDRESS0` writer"]
pub struct W(crate::W<READADDRESS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<READADDRESS0_SPEC>;
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
impl From<crate::W<READADDRESS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<READADDRESS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READADDRESS0` reader - Last burst address that was read from the layer's source buffer."]
pub type READADDRESS0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `READADDRESS0` writer - Last burst address that was read from the layer's source buffer."]
pub type READADDRESS0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, READADDRESS0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Last burst address that was read from the layer's source buffer."]
    #[inline(always)]
    pub fn readaddress0(&self) -> READADDRESS0_R {
        READADDRESS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last burst address that was read from the layer's source buffer."]
    #[inline(always)]
    #[must_use]
    pub fn readaddress0(&mut self) -> READADDRESS0_W<0> {
        READADDRESS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ring buffer synchronization for layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [readaddress0](index.html) module"]
pub struct READADDRESS0_SPEC;
impl crate::RegisterSpec for READADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [readaddress0::R](R) reader structure"]
impl crate::Readable for READADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [readaddress0::W](W) writer structure"]
impl crate::Writable for READADDRESS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets READADDRESS0 to value 0"]
impl crate::Resettable for READADDRESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
