#[doc = "Register `RTCFG1_RT2` reader"]
pub struct R(crate::R<RTCFG1_RT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCFG1_RT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCFG1_RT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCFG1_RT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCFG1_RT2` writer"]
pub struct W(crate::W<RTCFG1_RT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCFG1_RT2_SPEC>;
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
impl From<crate::W<RTCFG1_RT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCFG1_RT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARQ_LAT_BARRIER` reader - Maximum Latency Barrier for Read Channel - Route 2"]
pub type ARQ_LAT_BARRIER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQ_LAT_BARRIER` writer - Maximum Latency Barrier for Read Channel - Route 2"]
pub type ARQ_LAT_BARRIER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCFG1_RT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `AWQ_LAT_BARRIER` reader - Maximum Latency Barrier for Write Channel - Route 2"]
pub type AWQ_LAT_BARRIER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQ_LAT_BARRIER` writer - Maximum Latency Barrier for Write Channel - Route 2"]
pub type AWQ_LAT_BARRIER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCFG1_RT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARQ_STARV_TH` reader - Denotes Read Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
pub type ARQ_STARV_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARQ_STARV_TH` writer - Denotes Read Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
pub type ARQ_STARV_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCFG1_RT2_SPEC, u8, u8, 8, O>;
#[doc = "Field `AWQ_STARV_TH` reader - Denotes Write Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
pub type AWQ_STARV_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWQ_STARV_TH` writer - Denotes Write Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
pub type AWQ_STARV_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RTCFG1_RT2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Maximum Latency Barrier for Read Channel - Route 2"]
    #[inline(always)]
    pub fn arq_lat_barrier(&self) -> ARQ_LAT_BARRIER_R {
        ARQ_LAT_BARRIER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Maximum Latency Barrier for Write Channel - Route 2"]
    #[inline(always)]
    pub fn awq_lat_barrier(&self) -> AWQ_LAT_BARRIER_R {
        AWQ_LAT_BARRIER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Denotes Read Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
    #[inline(always)]
    pub fn arq_starv_th(&self) -> ARQ_STARV_TH_R {
        ARQ_STARV_TH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Denotes Write Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
    #[inline(always)]
    pub fn awq_starv_th(&self) -> AWQ_STARV_TH_R {
        AWQ_STARV_TH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Latency Barrier for Read Channel - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn arq_lat_barrier(&mut self) -> ARQ_LAT_BARRIER_W<0> {
        ARQ_LAT_BARRIER_W::new(self)
    }
    #[doc = "Bits 8:15 - Maximum Latency Barrier for Write Channel - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn awq_lat_barrier(&mut self) -> AWQ_LAT_BARRIER_W<8> {
        AWQ_LAT_BARRIER_W::new(self)
    }
    #[doc = "Bits 16:23 - Denotes Read Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn arq_starv_th(&mut self) -> ARQ_STARV_TH_W<16> {
        ARQ_STARV_TH_W::new(self)
    }
    #[doc = "Bits 24:31 - Denotes Write Address Queue Threshold for starvation preventing. Its value represents the number of hits in a hit chain - Route 2"]
    #[inline(always)]
    #[must_use]
    pub fn awq_starv_th(&mut self) -> AWQ_STARV_TH_W<24> {
        AWQ_STARV_TH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Route Configuration 1 - Route 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcfg1_rt2](index.html) module"]
pub struct RTCFG1_RT2_SPEC;
impl crate::RegisterSpec for RTCFG1_RT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcfg1_rt2::R](R) reader structure"]
impl crate::Readable for RTCFG1_RT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcfg1_rt2::W](W) writer structure"]
impl crate::Writable for RTCFG1_RT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCFG1_RT2 to value 0"]
impl crate::Resettable for RTCFG1_RT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
