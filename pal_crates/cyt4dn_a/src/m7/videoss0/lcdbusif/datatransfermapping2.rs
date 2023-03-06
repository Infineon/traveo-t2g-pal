#[doc = "Register `DATATRANSFERMAPPING2` reader"]
pub struct R(crate::R<DATATRANSFERMAPPING2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATATRANSFERMAPPING2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATATRANSFERMAPPING2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATATRANSFERMAPPING2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATATRANSFERMAPPING2` writer"]
pub struct W(crate::W<DATATRANSFERMAPPING2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATATRANSFERMAPPING2_SPEC>;
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
impl From<crate::W<DATATRANSFERMAPPING2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATATRANSFERMAPPING2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATATRANSFER2BITS` reader - Number of bits from DataFifo to be transfered with third transfer."]
pub type DATATRANSFER2BITS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER2BITS` writer - Number of bits from DataFifo to be transfered with third transfer."]
pub type DATATRANSFER2BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER2SRCLSB` reader - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
pub type DATATRANSFER2SRCLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER2SRCLSB` writer - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
pub type DATATRANSFER2SRCLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
#[doc = "Field `DATATRANSFER2DSTLSB` reader - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
pub type DATATRANSFER2DSTLSB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATATRANSFER2DSTLSB` writer - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
pub type DATATRANSFER2DSTLSB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATATRANSFERMAPPING2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with third transfer."]
    #[inline(always)]
    pub fn datatransfer2bits(&self) -> DATATRANSFER2BITS_R {
        DATATRANSFER2BITS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
    #[inline(always)]
    pub fn datatransfer2srclsb(&self) -> DATATRANSFER2SRCLSB_R {
        DATATRANSFER2SRCLSB_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
    #[inline(always)]
    pub fn datatransfer2dstlsb(&self) -> DATATRANSFER2DSTLSB_R {
        DATATRANSFER2DSTLSB_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits from DataFifo to be transfered with third transfer."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer2bits(&mut self) -> DATATRANSFER2BITS_W<0> {
        DATATRANSFER2BITS_W::new(self)
    }
    #[doc = "Bits 5:9 - Least significant bit of vector to extract from source (Wr=InstructionFifo, Rd=Interface data) (third transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer2srclsb(&mut self) -> DATATRANSFER2SRCLSB_W<5> {
        DATATRANSFER2SRCLSB_W::new(self)
    }
    #[doc = "Bits 10:14 - Least significant bit in destination word (Wr=Interface data, Rd=RxFifo) to insert vector into (third transfer)."]
    #[inline(always)]
    #[must_use]
    pub fn datatransfer2dstlsb(&mut self) -> DATATRANSFER2DSTLSB_W<10> {
        DATATRANSFER2DSTLSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer mapping for data/parameter data on LCDBus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datatransfermapping2](index.html) module"]
pub struct DATATRANSFERMAPPING2_SPEC;
impl crate::RegisterSpec for DATATRANSFERMAPPING2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datatransfermapping2::R](R) reader structure"]
impl crate::Readable for DATATRANSFERMAPPING2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datatransfermapping2::W](W) writer structure"]
impl crate::Writable for DATATRANSFERMAPPING2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATATRANSFERMAPPING2 to value 0"]
impl crate::Resettable for DATATRANSFERMAPPING2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
