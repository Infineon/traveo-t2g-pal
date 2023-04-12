#[doc = "Register `CTR2_CTL` reader"]
pub struct R(crate::R<CTR2_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR2_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR2_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR2_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR2_CTL` writer"]
pub struct W(crate::W<CTR2_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR2_CTL_SPEC>;
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
impl From<crate::W<CTR2_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR2_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLED` reader - Indicates actual state of this subcounter. May lag ENABLE by up to two clk_lf cycles."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` reader - Enable subcounter. May take up to 2 clk_lf cycles to take effect. When ENABLE changes from 1->0, the counter is cleared. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable subcounter. May take up to 2 clk_lf cycles to take effect. When ENABLE changes from 1->0, the counter is cleared. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTR2_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Indicates actual state of this subcounter. May lag ENABLE by up to two clk_lf cycles."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Enable subcounter. May take up to 2 clk_lf cycles to take effect. When ENABLE changes from 1->0, the counter is cleared. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable subcounter. May take up to 2 clk_lf cycles to take effect. When ENABLE changes from 1->0, the counter is cleared. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<31> {
        ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCWDT Subcounter 2 Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr2_ctl](index.html) module"]
pub struct CTR2_CTL_SPEC;
impl crate::RegisterSpec for CTR2_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr2_ctl::R](R) reader structure"]
impl crate::Readable for CTR2_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr2_ctl::W](W) writer structure"]
impl crate::Writable for CTR2_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTR2_CTL to value 0"]
impl crate::Resettable for CTR2_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
