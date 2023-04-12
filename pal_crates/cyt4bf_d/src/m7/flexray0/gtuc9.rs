#[doc = "Register `GTUC9` reader"]
pub struct R(crate::R<GTUC9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC9` writer"]
pub struct W(crate::W<GTUC9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC9_SPEC>;
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
impl From<crate::W<GTUC9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APO` reader - Action Point Offset (gdActionPointOffset) Configures the action point offset in macroticks within static slots and symbol window. Must be identical in all nodes of a cluster. Valid values are 1 to 63 MT."]
pub type APO_R = crate::FieldReader<u8, APO_A>;
#[doc = "Action Point Offset (gdActionPointOffset) Configures the action point offset in macroticks within static slots and symbol window. Must be identical in all nodes of a cluster. Valid values are 1 to 63 MT.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APO_A {
    #[doc = "1: N/A"]
    MIN = 1,
}
impl From<APO_A> for u8 {
    #[inline(always)]
    fn from(variant: APO_A) -> Self {
        variant as _
    }
}
impl APO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APO_A> {
        match self.bits {
            1 => Some(APO_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == APO_A::MIN
    }
}
#[doc = "Field `APO` writer - Action Point Offset (gdActionPointOffset) Configures the action point offset in macroticks within static slots and symbol window. Must be identical in all nodes of a cluster. Valid values are 1 to 63 MT."]
pub type APO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC9_SPEC, u8, APO_A, 6, O>;
impl<'a, const O: u8> APO_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(APO_A::MIN)
    }
}
#[doc = "Field `MAPO` reader - Minislot Action Point Offset (gdMinislotActionPointOffset) Configures the action point offset in macroticks within the minislots of the dynamic segment. Must be identical in all nodes of a cluster. Valid values are 1 to 31 MT."]
pub type MAPO_R = crate::FieldReader<u8, MAPO_A>;
#[doc = "Minislot Action Point Offset (gdMinislotActionPointOffset) Configures the action point offset in macroticks within the minislots of the dynamic segment. Must be identical in all nodes of a cluster. Valid values are 1 to 31 MT.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MAPO_A {
    #[doc = "1: N/A"]
    MIN = 1,
}
impl From<MAPO_A> for u8 {
    #[inline(always)]
    fn from(variant: MAPO_A) -> Self {
        variant as _
    }
}
impl MAPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MAPO_A> {
        match self.bits {
            1 => Some(MAPO_A::MIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == MAPO_A::MIN
    }
}
#[doc = "Field `MAPO` writer - Minislot Action Point Offset (gdMinislotActionPointOffset) Configures the action point offset in macroticks within the minislots of the dynamic segment. Must be identical in all nodes of a cluster. Valid values are 1 to 31 MT."]
pub type MAPO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC9_SPEC, u8, MAPO_A, 5, O>;
impl<'a, const O: u8> MAPO_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(MAPO_A::MIN)
    }
}
#[doc = "Field `DSI` reader - Dynamic Slot Idle Phase (gdDynamicSlotIdlePhase) The duration of the dynamic slot idle phase has to be greater or equal than the idle detection time. Must be identical in all nodes of a cluster. Valid values are 0 to 2 Minislot."]
pub type DSI_R = crate::FieldReader<u8, DSI_A>;
#[doc = "Dynamic Slot Idle Phase (gdDynamicSlotIdlePhase) The duration of the dynamic slot idle phase has to be greater or equal than the idle detection time. Must be identical in all nodes of a cluster. Valid values are 0 to 2 Minislot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DSI_A {
    #[doc = "2: N/A"]
    MAX = 2,
}
impl From<DSI_A> for u8 {
    #[inline(always)]
    fn from(variant: DSI_A) -> Self {
        variant as _
    }
}
impl DSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSI_A> {
        match self.bits {
            2 => Some(DSI_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == DSI_A::MAX
    }
}
#[doc = "Field `DSI` writer - Dynamic Slot Idle Phase (gdDynamicSlotIdlePhase) The duration of the dynamic slot idle phase has to be greater or equal than the idle detection time. Must be identical in all nodes of a cluster. Valid values are 0 to 2 Minislot."]
pub type DSI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC9_SPEC, u8, DSI_A, 2, O>;
impl<'a, const O: u8> DSI_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(DSI_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:5 - Action Point Offset (gdActionPointOffset) Configures the action point offset in macroticks within static slots and symbol window. Must be identical in all nodes of a cluster. Valid values are 1 to 63 MT."]
    #[inline(always)]
    pub fn apo(&self) -> APO_R {
        APO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Minislot Action Point Offset (gdMinislotActionPointOffset) Configures the action point offset in macroticks within the minislots of the dynamic segment. Must be identical in all nodes of a cluster. Valid values are 1 to 31 MT."]
    #[inline(always)]
    pub fn mapo(&self) -> MAPO_R {
        MAPO_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Dynamic Slot Idle Phase (gdDynamicSlotIdlePhase) The duration of the dynamic slot idle phase has to be greater or equal than the idle detection time. Must be identical in all nodes of a cluster. Valid values are 0 to 2 Minislot."]
    #[inline(always)]
    pub fn dsi(&self) -> DSI_R {
        DSI_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Action Point Offset (gdActionPointOffset) Configures the action point offset in macroticks within static slots and symbol window. Must be identical in all nodes of a cluster. Valid values are 1 to 63 MT."]
    #[inline(always)]
    #[must_use]
    pub fn apo(&mut self) -> APO_W<0> {
        APO_W::new(self)
    }
    #[doc = "Bits 8:12 - Minislot Action Point Offset (gdMinislotActionPointOffset) Configures the action point offset in macroticks within the minislots of the dynamic segment. Must be identical in all nodes of a cluster. Valid values are 1 to 31 MT."]
    #[inline(always)]
    #[must_use]
    pub fn mapo(&mut self) -> MAPO_W<8> {
        MAPO_W::new(self)
    }
    #[doc = "Bits 16:17 - Dynamic Slot Idle Phase (gdDynamicSlotIdlePhase) The duration of the dynamic slot idle phase has to be greater or equal than the idle detection time. Must be identical in all nodes of a cluster. Valid values are 0 to 2 Minislot."]
    #[inline(always)]
    #[must_use]
    pub fn dsi(&mut self) -> DSI_W<16> {
        DSI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc9](index.html) module"]
pub struct GTUC9_SPEC;
impl crate::RegisterSpec for GTUC9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc9::R](R) reader structure"]
impl crate::Readable for GTUC9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc9::W](W) writer structure"]
impl crate::Writable for GTUC9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC9 to value 0x0101"]
impl crate::Resettable for GTUC9_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
