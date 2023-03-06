#[doc = "Register `EXTDST0_STATIC` reader"]
pub struct R(crate::R<EXTDST0_STATIC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTDST0_STATIC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTDST0_STATIC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTDST0_STATIC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTDST0_STATIC` writer"]
pub struct W(crate::W<EXTDST0_STATIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTDST0_STATIC_SPEC>;
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
impl From<crate::W<EXTDST0_STATIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTDST0_STATIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDST0_SHDEN` reader - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst0."]
pub type EXTDST0_SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTDST0_SHDEN` writer - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst0."]
pub type EXTDST0_SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTDST0_STATIC_SPEC, bool, O>;
#[doc = "Field `EXTDST0_POWERDOWN` reader - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst0 endpoint."]
pub type EXTDST0_POWERDOWN_R = crate::BitReader<bool>;
#[doc = "Field `EXTDST0_POWERDOWN` writer - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst0 endpoint."]
pub type EXTDST0_POWERDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST0_STATIC_SPEC, bool, O>;
#[doc = "Field `EXTDST0_SYNC_MODE` reader - Synchronization mode for extdst0 pipeline endpoint synchronizer"]
pub type EXTDST0_SYNC_MODE_R = crate::BitReader<EXTDST0_SYNC_MODE_A>;
#[doc = "Synchronization mode for extdst0 pipeline endpoint synchronizer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDST0_SYNC_MODE_A {
    #[doc = "0: Reconfig pipeline after explicit trigger"]
    SINGLE = 0,
    #[doc = "1: Reconfig pipeline after every kick when idle"]
    AUTO = 1,
}
impl From<EXTDST0_SYNC_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDST0_SYNC_MODE_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDST0_SYNC_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST0_SYNC_MODE_A {
        match self.bits {
            false => EXTDST0_SYNC_MODE_A::SINGLE,
            true => EXTDST0_SYNC_MODE_A::AUTO,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == EXTDST0_SYNC_MODE_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == EXTDST0_SYNC_MODE_A::AUTO
    }
}
#[doc = "Field `EXTDST0_SYNC_MODE` writer - Synchronization mode for extdst0 pipeline endpoint synchronizer"]
pub type EXTDST0_SYNC_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST0_STATIC_SPEC, EXTDST0_SYNC_MODE_A, O>;
impl<'a, const O: u8> EXTDST0_SYNC_MODE_W<'a, O> {
    #[doc = "Reconfig pipeline after explicit trigger"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(EXTDST0_SYNC_MODE_A::SINGLE)
    }
    #[doc = "Reconfig pipeline after every kick when idle"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(EXTDST0_SYNC_MODE_A::AUTO)
    }
}
#[doc = "Field `EXTDST0_SW_RESET` reader - Software reset for extdst0 synchronizer, for debug purposes only"]
pub type EXTDST0_SW_RESET_R = crate::BitReader<EXTDST0_SW_RESET_A>;
#[doc = "Software reset for extdst0 synchronizer, for debug purposes only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDST0_SW_RESET_A {
    #[doc = "0: Normal Operation"]
    OPERATION = 0,
    #[doc = "1: Software Reset"]
    SWRESET = 1,
}
impl From<EXTDST0_SW_RESET_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDST0_SW_RESET_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDST0_SW_RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDST0_SW_RESET_A {
        match self.bits {
            false => EXTDST0_SW_RESET_A::OPERATION,
            true => EXTDST0_SW_RESET_A::SWRESET,
        }
    }
    #[doc = "Checks if the value of the field is `OPERATION`"]
    #[inline(always)]
    pub fn is_operation(&self) -> bool {
        *self == EXTDST0_SW_RESET_A::OPERATION
    }
    #[doc = "Checks if the value of the field is `SWRESET`"]
    #[inline(always)]
    pub fn is_swreset(&self) -> bool {
        *self == EXTDST0_SW_RESET_A::SWRESET
    }
}
#[doc = "Field `EXTDST0_SW_RESET` writer - Software reset for extdst0 synchronizer, for debug purposes only"]
pub type EXTDST0_SW_RESET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EXTDST0_STATIC_SPEC, EXTDST0_SW_RESET_A, O>;
impl<'a, const O: u8> EXTDST0_SW_RESET_W<'a, O> {
    #[doc = "Normal Operation"]
    #[inline(always)]
    pub fn operation(self) -> &'a mut W {
        self.variant(EXTDST0_SW_RESET_A::OPERATION)
    }
    #[doc = "Software Reset"]
    #[inline(always)]
    pub fn swreset(self) -> &'a mut W {
        self.variant(EXTDST0_SW_RESET_A::SWRESET)
    }
}
#[doc = "Field `EXTDST0_DIV` reader - N/A"]
pub type EXTDST0_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXTDST0_DIV` writer - N/A"]
pub type EXTDST0_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXTDST0_STATIC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst0."]
    #[inline(always)]
    pub fn extdst0_shden(&self) -> EXTDST0_SHDEN_R {
        EXTDST0_SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst0 endpoint."]
    #[inline(always)]
    pub fn extdst0_powerdown(&self) -> EXTDST0_POWERDOWN_R {
        EXTDST0_POWERDOWN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronization mode for extdst0 pipeline endpoint synchronizer"]
    #[inline(always)]
    pub fn extdst0_sync_mode(&self) -> EXTDST0_SYNC_MODE_R {
        EXTDST0_SYNC_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Software reset for extdst0 synchronizer, for debug purposes only"]
    #[inline(always)]
    pub fn extdst0_sw_reset(&self) -> EXTDST0_SW_RESET_R {
        EXTDST0_SW_RESET_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    pub fn extdst0_div(&self) -> EXTDST0_DIV_R {
        EXTDST0_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadowing of all RWS type registers (0=write_through, 1=shadowed) for pixelbus configuration of pipeline with endpoint extdst0."]
    #[inline(always)]
    #[must_use]
    pub fn extdst0_shden(&mut self) -> EXTDST0_SHDEN_W<0> {
        EXTDST0_SHDEN_W::new(self)
    }
    #[doc = "Bit 4 - Set this to 1 to activate powerdown or to 0 to deactivate powerdown for the extdst0 endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn extdst0_powerdown(&mut self) -> EXTDST0_POWERDOWN_W<4> {
        EXTDST0_POWERDOWN_W::new(self)
    }
    #[doc = "Bit 8 - Synchronization mode for extdst0 pipeline endpoint synchronizer"]
    #[inline(always)]
    #[must_use]
    pub fn extdst0_sync_mode(&mut self) -> EXTDST0_SYNC_MODE_W<8> {
        EXTDST0_SYNC_MODE_W::new(self)
    }
    #[doc = "Bit 11 - Software reset for extdst0 synchronizer, for debug purposes only"]
    #[inline(always)]
    #[must_use]
    pub fn extdst0_sw_reset(&mut self) -> EXTDST0_SW_RESET_W<11> {
        EXTDST0_SW_RESET_W::new(self)
    }
    #[doc = "Bits 16:23 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn extdst0_div(&mut self) -> EXTDST0_DIV_W<16> {
        EXTDST0_DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static pixel engine configuration for extdst0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extdst0_static](index.html) module"]
pub struct EXTDST0_STATIC_SPEC;
impl crate::RegisterSpec for EXTDST0_STATIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extdst0_static::R](R) reader structure"]
impl crate::Readable for EXTDST0_STATIC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extdst0_static::W](W) writer structure"]
impl crate::Writable for EXTDST0_STATIC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTDST0_STATIC to value 0x0080_0010"]
impl crate::Resettable for EXTDST0_STATIC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0080_0010;
}
