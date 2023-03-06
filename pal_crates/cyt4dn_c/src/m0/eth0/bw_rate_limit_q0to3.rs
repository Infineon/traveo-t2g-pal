#[doc = "Register `BW_RATE_LIMIT_Q0TO3` reader"]
pub struct R(crate::R<BW_RATE_LIMIT_Q0TO3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BW_RATE_LIMIT_Q0TO3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BW_RATE_LIMIT_Q0TO3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BW_RATE_LIMIT_Q0TO3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BW_RATE_LIMIT_Q0TO3` writer"]
pub struct W(crate::W<BW_RATE_LIMIT_Q0TO3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BW_RATE_LIMIT_Q0TO3_SPEC>;
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
impl From<crate::W<BW_RATE_LIMIT_Q0TO3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BW_RATE_LIMIT_Q0TO3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DWRR_ETS_WEIGHT_Q0` reader - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
pub type DWRR_ETS_WEIGHT_Q0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q0` writer - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
pub type DWRR_ETS_WEIGHT_Q0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BW_RATE_LIMIT_Q0TO3_SPEC, u8, u8, 8, O>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q1` reader - 'DWRR Weighting / ETS Bandwidth Allocation for queue 1'"]
pub type DWRR_ETS_WEIGHT_Q1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q1` writer - 'DWRR Weighting / ETS Bandwidth Allocation for queue 1'"]
pub type DWRR_ETS_WEIGHT_Q1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BW_RATE_LIMIT_Q0TO3_SPEC, u8, u8, 8, O>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q2` reader - 'DWRR Weighting / ETS Bandwidth Allocation for queue 2'"]
pub type DWRR_ETS_WEIGHT_Q2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q2` writer - 'DWRR Weighting / ETS Bandwidth Allocation for queue 2'"]
pub type DWRR_ETS_WEIGHT_Q2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BW_RATE_LIMIT_Q0TO3_SPEC, u8, u8, 8, O>;
#[doc = "Field `DWRR_ETS_WEIGHT_Q3` reader - Write ignore, read 0"]
pub type DWRR_ETS_WEIGHT_Q3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
    #[inline(always)]
    pub fn dwrr_ets_weight_q0(&self) -> DWRR_ETS_WEIGHT_Q0_R {
        DWRR_ETS_WEIGHT_Q0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 1'"]
    #[inline(always)]
    pub fn dwrr_ets_weight_q1(&self) -> DWRR_ETS_WEIGHT_Q1_R {
        DWRR_ETS_WEIGHT_Q1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 2'"]
    #[inline(always)]
    pub fn dwrr_ets_weight_q2(&self) -> DWRR_ETS_WEIGHT_Q2_R {
        DWRR_ETS_WEIGHT_Q2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Write ignore, read 0"]
    #[inline(always)]
    pub fn dwrr_ets_weight_q3(&self) -> DWRR_ETS_WEIGHT_Q3_R {
        DWRR_ETS_WEIGHT_Q3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
    #[inline(always)]
    #[must_use]
    pub fn dwrr_ets_weight_q0(&mut self) -> DWRR_ETS_WEIGHT_Q0_W<0> {
        DWRR_ETS_WEIGHT_Q0_W::new(self)
    }
    #[doc = "Bits 8:15 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 1'"]
    #[inline(always)]
    #[must_use]
    pub fn dwrr_ets_weight_q1(&mut self) -> DWRR_ETS_WEIGHT_Q1_W<8> {
        DWRR_ETS_WEIGHT_Q1_W::new(self)
    }
    #[doc = "Bits 16:23 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 2'"]
    #[inline(always)]
    #[must_use]
    pub fn dwrr_ets_weight_q2(&mut self) -> DWRR_ETS_WEIGHT_Q2_W<16> {
        DWRR_ETS_WEIGHT_Q2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register holds the DWRR weighting value or the ETS bandwidth percentage value used by the transmit scheduler for queues 0 to 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bw_rate_limit_q0to3](index.html) module"]
pub struct BW_RATE_LIMIT_Q0TO3_SPEC;
impl crate::RegisterSpec for BW_RATE_LIMIT_Q0TO3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bw_rate_limit_q0to3::R](R) reader structure"]
impl crate::Readable for BW_RATE_LIMIT_Q0TO3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bw_rate_limit_q0to3::W](W) writer structure"]
impl crate::Writable for BW_RATE_LIMIT_Q0TO3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BW_RATE_LIMIT_Q0TO3 to value 0"]
impl crate::Resettable for BW_RATE_LIMIT_Q0TO3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
