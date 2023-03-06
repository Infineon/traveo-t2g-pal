#[doc = "Register `GTUC1` reader"]
pub struct R(crate::R<GTUC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC1` writer"]
pub struct W(crate::W<GTUC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC1_SPEC>;
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
impl From<crate::W<GTUC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UT` reader - Microtick per Cycle (pMicroPerCycle) Configures the duration of the communication cycle in microticks. Valid values are 640 to 640000 uT."]
pub type UT_R = crate::FieldReader<u32, UT_A>;
#[doc = "Microtick per Cycle (pMicroPerCycle) Configures the duration of the communication cycle in microticks. Valid values are 640 to 640000 uT.\n\nValue on reset: 640"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UT_A {
    #[doc = "640: N/A"]
    MIN = 640,
    #[doc = "640000: N/A"]
    MAX = 640000,
}
impl From<UT_A> for u32 {
    #[inline(always)]
    fn from(variant: UT_A) -> Self {
        variant as _
    }
}
impl UT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UT_A> {
        match self.bits {
            640 => Some(UT_A::MIN),
            640000 => Some(UT_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == UT_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == UT_A::MAX
    }
}
#[doc = "Field `UT` writer - Microtick per Cycle (pMicroPerCycle) Configures the duration of the communication cycle in microticks. Valid values are 640 to 640000 uT."]
pub type UT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC1_SPEC, u32, UT_A, 20, O>;
impl<'a, const O: u8> UT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(UT_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(UT_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:19 - Microtick per Cycle (pMicroPerCycle) Configures the duration of the communication cycle in microticks. Valid values are 640 to 640000 uT."]
    #[inline(always)]
    pub fn ut(&self) -> UT_R {
        UT_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Microtick per Cycle (pMicroPerCycle) Configures the duration of the communication cycle in microticks. Valid values are 640 to 640000 uT."]
    #[inline(always)]
    #[must_use]
    pub fn ut(&mut self) -> UT_W<0> {
        UT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc1](index.html) module"]
pub struct GTUC1_SPEC;
impl crate::RegisterSpec for GTUC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc1::R](R) reader structure"]
impl crate::Readable for GTUC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc1::W](W) writer structure"]
impl crate::Writable for GTUC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC1 to value 0x0280"]
impl crate::Resettable for GTUC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0280;
}
