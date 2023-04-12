#[doc = "Register `SOURCEBUFFERDIMENSION0` reader"]
pub struct R(crate::R<SOURCEBUFFERDIMENSION0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOURCEBUFFERDIMENSION0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOURCEBUFFERDIMENSION0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOURCEBUFFERDIMENSION0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOURCEBUFFERDIMENSION0` writer"]
pub struct W(crate::W<SOURCEBUFFERDIMENSION0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOURCEBUFFERDIMENSION0_SPEC>;
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
impl From<crate::W<SOURCEBUFFERDIMENSION0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOURCEBUFFERDIMENSION0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINEWIDTH0` reader - Width of the source buffer of the layer in pixels minus one."]
pub type LINEWIDTH0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINEWIDTH0` writer - Width of the source buffer of the layer in pixels minus one."]
pub type LINEWIDTH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION0_SPEC, u16, u16, 14, O>;
#[doc = "Field `LINECOUNT0` reader - Number of lines of the source buffer of the layer minus one."]
pub type LINECOUNT0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LINECOUNT0` writer - Number of lines of the source buffer of the layer minus one."]
pub type LINECOUNT0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOURCEBUFFERDIMENSION0_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Width of the source buffer of the layer in pixels minus one."]
    #[inline(always)]
    pub fn linewidth0(&self) -> LINEWIDTH0_R {
        LINEWIDTH0_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - Number of lines of the source buffer of the layer minus one."]
    #[inline(always)]
    pub fn linecount0(&self) -> LINECOUNT0_R {
        LINECOUNT0_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Width of the source buffer of the layer in pixels minus one."]
    #[inline(always)]
    #[must_use]
    pub fn linewidth0(&mut self) -> LINEWIDTH0_W<0> {
        LINEWIDTH0_W::new(self)
    }
    #[doc = "Bits 16:29 - Number of lines of the source buffer of the layer minus one."]
    #[inline(always)]
    #[must_use]
    pub fn linecount0(&mut self) -> LINECOUNT0_W<16> {
        LINECOUNT0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer dimension of layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sourcebufferdimension0](index.html) module"]
pub struct SOURCEBUFFERDIMENSION0_SPEC;
impl crate::RegisterSpec for SOURCEBUFFERDIMENSION0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sourcebufferdimension0::R](R) reader structure"]
impl crate::Readable for SOURCEBUFFERDIMENSION0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sourcebufferdimension0::W](W) writer structure"]
impl crate::Writable for SOURCEBUFFERDIMENSION0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOURCEBUFFERDIMENSION0 to value 0x3fff_3fff"]
impl crate::Resettable for SOURCEBUFFERDIMENSION0_SPEC {
    const RESET_VALUE: Self::Ux = 0x3fff_3fff;
}
