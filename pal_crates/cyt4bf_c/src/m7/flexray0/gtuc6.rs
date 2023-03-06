#[doc = "Register `GTUC6` reader"]
pub struct R(crate::R<GTUC6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC6` writer"]
pub struct W(crate::W<GTUC6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC6_SPEC>;
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
impl From<crate::W<GTUC6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASR` reader - Accepted Startup Range (pdAcceptedStartupRange) Number of microticks constituting the expanded range of measured deviation for startup frames during integration. Valid values are 0 to 1875 uT."]
pub type ASR_R = crate::FieldReader<u16, ASR_A>;
#[doc = "Accepted Startup Range (pdAcceptedStartupRange) Number of microticks constituting the expanded range of measured deviation for startup frames during integration. Valid values are 0 to 1875 uT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ASR_A {
    #[doc = "1875: N/A"]
    MAX = 1875,
}
impl From<ASR_A> for u16 {
    #[inline(always)]
    fn from(variant: ASR_A) -> Self {
        variant as _
    }
}
impl ASR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASR_A> {
        match self.bits {
            1875 => Some(ASR_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == ASR_A::MAX
    }
}
#[doc = "Field `ASR` writer - Accepted Startup Range (pdAcceptedStartupRange) Number of microticks constituting the expanded range of measured deviation for startup frames during integration. Valid values are 0 to 1875 uT."]
pub type ASR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC6_SPEC, u16, ASR_A, 11, O>;
impl<'a, const O: u8> ASR_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(ASR_A::MAX)
    }
}
#[doc = "Field `MOD` reader - Maximum Oscillator Drift (pdMaxDrift) Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in uT. Valid values are 2 to 1923 uT."]
pub type MOD_R = crate::FieldReader<u16, MOD_A>;
#[doc = "Maximum Oscillator Drift (pdMaxDrift) Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in uT. Valid values are 2 to 1923 uT.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MOD_A {
    #[doc = "2: N/A"]
    MIN = 2,
    #[doc = "1923: N/A"]
    MAX = 1923,
}
impl From<MOD_A> for u16 {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as _
    }
}
impl MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MOD_A> {
        match self.bits {
            2 => Some(MOD_A::MIN),
            1923 => Some(MOD_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == MOD_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == MOD_A::MAX
    }
}
#[doc = "Field `MOD` writer - Maximum Oscillator Drift (pdMaxDrift) Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in uT. Valid values are 2 to 1923 uT."]
pub type MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC6_SPEC, u16, MOD_A, 11, O>;
impl<'a, const O: u8> MOD_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(MOD_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(MOD_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:10 - Accepted Startup Range (pdAcceptedStartupRange) Number of microticks constituting the expanded range of measured deviation for startup frames during integration. Valid values are 0 to 1875 uT."]
    #[inline(always)]
    pub fn asr(&self) -> ASR_R {
        ASR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Maximum Oscillator Drift (pdMaxDrift) Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in uT. Valid values are 2 to 1923 uT."]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Accepted Startup Range (pdAcceptedStartupRange) Number of microticks constituting the expanded range of measured deviation for startup frames during integration. Valid values are 0 to 1875 uT."]
    #[inline(always)]
    #[must_use]
    pub fn asr(&mut self) -> ASR_W<0> {
        ASR_W::new(self)
    }
    #[doc = "Bits 16:26 - Maximum Oscillator Drift (pdMaxDrift) Maximum drift offset between two nodes that operate with unsynchronized clocks over one communication cycle in uT. Valid values are 2 to 1923 uT."]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<16> {
        MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc6](index.html) module"]
pub struct GTUC6_SPEC;
impl crate::RegisterSpec for GTUC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc6::R](R) reader structure"]
impl crate::Readable for GTUC6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc6::W](W) writer structure"]
impl crate::Writable for GTUC6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC6 to value 0x0002_0000"]
impl crate::Resettable for GTUC6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0000;
}
