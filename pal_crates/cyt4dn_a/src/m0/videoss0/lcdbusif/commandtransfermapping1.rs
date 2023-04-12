#[doc = "Register `COMMANDTRANSFERMAPPING1` reader"]
pub struct R(crate::R<COMMANDTRANSFERMAPPING1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMANDTRANSFERMAPPING1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMANDTRANSFERMAPPING1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMANDTRANSFERMAPPING1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMANDTRANSFERMAPPING1` writer"]
pub struct W(crate::W<COMMANDTRANSFERMAPPING1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMANDTRANSFERMAPPING1_SPEC>;
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
impl From<crate::W<COMMANDTRANSFERMAPPING1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMANDTRANSFERMAPPING1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMANDTRANSFER1BITS` reader - Number of bits from InstructionFifo to be transfered with second transfer. Must be greater than 0 if CommandTransfer0Next is set."]
pub type COMMANDTRANSFER1BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER1BITS` writer - Number of bits from InstructionFifo to be transfered with second transfer. Must be greater than 0 if CommandTransfer0Next is set."]
pub type COMMANDTRANSFER1BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER1SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
pub type COMMANDTRANSFER1SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER1SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
pub type COMMANDTRANSFER1SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER1DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
pub type COMMANDTRANSFER1DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER1DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
pub type COMMANDTRANSFER1DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING1_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER1NEXT` reader - If set, a third transfer will be required to transmit the command data, see CommandTransferMapping2."]
pub type COMMANDTRANSFER1NEXT_R = crate::BitReader<bool>;
#[doc = "Field `COMMANDTRANSFER1NEXT` writer - If set, a third transfer will be required to transmit the command data, see CommandTransferMapping2."]
pub type COMMANDTRANSFER1NEXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMMANDTRANSFERMAPPING1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with second transfer. Must be greater than 0 if CommandTransfer0Next is set."]
    #[inline(always)]
    pub fn commandtransfer1bits(&self) -> COMMANDTRANSFER1BITS_R {
        COMMANDTRANSFER1BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
    #[inline(always)]
    pub fn commandtransfer1srclsb(&self) -> COMMANDTRANSFER1SRCLSB_R {
        COMMANDTRANSFER1SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
    #[inline(always)]
    pub fn commandtransfer1dstlsb(&self) -> COMMANDTRANSFER1DSTLSB_R {
        COMMANDTRANSFER1DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - If set, a third transfer will be required to transmit the command data, see CommandTransferMapping2."]
    #[inline(always)]
    pub fn commandtransfer1next(&self) -> COMMANDTRANSFER1NEXT_R {
        COMMANDTRANSFER1NEXT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with second transfer. Must be greater than 0 if CommandTransfer0Next is set."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer1bits(&mut self) -> COMMANDTRANSFER1BITS_W<0> {
        COMMANDTRANSFER1BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (second transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer1srclsb(&mut self) -> COMMANDTRANSFER1SRCLSB_W<5> {
        COMMANDTRANSFER1SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (second transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer1dstlsb(&mut self) -> COMMANDTRANSFER1DSTLSB_W<10> {
        COMMANDTRANSFER1DSTLSB_W::new(self)
    }
    #[doc = "Bit 15 - If set, a third transfer will be required to transmit the command data, see CommandTransferMapping2."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer1next(&mut self) -> COMMANDTRANSFER1NEXT_W<15> {
        COMMANDTRANSFER1NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for command data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [commandtransfermapping1](index.html) module"]
pub struct COMMANDTRANSFERMAPPING1_SPEC;
impl crate::RegisterSpec for COMMANDTRANSFERMAPPING1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [commandtransfermapping1::R](R) reader structure"]
impl crate::Readable for COMMANDTRANSFERMAPPING1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [commandtransfermapping1::W](W) writer structure"]
impl crate::Writable for COMMANDTRANSFERMAPPING1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMANDTRANSFERMAPPING1 to value 0"]
impl crate::Resettable for COMMANDTRANSFERMAPPING1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
