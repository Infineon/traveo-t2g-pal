#[doc = "Register `DATATRANSFERMAPPING1` reader"]
pub struct R(crate::R<DATATRANSFERMAPPING1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATATRANSFERMAPPING1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATATRANSFERMAPPING1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATATRANSFERMAPPING1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATATRANSFERMAPPING1` writer"]
pub struct W(crate::W<DATATRANSFERMAPPING1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATATRANSFERMAPPING1_SPEC>;
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
impl From<crate::W<DATATRANSFERMAPPING1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATATRANSFERMAPPING1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATRANSFER1BITS` reader - Number of bits from DataFifo to be transfered with second transfer."]
pub type DATATRANSFER1BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER1BITS` writer - Number of bits from DataFifo to be transfered with second transfer."]
pub type DATATRANSFER1BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER1SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
pub type DATATRANSFER1SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER1SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
pub type DATATRANSFER1SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER1DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
pub type DATATRANSFER1DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER1DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
pub type DATATRANSFER1DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER1NEXT` reader - If set, third transfer will be required to transmit a data word."]
pub type DATATRANSFER1NEXT_R = crate::BitReader<bool>;
#[doc = "Field `DATATRANSFER1NEXT` writer - If set, third transfer will be required to transmit a data word."]
pub type DATATRANSFER1NEXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DATATRANSFERMAPPING1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with second transfer."]
    #[inline(always)]
    pub fn datatransfer1bits(&self) -> DATATRANSFER1BITS_R {
        DATATRANSFER1BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
    #[inline(always)]
    pub fn datatransfer1srclsb(&self) -> DATATRANSFER1SRCLSB_R {
        DATATRANSFER1SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
    #[inline(always)]
    pub fn datatransfer1dstlsb(&self) -> DATATRANSFER1DSTLSB_R {
        DATATRANSFER1DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - If set, third transfer will be required to transmit a data word."]
    #[inline(always)]
    pub fn datatransfer1next(&self) -> DATATRANSFER1NEXT_R {
        DATATRANSFER1NEXT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with second transfer."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer1bits(&mut self) -> DATATRANSFER1BITS_W<0> {
        DATATRANSFER1BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer1srclsb(&mut self) -> DATATRANSFER1SRCLSB_W<5> {
        DATATRANSFER1SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer1dstlsb(&mut self) -> DATATRANSFER1DSTLSB_W<10> {
        DATATRANSFER1DSTLSB_W::new(self)
    }
    #[doc = "Bit 15 - If set, third transfer will be required to transmit a data word."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer1next(&mut self) -> DATATRANSFER1NEXT_W<15> {
        DATATRANSFER1NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for data/parameter data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datatransfermapping1](index.html) module"]
pub struct DATATRANSFERMAPPING1_SPEC;
impl crate::RegisterSpec for DATATRANSFERMAPPING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datatransfermapping1::R](R) reader structure"]
impl crate::Readable for DATATRANSFERMAPPING1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datatransfermapping1::W](W) writer structure"]
impl crate::Writable for DATATRANSFERMAPPING1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATATRANSFERMAPPING1 to value 0"]
impl crate::Resettable for DATATRANSFERMAPPING1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
