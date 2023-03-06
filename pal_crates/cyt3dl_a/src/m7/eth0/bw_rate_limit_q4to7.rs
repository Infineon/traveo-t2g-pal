#[doc = "Register `BW_RATE_LIMIT_Q4TO7` reader"]
pub struct R(crate::R<BW_RATE_LIMIT_Q4TO7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BW_RATE_LIMIT_Q4TO7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BW_RATE_LIMIT_Q4TO7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BW_RATE_LIMIT_Q4TO7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BW_RATE_LIMIT_Q4TO7` writer"]
pub struct W(crate::W<BW_RATE_LIMIT_Q4TO7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BW_RATE_LIMIT_Q4TO7_SPEC>;
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
impl From<crate::W<BW_RATE_LIMIT_Q4TO7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BW_RATE_LIMIT_Q4TO7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REMOVED_31_0` reader - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
pub type REMOVED_31_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REMOVED_31_0` writer - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
pub type REMOVED_31_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BW_RATE_LIMIT_Q4TO7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
    #[inline(always)]
    pub fn removed_31_0(&self) -> REMOVED_31_0_R {
        REMOVED_31_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 'DWRR Weighting / ETS Bandwidth Allocation for queue 0'"]
    #[inline(always)]
    #[must_use]
    pub fn removed_31_0(&mut self) -> REMOVED_31_0_W<0> {
        REMOVED_31_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Not presents. MXETH has only 3 queues. Access to the register returns AHB error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bw_rate_limit_q4to7](index.html) module"]
pub struct BW_RATE_LIMIT_Q4TO7_SPEC;
impl crate::RegisterSpec for BW_RATE_LIMIT_Q4TO7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bw_rate_limit_q4to7::R](R) reader structure"]
impl crate::Readable for BW_RATE_LIMIT_Q4TO7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bw_rate_limit_q4to7::W](W) writer structure"]
impl crate::Writable for BW_RATE_LIMIT_Q4TO7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BW_RATE_LIMIT_Q4TO7 to value 0"]
impl crate::Resettable for BW_RATE_LIMIT_Q4TO7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
