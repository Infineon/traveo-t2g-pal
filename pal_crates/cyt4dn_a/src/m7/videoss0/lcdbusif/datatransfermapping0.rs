#[doc = "Register `DATATRANSFERMAPPING0` reader"]
pub struct R(crate::R<DATATRANSFERMAPPING0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATATRANSFERMAPPING0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATATRANSFERMAPPING0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATATRANSFERMAPPING0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATATRANSFERMAPPING0` writer"]
pub struct W(crate::W<DATATRANSFERMAPPING0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATATRANSFERMAPPING0_SPEC>;
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
impl From<crate::W<DATATRANSFERMAPPING0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATATRANSFERMAPPING0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATRANSFER0BITS` reader - Number of bits from DataFifo to be transfered with first transfer."]
pub type DATATRANSFER0BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER0BITS` writer - Number of bits from DataFifo to be transfered with first transfer."]
pub type DATATRANSFER0BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER0SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
pub type DATATRANSFER0SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER0SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
pub type DATATRANSFER0SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER0DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
pub type DATATRANSFER0DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER0DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
pub type DATATRANSFER0DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER0NEXT` reader - If set, second transfer will be required to transmit a data word."]
pub type DATATRANSFER0NEXT_R = crate::BitReader<bool>;
#[doc = "Field `DATATRANSFER0NEXT` writer - If set, second transfer will be required to transmit a data word."]
pub type DATATRANSFER0NEXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DATATRANSFERMAPPING0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with first transfer."]
    #[inline(always)]
    pub fn datatransfer0bits(&self) -> DATATRANSFER0BITS_R {
        DATATRANSFER0BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
    #[inline(always)]
    pub fn datatransfer0srclsb(&self) -> DATATRANSFER0SRCLSB_R {
        DATATRANSFER0SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
    #[inline(always)]
    pub fn datatransfer0dstlsb(&self) -> DATATRANSFER0DSTLSB_R {
        DATATRANSFER0DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - If set, second transfer will be required to transmit a data word."]
    #[inline(always)]
    pub fn datatransfer0next(&self) -> DATATRANSFER0NEXT_R {
        DATATRANSFER0NEXT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with first transfer."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer0bits(&mut self) -> DATATRANSFER0BITS_W<0> {
        DATATRANSFER0BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer0srclsb(&mut self) -> DATATRANSFER0SRCLSB_W<5> {
        DATATRANSFER0SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer0dstlsb(&mut self) -> DATATRANSFER0DSTLSB_W<10> {
        DATATRANSFER0DSTLSB_W::new(self)
    }
    #[doc = "Bit 15 - If set, second transfer will be required to transmit a data word."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer0next(&mut self) -> DATATRANSFER0NEXT_W<15> {
        DATATRANSFER0NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for data/parameter data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datatransfermapping0](index.html) module"]
pub struct DATATRANSFERMAPPING0_SPEC;
impl crate::RegisterSpec for DATATRANSFERMAPPING0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datatransfermapping0::R](R) reader structure"]
impl crate::Readable for DATATRANSFERMAPPING0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datatransfermapping0::W](W) writer structure"]
impl crate::Writable for DATATRANSFERMAPPING0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATATRANSFERMAPPING0 to value 0x08"]
impl crate::Resettable for DATATRANSFERMAPPING0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
