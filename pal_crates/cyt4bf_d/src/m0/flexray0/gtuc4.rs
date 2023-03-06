#[doc = "Register `GTUC4` reader"]
pub struct R(crate::R<GTUC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC4` writer"]
pub struct W(crate::W<GTUC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC4_SPEC>;
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
impl From<crate::W<GTUC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NIT` reader - Network Idle Time Start (gMacroPerCycle - gdNIT - 1) Configures the starting point of the Network Idle Time NIT at the end of the communication cycle expressed in terms of macroticks from the beginning of the cycle. The start of NIT is recognized if Macrotick = gMacroPerCycle - gdNIT -1 and the increment pulse of Macrotick is set. Must be identical in all nodes of a cluster. Valid values are 7 to 15997 MT."]
pub type NIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NIT` writer - Network Idle Time Start (gMacroPerCycle - gdNIT - 1) Configures the starting point of the Network Idle Time NIT at the end of the communication cycle expressed in terms of macroticks from the beginning of the cycle. The start of NIT is recognized if Macrotick = gMacroPerCycle - gdNIT -1 and the increment pulse of Macrotick is set. Must be identical in all nodes of a cluster. Valid values are 7 to 15997 MT."]
pub type NIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC4_SPEC, u16, u16, 14, O>;
#[doc = "Field `OCS` reader - Offset Correction Start (gOffsetCorrectionStart - 1) Determines the start of the offset correction within the NIT phase, calculated from start of cycle. Must be identical in all nodes of a cluster. For cluster consisting of E-Ray implementations only, it is sufficient to program OCS = NIT + 1. Valid values are 8 to 15998 MT."]
pub type OCS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OCS` writer - Offset Correction Start (gOffsetCorrectionStart - 1) Determines the start of the offset correction within the NIT phase, calculated from start of cycle. Must be identical in all nodes of a cluster. For cluster consisting of E-Ray implementations only, it is sufficient to program OCS = NIT + 1. Valid values are 8 to 15998 MT."]
pub type OCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC4_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Network Idle Time Start (gMacroPerCycle - gdNIT - 1) Configures the starting point of the Network Idle Time NIT at the end of the communication cycle expressed in terms of macroticks from the beginning of the cycle. The start of NIT is recognized if Macrotick = gMacroPerCycle - gdNIT -1 and the increment pulse of Macrotick is set. Must be identical in all nodes of a cluster. Valid values are 7 to 15997 MT."]
    #[inline(always)]
    pub fn nit(&self) -> NIT_R {
        NIT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Offset Correction Start (gOffsetCorrectionStart - 1) Determines the start of the offset correction within the NIT phase, calculated from start of cycle. Must be identical in all nodes of a cluster. For cluster consisting of E-Ray implementations only, it is sufficient to program OCS = NIT + 1. Valid values are 8 to 15998 MT."]
    #[inline(always)]
    pub fn ocs(&self) -> OCS_R {
        OCS_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Network Idle Time Start (gMacroPerCycle - gdNIT - 1) Configures the starting point of the Network Idle Time NIT at the end of the communication cycle expressed in terms of macroticks from the beginning of the cycle. The start of NIT is recognized if Macrotick = gMacroPerCycle - gdNIT -1 and the increment pulse of Macrotick is set. Must be identical in all nodes of a cluster. Valid values are 7 to 15997 MT."]
    #[inline(always)]
    #[must_use]
    pub fn nit(&mut self) -> NIT_W<0> {
        NIT_W::new(self)
    }
    #[doc = "Bits 16:29 - Offset Correction Start (gOffsetCorrectionStart - 1) Determines the start of the offset correction within the NIT phase, calculated from start of cycle. Must be identical in all nodes of a cluster. For cluster consisting of E-Ray implementations only, it is sufficient to program OCS = NIT + 1. Valid values are 8 to 15998 MT."]
    #[inline(always)]
    #[must_use]
    pub fn ocs(&mut self) -> OCS_W<16> {
        OCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc4](index.html) module"]
pub struct GTUC4_SPEC;
impl crate::RegisterSpec for GTUC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc4::R](R) reader structure"]
impl crate::Readable for GTUC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc4::W](W) writer structure"]
impl crate::Writable for GTUC4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC4 to value 0x0008_0007"]
impl crate::Resettable for GTUC4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0007;
}
