#[doc = "Register `IPIDENTIFIER` reader"]
pub struct R(crate::R<IPIDENTIFIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPIDENTIFIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPIDENTIFIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPIDENTIFIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IPPLATFORM` reader - Name of SOC Platform."]
pub type IPPLATFORM_R = crate::BitReader<IPPLATFORM_A>;
#[doc = "Name of SOC Platform.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPPLATFORM_A {
    #[doc = "0: MXS40 High Temperature Variant."]
    MXS40_S40E = 0,
}
impl From<IPPLATFORM_A> for bool {
    #[inline(always)]
    fn from(variant: IPPLATFORM_A) -> Self {
        variant as u8 != 0
    }
}
impl IPPLATFORM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPPLATFORM_A> {
        match self.bits {
            false => Some(IPPLATFORM_A::MXS40_S40E),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MXS40_S40E`"]
    #[inline(always)]
    pub fn is_mxs40_s40e(&self) -> bool {
        *self == IPPLATFORM_A::MXS40_S40E
    }
}
#[doc = "Field `IPNAME` reader - Name of IP family."]
pub type IPNAME_R = crate::FieldReader<u8, IPNAME_A>;
#[doc = "Name of IP family.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPNAME_A {
    #[doc = "0: Video and Graphics Subsystem with internal SRAM."]
    VIDEOSS = 0,
    #[doc = "1: Video and Graphics Subsystem for external DDR-SDRAM."]
    VIDEOSSDDR = 1,
    #[doc = "2: Display Composition Subsystem."]
    VIDEOSSLITE = 2,
}
impl From<IPNAME_A> for u8 {
    #[inline(always)]
    fn from(variant: IPNAME_A) -> Self {
        variant as _
    }
}
impl IPNAME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPNAME_A> {
        match self.bits {
            0 => Some(IPNAME_A::VIDEOSS),
            1 => Some(IPNAME_A::VIDEOSSDDR),
            2 => Some(IPNAME_A::VIDEOSSLITE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VIDEOSS`"]
    #[inline(always)]
    pub fn is_videoss(&self) -> bool {
        *self == IPNAME_A::VIDEOSS
    }
    #[doc = "Checks if the value of the field is `VIDEOSSDDR`"]
    #[inline(always)]
    pub fn is_videossddr(&self) -> bool {
        *self == IPNAME_A::VIDEOSSDDR
    }
    #[doc = "Checks if the value of the field is `VIDEOSSLITE`"]
    #[inline(always)]
    pub fn is_videosslite(&self) -> bool {
        *self == IPNAME_A::VIDEOSSLITE
    }
}
#[doc = "Field `IPVERSION` reader - IP Version Number."]
pub type IPVERSION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPPHASE` reader - IP Phase Number."]
pub type IPPHASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPMATURITY` reader - IP Maturity Status with Chip Integration Cycle targets (CIC). This is independently set for each implementation type."]
pub type IPMATURITY_R = crate::FieldReader<u8, IPMATURITY_A>;
#[doc = "IP Maturity Status with Chip Integration Cycle targets (CIC). This is independently set for each implementation type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPMATURITY_A {
    #[doc = "0: Undefined"]
    NONE = 0,
    #[doc = "1: Implementation incomplete (for CIC Physical Readiness Review)."]
    CPR = 1,
    #[doc = "2: Implementation complete (for CIC Mock Review)."]
    CMR = 2,
    #[doc = "3: Verification complete (for CIC Final Handoff Review)."]
    CFR = 3,
}
impl From<IPMATURITY_A> for u8 {
    #[inline(always)]
    fn from(variant: IPMATURITY_A) -> Self {
        variant as _
    }
}
impl IPMATURITY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPMATURITY_A {
        match self.bits {
            0 => IPMATURITY_A::NONE,
            1 => IPMATURITY_A::CPR,
            2 => IPMATURITY_A::CMR,
            3 => IPMATURITY_A::CFR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == IPMATURITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `CPR`"]
    #[inline(always)]
    pub fn is_cpr(&self) -> bool {
        *self == IPMATURITY_A::CPR
    }
    #[doc = "Checks if the value of the field is `CMR`"]
    #[inline(always)]
    pub fn is_cmr(&self) -> bool {
        *self == IPMATURITY_A::CMR
    }
    #[doc = "Checks if the value of the field is `CFR`"]
    #[inline(always)]
    pub fn is_cfr(&self) -> bool {
        *self == IPMATURITY_A::CFR
    }
}
#[doc = "Field `IPRELEASE` reader - Release ID. This is independently set for each implementation type. The value starts with 0 and is increased for every new release of a an IP project, which is defined by IPName, IPVersion and IPPhase."]
pub type IPRELEASE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPIMPLEMENTATION` reader - Implementation type."]
pub type IPIMPLEMENTATION_R = crate::FieldReader<u8, IPIMPLEMENTATION_A>;
#[doc = "Implementation type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPIMPLEMENTATION_A {
    #[doc = "0: Logic design for FPGA and ASIC synthesis."]
    RTL = 0,
    #[doc = "1: Bit-true and un-timed reference model."]
    TLM = 1,
}
impl From<IPIMPLEMENTATION_A> for u8 {
    #[inline(always)]
    fn from(variant: IPIMPLEMENTATION_A) -> Self {
        variant as _
    }
}
impl IPIMPLEMENTATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPIMPLEMENTATION_A> {
        match self.bits {
            0 => Some(IPIMPLEMENTATION_A::RTL),
            1 => Some(IPIMPLEMENTATION_A::TLM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTL`"]
    #[inline(always)]
    pub fn is_rtl(&self) -> bool {
        *self == IPIMPLEMENTATION_A::RTL
    }
    #[doc = "Checks if the value of the field is `TLM`"]
    #[inline(always)]
    pub fn is_tlm(&self) -> bool {
        *self == IPIMPLEMENTATION_A::TLM
    }
}
impl R {
    #[doc = "Bit 0 - Name of SOC Platform."]
    #[inline(always)]
    pub fn ipplatform(&self) -> IPPLATFORM_R {
        IPPLATFORM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Name of IP family."]
    #[inline(always)]
    pub fn ipname(&self) -> IPNAME_R {
        IPNAME_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - IP Version Number."]
    #[inline(always)]
    pub fn ipversion(&self) -> IPVERSION_R {
        IPVERSION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - IP Phase Number."]
    #[inline(always)]
    pub fn ipphase(&self) -> IPPHASE_R {
        IPPHASE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - IP Maturity Status with Chip Integration Cycle targets (CIC). This is independently set for each implementation type."]
    #[inline(always)]
    pub fn ipmaturity(&self) -> IPMATURITY_R {
        IPMATURITY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:27 - Release ID. This is independently set for each implementation type. The value starts with 0 and is increased for every new release of a an IP project, which is defined by IPName, IPVersion and IPPhase."]
    #[inline(always)]
    pub fn iprelease(&self) -> IPRELEASE_R {
        IPRELEASE_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:29 - Implementation type."]
    #[inline(always)]
    pub fn ipimplementation(&self) -> IPIMPLEMENTATION_R {
        IPIMPLEMENTATION_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[doc = "IP and Design Release Identification\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidentifier](index.html) module"]
pub struct IPIDENTIFIER_SPEC;
impl crate::RegisterSpec for IPIDENTIFIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipidentifier::R](R) reader structure"]
impl crate::Readable for IPIDENTIFIER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IPIDENTIFIER to value 0"]
impl crate::Resettable for IPIDENTIFIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
