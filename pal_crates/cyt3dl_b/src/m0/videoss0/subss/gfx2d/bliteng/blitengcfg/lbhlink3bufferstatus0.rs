#[doc = "Register `LBHLINK3BUFFERSTATUS0` reader"]
pub struct R(crate::R<LBHLINK3BUFFERSTATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LBHLINK3BUFFERSTATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LBHLINK3BUFFERSTATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LBHLINK3BUFFERSTATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LBHLINK3BUFFERSTATUS0` writer"]
pub struct W(crate::W<LBHLINK3BUFFERSTATUS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LBHLINK3BUFFERSTATUS0_SPEC>;
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
impl From<crate::W<LBHLINK3BUFFERSTATUS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LBHLINK3BUFFERSTATUS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBHLINK3MAXFREELINES` reader - The maximal number of free lines in the line buffer that can be scheduled by CmdSeq."]
pub type LBHLINK3MAXFREELINES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK3MAXFREELINES` writer - The maximal number of free lines in the line buffer that can be scheduled by CmdSeq."]
pub type LBHLINK3MAXFREELINES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK3BUFFERSTATUS0_SPEC, u16, u16, 15, O>;
#[doc = "Field `LBHLINK3MAXFREELINESSTORE` reader - The maximal number of free lines in the line buffer that can be written by the store."]
pub type LBHLINK3MAXFREELINESSTORE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LBHLINK3MAXFREELINESSTORE` writer - The maximal number of free lines in the line buffer that can be written by the store."]
pub type LBHLINK3MAXFREELINESSTORE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LBHLINK3BUFFERSTATUS0_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - The maximal number of free lines in the line buffer that can be scheduled by CmdSeq."]
    #[inline(always)]
    pub fn lbhlink3maxfreelines(&self) -> LBHLINK3MAXFREELINES_R {
        LBHLINK3MAXFREELINES_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - The maximal number of free lines in the line buffer that can be written by the store."]
    #[inline(always)]
    pub fn lbhlink3maxfreelinesstore(&self) -> LBHLINK3MAXFREELINESSTORE_R {
        LBHLINK3MAXFREELINESSTORE_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - The maximal number of free lines in the line buffer that can be scheduled by CmdSeq."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink3maxfreelines(&mut self) -> LBHLINK3MAXFREELINES_W<0> {
        LBHLINK3MAXFREELINES_W::new(self)
    }
    #[doc = "Bits 16:30 - The maximal number of free lines in the line buffer that can be written by the store."]
    #[inline(always)]
    #[must_use]
    pub fn lbhlink3maxfreelinesstore(&mut self) -> LBHLINK3MAXFREELINESSTORE_W<16> {
        LBHLINK3MAXFREELINESSTORE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer status register 0 for the line buffer handshake. Writing to this register has no effect, reading provides the value and simultaneously resets the internal register. The value provided after the first read or if LBH is inactive is not valid.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lbhlink3bufferstatus0](index.html) module"]
pub struct LBHLINK3BUFFERSTATUS0_SPEC;
impl crate::RegisterSpec for LBHLINK3BUFFERSTATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lbhlink3bufferstatus0::R](R) reader structure"]
impl crate::Readable for LBHLINK3BUFFERSTATUS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lbhlink3bufferstatus0::W](W) writer structure"]
impl crate::Writable for LBHLINK3BUFFERSTATUS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LBHLINK3BUFFERSTATUS0 to value 0"]
impl crate::Resettable for LBHLINK3BUFFERSTATUS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
