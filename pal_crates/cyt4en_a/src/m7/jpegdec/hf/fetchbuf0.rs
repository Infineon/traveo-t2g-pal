#[doc = "Register `FETCHBUF0` reader"]
pub struct R(crate::R<FETCHBUF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FETCHBUF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FETCHBUF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FETCHBUF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FETCHBUF0` writer"]
pub struct W(crate::W<FETCHBUF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FETCHBUF0_SPEC>;
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
impl From<crate::W<FETCHBUF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FETCHBUF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHBASEADDRESS` reader - Start address of the JPEG image in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on configured burst length. So reading ahead of the actual start address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
pub type FETCHBASEADDRESS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FETCHBASEADDRESS` writer - Start address of the JPEG image in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on configured burst length. So reading ahead of the actual start address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
pub type FETCHBASEADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FETCHBUF0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Start address of the JPEG image in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on configured burst length. So reading ahead of the actual start address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
    #[inline(always)]
    pub fn fetchbaseaddress(&self) -> FETCHBASEADDRESS_R {
        FETCHBASEADDRESS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the JPEG image in bytes. There are no alignment restrictions, however, read access to memory is 32- or 64-byte aligned, depending on configured burst length. So reading ahead of the actual start address must be allowed, if it is not aligned accordingly. RWS field, which is activated by START command."]
    #[inline(always)]
    #[must_use]
    pub fn fetchbaseaddress(&mut self) -> FETCHBASEADDRESS_W<0> {
        FETCHBASEADDRESS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer configuration (JPEG).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fetchbuf0](index.html) module"]
pub struct FETCHBUF0_SPEC;
impl crate::RegisterSpec for FETCHBUF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fetchbuf0::R](R) reader structure"]
impl crate::Readable for FETCHBUF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fetchbuf0::W](W) writer structure"]
impl crate::Writable for FETCHBUF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FETCHBUF0 to value 0"]
impl crate::Resettable for FETCHBUF0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
