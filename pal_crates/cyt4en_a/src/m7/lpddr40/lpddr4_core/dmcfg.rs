#[doc = "Register `DMCFG` reader"]
pub struct R(crate::R<DMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCFG` writer"]
pub struct W(crate::W<DMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCFG_SPEC>;
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
impl From<crate::W<DMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REF_POST_PULL_EN` reader - Enables Postpone/Pull Automatic Refresh. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type REF_POST_PULL_EN_R = crate::BitReader<bool>;
#[doc = "Field `REF_POST_PULL_EN` writer - Enables Postpone/Pull Automatic Refresh. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type REF_POST_PULL_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
#[doc = "Field `INT_GC_FSM_EN` reader - Global FSM Error interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INT_GC_FSM_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_GC_FSM_EN` writer - Global FSM Error interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INT_GC_FSM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
#[doc = "Field `INT_GC_FSM_CLR` reader - Global FSM Error interrupt clear signal. DONT_CLEAR = 0 Don't clear. DO_CLEAR = 1 Clear. Set int_gc_fsm LOW."]
pub type INT_GC_FSM_CLR_R = crate::BitReader<bool>;
#[doc = "Field `INT_GC_FSM_CLR` writer - Global FSM Error interrupt clear signal. DONT_CLEAR = 0 Don't clear. DO_CLEAR = 1 Clear. Set int_gc_fsm LOW."]
pub type INT_GC_FSM_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
#[doc = "Field `INT_ECC_EN` reader - ECC interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INT_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `INT_ECC_EN` writer - ECC interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INT_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
#[doc = "Field `REQ_TH` reader - Look-ahead buffer request threshold"]
pub type REQ_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REQ_TH` writer - Look-ahead buffer request threshold"]
pub type REQ_TH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ZQ_AUTO_EN` reader - Automatic ZQ Calibration enable. In case Automatic ZQ Calibration is used, the rate is defined in TREG9.T_ZQ_ITV. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ZQ_AUTO_EN_R = crate::BitReader<bool>;
#[doc = "Field `ZQ_AUTO_EN` writer - Automatic ZQ Calibration enable. In case Automatic ZQ Calibration is used, the rate is defined in TREG9.T_ZQ_ITV. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type ZQ_AUTO_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
#[doc = "Field `INLINE_ECC_EN` reader - Inline ECC enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INLINE_ECC_EN_R = crate::BitReader<bool>;
#[doc = "Field `INLINE_ECC_EN` writer - Inline ECC enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
pub type INLINE_ECC_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables Postpone/Pull Automatic Refresh. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn ref_post_pull_en(&self) -> REF_POST_PULL_EN_R {
        REF_POST_PULL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Global FSM Error interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn int_gc_fsm_en(&self) -> INT_GC_FSM_EN_R {
        INT_GC_FSM_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global FSM Error interrupt clear signal. DONT_CLEAR = 0 Don't clear. DO_CLEAR = 1 Clear. Set int_gc_fsm LOW."]
    #[inline(always)]
    pub fn int_gc_fsm_clr(&self) -> INT_GC_FSM_CLR_R {
        INT_GC_FSM_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECC interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn int_ecc_en(&self) -> INT_ECC_EN_R {
        INT_ECC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Look-ahead buffer request threshold"]
    #[inline(always)]
    pub fn req_th(&self) -> REQ_TH_R {
        REQ_TH_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Automatic ZQ Calibration enable. In case Automatic ZQ Calibration is used, the rate is defined in TREG9.T_ZQ_ITV. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn zq_auto_en(&self) -> ZQ_AUTO_EN_R {
        ZQ_AUTO_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Inline ECC enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    pub fn inline_ecc_en(&self) -> INLINE_ECC_EN_R {
        INLINE_ECC_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables Postpone/Pull Automatic Refresh. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn ref_post_pull_en(&mut self) -> REF_POST_PULL_EN_W<0> {
        REF_POST_PULL_EN_W::new(self)
    }
    #[doc = "Bit 1 - Global FSM Error interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn int_gc_fsm_en(&mut self) -> INT_GC_FSM_EN_W<1> {
        INT_GC_FSM_EN_W::new(self)
    }
    #[doc = "Bit 2 - Global FSM Error interrupt clear signal. DONT_CLEAR = 0 Don't clear. DO_CLEAR = 1 Clear. Set int_gc_fsm LOW."]
    #[inline(always)]
    #[must_use]
    pub fn int_gc_fsm_clr(&mut self) -> INT_GC_FSM_CLR_W<2> {
        INT_GC_FSM_CLR_W::new(self)
    }
    #[doc = "Bit 3 - ECC interrupt enable signal. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn int_ecc_en(&mut self) -> INT_ECC_EN_W<3> {
        INT_ECC_EN_W::new(self)
    }
    #[doc = "Bits 4:6 - Look-ahead buffer request threshold"]
    #[inline(always)]
    #[must_use]
    pub fn req_th(&mut self) -> REQ_TH_W<4> {
        REQ_TH_W::new(self)
    }
    #[doc = "Bit 7 - Automatic ZQ Calibration enable. In case Automatic ZQ Calibration is used, the rate is defined in TREG9.T_ZQ_ITV. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn zq_auto_en(&mut self) -> ZQ_AUTO_EN_W<7> {
        ZQ_AUTO_EN_W::new(self)
    }
    #[doc = "Bit 8 - Inline ECC enable. Enable/disable coding: DISABLE = 0 Disable this feature ENABLE = 1 Enable this feature"]
    #[inline(always)]
    #[must_use]
    pub fn inline_ecc_en(&mut self) -> INLINE_ECC_EN_W<8> {
        INLINE_ECC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamo Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcfg](index.html) module"]
pub struct DMCFG_SPEC;
impl crate::RegisterSpec for DMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmcfg::R](R) reader structure"]
impl crate::Readable for DMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcfg::W](W) writer structure"]
impl crate::Writable for DMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMCFG to value 0"]
impl crate::Resettable for DMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
