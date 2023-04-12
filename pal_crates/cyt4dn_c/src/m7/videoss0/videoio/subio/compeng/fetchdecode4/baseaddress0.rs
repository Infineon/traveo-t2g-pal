#[doc = "Register `BASEADDRESS0` reader"]
pub struct R(crate::R<BASEADDRESS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BASEADDRESS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BASEADDRESS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BASEADDRESS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BASEADDRESS0` writer"]
pub struct W(crate::W<BASEADDRESS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BASEADDRESS0_SPEC>;
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
impl From<crate::W<BASEADDRESS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BASEADDRESS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEADDRESS0` reader - Byte aligned start address of the layer source buffer. For a pixel width of 32 bits or DECODE operations BaseAddress bits 1 downto 0 have to be 0 and for a pixel width of 16 bit BaseAddress\\[0\\]
has to be 0."]
pub type BASEADDRESS0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEADDRESS0` writer - Byte aligned start address of the layer source buffer. For a pixel width of 32 bits or DECODE operations BaseAddress bits 1 downto 0 have to be 0 and for a pixel width of 16 bit BaseAddress\\[0\\]
has to be 0."]
pub type BASEADDRESS0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BASEADDRESS0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Byte aligned start address of the layer source buffer. For a pixel width of 32 bits or DECODE operations BaseAddress bits 1 downto 0 have to be 0 and for a pixel width of 16 bit BaseAddress\\[0\\]
has to be 0."]
    #[inline(always)]
    pub fn baseaddress0(&self) -> BASEADDRESS0_R {
        BASEADDRESS0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Byte aligned start address of the layer source buffer. For a pixel width of 32 bits or DECODE operations BaseAddress bits 1 downto 0 have to be 0 and for a pixel width of 16 bit BaseAddress\\[0\\]
has to be 0."]
    #[inline(always)]
    #[must_use]
    pub fn baseaddress0(&mut self) -> BASEADDRESS0_W<0> {
        BASEADDRESS0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source buffer base address of layer 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baseaddress0](index.html) module"]
pub struct BASEADDRESS0_SPEC;
impl crate::RegisterSpec for BASEADDRESS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [baseaddress0::R](R) reader structure"]
impl crate::Readable for BASEADDRESS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baseaddress0::W](W) writer structure"]
impl crate::Writable for BASEADDRESS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BASEADDRESS0 to value 0"]
impl crate::Resettable for BASEADDRESS0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
