#[doc = "Register `SPEC_ADD1_TOP` reader"]
pub struct R(crate::R<SPEC_ADD1_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPEC_ADD1_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPEC_ADD1_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPEC_ADD1_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPEC_ADD1_TOP` writer"]
pub struct W(crate::W<SPEC_ADD1_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPEC_ADD1_TOP_SPEC>;
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
impl From<crate::W<SPEC_ADD1_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPEC_ADD1_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRESS_TOP` reader - Specific address 1. The most significant bits of the destination/source address that is to be compared, that is bits 47:32."]
pub type ADDRESS_TOP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDRESS_TOP` writer - Specific address 1. The most significant bits of the destination/source address that is to be compared, that is bits 47:32."]
pub type ADDRESS_TOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPEC_ADD1_TOP_SPEC, u16, u16, 16, O>;
#[doc = "Field `FILTER_TYPE` reader - This control bit selects whether this filter should be comparing the MAC source address or the MAC destination address of the received Ethernet frame. When set to zero, the filter is a destination address filter. When set to one, the filter is a source address filter. Specific address 1. The most significant bits of the destination address, that is bits 47:32."]
pub type FILTER_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `FILTER_TYPE` writer - This control bit selects whether this filter should be comparing the MAC source address or the MAC destination address of the received Ethernet frame. When set to zero, the filter is a destination address filter. When set to one, the filter is a source address filter. Specific address 1. The most significant bits of the destination address, that is bits 47:32."]
pub type FILTER_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPEC_ADD1_TOP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Specific address 1. The most significant bits of the destination/source address that is to be compared, that is bits 47:32."]
    #[inline(always)]
    pub fn address_top(&self) -> ADDRESS_TOP_R {
        ADDRESS_TOP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - This control bit selects whether this filter should be comparing the MAC source address or the MAC destination address of the received Ethernet frame. When set to zero, the filter is a destination address filter. When set to one, the filter is a source address filter. Specific address 1. The most significant bits of the destination address, that is bits 47:32."]
    #[inline(always)]
    pub fn filter_type(&self) -> FILTER_TYPE_R {
        FILTER_TYPE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific address 1. The most significant bits of the destination/source address that is to be compared, that is bits 47:32."]
    #[inline(always)]
    #[must_use]
    pub fn address_top(&mut self) -> ADDRESS_TOP_W<0> {
        ADDRESS_TOP_W::new(self)
    }
    #[doc = "Bit 16 - This control bit selects whether this filter should be comparing the MAC source address or the MAC destination address of the received Ethernet frame. When set to zero, the filter is a destination address filter. When set to one, the filter is a source address filter. Specific address 1. The most significant bits of the destination address, that is bits 47:32."]
    #[inline(always)]
    #[must_use]
    pub fn filter_type(&mut self) -> FILTER_TYPE_W<16> {
        FILTER_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spec_add1_top](index.html) module"]
pub struct SPEC_ADD1_TOP_SPEC;
impl crate::RegisterSpec for SPEC_ADD1_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spec_add1_top::R](R) reader structure"]
impl crate::Readable for SPEC_ADD1_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spec_add1_top::W](W) writer structure"]
impl crate::Writable for SPEC_ADD1_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPEC_ADD1_TOP to value 0"]
impl crate::Resettable for SPEC_ADD1_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
