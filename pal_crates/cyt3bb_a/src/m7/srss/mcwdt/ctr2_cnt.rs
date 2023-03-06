#[doc = "Register `CTR2_CNT` reader"]
pub struct R(crate::R<CTR2_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR2_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR2_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR2_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR2_CNT` writer"]
pub struct W(crate::W<CTR2_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR2_CNT_SPEC>;
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
impl From<crate::W<CTR2_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR2_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT2` reader - Current value of subcounter 2 for this MCWDT. This field may lag the actual count value by up to one clk_lf cycle, due to internal synchronization. When this subcounter is disabled and unlocked, the count value can be written for verification and debugging purposes. Software writes are always ignored when the subcounter is enabled."]
pub type CNT2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT2` writer - Current value of subcounter 2 for this MCWDT. This field may lag the actual count value by up to one clk_lf cycle, due to internal synchronization. When this subcounter is disabled and unlocked, the count value can be written for verification and debugging purposes. Software writes are always ignored when the subcounter is enabled."]
pub type CNT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTR2_CNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Current value of subcounter 2 for this MCWDT. This field may lag the actual count value by up to one clk_lf cycle, due to internal synchronization. When this subcounter is disabled and unlocked, the count value can be written for verification and debugging purposes. Software writes are always ignored when the subcounter is enabled."]
    #[inline(always)]
    pub fn cnt2(&self) -> CNT2_R {
        CNT2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current value of subcounter 2 for this MCWDT. This field may lag the actual count value by up to one clk_lf cycle, due to internal synchronization. When this subcounter is disabled and unlocked, the count value can be written for verification and debugging purposes. Software writes are always ignored when the subcounter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cnt2(&mut self) -> CNT2_W<0> {
        CNT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCWDT Subcounter 2 Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr2_cnt](index.html) module"]
pub struct CTR2_CNT_SPEC;
impl crate::RegisterSpec for CTR2_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr2_cnt::R](R) reader structure"]
impl crate::Readable for CTR2_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr2_cnt::W](W) writer structure"]
impl crate::Writable for CTR2_CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR2_CNT to value 0"]
impl crate::Resettable for CTR2_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
