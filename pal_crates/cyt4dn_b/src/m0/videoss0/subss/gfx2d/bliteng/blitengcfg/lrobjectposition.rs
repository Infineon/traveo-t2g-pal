#[doc = "Register `LROBJECTPOSITION` reader"]
pub struct R(crate::R<LROBJECTPOSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LROBJECTPOSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LROBJECTPOSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LROBJECTPOSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LROBJECTPOSITION` writer"]
pub struct W(crate::W<LROBJECTPOSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LROBJECTPOSITION_SPEC>;
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
impl From<crate::W<LROBJECTPOSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LROBJECTPOSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OBJECTOFFSETX` reader - Object position in x direction (start at 0)."]
pub type OBJECTOFFSETX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OBJECTOFFSETX` writer - Object position in x direction (start at 0)."]
pub type OBJECTOFFSETX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LROBJECTPOSITION_SPEC, u16, u16, 14, O>;
#[doc = "Field `OBJECTOFFSETY` reader - Object position in y direction (start at 0)."]
pub type OBJECTOFFSETY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OBJECTOFFSETY` writer - Object position in y direction (start at 0)."]
pub type OBJECTOFFSETY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LROBJECTPOSITION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Object position in x direction (start at 0)."]
    #[inline(always)]
    pub fn objectoffsetx(&self) -> OBJECTOFFSETX_R {
        OBJECTOFFSETX_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Object position in y direction (start at 0)."]
    #[inline(always)]
    pub fn objectoffsety(&self) -> OBJECTOFFSETY_R {
        OBJECTOFFSETY_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Object position in x direction (start at 0)."]
    #[inline(always)]
    #[must_use]
    pub fn objectoffsetx(&mut self) -> OBJECTOFFSETX_W<0> {
        OBJECTOFFSETX_W::new(self)
    }
    #[doc = "Bits 16:29 - Object position in y direction (start at 0)."]
    #[inline(always)]
    #[must_use]
    pub fn objectoffsety(&mut self) -> OBJECTOFFSETY_W<16> {
        OBJECTOFFSETY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of current object in frame coordinate system. Only possible in LBO mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lrobjectposition](index.html) module"]
pub struct LROBJECTPOSITION_SPEC;
impl crate::RegisterSpec for LROBJECTPOSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lrobjectposition::R](R) reader structure"]
impl crate::Readable for LROBJECTPOSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lrobjectposition::W](W) writer structure"]
impl crate::Writable for LROBJECTPOSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LROBJECTPOSITION to value 0"]
impl crate::Resettable for LROBJECTPOSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
