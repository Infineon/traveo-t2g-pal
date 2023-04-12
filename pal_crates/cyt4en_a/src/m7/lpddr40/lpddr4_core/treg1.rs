#[doc = "Register `TREG1` reader"]
pub struct R(crate::R<TREG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG1` writer"]
pub struct W(crate::W<TREG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG1_SPEC>;
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
impl From<crate::W<TREG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RCD` reader - ACT to internal read or write delay time = tRCD - 4"]
pub type T_RCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RCD` writer - ACT to internal read or write delay time = tRCD - 4"]
pub type T_RCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_FAW` reader - Four activate window = tFAW - 4"]
pub type T_FAW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_FAW` writer - Four activate window = tFAW - 4"]
pub type T_FAW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG1_SPEC, u8, u8, 7, O>;
#[doc = "Field `T_RTW` reader - Read to Write Command Latency (Bus Turnaround) = RL + BL/2 - WL +3"]
pub type T_RTW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RTW` writer - Read to Write Command Latency (Bus Turnaround) = RL + BL/2 - WL +3"]
pub type T_RTW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG1_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_CCDWM` reader - CAS_n to CAS_n command delay for Masked Write = tCCDMW = 32"]
pub type T_CCDWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CCDWM` writer - CAS_n to CAS_n command delay for Masked Write = tCCDMW = 32"]
pub type T_CCDWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG1_SPEC, u8, u8, 6, O>;
#[doc = "Field `T_MRS2LVLEN` reader - MRS leveling enable to leveling enable = tWLDQSEN + 5"]
pub type T_MRS2LVLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_MRS2LVLEN` writer - MRS leveling enable to leveling enable = tWLDQSEN + 5"]
pub type T_MRS2LVLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:5 - ACT to internal read or write delay time = tRCD - 4"]
    #[inline(always)]
    pub fn t_rcd(&self) -> T_RCD_R {
        T_RCD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:12 - Four activate window = tFAW - 4"]
    #[inline(always)]
    pub fn t_faw(&self) -> T_FAW_R {
        T_FAW_R::new(((self.bits >> 6) & 0x7f) as u8)
    }
    #[doc = "Bits 13:20 - Read to Write Command Latency (Bus Turnaround) = RL + BL/2 - WL +3"]
    #[inline(always)]
    pub fn t_rtw(&self) -> T_RTW_R {
        T_RTW_R::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:26 - CAS_n to CAS_n command delay for Masked Write = tCCDMW = 32"]
    #[inline(always)]
    pub fn t_ccdwm(&self) -> T_CCDWM_R {
        T_CCDWM_R::new(((self.bits >> 21) & 0x3f) as u8)
    }
    #[doc = "Bits 27:31 - MRS leveling enable to leveling enable = tWLDQSEN + 5"]
    #[inline(always)]
    pub fn t_mrs2lvlen(&self) -> T_MRS2LVLEN_R {
        T_MRS2LVLEN_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ACT to internal read or write delay time = tRCD - 4"]
    #[inline(always)]
    #[must_use]
    pub fn t_rcd(&mut self) -> T_RCD_W<0> {
        T_RCD_W::new(self)
    }
    #[doc = "Bits 6:12 - Four activate window = tFAW - 4"]
    #[inline(always)]
    #[must_use]
    pub fn t_faw(&mut self) -> T_FAW_W<6> {
        T_FAW_W::new(self)
    }
    #[doc = "Bits 13:20 - Read to Write Command Latency (Bus Turnaround) = RL + BL/2 - WL +3"]
    #[inline(always)]
    #[must_use]
    pub fn t_rtw(&mut self) -> T_RTW_W<13> {
        T_RTW_W::new(self)
    }
    #[doc = "Bits 21:26 - CAS_n to CAS_n command delay for Masked Write = tCCDMW = 32"]
    #[inline(always)]
    #[must_use]
    pub fn t_ccdwm(&mut self) -> T_CCDWM_W<21> {
        T_CCDWM_W::new(self)
    }
    #[doc = "Bits 27:31 - MRS leveling enable to leveling enable = tWLDQSEN + 5"]
    #[inline(always)]
    #[must_use]
    pub fn t_mrs2lvlen(&mut self) -> T_MRS2LVLEN_W<27> {
        T_MRS2LVLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg1](index.html) module"]
pub struct TREG1_SPEC;
impl crate::RegisterSpec for TREG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg1::R](R) reader structure"]
impl crate::Readable for TREG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg1::W](W) writer structure"]
impl crate::Writable for TREG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG1 to value 0x0b"]
impl crate::Resettable for TREG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
