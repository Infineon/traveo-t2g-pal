#[doc = "Register `SUCC3` reader"]
pub struct R(crate::R<SUCC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUCC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUCC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUCC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUCC3` writer"]
pub struct W(crate::W<SUCC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUCC3_SPEC>;
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
impl From<crate::W<SUCC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUCC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WCP` reader - Maximum Without Clock Correction Passive (gMaxWithoutClockCorrectionPassive) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE to NORMAL_PASSIVE state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
pub type WCP_R = crate::FieldReader<u8, WCP_A>;
#[doc = "Maximum Without Clock Correction Passive (gMaxWithoutClockCorrectionPassive) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE to NORMAL_PASSIVE state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WCP_A {
    #[doc = "1: N/A"]
    MIN = 1,
}
impl From<WCP_A> for u8 {
    #[inline(always)]
    fn from(variant: WCP_A) -> Self {
        variant as _
    }
}
impl WCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WCP_A> {
        match self.bits {
            1 => Some(WCP_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == WCP_A::MIN
    }
}
#[doc = "Field `WCP` writer - Maximum Without Clock Correction Passive (gMaxWithoutClockCorrectionPassive) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE to NORMAL_PASSIVE state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
pub type WCP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC3_SPEC, u8, WCP_A, 4, O>;
impl<'a, const O: u8> WCP_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(WCP_A::MIN)
    }
}
#[doc = "Field `WCF` reader - Maximum Without Clock Correction Fatal (gMaxWithoutClockCorrectionFatal) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE or NORMAL_PASSIVE to HALT state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
pub type WCF_R = crate::FieldReader<u8, WCF_A>;
#[doc = "Maximum Without Clock Correction Fatal (gMaxWithoutClockCorrectionFatal) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE or NORMAL_PASSIVE to HALT state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WCF_A {
    #[doc = "1: N/A"]
    MIN = 1,
}
impl From<WCF_A> for u8 {
    #[inline(always)]
    fn from(variant: WCF_A) -> Self {
        variant as _
    }
}
impl WCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WCF_A> {
        match self.bits {
            1 => Some(WCF_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == WCF_A::MIN
    }
}
#[doc = "Field `WCF` writer - Maximum Without Clock Correction Fatal (gMaxWithoutClockCorrectionFatal) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE or NORMAL_PASSIVE to HALT state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
pub type WCF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUCC3_SPEC, u8, WCF_A, 4, O>;
impl<'a, const O: u8> WCF_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(WCF_A::MIN)
    }
}
impl R {
    #[doc = "Bits 0:3 - Maximum Without Clock Correction Passive (gMaxWithoutClockCorrectionPassive) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE to NORMAL_PASSIVE state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
    #[inline(always)]
    pub fn wcp(&self) -> WCP_R {
        WCP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Maximum Without Clock Correction Fatal (gMaxWithoutClockCorrectionFatal) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE or NORMAL_PASSIVE to HALT state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
    #[inline(always)]
    pub fn wcf(&self) -> WCF_R {
        WCF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Maximum Without Clock Correction Passive (gMaxWithoutClockCorrectionPassive) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE to NORMAL_PASSIVE state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
    #[inline(always)]
    #[must_use]
    pub fn wcp(&mut self) -> WCP_W<0> {
        WCP_W::new(self)
    }
    #[doc = "Bits 4:7 - Maximum Without Clock Correction Fatal (gMaxWithoutClockCorrectionFatal) Defines the number of consecutive even / odd cycle pairs with missing clock correction terms that will cause a transition from NORMAL_ACTIVE or NORMAL_PASSIVE to HALT state. Must be identical in all nodes of a cluster. Valid values are 1 to 15 cycle pairs."]
    #[inline(always)]
    #[must_use]
    pub fn wcf(&mut self) -> WCF_W<4> {
        WCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SUC Configuration Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [succ3](index.html) module"]
pub struct SUCC3_SPEC;
impl crate::RegisterSpec for SUCC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [succ3::R](R) reader structure"]
impl crate::Readable for SUCC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [succ3::W](W) writer structure"]
impl crate::Writable for SUCC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SUCC3 to value 0x11"]
impl crate::Resettable for SUCC3_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
