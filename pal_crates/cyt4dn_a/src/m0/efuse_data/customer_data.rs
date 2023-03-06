#[doc = "Register `CUSTOMER_DATA[%s]` reader"]
pub struct R(crate::R<CUSTOMER_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CUSTOMER_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CUSTOMER_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CUSTOMER_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CUSTOMER_DATA[%s]` writer"]
pub struct W(crate::W<CUSTOMER_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUSTOMER_DATA_SPEC>;
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
impl From<crate::W<CUSTOMER_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUSTOMER_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_BYTE` reader - Customer Data"]
pub type DATA_BYTE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA_BYTE` writer - Customer Data"]
pub type DATA_BYTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CUSTOMER_DATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Customer Data"]
    #[inline(always)]
    pub fn data_byte(&self) -> DATA_BYTE_R {
        DATA_BYTE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Customer Data"]
    #[inline(always)]
    #[must_use]
    pub fn data_byte(&mut self) -> DATA_BYTE_W<0> {
        DATA_BYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Customer Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer_data](index.html) module"]
pub struct CUSTOMER_DATA_SPEC;
impl crate::RegisterSpec for CUSTOMER_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [customer_data::R](R) reader structure"]
impl crate::Readable for CUSTOMER_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [customer_data::W](W) writer structure"]
impl crate::Writable for CUSTOMER_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CUSTOMER_DATA[%s]
to value 0"]
impl crate::Resettable for CUSTOMER_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
