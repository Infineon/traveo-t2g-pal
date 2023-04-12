#[doc = "Register `SWRESET` reader"]
pub struct R(crate::R<SWRESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWRESET` writer"]
pub struct W(crate::W<SWRESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRESET_SPEC>;
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
impl From<crate::W<SWRESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRESET` reader - Software reset"]
pub type SWRESET_R = crate::BitReader<SWRESET_A>;
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRESET_A {
    #[doc = "0: operation mode"]
    OPERATION = 0,
    #[doc = "1: So long SWReset = 0x1 tcon is in 'SW reset state' and it is released by internal logic (SWReset is released and end of frame arrived), read 0b reset not active 1b reset active (that means NO pixel of video frame is excepted until 'SW reset state' is released)"]
    SWRESET = 1,
}
impl From<SWRESET_A> for bool {
    #[inline(always)]
    fn from(variant: SWRESET_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRESET_A {
        match self.bits {
            false => SWRESET_A::OPERATION,
            true => SWRESET_A::SWRESET,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == SWRESET_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `SWRESET`"]
    #[inline(always)]
    pub fn is_swreset(&self) -> bool {
        *self == SWRESET_A::SWRESET
    }
}
#[doc = "Field `SWRESET` writer - Software reset"]
pub type SWRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWRESET_SPEC, SWRESET_A, O>;
impl<'a, const O: u8> SWRESET_W<'a, O> {
    #[doc = "operation mode"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(SWRESET_A::OPERATION)
    }
    #[doc = "So long SWReset = 0x1 tcon is in 'SW reset state' and it is released by internal logic (SWReset is released and end of frame arrived), read 0b reset not active 1b reset active (that means NO pixel of video frame is excepted until 'SW reset state' is released)"]
    #[inline(always)]
    pub fn swreset(self) -> &'a mut W {
        self.variant(SWRESET_A::SWRESET)
    }
}
#[doc = "Field `ENRESETWORD` reader - Enable to blend ResetWord into miniLVDS stream EnResetWord\\[5-0\\]
mapped to enable Blending Reset Pulse to \\[RLV5.RLV0\\]. EnResetWord\\[11-6\\]
mapped to enable Blending Reset Pulse to \\[LLV5.LLV0\\]."]
pub type ENRESETWORD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENRESETWORD` writer - Enable to blend ResetWord into miniLVDS stream EnResetWord\\[5-0\\]
mapped to enable Blending Reset Pulse to \\[RLV5.RLV0\\]. EnResetWord\\[11-6\\]
mapped to enable Blending Reset Pulse to \\[LLV5.LLV0\\]."]
pub type ENRESETWORD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWRESET_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESETWORDEND` reader - 8-Bits value, that will be blended on falling edge of tsig\\[11\\]
into miniLVDS stream"]
pub type RESETWORDEND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESETWORDEND` writer - 8-Bits value, that will be blended on falling edge of tsig\\[11\\]
into miniLVDS stream"]
pub type RESETWORDEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWRESET_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESETWORDSTART` reader - 8-Bits value, that will be blended on rising edge of tsig\\[11\\]
into miniLVDS stream"]
pub type RESETWORDSTART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESETWORDSTART` writer - 8-Bits value, that will be blended on rising edge of tsig\\[11\\]
into miniLVDS stream"]
pub type RESETWORDSTART_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SWRESET_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SWRESET_R {
        SWRESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:15 - Enable to blend ResetWord into miniLVDS stream EnResetWord\\[5-0\\]
mapped to enable Blending Reset Pulse to \\[RLV5.RLV0\\]. EnResetWord\\[11-6\\]
mapped to enable Blending Reset Pulse to \\[LLV5.LLV0\\]."]
    #[inline(always)]
    pub fn enresetword(&self) -> ENRESETWORD_R {
        ENRESETWORD_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:23 - 8-Bits value, that will be blended on falling edge of tsig\\[11\\]
into miniLVDS stream"]
    #[inline(always)]
    pub fn resetwordend(&self) -> RESETWORDEND_R {
        RESETWORDEND_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 8-Bits value, that will be blended on rising edge of tsig\\[11\\]
into miniLVDS stream"]
    #[inline(always)]
    pub fn resetwordstart(&self) -> RESETWORDSTART_R {
        RESETWORDSTART_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SWRESET_W<0> {
        SWRESET_W::new(self)
    }
    #[doc = "Bits 4:15 - Enable to blend ResetWord into miniLVDS stream EnResetWord\\[5-0\\]
mapped to enable Blending Reset Pulse to \\[RLV5.RLV0\\]. EnResetWord\\[11-6\\]
mapped to enable Blending Reset Pulse to \\[LLV5.LLV0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn enresetword(&mut self) -> ENRESETWORD_W<4> {
        ENRESETWORD_W::new(self)
    }
    #[doc = "Bits 16:23 - 8-Bits value, that will be blended on falling edge of tsig\\[11\\]
into miniLVDS stream"]
    #[inline(always)]
    #[must_use]
    pub fn resetwordend(&mut self) -> RESETWORDEND_W<16> {
        RESETWORDEND_W::new(self)
    }
    #[doc = "Bits 24:31 - 8-Bits value, that will be blended on rising edge of tsig\\[11\\]
into miniLVDS stream"]
    #[inline(always)]
    #[must_use]
    pub fn resetwordstart(&mut self) -> RESETWORDSTART_W<24> {
        RESETWORDSTART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCON Software Reset - Reset all tcon registers except configuration registers. Detailed description in specification document Note 1/ if tsig\\[11\\]
pulse=n*pixel_period, (n-1)*0xFF will be blended between ResetWordStart and ResetWordEnd into miniLVDS stream Note 2/ if( EnResetWord=0) Reset-Pulse (ResetWordStart,ResetWordEnd) won't be blended into miniLVDS stream. Pixels will be transferred unchanged\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreset](index.html) module"]
pub struct SWRESET_SPEC;
impl crate::RegisterSpec for SWRESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swreset::R](R) reader structure"]
impl crate::Readable for SWRESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swreset::W](W) writer structure"]
impl crate::Writable for SWRESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0x3fc0_0410"]
impl crate::Resettable for SWRESET_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fc0_0410;
}
