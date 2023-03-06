#[doc = "Register `TREG11` reader"]
pub struct R(crate::R<TREG11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG11` writer"]
pub struct W(crate::W<TREG11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG11_SPEC>;
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
impl From<crate::W<TREG11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_DLLLOCK` reader - DLL lock timeout. The number of DTI_CLOCKs from when the DLL reset is de-asserted to when the DLL is locked and ready for use = 'h7d00"]
pub type T_DLLLOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_DLLLOCK` writer - DLL lock timeout. The number of DTI_CLOCKs from when the DLL reset is de-asserted to when the DLL is locked and ready for use = 'h7d00"]
pub type T_DLLLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG11_SPEC, u16, u16, 16, O>;
#[doc = "Field `T_INIT5` reader - Minimum idle time before first MRW/MRR command = tINIT5"]
pub type T_INIT5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_INIT5` writer - Minimum idle time before first MRW/MRR command = tINIT5"]
pub type T_INIT5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG11_SPEC, u8, u8, 7, O>;
#[doc = "Field `T_CAENT` reader - First CA Bus Training Command Following CKE Low = tCAENT"]
pub type T_CAENT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_CAENT` writer - First CA Bus Training Command Following CKE Low = tCAENT"]
pub type T_CAENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG11_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:15 - DLL lock timeout. The number of DTI_CLOCKs from when the DLL reset is de-asserted to when the DLL is locked and ready for use = 'h7d00"]
    #[inline(always)]
    pub fn t_dlllock(&self) -> T_DLLLOCK_R {
        T_DLLLOCK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:22 - Minimum idle time before first MRW/MRR command = tINIT5"]
    #[inline(always)]
    pub fn t_init5(&self) -> T_INIT5_R {
        T_INIT5_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31 - First CA Bus Training Command Following CKE Low = tCAENT"]
    #[inline(always)]
    pub fn t_caent(&self) -> T_CAENT_R {
        T_CAENT_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DLL lock timeout. The number of DTI_CLOCKs from when the DLL reset is de-asserted to when the DLL is locked and ready for use = 'h7d00"]
    #[inline(always)]
    #[must_use]
    pub fn t_dlllock(&mut self) -> T_DLLLOCK_W<0> {
        T_DLLLOCK_W::new(self)
    }
    #[doc = "Bits 16:22 - Minimum idle time before first MRW/MRR command = tINIT5"]
    #[inline(always)]
    #[must_use]
    pub fn t_init5(&mut self) -> T_INIT5_W<16> {
        T_INIT5_W::new(self)
    }
    #[doc = "Bits 23:31 - First CA Bus Training Command Following CKE Low = tCAENT"]
    #[inline(always)]
    #[must_use]
    pub fn t_caent(&mut self) -> T_CAENT_W<23> {
        T_CAENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg11](index.html) module"]
pub struct TREG11_SPEC;
impl crate::RegisterSpec for TREG11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg11::R](R) reader structure"]
impl crate::Readable for TREG11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg11::W](W) writer structure"]
impl crate::Writable for TREG11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG11 to value 0x6470_7d00"]
impl crate::Resettable for TREG11_SPEC {
    const RESET_VALUE: Self::Ux = 0x6470_7d00;
}
