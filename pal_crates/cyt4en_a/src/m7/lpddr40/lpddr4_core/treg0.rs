#[doc = "Register `TREG0` reader"]
pub struct R(crate::R<TREG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG0` writer"]
pub struct W(crate::W<TREG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG0_SPEC>;
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
impl From<crate::W<TREG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ALRTP` reader - Internal READ Command to PRECHARGE Command delay = tRTP + BL/2 - 12"]
pub type T_ALRTP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_ALRTP` writer - Internal READ Command to PRECHARGE Command delay = tRTP + BL/2 - 12"]
pub type T_ALRTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_CKESR` reader - Minimum Self Refresh Time (Entry to Exit) = tSR"]
pub type T_CKESR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKESR` writer - Minimum Self Refresh Time (Entry to Exit) = tSR"]
pub type T_CKESR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_CKEHDQS` reader - CKE high to DQS = tCKEHDQS + 12"]
pub type T_CKEHDQS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKEHDQS` writer - CKE high to DQS = tCKEHDQS + 12"]
pub type T_CKEHDQS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_PD` reader - Power Down Entry to Exit Timing = tCKE"]
pub type T_PD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_PD` writer - Power Down Entry to Exit Timing = tCKE"]
pub type T_PD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_DTRAIN` reader - tDStrain and tDHtrain = tDtrain"]
pub type T_DTRAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DTRAIN` writer - tDStrain and tDHtrain = tDtrain"]
pub type T_DTRAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 3, O>;
#[doc = "Field `T_DLLEN` reader - DLL enable time. Number of DTI_CLOCK that the DLL_EN must be asserted before DLL_RESET can be asserted. Always write 10."]
pub type T_DLLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_DLLEN` writer - DLL enable time. Number of DTI_CLOCK that the DLL_EN must be asserted before DLL_RESET can be asserted. Always write 10."]
pub type T_DLLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5 - Internal READ Command to PRECHARGE Command delay = tRTP + BL/2 - 12"]
    #[inline(always)]
    pub fn t_alrtp(&self) -> T_ALRTP_R {
        T_ALRTP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:10 - Minimum Self Refresh Time (Entry to Exit) = tSR"]
    #[inline(always)]
    pub fn t_ckesr(&self) -> T_CKESR_R {
        T_CKESR_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - CKE high to DQS = tCKEHDQS + 12"]
    #[inline(always)]
    pub fn t_ckehdqs(&self) -> T_CKEHDQS_R {
        T_CKEHDQS_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Power Down Entry to Exit Timing = tCKE"]
    #[inline(always)]
    pub fn t_pd(&self) -> T_PD_R {
        T_PD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - tDStrain and tDHtrain = tDtrain"]
    #[inline(always)]
    pub fn t_dtrain(&self) -> T_DTRAIN_R {
        T_DTRAIN_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:31 - DLL enable time. Number of DTI_CLOCK that the DLL_EN must be asserted before DLL_RESET can be asserted. Always write 10."]
    #[inline(always)]
    pub fn t_dllen(&self) -> T_DLLEN_R {
        T_DLLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Internal READ Command to PRECHARGE Command delay = tRTP + BL/2 - 12"]
    #[inline(always)]
    #[must_use]
    pub fn t_alrtp(&mut self) -> T_ALRTP_W<0> {
        T_ALRTP_W::new(self)
    }
    #[doc = "Bits 6:10 - Minimum Self Refresh Time (Entry to Exit) = tSR"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckesr(&mut self) -> T_CKESR_W<6> {
        T_CKESR_W::new(self)
    }
    #[doc = "Bits 11:15 - CKE high to DQS = tCKEHDQS + 12"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckehdqs(&mut self) -> T_CKEHDQS_W<11> {
        T_CKEHDQS_W::new(self)
    }
    #[doc = "Bits 16:20 - Power Down Entry to Exit Timing = tCKE"]
    #[inline(always)]
    #[must_use]
    pub fn t_pd(&mut self) -> T_PD_W<16> {
        T_PD_W::new(self)
    }
    #[doc = "Bits 21:23 - tDStrain and tDHtrain = tDtrain"]
    #[inline(always)]
    #[must_use]
    pub fn t_dtrain(&mut self) -> T_DTRAIN_W<21> {
        T_DTRAIN_W::new(self)
    }
    #[doc = "Bits 24:31 - DLL enable time. Number of DTI_CLOCK that the DLL_EN must be asserted before DLL_RESET can be asserted. Always write 10."]
    #[inline(always)]
    #[must_use]
    pub fn t_dllen(&mut self) -> T_DLLEN_W<24> {
        T_DLLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg0](index.html) module"]
pub struct TREG0_SPEC;
impl crate::RegisterSpec for TREG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg0::R](R) reader structure"]
impl crate::Readable for TREG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg0::W](W) writer structure"]
impl crate::Writable for TREG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG0 to value 0x0a40_4000"]
impl crate::Resettable for TREG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a40_4000;
}
