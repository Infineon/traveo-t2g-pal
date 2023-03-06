#[doc = "Register `TREG3` reader"]
pub struct R(crate::R<TREG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG3` writer"]
pub struct W(crate::W<TREG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG3_SPEC>;
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
impl From<crate::W<TREG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RP` reader - PRE command period = tRP -4"]
pub type T_RP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RP` writer - PRE command period = tRP -4"]
pub type T_RP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_MRR` reader - Mode Register Read register = tMRR"]
pub type T_MRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRR` writer - Mode Register Read register = tMRR"]
pub type T_MRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 4, O>;
#[doc = "Field `T_DQSCKE` reader - Valid Strobe Requirement before CKE Low = tDQSCKE"]
pub type T_DQSCKE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DQSCKE` writer - Valid Strobe Requirement before CKE Low = tDQSCKE"]
pub type T_DQSCKE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_DQRPT` reader - This field defines the number of consecutive times of reading data correctly to confirm the stability of DRAM read before DQS2DQ training finishes = 8."]
pub type T_DQRPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DQRPT` writer - This field defines the number of consecutive times of reading data correctly to confirm the stability of DRAM read before DQS2DQ training finishes = 8."]
pub type T_DQRPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_MRD` reader - Mode Register Set command cycle time = tMRD + 4"]
pub type T_MRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRD` writer - Mode Register Set command cycle time = tMRD + 4"]
pub type T_MRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_ZQLAT` reader - ZQ latch time = tZQLAT"]
pub type T_ZQLAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_ZQLAT` writer - ZQ latch time = tZQLAT"]
pub type T_ZQLAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG3_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:5 - PRE command period = tRP -4"]
    #[inline(always)]
    pub fn t_rp(&self) -> T_RP_R {
        T_RP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - Mode Register Read register = tMRR"]
    #[inline(always)]
    pub fn t_mrr(&self) -> T_MRR_R {
        T_MRR_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:14 - Valid Strobe Requirement before CKE Low = tDQSCKE"]
    #[inline(always)]
    pub fn t_dqscke(&self) -> T_DQSCKE_R {
        T_DQSCKE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - This field defines the number of consecutive times of reading data correctly to confirm the stability of DRAM read before DQS2DQ training finishes = 8."]
    #[inline(always)]
    pub fn t_dqrpt(&self) -> T_DQRPT_R {
        T_DQRPT_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Mode Register Set command cycle time = tMRD + 4"]
    #[inline(always)]
    pub fn t_mrd(&self) -> T_MRD_R {
        T_MRD_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:31 - ZQ latch time = tZQLAT"]
    #[inline(always)]
    pub fn t_zqlat(&self) -> T_ZQLAT_R {
        T_ZQLAT_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PRE command period = tRP -4"]
    #[inline(always)]
    #[must_use]
    pub fn t_rp(&mut self) -> T_RP_W<0> {
        T_RP_W::new(self)
    }
    #[doc = "Bits 6:9 - Mode Register Read register = tMRR"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrr(&mut self) -> T_MRR_W<6> {
        T_MRR_W::new(self)
    }
    #[doc = "Bits 10:14 - Valid Strobe Requirement before CKE Low = tDQSCKE"]
    #[inline(always)]
    #[must_use]
    pub fn t_dqscke(&mut self) -> T_DQSCKE_W<10> {
        T_DQSCKE_W::new(self)
    }
    #[doc = "Bits 15:19 - This field defines the number of consecutive times of reading data correctly to confirm the stability of DRAM read before DQS2DQ training finishes = 8."]
    #[inline(always)]
    #[must_use]
    pub fn t_dqrpt(&mut self) -> T_DQRPT_W<15> {
        T_DQRPT_W::new(self)
    }
    #[doc = "Bits 20:24 - Mode Register Set command cycle time = tMRD + 4"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrd(&mut self) -> T_MRD_W<20> {
        T_MRD_W::new(self)
    }
    #[doc = "Bits 25:31 - ZQ latch time = tZQLAT"]
    #[inline(always)]
    #[must_use]
    pub fn t_zqlat(&mut self) -> T_ZQLAT_W<25> {
        T_ZQLAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg3](index.html) module"]
pub struct TREG3_SPEC;
impl crate::RegisterSpec for TREG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg3::R](R) reader structure"]
impl crate::Readable for TREG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg3::W](W) writer structure"]
impl crate::Writable for TREG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG3 to value 0x1172_220b"]
impl crate::Resettable for TREG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x1172_220b;
}
