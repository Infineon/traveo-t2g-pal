#[doc = "Register `STACKED_VLAN` reader"]
pub struct R(crate::R<STACKED_VLAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STACKED_VLAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STACKED_VLAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STACKED_VLAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STACKED_VLAN` writer"]
pub struct W(crate::W<STACKED_VLAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STACKED_VLAN_SPEC>;
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
impl From<crate::W<STACKED_VLAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STACKED_VLAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MATCH` reader - User defined VLAN_TYPE field. When Stacked VLAN is enabled, the first VLAN tag in a received frame will only be accepted if the VLAN type field is equal to this user defined VLAN_TYPE OR equal to the standard VLAN type (0x8100). Note that the second VLAN tag of a Stacked VLAN packet will only be matched correctly if its VLAN_TYPE field equals 0x8100."]
pub type MATCH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MATCH` writer - User defined VLAN_TYPE field. When Stacked VLAN is enabled, the first VLAN tag in a received frame will only be accepted if the VLAN type field is equal to this user defined VLAN_TYPE OR equal to the standard VLAN type (0x8100). Note that the second VLAN tag of a Stacked VLAN packet will only be matched correctly if its VLAN_TYPE field equals 0x8100."]
pub type MATCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STACKED_VLAN_SPEC, u16, u16, 16, O>;
#[doc = "Field `ENABLE_PROCESSING` reader - Enable stacked VLAN processing mode"]
pub type ENABLE_PROCESSING_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_PROCESSING` writer - Enable stacked VLAN processing mode"]
pub type ENABLE_PROCESSING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STACKED_VLAN_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field. When Stacked VLAN is enabled, the first VLAN tag in a received frame will only be accepted if the VLAN type field is equal to this user defined VLAN_TYPE OR equal to the standard VLAN type (0x8100). Note that the second VLAN tag of a Stacked VLAN packet will only be matched correctly if its VLAN_TYPE field equals 0x8100."]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enable_processing(&self) -> ENABLE_PROCESSING_R {
        ENABLE_PROCESSING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field. When Stacked VLAN is enabled, the first VLAN tag in a received frame will only be accepted if the VLAN type field is equal to this user defined VLAN_TYPE OR equal to the standard VLAN type (0x8100). Note that the second VLAN tag of a Stacked VLAN packet will only be matched correctly if its VLAN_TYPE field equals 0x8100."]
    #[inline(always)]
    #[must_use]
    pub fn match_(&mut self) -> MATCH_W<0> {
        MATCH_W::new(self)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    #[must_use]
    pub fn enable_processing(&mut self) -> ENABLE_PROCESSING_W<31> {
        ENABLE_PROCESSING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stacked_vlan](index.html) module"]
pub struct STACKED_VLAN_SPEC;
impl crate::RegisterSpec for STACKED_VLAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stacked_vlan::R](R) reader structure"]
impl crate::Readable for STACKED_VLAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stacked_vlan::W](W) writer structure"]
impl crate::Writable for STACKED_VLAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STACKED_VLAN to value 0"]
impl crate::Resettable for STACKED_VLAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
