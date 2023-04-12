#[doc = "Register `EXTDST5_STATIC` reader"]
pub struct R(crate::R<EXTDST5_STATIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST5_STATIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST5_STATIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST5_STATIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST5_STATIC` writer"]
pub struct W(crate::W<EXTDST5_STATIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST5_STATIC_SPEC>;
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
impl From<crate::W<EXTDST5_STATIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST5_STATIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST5_SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst5."]
pub type EXTDST5_SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTDST5_SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst5."]
pub type EXTDST5_SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTDST5_STATIC_SPEC, bool, O>;
#[doc = "Field `EXTDST5_POWERDOWN` reader - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst5 endpoint."]
pub type EXTDST5_POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `EXTDST5_POWERDOWN` writer - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst5 endpoint."]
pub type EXTDST5_POWERDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST5_STATIC_SPEC, bool, O>;
#[doc = "Field `EXTDST5_SYNC_MODE` reader - Synchronization mode for extdst5 pipeline endpoint synchronizer"]
pub type EXTDST5_SYNC_MODE_R = crate::BitReader<EXTDST5_SYNC_MODE_A>;
#[doc = "Synchronization mode for extdst5 pipeline endpoint synchronizer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDST5_SYNC_MODE_A {
    #[doc = "0: Reconfig pipeline after explicit trigger"]
    SINGLE = 0,
    #[doc = "1: Reconfig pipeline after every kick when idle"]
    AUTO = 1,
}
impl From<EXTDST5_SYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDST5_SYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDST5_SYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST5_SYNC_MODE_A {
        match self.bits {
            false => EXTDST5_SYNC_MODE_A::SINGLE,
            true => EXTDST5_SYNC_MODE_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == EXTDST5_SYNC_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == EXTDST5_SYNC_MODE_A::AUTO
    }
}
#[doc = "Field `EXTDST5_SYNC_MODE` writer - Synchronization mode for extdst5 pipeline endpoint synchronizer"]
pub type EXTDST5_SYNC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST5_STATIC_SPEC, EXTDST5_SYNC_MODE_A, O>;
impl<'a, const O: u8> EXTDST5_SYNC_MODE_W<'a, O> {
    #[doc = "Reconfig pipeline after explicit trigger"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(EXTDST5_SYNC_MODE_A::SINGLE)
    }
    #[doc = "Reconfig pipeline after every kick when idle"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(EXTDST5_SYNC_MODE_A::AUTO)
    }
}
#[doc = "Field `EXTDST5_SW_RESET` reader - Software reset for extdst5 synchronizer, for debug purposes only"]
pub type EXTDST5_SW_RESET_R = crate::BitReader<EXTDST5_SW_RESET_A>;
#[doc = "Software reset for extdst5 synchronizer, for debug purposes only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDST5_SW_RESET_A {
    #[doc = "0: Normal Operation"]
    OPERATION = 0,
    #[doc = "1: Software Reset"]
    SWRESET = 1,
}
impl From<EXTDST5_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDST5_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDST5_SW_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST5_SW_RESET_A {
        match self.bits {
            false => EXTDST5_SW_RESET_A::OPERATION,
            true => EXTDST5_SW_RESET_A::SWRESET,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == EXTDST5_SW_RESET_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `SWRESET`"]
    #[inline(always)]
    pub fn is_swreset(&self) -> bool {
        *self == EXTDST5_SW_RESET_A::SWRESET
    }
}
#[doc = "Field `EXTDST5_SW_RESET` writer - Software reset for extdst5 synchronizer, for debug purposes only"]
pub type EXTDST5_SW_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST5_STATIC_SPEC, EXTDST5_SW_RESET_A, O>;
impl<'a, const O: u8> EXTDST5_SW_RESET_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(EXTDST5_SW_RESET_A::OPERATION)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn swreset(self) -> &'a mut W {
        self.variant(EXTDST5_SW_RESET_A::SWRESET)
    }
}
#[doc = "Field `EXTDST5_DIV` reader - N/A"]
pub type EXTDST5_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTDST5_DIV` writer - N/A"]
pub type EXTDST5_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST5_STATIC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst5."]
    #[inline(always)]
    pub fn extdst5_shden(&self) -> EXTDST5_SHDEN_R {
        EXTDST5_SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst5 endpoint."]
    #[inline(always)]
    pub fn extdst5_powerdown(&self) -> EXTDST5_POWERDOWN_R {
        EXTDST5_POWERDOWN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization mode for extdst5 pipeline endpoint synchronizer"]
    #[inline(always)]
    pub fn extdst5_sync_mode(&self) -> EXTDST5_SYNC_MODE_R {
        EXTDST5_SYNC_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Software reset for extdst5 synchronizer, for debug purposes only"]
    #[inline(always)]
    pub fn extdst5_sw_reset(&self) -> EXTDST5_SW_RESET_R {
        EXTDST5_SW_RESET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn extdst5_div(&self) -> EXTDST5_DIV_R {
        EXTDST5_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst5."]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_shden(&mut self) -> EXTDST5_SHDEN_W<0> {
        EXTDST5_SHDEN_W::new(self)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst5 endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_powerdown(&mut self) -> EXTDST5_POWERDOWN_W<4> {
        EXTDST5_POWERDOWN_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization mode for extdst5 pipeline endpoint synchronizer"]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_sync_mode(&mut self) -> EXTDST5_SYNC_MODE_W<8> {
        EXTDST5_SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Software reset for extdst5 synchronizer, for debug purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_sw_reset(&mut self) -> EXTDST5_SW_RESET_W<11> {
        EXTDST5_SW_RESET_W::new(self)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn extdst5_div(&mut self) -> EXTDST5_DIV_W<16> {
        EXTDST5_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static pixel engine configuration for extdst5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst5_static](index.html) module"]
pub struct EXTDST5_STATIC_SPEC;
impl crate::RegisterSpec for EXTDST5_STATIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst5_static::R](R) reader structure"]
impl crate::Readable for EXTDST5_STATIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst5_static::W](W) writer structure"]
impl crate::Writable for EXTDST5_STATIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST5_STATIC to value 0x0080_0010"]
impl crate::Resettable for EXTDST5_STATIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0010;
}
