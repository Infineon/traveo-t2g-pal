#[doc = "Register `INSTRUCTIONFIFOCONFIG` reader"]
pub struct R(crate::R<INSTRUCTIONFIFOCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSTRUCTIONFIFOCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSTRUCTIONFIFOCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSTRUCTIONFIFOCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSTRUCTIONFIFOCONFIG` writer"]
pub struct W(crate::W<INSTRUCTIONFIFOCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSTRUCTIONFIFOCONFIG_SPEC>;
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
impl From<crate::W<INSTRUCTIONFIFOCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSTRUCTIONFIFOCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTRTHRESHOLDLOW` reader - The threshold below which the instruction fifo should generate a low interrupt."]
pub type INSTRTHRESHOLDLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRTHRESHOLDLOW` writer - The threshold below which the instruction fifo should generate a low interrupt."]
pub type INSTRTHRESHOLDLOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INSTRUCTIONFIFOCONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `INSTRTHRESHOLDHIGH` reader - The threshold above which the instruction fifo should generate a high interrupt."]
pub type INSTRTHRESHOLDHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTRTHRESHOLDHIGH` writer - The threshold above which the instruction fifo should generate a high interrupt."]
pub type INSTRTHRESHOLDHIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INSTRUCTIONFIFOCONFIG_SPEC, u8, u8, 6, O>;
#[doc = "Field `INSTRTHRESHOLDTRIGGER` reader - Select trigger condition for InstrFifoInterrupt."]
pub type INSTRTHRESHOLDTRIGGER_R = crate::FieldReader<u8, INSTRTHRESHOLDTRIGGER_A>;
#[doc = "Select trigger condition for InstrFifoInterrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INSTRTHRESHOLDTRIGGER_A {
    #[doc = "0: InstrFifoInterrupt triggers if level falls below low threshold after being above high level."]
    LOW = 0,
    #[doc = "1: InstrFifoInterrupt triggers if level raises above high threshold after being below low level."]
    HIGH = 1,
    #[doc = "2: InstrFifoInterrupt triggers for both low and high threshold conditions."]
    BOTH = 2,
}
impl From<INSTRTHRESHOLDTRIGGER_A> for u8 {
    #[inline(always)]
    fn from(variant: INSTRTHRESHOLDTRIGGER_A) -> Self {
        variant as _
    }
}
impl INSTRTHRESHOLDTRIGGER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INSTRTHRESHOLDTRIGGER_A> {
        match self.bits {
            0 => Some(INSTRTHRESHOLDTRIGGER_A::LOW),
            1 => Some(INSTRTHRESHOLDTRIGGER_A::HIGH),
            2 => Some(INSTRTHRESHOLDTRIGGER_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == INSTRTHRESHOLDTRIGGER_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == INSTRTHRESHOLDTRIGGER_A::HIGH
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == INSTRTHRESHOLDTRIGGER_A::BOTH
    }
}
#[doc = "Field `INSTRTHRESHOLDTRIGGER` writer - Select trigger condition for InstrFifoInterrupt."]
pub type INSTRTHRESHOLDTRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INSTRUCTIONFIFOCONFIG_SPEC, u8, INSTRTHRESHOLDTRIGGER_A, 2, O>;
impl<'a, const O: u8> INSTRTHRESHOLDTRIGGER_W<'a, O> {
    #[doc = "InstrFifoInterrupt triggers if level falls below low threshold after being above high level."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(INSTRTHRESHOLDTRIGGER_A::LOW)
    }
    #[doc = "InstrFifoInterrupt triggers if level raises above high threshold after being below low level."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(INSTRTHRESHOLDTRIGGER_A::HIGH)
    }
    #[doc = "InstrFifoInterrupt triggers for both low and high threshold conditions."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INSTRTHRESHOLDTRIGGER_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:5 - The threshold below which the instruction fifo should generate a low interrupt."]
    #[inline(always)]
    pub fn instrthresholdlow(&self) -> INSTRTHRESHOLDLOW_R {
        INSTRTHRESHOLDLOW_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - The threshold above which the instruction fifo should generate a high interrupt."]
    #[inline(always)]
    pub fn instrthresholdhigh(&self) -> INSTRTHRESHOLDHIGH_R {
        INSTRTHRESHOLDHIGH_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Select trigger condition for InstrFifoInterrupt."]
    #[inline(always)]
    pub fn instrthresholdtrigger(&self) -> INSTRTHRESHOLDTRIGGER_R {
        INSTRTHRESHOLDTRIGGER_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - The threshold below which the instruction fifo should generate a low interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn instrthresholdlow(&mut self) -> INSTRTHRESHOLDLOW_W<0> {
        INSTRTHRESHOLDLOW_W::new(self)
    }
    #[doc = "Bits 8:13 - The threshold above which the instruction fifo should generate a high interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn instrthresholdhigh(&mut self) -> INSTRTHRESHOLDHIGH_W<8> {
        INSTRTHRESHOLDHIGH_W::new(self)
    }
    #[doc = "Bits 16:17 - Select trigger condition for InstrFifoInterrupt."]
    #[inline(always)]
    #[must_use]
    pub fn instrthresholdtrigger(&mut self) -> INSTRTHRESHOLDTRIGGER_W<16> {
        INSTRTHRESHOLDTRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Instruction fifo configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [instructionfifoconfig](index.html) module"]
pub struct INSTRUCTIONFIFOCONFIG_SPEC;
impl crate::RegisterSpec for INSTRUCTIONFIFOCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [instructionfifoconfig::R](R) reader structure"]
impl crate::Readable for INSTRUCTIONFIFOCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [instructionfifoconfig::W](W) writer structure"]
impl crate::Writable for INSTRUCTIONFIFOCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSTRUCTIONFIFOCONFIG to value 0x0001_2000"]
impl crate::Resettable for INSTRUCTIONFIFOCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_2000;
}
