#[doc = "Register `STORE4_STATIC` reader"]
pub struct R(crate::R<STORE4_STATIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STORE4_STATIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STORE4_STATIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STORE4_STATIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STORE4_STATIC` writer"]
pub struct W(crate::W<STORE4_STATIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STORE4_STATIC_SPEC>;
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
impl From<crate::W<STORE4_STATIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STORE4_STATIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STORE4_SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint store4."]
pub type STORE4_SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `STORE4_SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint store4."]
pub type STORE4_SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STORE4_STATIC_SPEC, bool, O>;
#[doc = "Field `STORE4_POWERDOWN` reader - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the store4 endpoint."]
pub type STORE4_POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `STORE4_POWERDOWN` writer - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the store4 endpoint."]
pub type STORE4_POWERDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STORE4_STATIC_SPEC, bool, O>;
#[doc = "Field `STORE4_SYNC_MODE` reader - Synchronization mode for store4 pipeline endpoint synchronizer"]
pub type STORE4_SYNC_MODE_R = crate::BitReader<STORE4_SYNC_MODE_A>;
#[doc = "Synchronization mode for store4 pipeline endpoint synchronizer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STORE4_SYNC_MODE_A {
    #[doc = "0: Reconfig pipeline after explicit trigger"]
    SINGLE = 0,
    #[doc = "1: Reconfig pipeline after every kick when idle"]
    AUTO = 1,
}
impl From<STORE4_SYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: STORE4_SYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl STORE4_SYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORE4_SYNC_MODE_A {
        match self.bits {
            false => STORE4_SYNC_MODE_A::SINGLE,
            true => STORE4_SYNC_MODE_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == STORE4_SYNC_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == STORE4_SYNC_MODE_A::AUTO
    }
}
#[doc = "Field `STORE4_SYNC_MODE` writer - Synchronization mode for store4 pipeline endpoint synchronizer"]
pub type STORE4_SYNC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STORE4_STATIC_SPEC, STORE4_SYNC_MODE_A, O>;
impl<'a, const O: u8> STORE4_SYNC_MODE_W<'a, O> {
    #[doc = "Reconfig pipeline after explicit trigger"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(STORE4_SYNC_MODE_A::SINGLE)
    }
    #[doc = "Reconfig pipeline after every kick when idle"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(STORE4_SYNC_MODE_A::AUTO)
    }
}
#[doc = "Field `STORE4_SW_RESET` reader - Software reset for store4 synchronizer, for debug purposes only"]
pub type STORE4_SW_RESET_R = crate::BitReader<STORE4_SW_RESET_A>;
#[doc = "Software reset for store4 synchronizer, for debug purposes only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STORE4_SW_RESET_A {
    #[doc = "0: Normal Operation"]
    OPERATION = 0,
    #[doc = "1: Software Reset"]
    SWRESET = 1,
}
impl From<STORE4_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: STORE4_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl STORE4_SW_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STORE4_SW_RESET_A {
        match self.bits {
            false => STORE4_SW_RESET_A::OPERATION,
            true => STORE4_SW_RESET_A::SWRESET,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == STORE4_SW_RESET_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `SWRESET`"]
    #[inline(always)]
    pub fn is_swreset(&self) -> bool {
        *self == STORE4_SW_RESET_A::SWRESET
    }
}
#[doc = "Field `STORE4_SW_RESET` writer - Software reset for store4 synchronizer, for debug purposes only"]
pub type STORE4_SW_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STORE4_STATIC_SPEC, STORE4_SW_RESET_A, O>;
impl<'a, const O: u8> STORE4_SW_RESET_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(STORE4_SW_RESET_A::OPERATION)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn swreset(self) -> &'a mut W {
        self.variant(STORE4_SW_RESET_A::SWRESET)
    }
}
#[doc = "Field `STORE4_DIV` reader - N/A"]
pub type STORE4_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STORE4_DIV` writer - N/A"]
pub type STORE4_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STORE4_STATIC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint store4."]
    #[inline(always)]
    pub fn store4_shden(&self) -> STORE4_SHDEN_R {
        STORE4_SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the store4 endpoint."]
    #[inline(always)]
    pub fn store4_powerdown(&self) -> STORE4_POWERDOWN_R {
        STORE4_POWERDOWN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization mode for store4 pipeline endpoint synchronizer"]
    #[inline(always)]
    pub fn store4_sync_mode(&self) -> STORE4_SYNC_MODE_R {
        STORE4_SYNC_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Software reset for store4 synchronizer, for debug purposes only"]
    #[inline(always)]
    pub fn store4_sw_reset(&self) -> STORE4_SW_RESET_R {
        STORE4_SW_RESET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn store4_div(&self) -> STORE4_DIV_R {
        STORE4_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint store4."]
    #[inline(always)]
    #[must_use]
    pub fn store4_shden(&mut self) -> STORE4_SHDEN_W<0> {
        STORE4_SHDEN_W::new(self)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the store4 endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn store4_powerdown(&mut self) -> STORE4_POWERDOWN_W<4> {
        STORE4_POWERDOWN_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization mode for store4 pipeline endpoint synchronizer"]
    #[inline(always)]
    #[must_use]
    pub fn store4_sync_mode(&mut self) -> STORE4_SYNC_MODE_W<8> {
        STORE4_SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Software reset for store4 synchronizer, for debug purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn store4_sw_reset(&mut self) -> STORE4_SW_RESET_W<11> {
        STORE4_SW_RESET_W::new(self)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn store4_div(&mut self) -> STORE4_DIV_W<16> {
        STORE4_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static pixel engine configuration for store4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [store4_static](index.html) module"]
pub struct STORE4_STATIC_SPEC;
impl crate::RegisterSpec for STORE4_STATIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [store4_static::R](R) reader structure"]
impl crate::Readable for STORE4_STATIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [store4_static::W](W) writer structure"]
impl crate::Writable for STORE4_STATIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STORE4_STATIC to value 0x0080_0010"]
impl crate::Resettable for STORE4_STATIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0010;
}
