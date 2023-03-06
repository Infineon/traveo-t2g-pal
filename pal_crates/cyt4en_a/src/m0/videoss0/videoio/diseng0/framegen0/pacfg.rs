#[doc = "Register `PACFG` reader"]
pub struct R(crate::R<PACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACFG` writer"]
pub struct W(crate::W<PACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACFG_SPEC>;
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
impl From<crate::W<PACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSTARTX` reader - Primary screen upper left corner, x component. Counts from 1. Pstartx = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
pub type PSTARTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSTARTX` writer - Primary screen upper left corner, x component. Counts from 1. Pstartx = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
pub type PSTARTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PACFG_SPEC, u16, u16, 14, O>;
#[doc = "Field `PSTARTY` reader - Primary screen upper left corner, y component. Counts from 1. Pstarty = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
pub type PSTARTY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSTARTY` writer - Primary screen upper left corner, y component. Counts from 1. Pstarty = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
pub type PSTARTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PACFG_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Primary screen upper left corner, x component. Counts from 1. Pstartx = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
    #[inline(always)]
    pub fn pstartx(&self) -> PSTARTX_R {
        PSTARTX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Primary screen upper left corner, y component. Counts from 1. Pstarty = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
    #[inline(always)]
    pub fn pstarty(&self) -> PSTARTY_R {
        PSTARTY_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Primary screen upper left corner, x component. Counts from 1. Pstartx = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
    #[inline(always)]
    #[must_use]
    pub fn pstartx(&mut self) -> PSTARTX_W<0> {
        PSTARTX_W::new(self)
    }
    #[doc = "Bits 16:29 - Primary screen upper left corner, y component. Counts from 1. Pstarty = 0 is not allowed. Primary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal."]
    #[inline(always)]
    #[must_use]
    pub fn pstarty(&mut self) -> PSTARTY_W<16> {
        PSTARTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Primary Area Config Register 1 (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacfg](index.html) module"]
pub struct PACFG_SPEC;
impl crate::RegisterSpec for PACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pacfg::R](R) reader structure"]
impl crate::Readable for PACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pacfg::W](W) writer structure"]
impl crate::Writable for PACFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PACFG to value 0x0001_0001"]
impl crate::Resettable for PACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
