#[doc = "Register `DESTINATIONBUFFERATTRIBUTES` reader"]
pub struct R(crate::R<DESTINATIONBUFFERATTRIBUTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESTINATIONBUFFERATTRIBUTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESTINATIONBUFFERATTRIBUTES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESTINATIONBUFFERATTRIBUTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESTINATIONBUFFERATTRIBUTES` writer"]
pub struct W(crate::W<DESTINATIONBUFFERATTRIBUTES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESTINATIONBUFFERATTRIBUTES_SPEC>;
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
impl From<crate::W<DESTINATIONBUFFERATTRIBUTES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESTINATIONBUFFERATTRIBUTES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRIDE` reader - Destination buffer stride in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
pub type STRIDE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `STRIDE` writer - Destination buffer stride in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
pub type STRIDE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERATTRIBUTES_SPEC, u32, u32, 17, O>;
#[doc = "Field `BITSPERPIXEL` reader - Size of a pixel in the destination buffer in bits. Has to be a power of two, 18 or 24. When 64 bit is selected, input pixels are converted into a 32 bit value that is replicated into low and high word of the 64 bit output. Set SetBurstLength at least to 2 in that case to get best possible performance."]
pub type BITSPERPIXEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BITSPERPIXEL` writer - Size of a pixel in the destination buffer in bits. Has to be a power of two, 18 or 24. When 64 bit is selected, input pixels are converted into a 32 bit value that is replicated into low and high word of the 64 bit output. Set SetBurstLength at least to 2 in that case to get best possible performance."]
pub type BITSPERPIXEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DESTINATIONBUFFERATTRIBUTES_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:16 - Destination buffer stride in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
    #[inline(always)]
    pub fn stride(&self) -> STRIDE_R {
        STRIDE_R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:30 - Size of a pixel in the destination buffer in bits. Has to be a power of two, 18 or 24. When 64 bit is selected, input pixels are converted into a 32 bit value that is replicated into low and high word of the 64 bit output. Set SetBurstLength at least to 2 in that case to get best possible performance."]
    #[inline(always)]
    pub fn bitsperpixel(&self) -> BITSPERPIXEL_R {
        BITSPERPIXEL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - Destination buffer stride in bytes minus one, used for address generation. For a pixel width of 32 bits Stride has to be dividable by 4 and given minus one and for a pixel width of 16 bit Stride has to be dividable by two and given minus one."]
    #[inline(always)]
    #[must_use]
    pub fn stride(&mut self) -> STRIDE_W<0> {
        STRIDE_W::new(self)
    }
    #[doc = "Bits 24:30 - Size of a pixel in the destination buffer in bits. Has to be a power of two, 18 or 24. When 64 bit is selected, input pixels are converted into a 32 bit value that is replicated into low and high word of the 64 bit output. Set SetBurstLength at least to 2 in that case to get best possible performance."]
    #[inline(always)]
    #[must_use]
    pub fn bitsperpixel(&mut self) -> BITSPERPIXEL_W<24> {
        BITSPERPIXEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination buffer attributes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [destinationbufferattributes](index.html) module"]
pub struct DESTINATIONBUFFERATTRIBUTES_SPEC;
impl crate::RegisterSpec for DESTINATIONBUFFERATTRIBUTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [destinationbufferattributes::R](R) reader structure"]
impl crate::Readable for DESTINATIONBUFFERATTRIBUTES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [destinationbufferattributes::W](W) writer structure"]
impl crate::Writable for DESTINATIONBUFFERATTRIBUTES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESTINATIONBUFFERATTRIBUTES to value 0x2000_04ff"]
impl crate::Resettable for DESTINATIONBUFFERATTRIBUTES_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_04ff;
}
