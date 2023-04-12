#[doc = "Register `RTCFG0_RT2` reader"]
pub struct R(crate::R<RTCFG0_RT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCFG0_RT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCFG0_RT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCFG0_RT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCFG0_RT2` writer"]
pub struct W(crate::W<RTCFG0_RT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCFG0_RT2_SPEC>;
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
impl From<crate::W<RTCFG0_RT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCFG0_RT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_PRI` reader - Selects whether to use external AXI QoS field or internal combination of static and dynamic priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type EXT_PRI_R = crate::BitReader<bool>;
#[doc = "Field `EXT_PRI` writer - Selects whether to use external AXI QoS field or internal combination of static and dynamic priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type EXT_PRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `MAX_PRI` reader - Denotes Static Maximum Priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type MAX_PRI_R = crate::BitReader<bool>;
#[doc = "Field `MAX_PRI` writer - Denotes Static Maximum Priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type MAX_PRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `ARQ_LVL_HI` reader - Denotes Read Address Queue High Priority Threshold - Route 2"]
pub type ARQ_LVL_HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQ_LVL_HI` writer - Denotes Read Address Queue High Priority Threshold - Route 2"]
pub type ARQ_LVL_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCFG0_RT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ARQ_LVL_LO` reader - Denotes Read Address Queue Low Priority Threshold - Route 2"]
pub type ARQ_LVL_LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQ_LVL_LO` writer - Denotes Read Address Queue Low Priority Threshold - Route 2"]
pub type ARQ_LVL_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCFG0_RT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWQ_LVL_HI` reader - Denotes Write Address Queue High Priority Threshold - Route 2"]
pub type AWQ_LVL_HI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQ_LVL_HI` writer - Denotes Write Address Queue High Priority Threshold - Route 2"]
pub type AWQ_LVL_HI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCFG0_RT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWQ_LVL_LO` reader - Denotes Write Address Queue Low Priority Threshold - Route 2"]
pub type AWQ_LVL_LO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQ_LVL_LO` writer - Denotes Write Address Queue Low Priority Threshold - Route 2"]
pub type AWQ_LVL_LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTCFG0_RT2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ARQ_LAT_BARRIER_EN` reader - Maximum Latency Barrier Enable for Read Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ARQ_LAT_BARRIER_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARQ_LAT_BARRIER_EN` writer - Maximum Latency Barrier Enable for Read Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ARQ_LAT_BARRIER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `AWQ_LAT_BARRIER_EN` reader - Maximum Latency Barrier Enable for Write Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type AWQ_LAT_BARRIER_EN_R = crate::BitReader<bool>;
#[doc = "Field `AWQ_LAT_BARRIER_EN` writer - Maximum Latency Barrier Enable for Write Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type AWQ_LAT_BARRIER_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `ARQ_OOO_EN` reader - Out of order enable for read channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ARQ_OOO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARQ_OOO_EN` writer - Out of order enable for read channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ARQ_OOO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `AWQ_OOO_EN` reader - Out of order enable for write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type AWQ_OOO_EN_R = crate::BitReader<bool>;
#[doc = "Field `AWQ_OOO_EN` writer - Out of order enable for write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type AWQ_OOO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `ACQ_REALTIME_EN` reader - Realtime priority enable for Read/Write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ACQ_REALTIME_EN_R = crate::BitReader<bool>;
#[doc = "Field `ACQ_REALTIME_EN` writer - Realtime priority enable for Read/Write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ACQ_REALTIME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `WM_ENABLE` reader - Masked Write Support - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WM_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `WM_ENABLE` writer - Masked Write Support - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type WM_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `ARQ_LAHEAD_EN` reader - Out of order enable for Look-ahead buffer in read channel - Route 2"]
pub type ARQ_LAHEAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `ARQ_LAHEAD_EN` writer - Out of order enable for Look-ahead buffer in read channel - Route 2"]
pub type ARQ_LAHEAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
#[doc = "Field `AWQ_LAHEAD_EN` reader - Out of order enable for Look-ahead buffer in Write channel - Route 2"]
pub type AWQ_LAHEAD_EN_R = crate::BitReader<bool>;
#[doc = "Field `AWQ_LAHEAD_EN` writer - Out of order enable for Look-ahead buffer in Write channel - Route 2"]
pub type AWQ_LAHEAD_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RTCFG0_RT2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Selects whether to use external AXI QoS field or internal combination of static and dynamic priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn ext_pri(&self) -> EXT_PRI_R {
        EXT_PRI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Denotes Static Maximum Priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn max_pri(&self) -> MAX_PRI_R {
        MAX_PRI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Denotes Read Address Queue High Priority Threshold - Route 2"]
    #[inline(always)]
    pub fn arq_lvl_hi(&self) -> ARQ_LVL_HI_R {
        ARQ_LVL_HI_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Denotes Read Address Queue Low Priority Threshold - Route 2"]
    #[inline(always)]
    pub fn arq_lvl_lo(&self) -> ARQ_LVL_LO_R {
        ARQ_LVL_LO_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Denotes Write Address Queue High Priority Threshold - Route 2"]
    #[inline(always)]
    pub fn awq_lvl_hi(&self) -> AWQ_LVL_HI_R {
        AWQ_LVL_HI_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bits 14:17 - Denotes Write Address Queue Low Priority Threshold - Route 2"]
    #[inline(always)]
    pub fn awq_lvl_lo(&self) -> AWQ_LVL_LO_R {
        AWQ_LVL_LO_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Maximum Latency Barrier Enable for Read Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn arq_lat_barrier_en(&self) -> ARQ_LAT_BARRIER_EN_R {
        ARQ_LAT_BARRIER_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Maximum Latency Barrier Enable for Write Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn awq_lat_barrier_en(&self) -> AWQ_LAT_BARRIER_EN_R {
        AWQ_LAT_BARRIER_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Out of order enable for read channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn arq_ooo_en(&self) -> ARQ_OOO_EN_R {
        ARQ_OOO_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Out of order enable for write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn awq_ooo_en(&self) -> AWQ_OOO_EN_R {
        AWQ_OOO_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Realtime priority enable for Read/Write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn acq_realtime_en(&self) -> ACQ_REALTIME_EN_R {
        ACQ_REALTIME_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Masked Write Support - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn wm_enable(&self) -> WM_ENABLE_R {
        WM_ENABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Out of order enable for Look-ahead buffer in read channel - Route 2"]
    #[inline(always)]
    pub fn arq_lahead_en(&self) -> ARQ_LAHEAD_EN_R {
        ARQ_LAHEAD_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Out of order enable for Look-ahead buffer in Write channel - Route 2"]
    #[inline(always)]
    pub fn awq_lahead_en(&self) -> AWQ_LAHEAD_EN_R {
        AWQ_LAHEAD_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects whether to use external AXI QoS field or internal combination of static and dynamic priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn ext_pri(&mut self) -> EXT_PRI_W<0> {
        EXT_PRI_W::new(self)
    }
    #[doc = "Bit 1 - Denotes Static Maximum Priority - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn max_pri(&mut self) -> MAX_PRI_W<1> {
        MAX_PRI_W::new(self)
    }
    #[doc = "Bits 2:5 - Denotes Read Address Queue High Priority Threshold - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn arq_lvl_hi(&mut self) -> ARQ_LVL_HI_W<2> {
        ARQ_LVL_HI_W::new(self)
    }
    #[doc = "Bits 6:9 - Denotes Read Address Queue Low Priority Threshold - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn arq_lvl_lo(&mut self) -> ARQ_LVL_LO_W<6> {
        ARQ_LVL_LO_W::new(self)
    }
    #[doc = "Bits 10:13 - Denotes Write Address Queue High Priority Threshold - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn awq_lvl_hi(&mut self) -> AWQ_LVL_HI_W<10> {
        AWQ_LVL_HI_W::new(self)
    }
    #[doc = "Bits 14:17 - Denotes Write Address Queue Low Priority Threshold - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn awq_lvl_lo(&mut self) -> AWQ_LVL_LO_W<14> {
        AWQ_LVL_LO_W::new(self)
    }
    #[doc = "Bit 18 - Maximum Latency Barrier Enable for Read Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn arq_lat_barrier_en(&mut self) -> ARQ_LAT_BARRIER_EN_W<18> {
        ARQ_LAT_BARRIER_EN_W::new(self)
    }
    #[doc = "Bit 19 - Maximum Latency Barrier Enable for Write Channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn awq_lat_barrier_en(&mut self) -> AWQ_LAT_BARRIER_EN_W<19> {
        AWQ_LAT_BARRIER_EN_W::new(self)
    }
    #[doc = "Bit 20 - Out of order enable for read channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn arq_ooo_en(&mut self) -> ARQ_OOO_EN_W<20> {
        ARQ_OOO_EN_W::new(self)
    }
    #[doc = "Bit 21 - Out of order enable for write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn awq_ooo_en(&mut self) -> AWQ_OOO_EN_W<21> {
        AWQ_OOO_EN_W::new(self)
    }
    #[doc = "Bit 22 - Realtime priority enable for Read/Write channel - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn acq_realtime_en(&mut self) -> ACQ_REALTIME_EN_W<22> {
        ACQ_REALTIME_EN_W::new(self)
    }
    #[doc = "Bit 23 - Masked Write Support - Route 2 Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn wm_enable(&mut self) -> WM_ENABLE_W<23> {
        WM_ENABLE_W::new(self)
    }
    #[doc = "Bit 24 - Out of order enable for Look-ahead buffer in read channel - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn arq_lahead_en(&mut self) -> ARQ_LAHEAD_EN_W<24> {
        ARQ_LAHEAD_EN_W::new(self)
    }
    #[doc = "Bit 25 - Out of order enable for Look-ahead buffer in Write channel - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn awq_lahead_en(&mut self) -> AWQ_LAHEAD_EN_W<25> {
        AWQ_LAHEAD_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Route Configuration 0 - Route 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcfg0_rt2](index.html) module"]
pub struct RTCFG0_RT2_SPEC;
impl crate::RegisterSpec for RTCFG0_RT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcfg0_rt2::R](R) reader structure"]
impl crate::Readable for RTCFG0_RT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcfg0_rt2::W](W) writer structure"]
impl crate::Writable for RTCFG0_RT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCFG0_RT2 to value 0x3c3c"]
impl crate::Resettable for RTCFG0_RT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x3c3c;
}
