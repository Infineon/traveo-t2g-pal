#[doc = "Register `TREG9` reader"]
pub struct R(crate::R<TREG9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG9` writer"]
pub struct W(crate::W<TREG9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG9_SPEC>;
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
impl From<crate::W<TREG9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ZQ_ITV` reader - Auto ZQ Calibration interval in case DMCFG.ZQ_AUTO_EN = 1."]
pub type T_ZQ_ITV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `T_ZQ_ITV` writer - Auto ZQ Calibration interval in case DMCFG.ZQ_AUTO_EN = 1."]
pub type T_ZQ_ITV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG9_SPEC, u32, u32, 28, O>;
#[doc = "Field `T_CMDCKE` reader - Delay from valid command to CKE input LOW = tCMDCKE"]
pub type T_CMDCKE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CMDCKE` writer - Delay from valid command to CKE input LOW = tCMDCKE"]
pub type T_CMDCKE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG9_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:27 - Auto ZQ Calibration interval in case DMCFG.ZQ_AUTO_EN = 1."]
    #[inline(always)]
    pub fn t_zq_itv(&self) -> T_ZQ_ITV_R {
        T_ZQ_ITV_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Delay from valid command to CKE input LOW = tCMDCKE"]
    #[inline(always)]
    pub fn t_cmdcke(&self) -> T_CMDCKE_R {
        T_CMDCKE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Auto ZQ Calibration interval in case DMCFG.ZQ_AUTO_EN = 1."]
    #[inline(always)]
    #[must_use]
    pub fn t_zq_itv(&mut self) -> T_ZQ_ITV_W<0> {
        T_ZQ_ITV_W::new(self)
    }
    #[doc = "Bits 28:31 - Delay from valid command to CKE input LOW = tCMDCKE"]
    #[inline(always)]
    #[must_use]
    pub fn t_cmdcke(&mut self) -> T_CMDCKE_W<28> {
        T_CMDCKE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg9](index.html) module"]
pub struct TREG9_SPEC;
impl crate::RegisterSpec for TREG9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg9::R](R) reader structure"]
impl crate::Readable for TREG9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg9::W](W) writer structure"]
impl crate::Writable for TREG9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG9 to value 0"]
impl crate::Resettable for TREG9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
