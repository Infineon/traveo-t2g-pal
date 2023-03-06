#[doc = "Register `PRTC2` reader"]
pub struct R(crate::R<PRTC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRTC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRTC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRTC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRTC2` writer"]
pub struct W(crate::W<PRTC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRTC2_SPEC>;
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
impl From<crate::W<PRTC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRTC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXI_` reader - Wakeup Symbol Receive Idle (gdWakeupSymbolRxIdle) Configures the number of bit times used by the node to test the duration of the idle phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 14 to 59 bit times."]
pub type RXI__R = crate::FieldReader<u8, RXI__A>;
#[doc = "Wakeup Symbol Receive Idle (gdWakeupSymbolRxIdle) Configures the number of bit times used by the node to test the duration of the idle phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 14 to 59 bit times.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXI__A {
    #[doc = "14: N/A"]
    MIN = 14,
    #[doc = "59: N/A"]
    MAX = 59,
}
impl From<RXI__A> for u8 {
    #[inline(always)]
    fn from(variant: RXI__A) -> Self {
        variant as _
    }
}
impl RXI__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXI__A> {
        match self.bits {
            14 => Some(RXI__A::MIN),
            59 => Some(RXI__A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == RXI__A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == RXI__A::MAX
    }
}
#[doc = "Field `RXI_` writer - Wakeup Symbol Receive Idle (gdWakeupSymbolRxIdle) Configures the number of bit times used by the node to test the duration of the idle phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 14 to 59 bit times."]
pub type RXI__W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC2_SPEC, u8, RXI__A, 6, O>;
impl<'a, const O: u8> RXI__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(RXI__A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(RXI__A::MAX)
    }
}
#[doc = "Field `RXL` reader - Wakeup Symbol Receive Low (gdWakeupSymbolRxLow) Configures the number of bit times used by the node to test the duration of the low phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 10 to 55 bit times."]
pub type RXL_R = crate::FieldReader<u8, RXL_A>;
#[doc = "Wakeup Symbol Receive Low (gdWakeupSymbolRxLow) Configures the number of bit times used by the node to test the duration of the low phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 10 to 55 bit times.\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXL_A {
    #[doc = "10: N/A"]
    MIN = 10,
    #[doc = "55: N/A"]
    MAX = 55,
}
impl From<RXL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXL_A) -> Self {
        variant as _
    }
}
impl RXL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXL_A> {
        match self.bits {
            10 => Some(RXL_A::MIN),
            55 => Some(RXL_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == RXL_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == RXL_A::MAX
    }
}
#[doc = "Field `RXL` writer - Wakeup Symbol Receive Low (gdWakeupSymbolRxLow) Configures the number of bit times used by the node to test the duration of the low phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 10 to 55 bit times."]
pub type RXL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC2_SPEC, u8, RXL_A, 6, O>;
impl<'a, const O: u8> RXL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(RXL_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(RXL_A::MAX)
    }
}
#[doc = "Field `TXI_` reader - Wakeup Symbol Transmit Idle (gdWakeupSymbolTxIdle) Configures the number of bit times used by the node to transmit the idle phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 45 to 180 bit times."]
pub type TXI__R = crate::FieldReader<u8, TXI__A>;
#[doc = "Wakeup Symbol Transmit Idle (gdWakeupSymbolTxIdle) Configures the number of bit times used by the node to transmit the idle phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 45 to 180 bit times.\n\nValue on reset: 45"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXI__A {
    #[doc = "45: N/A"]
    MIN = 45,
    #[doc = "180: N/A"]
    MAX = 180,
}
impl From<TXI__A> for u8 {
    #[inline(always)]
    fn from(variant: TXI__A) -> Self {
        variant as _
    }
}
impl TXI__R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXI__A> {
        match self.bits {
            45 => Some(TXI__A::MIN),
            180 => Some(TXI__A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == TXI__A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == TXI__A::MAX
    }
}
#[doc = "Field `TXI_` writer - Wakeup Symbol Transmit Idle (gdWakeupSymbolTxIdle) Configures the number of bit times used by the node to transmit the idle phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 45 to 180 bit times."]
pub type TXI__W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC2_SPEC, u8, TXI__A, 8, O>;
impl<'a, const O: u8> TXI__W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(TXI__A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(TXI__A::MAX)
    }
}
#[doc = "Field `TXL` reader - Wakeup Symbol Transmit Low (gdWakeupSymbolTxLow) Configures the number of bit times used by the node to transmit the low phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 15 to 60 bit times."]
pub type TXL_R = crate::FieldReader<u8, TXL_A>;
#[doc = "Wakeup Symbol Transmit Low (gdWakeupSymbolTxLow) Configures the number of bit times used by the node to transmit the low phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 15 to 60 bit times.\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXL_A {
    #[doc = "15: N/A"]
    MIN = 15,
    #[doc = "60: N/A"]
    MAX = 60,
}
impl From<TXL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXL_A) -> Self {
        variant as _
    }
}
impl TXL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXL_A> {
        match self.bits {
            15 => Some(TXL_A::MIN),
            60 => Some(TXL_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MIN`"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == TXL_A::MIN
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == TXL_A::MAX
    }
}
#[doc = "Field `TXL` writer - Wakeup Symbol Transmit Low (gdWakeupSymbolTxLow) Configures the number of bit times used by the node to transmit the low phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 15 to 60 bit times."]
pub type TXL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRTC2_SPEC, u8, TXL_A, 6, O>;
impl<'a, const O: u8> TXL_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn min(self) -> &'a mut W {
        self.variant(TXL_A::MIN)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(TXL_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:5 - Wakeup Symbol Receive Idle (gdWakeupSymbolRxIdle) Configures the number of bit times used by the node to test the duration of the idle phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 14 to 59 bit times."]
    #[inline(always)]
    pub fn rxi_(&self) -> RXI__R {
        RXI__R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Wakeup Symbol Receive Low (gdWakeupSymbolRxLow) Configures the number of bit times used by the node to test the duration of the low phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 10 to 55 bit times."]
    #[inline(always)]
    pub fn rxl(&self) -> RXL_R {
        RXL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Wakeup Symbol Transmit Idle (gdWakeupSymbolTxIdle) Configures the number of bit times used by the node to transmit the idle phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 45 to 180 bit times."]
    #[inline(always)]
    pub fn txi_(&self) -> TXI__R {
        TXI__R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:29 - Wakeup Symbol Transmit Low (gdWakeupSymbolTxLow) Configures the number of bit times used by the node to transmit the low phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 15 to 60 bit times."]
    #[inline(always)]
    pub fn txl(&self) -> TXL_R {
        TXL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Wakeup Symbol Receive Idle (gdWakeupSymbolRxIdle) Configures the number of bit times used by the node to test the duration of the idle phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 14 to 59 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn rxi_(&mut self) -> RXI__W<0> {
        RXI__W::new(self)
    }
    #[doc = "Bits 8:13 - Wakeup Symbol Receive Low (gdWakeupSymbolRxLow) Configures the number of bit times used by the node to test the duration of the low phase of the received wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 10 to 55 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn rxl(&mut self) -> RXL_W<8> {
        RXL_W::new(self)
    }
    #[doc = "Bits 16:23 - Wakeup Symbol Transmit Idle (gdWakeupSymbolTxIdle) Configures the number of bit times used by the node to transmit the idle phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 45 to 180 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn txi_(&mut self) -> TXI__W<16> {
        TXI__W::new(self)
    }
    #[doc = "Bits 24:29 - Wakeup Symbol Transmit Low (gdWakeupSymbolTxLow) Configures the number of bit times used by the node to transmit the low phase of the wakeup symbol. Must be identical in all nodes of a cluster. Valid values are 15 to 60 bit times."]
    #[inline(always)]
    #[must_use]
    pub fn txl(&mut self) -> TXL_W<24> {
        TXL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRT Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prtc2](index.html) module"]
pub struct PRTC2_SPEC;
impl crate::RegisterSpec for PRTC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prtc2::R](R) reader structure"]
impl crate::Readable for PRTC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prtc2::W](W) writer structure"]
impl crate::Writable for PRTC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRTC2 to value 0x0f2d_0a0e"]
impl crate::Resettable for PRTC2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f2d_0a0e;
}
