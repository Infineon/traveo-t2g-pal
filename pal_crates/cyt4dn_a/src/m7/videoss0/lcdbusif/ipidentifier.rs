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
#[doc = "Register `IPIDENTIFIER` writer"]
pub struct W(crate::W<IPIDENTIFIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPIDENTIFIER_SPEC>;
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
impl From<crate::W<IPIDENTIFIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPIDENTIFIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD` reader - N/A"]
pub type RSVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD` writer - N/A"]
pub type RSVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPIDENTIFIER_SPEC, u8, u8, 4, O>;
#[doc = "Field `DESIGNDELIVERYID` reader - Design delivery ID (increased with each official delivery when maturity keeps the same)"]
pub type DESIGNDELIVERYID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DESIGNDELIVERYID` writer - Design delivery ID (increased with each official delivery when maturity keeps the same)"]
pub type DESIGNDELIVERYID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPIDENTIFIER_SPEC, u8, u8, 4, O>;
#[doc = "Field `DESIGNMATURITYLEVEL` reader - Design maturity level (corresponds to status at time of IP delivery, Fujitsu internal development stages)"]
pub type DESIGNMATURITYLEVEL_R = crate::FieldReader<u8, DESIGNMATURITYLEVEL_A>;
#[doc = "Design maturity level (corresponds to status at time of IP delivery, Fujitsu internal development stages)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DESIGNMATURITYLEVEL_A {
    #[doc = "1: Pre feasibility study"]
    PREFS = 1,
    #[doc = "2: Feasibility study"]
    FS = 2,
    #[doc = "3: Functionality complete"]
    R0 = 3,
    #[doc = "4: Verification complete"]
    R1 = 4,
}
impl From<DESIGNMATURITYLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DESIGNMATURITYLEVEL_A) -> Self {
        variant as _
    }
}
impl DESIGNMATURITYLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DESIGNMATURITYLEVEL_A> {
        match self.bits {
            1 => Some(DESIGNMATURITYLEVEL_A::PREFS),
            2 => Some(DESIGNMATURITYLEVEL_A::FS),
            3 => Some(DESIGNMATURITYLEVEL_A::R0),
            4 => Some(DESIGNMATURITYLEVEL_A::R1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PREFS`"]
    #[inline(always)]
    pub fn is_prefs(&self) -> bool {
        *self == DESIGNMATURITYLEVEL_A::PREFS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        *self == DESIGNMATURITYLEVEL_A::FS
    }
    #[doc = "Checks if the value of the field is `R0`"]
    #[inline(always)]
    pub fn is_r0(&self) -> bool {
        *self == DESIGNMATURITYLEVEL_A::R0
    }
    #[doc = "Checks if the value of the field is `R1`"]
    #[inline(always)]
    pub fn is_r1(&self) -> bool {
        *self == DESIGNMATURITYLEVEL_A::R1
    }
}
#[doc = "Field `DESIGNMATURITYLEVEL` writer - Design maturity level (corresponds to status at time of IP delivery, Fujitsu internal development stages)"]
pub type DESIGNMATURITYLEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPIDENTIFIER_SPEC, u8, DESIGNMATURITYLEVEL_A, 4, O>;
impl<'a, const O: u8> DESIGNMATURITYLEVEL_W<'a, O> {
    #[doc = "Pre feasibility study"]
    #[inline(always)]
    pub fn prefs(self) -> &'a mut W {
        self.variant(DESIGNMATURITYLEVEL_A::PREFS)
    }
    #[doc = "Feasibility study"]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(DESIGNMATURITYLEVEL_A::FS)
    }
    #[doc = "Functionality complete"]
    #[inline(always)]
    pub fn r0(self) -> &'a mut W {
        self.variant(DESIGNMATURITYLEVEL_A::R0)
    }
    #[doc = "Verification complete"]
    #[inline(always)]
    pub fn r1(self) -> &'a mut W {
        self.variant(DESIGNMATURITYLEVEL_A::R1)
    }
}
#[doc = "Field `IPEVOLUTION` reader - IP evolution (increased for functional spec changes only when feature set keeps the same)"]
pub type IPEVOLUTION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPEVOLUTION` writer - IP evolution (increased for functional spec changes only when feature set keeps the same)"]
pub type IPEVOLUTION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPIDENTIFIER_SPEC, u8, u8, 4, O>;
#[doc = "Field `IPFEATURESET` reader - IP feature set (complexity of implemented features, e.g. availability of re-sampling filter etc)"]
pub type IPFEATURESET_R = crate::FieldReader<u8, IPFEATURESET_A>;
#[doc = "IP feature set (complexity of implemented features, e.g. availability of re-sampling filter etc)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IPFEATURESET_A {
    #[doc = "0: LCDBusIF without data master functionality."]
    LIGHT = 0,
    #[doc = "1: LCDBusIF with data master functionality."]
    STANDARD = 1,
}
impl From<IPFEATURESET_A> for u8 {
    #[inline(always)]
    fn from(variant: IPFEATURESET_A) -> Self {
        variant as _
    }
}
impl IPFEATURESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IPFEATURESET_A> {
        match self.bits {
            0 => Some(IPFEATURESET_A::LIGHT),
            1 => Some(IPFEATURESET_A::STANDARD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LIGHT`"]
    #[inline(always)]
    pub fn is_light(&self) -> bool {
        *self == IPFEATURESET_A::LIGHT
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == IPFEATURESET_A::STANDARD
    }
}
#[doc = "Field `IPFEATURESET` writer - IP feature set (complexity of implemented features, e.g. availability of re-sampling filter etc)"]
pub type IPFEATURESET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IPIDENTIFIER_SPEC, u8, IPFEATURESET_A, 4, O>;
impl<'a, const O: u8> IPFEATURESET_W<'a, O> {
    #[doc = "LCDBusIF without data master functionality."]
    #[inline(always)]
    pub fn light(self) -> &'a mut W {
        self.variant(IPFEATURESET_A::LIGHT)
    }
    #[doc = "LCDBusIF with data master functionality."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(IPFEATURESET_A::STANDARD)
    }
}
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn rsvd(&self) -> RSVD_R {
        RSVD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Design delivery ID (increased with each official delivery when maturity keeps the same)"]
    #[inline(always)]
    pub fn designdeliveryid(&self) -> DESIGNDELIVERYID_R {
        DESIGNDELIVERYID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Design maturity level (corresponds to status at time of IP delivery, Fujitsu internal development stages)"]
    #[inline(always)]
    pub fn designmaturitylevel(&self) -> DESIGNMATURITYLEVEL_R {
        DESIGNMATURITYLEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - IP evolution (increased for functional spec changes only when feature set keeps the same)"]
    #[inline(always)]
    pub fn ipevolution(&self) -> IPEVOLUTION_R {
        IPEVOLUTION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IP feature set (complexity of implemented features, e.g. availability of re-sampling filter etc)"]
    #[inline(always)]
    pub fn ipfeatureset(&self) -> IPFEATURESET_R {
        IPFEATURESET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RSVD_W<0> {
        RSVD_W::new(self)
    }
    #[doc = "Bits 4:7 - Design delivery ID (increased with each official delivery when maturity keeps the same)"]
    #[inline(always)]
    #[must_use]
    pub fn designdeliveryid(&mut self) -> DESIGNDELIVERYID_W<4> {
        DESIGNDELIVERYID_W::new(self)
    }
    #[doc = "Bits 8:11 - Design maturity level (corresponds to status at time of IP delivery, Fujitsu internal development stages)"]
    #[inline(always)]
    #[must_use]
    pub fn designmaturitylevel(&mut self) -> DESIGNMATURITYLEVEL_W<8> {
        DESIGNMATURITYLEVEL_W::new(self)
    }
    #[doc = "Bits 12:15 - IP evolution (increased for functional spec changes only when feature set keeps the same)"]
    #[inline(always)]
    #[must_use]
    pub fn ipevolution(&mut self) -> IPEVOLUTION_W<12> {
        IPEVOLUTION_W::new(self)
    }
    #[doc = "Bits 16:19 - IP feature set (complexity of implemented features, e.g. availability of re-sampling filter etc)"]
    #[inline(always)]
    #[must_use]
    pub fn ipfeatureset(&mut self) -> IPFEATURESET_W<16> {
        IPFEATURESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Identifier for this LCDBusIF derivate, needs to be unlocked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidentifier](index.html) module"]
pub struct IPIDENTIFIER_SPEC;
impl crate::RegisterSpec for IPIDENTIFIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipidentifier::R](R) reader structure"]
impl crate::Readable for IPIDENTIFIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipidentifier::W](W) writer structure"]
impl crate::Writable for IPIDENTIFIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPIDENTIFIER to value 0x0001_0400"]
impl crate::Resettable for IPIDENTIFIER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0400;
}
