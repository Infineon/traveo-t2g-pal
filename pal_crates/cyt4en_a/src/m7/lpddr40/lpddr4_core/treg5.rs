#[doc = "Register `TREG5` reader"]
pub struct R(crate::R<TREG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG5` writer"]
pub struct W(crate::W<TREG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG5_SPEC>;
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
impl From<crate::W<TREG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_ZQRS` reader - ZQ Calibration Reset time = tZQRESET"]
pub type T_ZQRS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_ZQRS` writer - ZQ Calibration Reset time = tZQRESET"]
pub type T_ZQRS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG5_SPEC, u8, u8, 8, O>;
#[doc = "Field `T_CKCKEH` reader - CK to CKE high = tCKCKEH"]
pub type T_CKCKEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_CKCKEH` writer - CK to CKE high = tCKCKEH"]
pub type T_CKCKEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG5_SPEC, u8, u8, 2, O>;
#[doc = "Field `T_REFI` reader - Average periodic refresh interval = tREFI"]
pub type T_REFI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_REFI` writer - Average periodic refresh interval = tREFI"]
pub type T_REFI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG5_SPEC, u16, u16, 14, O>;
#[doc = "Field `T_OSCO` reader - Delay time from OSC stop to Mode Register Readout = tOSCO"]
pub type T_OSCO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_OSCO` writer - Delay time from OSC stop to Mode Register Readout = tOSCO"]
pub type T_OSCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ZQ Calibration Reset time = tZQRESET"]
    #[inline(always)]
    pub fn t_zqrs(&self) -> T_ZQRS_R {
        T_ZQRS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - CK to CKE high = tCKCKEH"]
    #[inline(always)]
    pub fn t_ckckeh(&self) -> T_CKCKEH_R {
        T_CKCKEH_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:23 - Average periodic refresh interval = tREFI"]
    #[inline(always)]
    pub fn t_refi(&self) -> T_REFI_R {
        T_REFI_R::new(((self.bits >> 10) & 0x3fff) as u16)
    }
    #[doc = "Bits 24:31 - Delay time from OSC stop to Mode Register Readout = tOSCO"]
    #[inline(always)]
    pub fn t_osco(&self) -> T_OSCO_R {
        T_OSCO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ZQ Calibration Reset time = tZQRESET"]
    #[inline(always)]
    #[must_use]
    pub fn t_zqrs(&mut self) -> T_ZQRS_W<0> {
        T_ZQRS_W::new(self)
    }
    #[doc = "Bits 8:9 - CK to CKE high = tCKCKEH"]
    #[inline(always)]
    #[must_use]
    pub fn t_ckckeh(&mut self) -> T_CKCKEH_W<8> {
        T_CKCKEH_W::new(self)
    }
    #[doc = "Bits 10:23 - Average periodic refresh interval = tREFI"]
    #[inline(always)]
    #[must_use]
    pub fn t_refi(&mut self) -> T_REFI_W<10> {
        T_REFI_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay time from OSC stop to Mode Register Readout = tOSCO"]
    #[inline(always)]
    #[must_use]
    pub fn t_osco(&mut self) -> T_OSCO_W<24> {
        T_OSCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg5](index.html) module"]
pub struct TREG5_SPEC;
impl crate::RegisterSpec for TREG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg5::R](R) reader structure"]
impl crate::Readable for TREG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg5::W](W) writer structure"]
impl crate::Writable for TREG5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG5 to value 0x0300"]
impl crate::Resettable for TREG5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300;
}
