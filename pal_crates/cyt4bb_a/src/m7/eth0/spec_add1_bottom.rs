#[doc = "Register `SPEC_ADD1_BOTTOM` reader"]
pub struct R(crate::R<SPEC_ADD1_BOTTOM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEC_ADD1_BOTTOM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEC_ADD1_BOTTOM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEC_ADD1_BOTTOM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPEC_ADD1_BOTTOM` writer"]
pub struct W(crate::W<SPEC_ADD1_BOTTOM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPEC_ADD1_BOTTOM_SPEC>;
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
impl From<crate::W<SPEC_ADD1_BOTTOM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPEC_ADD1_BOTTOM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_ADD1_B` reader - 'Least significant 32 bits of the destination address, that is bits 31:0. Bit zero indicates whether the address is multicast or unicast and corresponds to the least significant bit of the first byte received.'"]
pub type ADDRESS_ADD1_B_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDRESS_ADD1_B` writer - 'Least significant 32 bits of the destination address, that is bits 31:0. Bit zero indicates whether the address is multicast or unicast and corresponds to the least significant bit of the first byte received.'"]
pub type ADDRESS_ADD1_B_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPEC_ADD1_BOTTOM_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 'Least significant 32 bits of the destination address, that is bits 31:0. Bit zero indicates whether the address is multicast or unicast and corresponds to the least significant bit of the first byte received.'"]
    #[inline(always)]
    pub fn address_add1_b(&self) -> ADDRESS_ADD1_B_R {
        ADDRESS_ADD1_B_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 'Least significant 32 bits of the destination address, that is bits 31:0. Bit zero indicates whether the address is multicast or unicast and corresponds to the least significant bit of the first byte received.'"]
    #[inline(always)]
    #[must_use]
    pub fn address_add1_b(&mut self) -> ADDRESS_ADD1_B_W<0> {
        ADDRESS_ADD1_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The addresses stored in the specific address registers are deactivated at reset or when their corresponding specific address register bottom is written. They are activated when specific address register top is written.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spec_add1_bottom](index.html) module"]
pub struct SPEC_ADD1_BOTTOM_SPEC;
impl crate::RegisterSpec for SPEC_ADD1_BOTTOM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spec_add1_bottom::R](R) reader structure"]
impl crate::Readable for SPEC_ADD1_BOTTOM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spec_add1_bottom::W](W) writer structure"]
impl crate::Writable for SPEC_ADD1_BOTTOM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPEC_ADD1_BOTTOM to value 0"]
impl crate::Resettable for SPEC_ADD1_BOTTOM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
