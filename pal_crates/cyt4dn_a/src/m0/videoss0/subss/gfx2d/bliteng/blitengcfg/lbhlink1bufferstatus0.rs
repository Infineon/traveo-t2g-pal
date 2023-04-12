#[doc = "Register `LBHLINK1BUFFERSTATUS0` reader"]
pub struct R(crate::R<LBHLINK1BUFFERSTATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK1BUFFERSTATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK1BUFFERSTATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK1BUFFERSTATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHLINK1BUFFERSTATUS0` writer"]
pub struct W(crate::W<LBHLINK1BUFFERSTATUS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHLINK1BUFFERSTATUS0_SPEC>;
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
impl From<crate::W<LBHLINK1BUFFERSTATUS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHLINK1BUFFERSTATUS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHLINK1MAXFREELINES` reader - The maximal number of free lines in the line buffer that can be written by the store. Writting to this register has no effect, reading provides the value and simultaneously resets the register."]
pub type LBHLINK1MAXFREELINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK1MAXFREELINES` writer - The maximal number of free lines in the line buffer that can be written by the store. Writting to this register has no effect, reading provides the value and simultaneously resets the register."]
pub type LBHLINK1MAXFREELINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK1BUFFERSTATUS0_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - The maximal number of free lines in the line buffer that can be written by the store. Writting to this register has no effect, reading provides the value and simultaneously resets the register."]
    #[inline(always)]
    pub fn lbhlink1maxfreelines(&self) -> LBHLINK1MAXFREELINES_R {
        LBHLINK1MAXFREELINES_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - The maximal number of free lines in the line buffer that can be written by the store. Writting to this register has no effect, reading provides the value and simultaneously resets the register."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink1maxfreelines(&mut self) -> LBHLINK1MAXFREELINES_W<0> {
        LBHLINK1MAXFREELINES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer status register 0 for the line buffer handshake.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink1bufferstatus0](index.html) module"]
pub struct LBHLINK1BUFFERSTATUS0_SPEC;
impl crate::RegisterSpec for LBHLINK1BUFFERSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink1bufferstatus0::R](R) reader structure"]
impl crate::Readable for LBHLINK1BUFFERSTATUS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhlink1bufferstatus0::W](W) writer structure"]
impl crate::Writable for LBHLINK1BUFFERSTATUS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHLINK1BUFFERSTATUS0 to value 0"]
impl crate::Resettable for LBHLINK1BUFFERSTATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
