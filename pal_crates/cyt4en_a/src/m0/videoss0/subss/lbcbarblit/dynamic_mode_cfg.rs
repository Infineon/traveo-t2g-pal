#[doc = "Register `DYNAMIC_MODE_CFG` reader"]
pub struct R(crate::R<DYNAMIC_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMIC_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMIC_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMIC_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMIC_MODE_CFG` writer"]
pub struct W(crate::W<DYNAMIC_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMIC_MODE_CFG_SPEC>;
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
impl From<crate::W<DYNAMIC_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMIC_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MST0_ENABLED` reader - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST0_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `MST0_ENABLED` writer - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST0_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DYNAMIC_MODE_CFG_SPEC, bool, O>;
#[doc = "Field `MST1_ENABLED` reader - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST1_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `MST1_ENABLED` writer - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST1_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DYNAMIC_MODE_CFG_SPEC, bool, O>;
#[doc = "Field `MST2_ENABLED` reader - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST2_ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `MST2_ENABLED` writer - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
pub type MST2_ENABLED_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DYNAMIC_MODE_CFG_SPEC, bool, O>;
#[doc = "Field `LOAD_MIN` reader - Minimum threshold for the transfer counter in multiple of 4 beats. While the counter for a certain master port is lower than the threshold, all requests received on the slave port with the same ID are routed to that master, even if other ports have a lower counter value. Setting this field to 0 disables the threshold function. For a disabled master port this field has no effect."]
pub type LOAD_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOAD_MIN` writer - Minimum threshold for the transfer counter in multiple of 4 beats. While the counter for a certain master port is lower than the threshold, all requests received on the slave port with the same ID are routed to that master, even if other ports have a lower counter value. Setting this field to 0 disables the threshold function. For a disabled master port this field has no effect."]
pub type LOAD_MIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DYNAMIC_MODE_CFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `LOAD_MAX` reader - Maximum threshold for transfer counter in multiple of 4 beats. While the counter for a certain master port is higher than the threshold, the port is disabled; no requests will be routed to it. While all ports have reached that limit, incoming requests are stalled. Setting this field to 0 disables the threshold function. The effect of LOAD_MAX has higher priority than LOAD_MIN."]
pub type LOAD_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOAD_MAX` writer - Maximum threshold for transfer counter in multiple of 4 beats. While the counter for a certain master port is higher than the threshold, the port is disabled; no requests will be routed to it. While all ports have reached that limit, incoming requests are stalled. Setting this field to 0 disables the threshold function. The effect of LOAD_MAX has higher priority than LOAD_MIN."]
pub type LOAD_MAX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DYNAMIC_MODE_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    pub fn mst0_enabled(&self) -> MST0_ENABLED_R {
        MST0_ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    pub fn mst1_enabled(&self) -> MST1_ENABLED_R {
        MST1_ENABLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    pub fn mst2_enabled(&self) -> MST2_ENABLED_R {
        MST2_ENABLED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Minimum threshold for the transfer counter in multiple of 4 beats. While the counter for a certain master port is lower than the threshold, all requests received on the slave port with the same ID are routed to that master, even if other ports have a lower counter value. Setting this field to 0 disables the threshold function. For a disabled master port this field has no effect."]
    #[inline(always)]
    pub fn load_min(&self) -> LOAD_MIN_R {
        LOAD_MIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Maximum threshold for transfer counter in multiple of 4 beats. While the counter for a certain master port is higher than the threshold, the port is disabled; no requests will be routed to it. While all ports have reached that limit, incoming requests are stalled. Setting this field to 0 disables the threshold function. The effect of LOAD_MAX has higher priority than LOAD_MIN."]
    #[inline(always)]
    pub fn load_max(&self) -> LOAD_MAX_R {
        LOAD_MAX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    #[must_use]
    pub fn mst0_enabled(&mut self) -> MST0_ENABLED_W<0> {
        MST0_ENABLED_W::new(self)
    }
    #[doc = "Bit 4 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    #[must_use]
    pub fn mst1_enabled(&mut self) -> MST1_ENABLED_W<4> {
        MST1_ENABLED_W::new(self)
    }
    #[doc = "Bit 8 - Enable bit for master port 0. While disabled, no new requests will be routed to this port."]
    #[inline(always)]
    #[must_use]
    pub fn mst2_enabled(&mut self) -> MST2_ENABLED_W<8> {
        MST2_ENABLED_W::new(self)
    }
    #[doc = "Bits 16:23 - Minimum threshold for the transfer counter in multiple of 4 beats. While the counter for a certain master port is lower than the threshold, all requests received on the slave port with the same ID are routed to that master, even if other ports have a lower counter value. Setting this field to 0 disables the threshold function. For a disabled master port this field has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn load_min(&mut self) -> LOAD_MIN_W<16> {
        LOAD_MIN_W::new(self)
    }
    #[doc = "Bits 24:31 - Maximum threshold for transfer counter in multiple of 4 beats. While the counter for a certain master port is higher than the threshold, the port is disabled; no requests will be routed to it. While all ports have reached that limit, incoming requests are stalled. Setting this field to 0 disables the threshold function. The effect of LOAD_MAX has higher priority than LOAD_MIN."]
    #[inline(always)]
    #[must_use]
    pub fn load_max(&mut self) -> LOAD_MAX_W<24> {
        LOAD_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for dynamic arbitration modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamic_mode_cfg](index.html) module"]
pub struct DYNAMIC_MODE_CFG_SPEC;
impl crate::RegisterSpec for DYNAMIC_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamic_mode_cfg::R](R) reader structure"]
impl crate::Readable for DYNAMIC_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamic_mode_cfg::W](W) writer structure"]
impl crate::Writable for DYNAMIC_MODE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DYNAMIC_MODE_CFG to value 0x0111"]
impl crate::Resettable for DYNAMIC_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0111;
}
