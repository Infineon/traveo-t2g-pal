#[doc = "Register `MASK_ADD1_BOTTOM` reader"]
pub struct R(crate::R<MASK_ADD1_BOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_ADD1_BOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_ADD1_BOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_ADD1_BOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_ADD1_BOTTOM` writer"]
pub struct W(crate::W<MASK_ADD1_BOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_ADD1_BOTTOM_SPEC>;
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
impl From<crate::W<MASK_ADD1_BOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_ADD1_BOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_MASK_BOTTOM` reader - Specific Address Mask. Setting a bit to one masks the corresponding bit in the specific address 1 register (31 to 0 bits)"]
pub type ADDRESS_MASK_BOTTOM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_MASK_BOTTOM` writer - Specific Address Mask. Setting a bit to one masks the corresponding bit in the specific address 1 register (31 to 0 bits)"]
pub type ADDRESS_MASK_BOTTOM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MASK_ADD1_BOTTOM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Specific Address Mask. Setting a bit to one masks the corresponding bit in the specific address 1 register (31 to 0 bits)"]
    #[inline(always)]
    pub fn address_mask_bottom(&self) -> ADDRESS_MASK_BOTTOM_R {
        ADDRESS_MASK_BOTTOM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specific Address Mask. Setting a bit to one masks the corresponding bit in the specific address 1 register (31 to 0 bits)"]
    #[inline(always)]
    #[must_use]
    pub fn address_mask_bottom(&mut self) -> ADDRESS_MASK_BOTTOM_W<0> {
        ADDRESS_MASK_BOTTOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address Mask 1 Bottom (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_add1_bottom](index.html) module"]
pub struct MASK_ADD1_BOTTOM_SPEC;
impl crate::RegisterSpec for MASK_ADD1_BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_add1_bottom::R](R) reader structure"]
impl crate::Readable for MASK_ADD1_BOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_add1_bottom::W](W) writer structure"]
impl crate::Writable for MASK_ADD1_BOTTOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK_ADD1_BOTTOM to value 0"]
impl crate::Resettable for MASK_ADD1_BOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
