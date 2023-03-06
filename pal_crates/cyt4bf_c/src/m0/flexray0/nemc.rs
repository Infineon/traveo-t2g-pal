#[doc = "Register `NEMC` reader"]
pub struct R(crate::R<NEMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NEMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NEMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NEMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NEMC` writer"]
pub struct W(crate::W<NEMC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NEMC_SPEC>;
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
impl From<crate::W<NEMC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NEMC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NML` reader - Network Management Vector Length (gNetworkManagementVectorLength) These bits configure the length of the NM vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12 bytes."]
pub type NML_R = crate::FieldReader<u8, NML_A>;
#[doc = "Network Management Vector Length (gNetworkManagementVectorLength) These bits configure the length of the NM vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12 bytes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NML_A {
    #[doc = "12: N/A"]
    MAX = 12,
}
impl From<NML_A> for u8 {
    #[inline(always)]
    fn from(variant: NML_A) -> Self {
        variant as _
    }
}
impl NML_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NML_A> {
        match self.bits {
            12 => Some(NML_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == NML_A::MAX
    }
}
#[doc = "Field `NML` writer - Network Management Vector Length (gNetworkManagementVectorLength) These bits configure the length of the NM vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12 bytes."]
pub type NML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NEMC_SPEC, u8, NML_A, 4, O>;
impl<'a, const O: u8> NML_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(NML_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:3 - Network Management Vector Length (gNetworkManagementVectorLength) These bits configure the length of the NM vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12 bytes."]
    #[inline(always)]
    pub fn nml(&self) -> NML_R {
        NML_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Network Management Vector Length (gNetworkManagementVectorLength) These bits configure the length of the NM vector. The configured length must be identical in all nodes of a cluster. Valid values are 0 to 12 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn nml(&mut self) -> NML_W<0> {
        NML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NEM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nemc](index.html) module"]
pub struct NEMC_SPEC;
impl crate::RegisterSpec for NEMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nemc::R](R) reader structure"]
impl crate::Readable for NEMC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nemc::W](W) writer structure"]
impl crate::Writable for NEMC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NEMC to value 0"]
impl crate::Resettable for NEMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
