#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - Reflects the state of the IP: '0': Idle/no busy. '1': Busy: - Instruction is pending in the instruction FIFO. - Instruction is busy in a IP component (e.g. SHA1, SHA2, SHA3, DES, TDES, AES, CHACHA, ...). - Store FIFO is busy. - TR or PR command is busy."]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 31 - Reflects the state of the IP: '0': Idle/no busy. '1': Busy: - Instruction is pending in the instruction FIFO. - Instruction is busy in a IP component (e.g. SHA1, SHA2, SHA3, DES, TDES, AES, CHACHA, ...). - Store FIFO is busy. - TR or PR command is busy."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
