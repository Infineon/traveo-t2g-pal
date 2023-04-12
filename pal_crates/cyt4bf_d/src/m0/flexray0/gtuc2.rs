#[doc = "Register `GTUC2` reader"]
pub struct R(crate::R<GTUC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC2` writer"]
pub struct W(crate::W<GTUC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC2_SPEC>;
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
impl From<crate::W<GTUC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPC` reader - Macrotick Per Cycle (gMacroPerCycle) Configures the duration of one communication cycle in macroticks. The cycle length must be identical in all nodes of a cluster. Valid values are 10 to 16000 MT."]
pub type MPC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MPC` writer - Macrotick Per Cycle (gMacroPerCycle) Configures the duration of one communication cycle in macroticks. The cycle length must be identical in all nodes of a cluster. Valid values are 10 to 16000 MT."]
pub type MPC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC2_SPEC, u16, u16, 14, O>;
#[doc = "Field `SNM` reader - Sync Node Max (gSyncNodeMax) Maximum number of frames within a cluster with sync frame indicator bit SYN set to '1'. Must be identical in all nodes of a cluster. Valid values are 2 to 15."]
pub type SNM_R = crate::FieldReader<u8, SNM_A>;
#[doc = "Sync Node Max (gSyncNodeMax) Maximum number of frames within a cluster with sync frame indicator bit SYN set to '1'. Must be identical in all nodes of a cluster. Valid values are 2 to 15.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SNM_A {
    #[doc = "2: N/A"]
    MIN = 2,
    #[doc = "15: N/A"]
    MAX = 15,
}
impl From<SNM_A> for u8 {
    #[inline(always)]
    fn from(variant: SNM_A) -> Self {
        variant as _
    }
}
impl SNM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SNM_A> {
        match self.bits {
            2 => Some(SNM_A::MIN),
            15 => Some(SNM_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == SNM_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == SNM_A::MAX
    }
}
#[doc = "Field `SNM` writer - Sync Node Max (gSyncNodeMax) Maximum number of frames within a cluster with sync frame indicator bit SYN set to '1'. Must be identical in all nodes of a cluster. Valid values are 2 to 15."]
pub type SNM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC2_SPEC, u8, SNM_A, 4, O>;
impl<'a, const O: u8> SNM_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(SNM_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(SNM_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:13 - Macrotick Per Cycle (gMacroPerCycle) Configures the duration of one communication cycle in macroticks. The cycle length must be identical in all nodes of a cluster. Valid values are 10 to 16000 MT."]
    #[inline(always)]
    pub fn mpc(&self) -> MPC_R {
        MPC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:19 - Sync Node Max (gSyncNodeMax) Maximum number of frames within a cluster with sync frame indicator bit SYN set to '1'. Must be identical in all nodes of a cluster. Valid values are 2 to 15."]
    #[inline(always)]
    pub fn snm(&self) -> SNM_R {
        SNM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Macrotick Per Cycle (gMacroPerCycle) Configures the duration of one communication cycle in macroticks. The cycle length must be identical in all nodes of a cluster. Valid values are 10 to 16000 MT."]
    #[inline(always)]
    #[must_use]
    pub fn mpc(&mut self) -> MPC_W<0> {
        MPC_W::new(self)
    }
    #[doc = "Bits 16:19 - Sync Node Max (gSyncNodeMax) Maximum number of frames within a cluster with sync frame indicator bit SYN set to '1'. Must be identical in all nodes of a cluster. Valid values are 2 to 15."]
    #[inline(always)]
    #[must_use]
    pub fn snm(&mut self) -> SNM_W<16> {
        SNM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc2](index.html) module"]
pub struct GTUC2_SPEC;
impl crate::RegisterSpec for GTUC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc2::R](R) reader structure"]
impl crate::Readable for GTUC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc2::W](W) writer structure"]
impl crate::Writable for GTUC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC2 to value 0x0002_000a"]
impl crate::Resettable for GTUC2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_000a;
}
