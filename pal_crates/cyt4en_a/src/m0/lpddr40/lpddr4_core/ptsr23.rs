#[doc = "Register `PTSR23` reader"]
pub struct R(crate::R<PTSR23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTSR23` writer"]
pub struct W(crate::W<PTSR23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTSR23_SPEC>;
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
impl From<crate::W<PTSR23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTSR23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFDQRDS0` reader - VREF-DQ setting for slice 0 READ path"]
pub type VREFDQRDS0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQRDS0` writer - VREF-DQ setting for slice 0 READ path"]
pub type VREFDQRDS0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR23_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFDQRDS1` reader - VREF-DQ setting for slice 1 READ path"]
pub type VREFDQRDS1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQRDS1` writer - VREF-DQ setting for slice 1 READ path"]
pub type VREFDQRDS1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR23_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFDQRDS2` reader - VREF-DQ setting for slice 2 READ path"]
pub type VREFDQRDS2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQRDS2` writer - VREF-DQ setting for slice 2 READ path"]
pub type VREFDQRDS2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR23_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFDQRDS3` reader - VREF-DQ setting for slice 3 READ path"]
pub type VREFDQRDS3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VREFDQRDS3` writer - VREF-DQ setting for slice 3 READ path"]
pub type VREFDQRDS3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTSR23_SPEC, u8, u8, 6, O>;
#[doc = "Field `VREFDQRDR` reader - VREF-DQ range for READ path"]
pub type VREFDQRDR_R = crate::BitReader<bool>;
#[doc = "Field `VREFDQRDR` writer - VREF-DQ range for READ path"]
pub type VREFDQRDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTSR23_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - VREF-DQ setting for slice 0 READ path"]
    #[inline(always)]
    pub fn vrefdqrds0(&self) -> VREFDQRDS0_R {
        VREFDQRDS0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - VREF-DQ setting for slice 1 READ path"]
    #[inline(always)]
    pub fn vrefdqrds1(&self) -> VREFDQRDS1_R {
        VREFDQRDS1_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - VREF-DQ setting for slice 2 READ path"]
    #[inline(always)]
    pub fn vrefdqrds2(&self) -> VREFDQRDS2_R {
        VREFDQRDS2_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - VREF-DQ setting for slice 3 READ path"]
    #[inline(always)]
    pub fn vrefdqrds3(&self) -> VREFDQRDS3_R {
        VREFDQRDS3_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - VREF-DQ range for READ path"]
    #[inline(always)]
    pub fn vrefdqrdr(&self) -> VREFDQRDR_R {
        VREFDQRDR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - VREF-DQ setting for slice 0 READ path"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqrds0(&mut self) -> VREFDQRDS0_W<0> {
        VREFDQRDS0_W::new(self)
    }
    #[doc = "Bits 6:11 - VREF-DQ setting for slice 1 READ path"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqrds1(&mut self) -> VREFDQRDS1_W<6> {
        VREFDQRDS1_W::new(self)
    }
    #[doc = "Bits 12:17 - VREF-DQ setting for slice 2 READ path"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqrds2(&mut self) -> VREFDQRDS2_W<12> {
        VREFDQRDS2_W::new(self)
    }
    #[doc = "Bits 18:23 - VREF-DQ setting for slice 3 READ path"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqrds3(&mut self) -> VREFDQRDS3_W<18> {
        VREFDQRDS3_W::new(self)
    }
    #[doc = "Bit 24 - VREF-DQ range for READ path"]
    #[inline(always)]
    #[must_use]
    pub fn vrefdqrdr(&mut self) -> VREFDQRDR_W<24> {
        VREFDQRDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Training Setting Register 23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr23](index.html) module"]
pub struct PTSR23_SPEC;
impl crate::RegisterSpec for PTSR23_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr23::R](R) reader structure"]
impl crate::Readable for PTSR23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptsr23::W](W) writer structure"]
impl crate::Writable for PTSR23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTSR23 to value 0"]
impl crate::Resettable for PTSR23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
