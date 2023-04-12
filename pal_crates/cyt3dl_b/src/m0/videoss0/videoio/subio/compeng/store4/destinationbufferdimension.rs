#[doc = "Register `DESTINATIONBUFFERDIMENSION` reader"]
pub struct R(crate::R<DESTINATIONBUFFERDIMENSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONBUFFERDIMENSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONBUFFERDIMENSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONBUFFERDIMENSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONBUFFERDIMENSION` writer"]
pub struct W(crate::W<DESTINATIONBUFFERDIMENSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONBUFFERDIMENSION_SPEC>;
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
impl From<crate::W<DESTINATIONBUFFERDIMENSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONBUFFERDIMENSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH` reader - Width of the destination buffer in pixels minus one."]
pub type LINEWIDTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH` writer - Width of the destination buffer in pixels minus one."]
pub type LINEWIDTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERDIMENSION_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT` reader - Number of lines of the destination buffer in pixels minus one."]
pub type LINECOUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT` writer - Number of lines of the destination buffer in pixels minus one."]
pub type LINECOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERDIMENSION_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width of the destination buffer in pixels minus one."]
    #[inline(always)]
    pub fn linewidth(&self) -> LINEWIDTH_R {
        LINEWIDTH_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Number of lines of the destination buffer in pixels minus one."]
    #[inline(always)]
    pub fn linecount(&self) -> LINECOUNT_R {
        LINECOUNT_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of the destination buffer in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth(&mut self) -> LINEWIDTH_W<0> {
        LINEWIDTH_W::new(self)
    }
    #[doc = "Bits 16:29 - Number of lines of the destination buffer in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn linecount(&mut self) -> LINECOUNT_W<16> {
        LINECOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer dimension.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationbufferdimension](index.html) module"]
pub struct DESTINATIONBUFFERDIMENSION_SPEC;
impl crate::RegisterSpec for DESTINATIONBUFFERDIMENSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationbufferdimension::R](R) reader structure"]
impl crate::Readable for DESTINATIONBUFFERDIMENSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationbufferdimension::W](W) writer structure"]
impl crate::Writable for DESTINATIONBUFFERDIMENSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONBUFFERDIMENSION to value 0x3fff_3fff"]
impl crate::Resettable for DESTINATIONBUFFERDIMENSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
