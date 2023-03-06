#[doc = "Register `DESTINATIONATTRIBUTES` reader"]
pub struct R(crate::R<DESTINATIONATTRIBUTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONATTRIBUTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONATTRIBUTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONATTRIBUTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONATTRIBUTES` writer"]
pub struct W(crate::W<DESTINATIONATTRIBUTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONATTRIBUTES_SPEC>;
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
impl From<crate::W<DESTINATIONATTRIBUTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONATTRIBUTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITSPERTRANSFER` reader - How many bits can be packed into one transfer (allows packing multiple bits into one LCDBus transfer)."]
pub type BITSPERTRANSFER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERTRANSFER` writer - How many bits can be packed into one transfer (allows packing multiple bits into one LCDBus transfer)."]
pub type BITSPERTRANSFER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONATTRIBUTES_SPEC, u8, u8, 5, O>;
#[doc = "Field `BITSPERPIXEL` reader - How many bits are required for one pixel."]
pub type BITSPERPIXEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL` writer - How many bits are required for one pixel."]
pub type BITSPERPIXEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONATTRIBUTES_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - How many bits can be packed into one transfer (allows packing multiple bits into one LCDBus transfer)."]
    #[inline(always)]
    pub fn bitspertransfer(&self) -> BITSPERTRANSFER_R {
        BITSPERTRANSFER_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - How many bits are required for one pixel."]
    #[inline(always)]
    pub fn bitsperpixel(&self) -> BITSPERPIXEL_R {
        BITSPERPIXEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - How many bits can be packed into one transfer (allows packing multiple bits into one LCDBus transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn bitspertransfer(&mut self) -> BITSPERTRANSFER_W<0> {
        BITSPERTRANSFER_W::new(self)
    }
    #[doc = "Bits 8:12 - How many bits are required for one pixel."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel(&mut self) -> BITSPERPIXEL_W<8> {
        BITSPERPIXEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for packing unit.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationattributes](index.html) module"]
pub struct DESTINATIONATTRIBUTES_SPEC;
impl crate::RegisterSpec for DESTINATIONATTRIBUTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationattributes::R](R) reader structure"]
impl crate::Readable for DESTINATIONATTRIBUTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationattributes::W](W) writer structure"]
impl crate::Writable for DESTINATIONATTRIBUTES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONATTRIBUTES to value 0x1010"]
impl crate::Resettable for DESTINATIONATTRIBUTES_SPEC {
    const RESET_VALUE: Self::Ux = 0x1010;
}
