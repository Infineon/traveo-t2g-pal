#[doc = "Register `SACFG` reader"]
pub struct R(crate::R<SACFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SACFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SACFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SACFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SACFG` writer"]
pub struct W(crate::W<SACFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SACFG_SPEC>;
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
impl From<crate::W<SACFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SACFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTARTX` reader - Secondary screen upper left corner, x component. Counts from 1 . Sstartx = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
pub type SSTARTX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSTARTX` writer - Secondary screen upper left corner, x component. Counts from 1 . Sstartx = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
pub type SSTARTX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SACFG_SPEC, u16, u16, 14, O>;
#[doc = "Field `SSTARTY` reader - Secondary screen upper left corner, y component. Counts from 1 . Sstarty = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
pub type SSTARTY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SSTARTY` writer - Secondary screen upper left corner, y component. Counts from 1 . Sstarty = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
pub type SSTARTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SACFG_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Secondary screen upper left corner, x component. Counts from 1 . Sstartx = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
    #[inline(always)]
    pub fn sstartx(&self) -> SSTARTX_R {
        SSTARTX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Secondary screen upper left corner, y component. Counts from 1 . Sstarty = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
    #[inline(always)]
    pub fn sstarty(&self) -> SSTARTY_R {
        SSTARTY_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Secondary screen upper left corner, x component. Counts from 1 . Sstartx = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
    #[inline(always)]
    #[must_use]
    pub fn sstartx(&mut self) -> SSTARTX_W<0> {
        SSTARTX_W::new(self)
    }
    #[doc = "Bits 16:29 - Secondary screen upper left corner, y component. Counts from 1 . Sstarty = 0 is not allowed. Secondary screen shall not overlap the last column/ last row of the total frame defined by Htotal/ Vtotal. When skew regulation feature is enabled, the secondary screen shall not overlap the active display area defined by Hact and Vact."]
    #[inline(always)]
    #[must_use]
    pub fn sstarty(&mut self) -> SSTARTY_W<16> {
        SSTARTY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FrameGen Secondary Area Config Register 1 (shadowed)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sacfg](index.html) module"]
pub struct SACFG_SPEC;
impl crate::RegisterSpec for SACFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sacfg::R](R) reader structure"]
impl crate::Readable for SACFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sacfg::W](W) writer structure"]
impl crate::Writable for SACFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SACFG to value 0x0001_0001"]
impl crate::Resettable for SACFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0001;
}
