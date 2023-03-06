#[doc = "Register `ADFT` reader"]
pub struct R(crate::R<ADFT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADFT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADFT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADFT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADFT` writer"]
pub struct W(crate::W<ADFT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADFT_SPEC>;
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
impl From<crate::W<ADFT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADFT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TST_EN_CA` reader - Test enable for ADFT bus - Control Block adft_tst_en_ca\\[0\\]: if set to 1: enable for VREF of channel A adft_tst_en_ca\\[1\\]: if set to 1: enable for VREF of channel B"]
pub type TST_EN_CA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TST_EN_CA` writer - Test enable for ADFT bus - Control Block adft_tst_en_ca\\[0\\]: if set to 1: enable for VREF of channel A adft_tst_en_ca\\[1\\]: if set to 1: enable for VREF of channel B"]
pub type TST_EN_CA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFT_SPEC, u8, u8, 2, O>;
#[doc = "Field `TST_EN_DQ` reader - Test enable for ADFT bus - DQ Blocks adft_tst_en_dq\\[0\\]: if set to 1: enable for VREF of Data Slice 0 adft_tst_en_dq\\[1\\]: if set to 1: enable for VREF of Data Slice 1 adft_tst_en_dq\\[2\\]: if set to 1: enable for VREF of Data Slice 2 adft_tst_en_dq\\[3\\]: if set to 1: enable for VREF of Data Slice 3"]
pub type TST_EN_DQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TST_EN_DQ` writer - Test enable for ADFT bus - DQ Blocks adft_tst_en_dq\\[0\\]: if set to 1: enable for VREF of Data Slice 0 adft_tst_en_dq\\[1\\]: if set to 1: enable for VREF of Data Slice 1 adft_tst_en_dq\\[2\\]: if set to 1: enable for VREF of Data Slice 2 adft_tst_en_dq\\[3\\]: if set to 1: enable for VREF of Data Slice 3"]
pub type TST_EN_DQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADFT_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Test enable for ADFT bus - Control Block adft_tst_en_ca\\[0\\]: if set to 1: enable for VREF of channel A adft_tst_en_ca\\[1\\]: if set to 1: enable for VREF of channel B"]
    #[inline(always)]
    pub fn tst_en_ca(&self) -> TST_EN_CA_R {
        TST_EN_CA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Test enable for ADFT bus - DQ Blocks adft_tst_en_dq\\[0\\]: if set to 1: enable for VREF of Data Slice 0 adft_tst_en_dq\\[1\\]: if set to 1: enable for VREF of Data Slice 1 adft_tst_en_dq\\[2\\]: if set to 1: enable for VREF of Data Slice 2 adft_tst_en_dq\\[3\\]: if set to 1: enable for VREF of Data Slice 3"]
    #[inline(always)]
    pub fn tst_en_dq(&self) -> TST_EN_DQ_R {
        TST_EN_DQ_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Test enable for ADFT bus - Control Block adft_tst_en_ca\\[0\\]: if set to 1: enable for VREF of channel A adft_tst_en_ca\\[1\\]: if set to 1: enable for VREF of channel B"]
    #[inline(always)]
    #[must_use]
    pub fn tst_en_ca(&mut self) -> TST_EN_CA_W<0> {
        TST_EN_CA_W::new(self)
    }
    #[doc = "Bits 2:5 - Test enable for ADFT bus - DQ Blocks adft_tst_en_dq\\[0\\]: if set to 1: enable for VREF of Data Slice 0 adft_tst_en_dq\\[1\\]: if set to 1: enable for VREF of Data Slice 1 adft_tst_en_dq\\[2\\]: if set to 1: enable for VREF of Data Slice 2 adft_tst_en_dq\\[3\\]: if set to 1: enable for VREF of Data Slice 3"]
    #[inline(always)]
    #[must_use]
    pub fn tst_en_dq(&mut self) -> TST_EN_DQ_W<2> {
        TST_EN_DQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADFT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adft](index.html) module"]
pub struct ADFT_SPEC;
impl crate::RegisterSpec for ADFT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adft::R](R) reader structure"]
impl crate::Readable for ADFT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adft::W](W) writer structure"]
impl crate::Writable for ADFT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADFT to value 0"]
impl crate::Resettable for ADFT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
