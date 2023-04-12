#[doc = "Register `COMMANDTRANSFERMAPPING2` reader"]
pub struct R(crate::R<COMMANDTRANSFERMAPPING2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMMANDTRANSFERMAPPING2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMMANDTRANSFERMAPPING2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMMANDTRANSFERMAPPING2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMANDTRANSFERMAPPING2` writer"]
pub struct W(crate::W<COMMANDTRANSFERMAPPING2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMMANDTRANSFERMAPPING2_SPEC>;
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
impl From<crate::W<COMMANDTRANSFERMAPPING2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMMANDTRANSFERMAPPING2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMANDTRANSFER2BITS` reader - Number of bits from InstructionFifo to be transfered with third transfer."]
pub type COMMANDTRANSFER2BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER2BITS` writer - Number of bits from InstructionFifo to be transfered with third transfer."]
pub type COMMANDTRANSFER2BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER2SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
pub type COMMANDTRANSFER2SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER2SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
pub type COMMANDTRANSFER2SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMMANDTRANSFER2DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
pub type COMMANDTRANSFER2DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMMANDTRANSFER2DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
pub type COMMANDTRANSFER2DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMMANDTRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with third transfer."]
    #[inline(always)]
    pub fn commandtransfer2bits(&self) -> COMMANDTRANSFER2BITS_R {
        COMMANDTRANSFER2BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
    #[inline(always)]
    pub fn commandtransfer2srclsb(&self) -> COMMANDTRANSFER2SRCLSB_R {
        COMMANDTRANSFER2SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
    #[inline(always)]
    pub fn commandtransfer2dstlsb(&self) -> COMMANDTRANSFER2DSTLSB_R {
        COMMANDTRANSFER2DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from InstructionFifo to be transfered with third transfer."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer2bits(&mut self) -> COMMANDTRANSFER2BITS_W<0> {
        COMMANDTRANSFER2BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer2srclsb(&mut self) -> COMMANDTRANSFER2SRCLSB_W<5> {
        COMMANDTRANSFER2SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn commandtransfer2dstlsb(&mut self) -> COMMANDTRANSFER2DSTLSB_W<10> {
        COMMANDTRANSFER2DSTLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for command data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [commandtransfermapping2](index.html) module"]
pub struct COMMANDTRANSFERMAPPING2_SPEC;
impl crate::RegisterSpec for COMMANDTRANSFERMAPPING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [commandtransfermapping2::R](R) reader structure"]
impl crate::Readable for COMMANDTRANSFERMAPPING2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [commandtransfermapping2::W](W) writer structure"]
impl crate::Writable for COMMANDTRANSFERMAPPING2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMMANDTRANSFERMAPPING2 to value 0"]
impl crate::Resettable for COMMANDTRANSFERMAPPING2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
