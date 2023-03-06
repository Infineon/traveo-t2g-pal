#[doc = "Register `TREG12` reader"]
pub struct R(crate::R<TREG12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TREG12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TREG12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TREG12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TREG12` writer"]
pub struct W(crate::W<TREG12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TREG12_SPEC>;
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
impl From<crate::W<TREG12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TREG12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_RRD` reader - ACTIVATE to ACTIVATE Command delay = tRRD"]
pub type T_RRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `T_RRD` writer - ACTIVATE to ACTIVATE Command delay = tRRD"]
pub type T_RRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG12_SPEC, u8, u8, 5, O>;
#[doc = "Field `T_INIT3` reader - Minimum CKE LOW time after RESET_n goes HIGH = tINIT3"]
pub type T_INIT3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `T_INIT3` writer - Minimum CKE LOW time after RESET_n goes HIGH = tINIT3"]
pub type T_INIT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG12_SPEC, u32, u32, 17, O>;
#[doc = "Field `T_XSR` reader - Self refresh exit to next valid command delay. = tXSR"]
pub type T_XSR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `T_XSR` writer - Self refresh exit to next valid command delay. = tXSR"]
pub type T_XSR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TREG12_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:4 - ACTIVATE to ACTIVATE Command delay = tRRD"]
    #[inline(always)]
    pub fn t_rrd(&self) -> T_RRD_R {
        T_RRD_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:21 - Minimum CKE LOW time after RESET_n goes HIGH = tINIT3"]
    #[inline(always)]
    pub fn t_init3(&self) -> T_INIT3_R {
        T_INIT3_R::new((self.bits >> 5) & 0x0001_ffff)
    }
    #[doc = "Bits 22:31 - Self refresh exit to next valid command delay. = tXSR"]
    #[inline(always)]
    pub fn t_xsr(&self) -> T_XSR_R {
        T_XSR_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - ACTIVATE to ACTIVATE Command delay = tRRD"]
    #[inline(always)]
    #[must_use]
    pub fn t_rrd(&mut self) -> T_RRD_W<0> {
        T_RRD_W::new(self)
    }
    #[doc = "Bits 5:21 - Minimum CKE LOW time after RESET_n goes HIGH = tINIT3"]
    #[inline(always)]
    #[must_use]
    pub fn t_init3(&mut self) -> T_INIT3_W<5> {
        T_INIT3_W::new(self)
    }
    #[doc = "Bits 22:31 - Self refresh exit to next valid command delay. = tXSR"]
    #[inline(always)]
    #[must_use]
    pub fn t_xsr(&mut self) -> T_XSR_W<22> {
        T_XSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register 12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [treg12](index.html) module"]
pub struct TREG12_SPEC;
impl crate::RegisterSpec for TREG12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [treg12::R](R) reader structure"]
impl crate::Readable for TREG12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [treg12::W](W) writer structure"]
impl crate::Writable for TREG12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TREG12 to value 0x0036_4100"]
impl crate::Resettable for TREG12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0036_4100;
}
