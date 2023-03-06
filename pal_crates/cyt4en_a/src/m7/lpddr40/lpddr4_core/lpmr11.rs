#[doc = "Register `LPMR11` reader"]
pub struct R(crate::R<LPMR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPMR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPMR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPMR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPMR11` writer"]
pub struct W(crate::W<LPMR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPMR11_SPEC>;
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
impl From<crate::W<LPMR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPMR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FS0_DQODT` reader - N/A"]
pub type FS0_DQODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_DQODT` writer - N/A"]
pub type FS0_DQODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR11_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS0_CAODT` reader - N/A"]
pub type FS0_CAODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS0_CAODT` writer - N/A"]
pub type FS0_CAODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR11_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_DQODT` reader - N/A"]
pub type FS1_DQODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_DQODT` writer - N/A"]
pub type FS1_DQODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR11_SPEC, u8, u8, 3, O>;
#[doc = "Field `FS1_CAODT` reader - N/A"]
pub type FS1_CAODT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FS1_CAODT` writer - N/A"]
pub type FS1_CAODT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPMR11_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn fs0_dqodt(&self) -> FS0_DQODT_R {
        FS0_DQODT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    pub fn fs0_caodt(&self) -> FS0_CAODT_R {
        FS0_CAODT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - N/A"]
    #[inline(always)]
    pub fn fs1_dqodt(&self) -> FS1_DQODT_R {
        FS1_DQODT_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - N/A"]
    #[inline(always)]
    pub fn fs1_caodt(&self) -> FS1_CAODT_R {
        FS1_CAODT_R::new(((self.bits >> 9) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_dqodt(&mut self) -> FS0_DQODT_W<0> {
        FS0_DQODT_W::new(self)
    }
    #[doc = "Bits 3:5 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs0_caodt(&mut self) -> FS0_CAODT_W<3> {
        FS0_CAODT_W::new(self)
    }
    #[doc = "Bits 6:8 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_dqodt(&mut self) -> FS1_DQODT_W<6> {
        FS1_DQODT_W::new(self)
    }
    #[doc = "Bits 9:11 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn fs1_caodt(&mut self) -> FS1_CAODT_W<9> {
        FS1_CAODT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPDDR Mode Register 11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpmr11](index.html) module"]
pub struct LPMR11_SPEC;
impl crate::RegisterSpec for LPMR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpmr11::R](R) reader structure"]
impl crate::Readable for LPMR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpmr11::W](W) writer structure"]
impl crate::Writable for LPMR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPMR11 to value 0"]
impl crate::Resettable for LPMR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
