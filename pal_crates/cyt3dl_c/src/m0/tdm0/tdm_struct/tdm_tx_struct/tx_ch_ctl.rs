#[doc = "Register `TX_CH_CTL` reader"]
pub struct R(crate::R<TX_CH_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_CH_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_CH_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_CH_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_CH_CTL` writer"]
pub struct W(crate::W<TX_CH_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_CH_CTL_SPEC>;
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
impl From<crate::W<TX_CH_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_CH_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_EN` reader - Channel enables: channel i is controlled by CH_EN\\[i\\]. '0': Disabled. The TX FIFO does not produce channel i words and the transmitted channel i words on the interface are not driven (the output enable of the 'tdm_tx_sd_out' output signal is not driven). '1': Enabled. Note: Only bit 0 through TX_IF_CTL.CH_NR may be set to '1'; i.e. only channels that are present in the frame can be enabled."]
pub type CH_EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CH_EN` writer - Channel enables: channel i is controlled by CH_EN\\[i\\]. '0': Disabled. The TX FIFO does not produce channel i words and the transmitted channel i words on the interface are not driven (the output enable of the 'tdm_tx_sd_out' output signal is not driven). '1': Enabled. Note: Only bit 0 through TX_IF_CTL.CH_NR may be set to '1'; i.e. only channels that are present in the frame can be enabled."]
pub type CH_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_CH_CTL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel enables: channel i is controlled by CH_EN\\[i\\]. '0': Disabled. The TX FIFO does not produce channel i words and the transmitted channel i words on the interface are not driven (the output enable of the 'tdm_tx_sd_out' output signal is not driven). '1': Enabled. Note: Only bit 0 through TX_IF_CTL.CH_NR may be set to '1'; i.e. only channels that are present in the frame can be enabled."]
    #[inline(always)]
    pub fn ch_en(&self) -> CH_EN_R {
        CH_EN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel enables: channel i is controlled by CH_EN\\[i\\]. '0': Disabled. The TX FIFO does not produce channel i words and the transmitted channel i words on the interface are not driven (the output enable of the 'tdm_tx_sd_out' output signal is not driven). '1': Enabled. Note: Only bit 0 through TX_IF_CTL.CH_NR may be set to '1'; i.e. only channels that are present in the frame can be enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch_en(&mut self) -> CH_EN_W<0> {
        CH_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX channel control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_ch_ctl](index.html) module"]
pub struct TX_CH_CTL_SPEC;
impl crate::RegisterSpec for TX_CH_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_ch_ctl::R](R) reader structure"]
impl crate::Readable for TX_CH_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_ch_ctl::W](W) writer structure"]
impl crate::Writable for TX_CH_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_CH_CTL to value 0"]
impl crate::Resettable for TX_CH_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
