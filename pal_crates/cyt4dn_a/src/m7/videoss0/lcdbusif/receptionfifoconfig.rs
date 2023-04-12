#[doc = "Register `RECEPTIONFIFOCONFIG` reader"]
pub struct R(crate::R<RECEPTIONFIFOCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RECEPTIONFIFOCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RECEPTIONFIFOCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RECEPTIONFIFOCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RECEPTIONFIFOCONFIG` writer"]
pub struct W(crate::W<RECEPTIONFIFOCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RECEPTIONFIFOCONFIG_SPEC>;
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
impl From<crate::W<RECEPTIONFIFOCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RECEPTIONFIFOCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTHRESHOLDLOW` reader - The threshold below which the reception fifo should generate a low interrupt."]
pub type RXTHRESHOLDLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTHRESHOLDLOW` writer - The threshold below which the reception fifo should generate a low interrupt."]
pub type RXTHRESHOLDLOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEPTIONFIFOCONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RXTHRESHOLDHIGH` reader - The threshold above which the reception fifo should generate a high interrupt."]
pub type RXTHRESHOLDHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXTHRESHOLDHIGH` writer - The threshold above which the reception fifo should generate a high interrupt."]
pub type RXTHRESHOLDHIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEPTIONFIFOCONFIG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RXTHRESHOLDTRIGGER` reader - Select trigger condition for RxFifoInterrupt."]
pub type RXTHRESHOLDTRIGGER_R = crate::FieldReader<u8, RXTHRESHOLDTRIGGER_A>;
#[doc = "Select trigger condition for RxFifoInterrupt.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXTHRESHOLDTRIGGER_A {
    #[doc = "0: RxFifoInterrupt triggers if level falls below low threshold after being above high level."]
    LOW = 0,
    #[doc = "1: RxFifoInterrupt triggers if level raises above high threshold after being below low level."]
    HIGH = 1,
    #[doc = "2: RxFifoInterrupt triggers for both low and high threshold conditions."]
    BOTH = 2,
}
impl From<RXTHRESHOLDTRIGGER_A> for u8 {
    #[inline(always)]
    fn from(variant: RXTHRESHOLDTRIGGER_A) -> Self {
        variant as _
    }
}
impl RXTHRESHOLDTRIGGER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXTHRESHOLDTRIGGER_A> {
        match self.bits {
            0 => Some(RXTHRESHOLDTRIGGER_A::LOW),
            1 => Some(RXTHRESHOLDTRIGGER_A::HIGH),
            2 => Some(RXTHRESHOLDTRIGGER_A::BOTH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == RXTHRESHOLDTRIGGER_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == RXTHRESHOLDTRIGGER_A::HIGH
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == RXTHRESHOLDTRIGGER_A::BOTH
    }
}
#[doc = "Field `RXTHRESHOLDTRIGGER` writer - Select trigger condition for RxFifoInterrupt."]
pub type RXTHRESHOLDTRIGGER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RECEPTIONFIFOCONFIG_SPEC, u8, RXTHRESHOLDTRIGGER_A, 2, O>;
impl<'a, const O: u8> RXTHRESHOLDTRIGGER_W<'a, O> {
    #[doc = "RxFifoInterrupt triggers if level falls below low threshold after being above high level."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RXTHRESHOLDTRIGGER_A::LOW)
    }
    #[doc = "RxFifoInterrupt triggers if level raises above high threshold after being below low level."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RXTHRESHOLDTRIGGER_A::HIGH)
    }
    #[doc = "RxFifoInterrupt triggers for both low and high threshold conditions."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(RXTHRESHOLDTRIGGER_A::BOTH)
    }
}
impl R {
    #[doc = "Bits 0:4 - The threshold below which the reception fifo should generate a low interrupt."]
    #[inline(always)]
    pub fn rxthresholdlow(&self) -> RXTHRESHOLDLOW_R {
        RXTHRESHOLDLOW_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - The threshold above which the reception fifo should generate a high interrupt."]
    #[inline(always)]
    pub fn rxthresholdhigh(&self) -> RXTHRESHOLDHIGH_R {
        RXTHRESHOLDHIGH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Select trigger condition for RxFifoInterrupt."]
    #[inline(always)]
    pub fn rxthresholdtrigger(&self) -> RXTHRESHOLDTRIGGER_R {
        RXTHRESHOLDTRIGGER_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The threshold below which the reception fifo should generate a low interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxthresholdlow(&mut self) -> RXTHRESHOLDLOW_W<0> {
        RXTHRESHOLDLOW_W::new(self)
    }
    #[doc = "Bits 8:12 - The threshold above which the reception fifo should generate a high interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxthresholdhigh(&mut self) -> RXTHRESHOLDHIGH_W<8> {
        RXTHRESHOLDHIGH_W::new(self)
    }
    #[doc = "Bits 16:17 - Select trigger condition for RxFifoInterrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rxthresholdtrigger(&mut self) -> RXTHRESHOLDTRIGGER_W<16> {
        RXTHRESHOLDTRIGGER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reception fifo configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [receptionfifoconfig](index.html) module"]
pub struct RECEPTIONFIFOCONFIG_SPEC;
impl crate::RegisterSpec for RECEPTIONFIFOCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [receptionfifoconfig::R](R) reader structure"]
impl crate::Readable for RECEPTIONFIFOCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [receptionfifoconfig::W](W) writer structure"]
impl crate::Writable for RECEPTIONFIFOCONFIG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RECEPTIONFIFOCONFIG to value 0x0001_1000"]
impl crate::Resettable for RECEPTIONFIFOCONFIG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_1000;
}
