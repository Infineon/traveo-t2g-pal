#[doc = "Register `COMMANDTRANSFERMAPPING0` reader"]
pub struct R(crate::R<COMMANDTRANSFERMAPPING0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMANDTRANSFERMAPPING0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMANDTRANSFERMAPPING0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMANDTRANSFERMAPPING0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMANDTRANSFERMAPPING0` writer"]
pub struct W(crate::W<COMMANDTRANSFERMAPPING0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMANDTRANSFERMAPPING0_SPEC>;
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
impl From<crate::W<COMMANDTRANSFERMAPPING0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMANDTRANSFERMAPPING0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMANDTRANSFER0BITS` reader - Number of bits from InstructionFifo to be transfered with first transfer. Must be greater than 0."]
pub type COMMANDTRANSFER0BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER0BITS` writer - Number of bits from InstructionFifo to be transfered with first transfer. Must be greater than 0."]
pub type COMMANDTRANSFER0BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER0SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
pub type COMMANDTRANSFER0SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER0SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
pub type COMMANDTRANSFER0SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER0DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
pub type COMMANDTRANSFER0DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER0DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
pub type COMMANDTRANSFER0DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING0_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER0NEXT` reader - If set, a second transfer will be required to transmit the command data, see CommandTransferMapping1."]
pub type COMMANDTRANSFER0NEXT_R = crate::BitReader<bool>;
#[doc = "Field `COMMANDTRANSFER0NEXT` writer - If set, a second transfer will be required to transmit the command data, see CommandTransferMapping1."]
pub type COMMANDTRANSFER0NEXT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMMANDTRANSFERMAPPING0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with first transfer. Must be greater than 0."]
    #[inline(always)]
    pub fn commandtransfer0bits(&self) -> COMMANDTRANSFER0BITS_R {
        COMMANDTRANSFER0BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
    #[inline(always)]
    pub fn commandtransfer0srclsb(&self) -> COMMANDTRANSFER0SRCLSB_R {
        COMMANDTRANSFER0SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
    #[inline(always)]
    pub fn commandtransfer0dstlsb(&self) -> COMMANDTRANSFER0DSTLSB_R {
        COMMANDTRANSFER0DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - If set, a second transfer will be required to transmit the command data, see CommandTransferMapping1."]
    #[inline(always)]
    pub fn commandtransfer0next(&self) -> COMMANDTRANSFER0NEXT_R {
        COMMANDTRANSFER0NEXT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with first transfer. Must be greater than 0."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer0bits(&mut self) -> COMMANDTRANSFER0BITS_W<0> {
        COMMANDTRANSFER0BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (first transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer0srclsb(&mut self) -> COMMANDTRANSFER0SRCLSB_W<5> {
        COMMANDTRANSFER0SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (first transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer0dstlsb(&mut self) -> COMMANDTRANSFER0DSTLSB_W<10> {
        COMMANDTRANSFER0DSTLSB_W::new(self)
    }
    #[doc = "Bit 15 - If set, a second transfer will be required to transmit the command data, see CommandTransferMapping1."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer0next(&mut self) -> COMMANDTRANSFER0NEXT_W<15> {
        COMMANDTRANSFER0NEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for command data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [commandtransfermapping0](index.html) module"]
pub struct COMMANDTRANSFERMAPPING0_SPEC;
impl crate::RegisterSpec for COMMANDTRANSFERMAPPING0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [commandtransfermapping0::R](R) reader structure"]
impl crate::Readable for COMMANDTRANSFERMAPPING0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [commandtransfermapping0::W](W) writer structure"]
impl crate::Writable for COMMANDTRANSFERMAPPING0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMANDTRANSFERMAPPING0 to value 0x08"]
impl crate::Resettable for COMMANDTRANSFERMAPPING0_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
