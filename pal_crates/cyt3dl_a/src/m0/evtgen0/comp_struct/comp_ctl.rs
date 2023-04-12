#[doc = "Register `COMP_CTL` reader"]
pub struct R(crate::R<COMP_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_CTL` writer"]
pub struct W(crate::W<COMP_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_CTL_SPEC>;
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
impl From<crate::W<COMP_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP0_EN` reader - Active comparator (COMP0) enable: '0': Disabled. The comparator output 'comp0_out' is '0'. '1': Enabled."]
pub type COMP0_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP0_EN` writer - Active comparator (COMP0) enable: '0': Disabled. The comparator output 'comp0_out' is '0'. '1': Enabled."]
pub type COMP0_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CTL_SPEC, bool, O>;
#[doc = "Field `COMP1_EN` reader - DeepSleep comparator (COMP1) enable: '0': Disabled. The comparator output 'comp1_out_lf' is '0'. '1': Enabled."]
pub type COMP1_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMP1_EN` writer - DeepSleep comparator (COMP1) enable: '0': Disabled. The comparator output 'comp1_out_lf' is '0'. '1': Enabled."]
pub type COMP1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CTL_SPEC, bool, O>;
#[doc = "Field `TR_OUT_EDGE` reader - Specifies the 'tr_out' output trigger: '0': The trigger is a level sensitive trigger. The Active comparator output ('comp0_out') is reflected on 'tr_out'. '1': The trigger is an edge sensitive trigger. Activation of the Active comparator output (rising edge on 'comp0_out') results in a two cycle '1'/high pulse on 'tr_out'."]
pub type TR_OUT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `TR_OUT_EDGE` writer - Specifies the 'tr_out' output trigger: '0': The trigger is a level sensitive trigger. The Active comparator output ('comp0_out') is reflected on 'tr_out'. '1': The trigger is an edge sensitive trigger. Activation of the Active comparator output (rising edge on 'comp0_out') results in a two cycle '1'/high pulse on 'tr_out'."]
pub type TR_OUT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Comparator structure enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Comparator structure enable: '0': Disabled. '1': Enabled."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Active comparator (COMP0) enable: '0': Disabled. The comparator output 'comp0_out' is '0'. '1': Enabled."]
    #[inline(always)]
    pub fn comp0_en(&self) -> COMP0_EN_R {
        COMP0_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DeepSleep comparator (COMP1) enable: '0': Disabled. The comparator output 'comp1_out_lf' is '0'. '1': Enabled."]
    #[inline(always)]
    pub fn comp1_en(&self) -> COMP1_EN_R {
        COMP1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Specifies the 'tr_out' output trigger: '0': The trigger is a level sensitive trigger. The Active comparator output ('comp0_out') is reflected on 'tr_out'. '1': The trigger is an edge sensitive trigger. Activation of the Active comparator output (rising edge on 'comp0_out') results in a two cycle '1'/high pulse on 'tr_out'."]
    #[inline(always)]
    pub fn tr_out_edge(&self) -> TR_OUT_EDGE_R {
        TR_OUT_EDGE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator structure enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active comparator (COMP0) enable: '0': Disabled. The comparator output 'comp0_out' is '0'. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp0_en(&mut self) -> COMP0_EN_W<0> {
        COMP0_EN_W::new(self)
    }
    #[doc = "Bit 1 - DeepSleep comparator (COMP1) enable: '0': Disabled. The comparator output 'comp1_out_lf' is '0'. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn comp1_en(&mut self) -> COMP1_EN_W<1> {
        COMP1_EN_W::new(self)
    }
    #[doc = "Bit 16 - Specifies the 'tr_out' output trigger: '0': The trigger is a level sensitive trigger. The Active comparator output ('comp0_out') is reflected on 'tr_out'. '1': The trigger is an edge sensitive trigger. Activation of the Active comparator output (rising edge on 'comp0_out') results in a two cycle '1'/high pulse on 'tr_out'."]
    #[inline(always)]
    #[must_use]
    pub fn tr_out_edge(&mut self) -> TR_OUT_EDGE_W<16> {
        TR_OUT_EDGE_W::new(self)
    }
    #[doc = "Bit 31 - Comparator structure enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_ctl](index.html) module"]
pub struct COMP_CTL_SPEC;
impl crate::RegisterSpec for COMP_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_ctl::R](R) reader structure"]
impl crate::Readable for COMP_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_ctl::W](W) writer structure"]
impl crate::Writable for COMP_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_CTL to value 0"]
impl crate::Resettable for COMP_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
