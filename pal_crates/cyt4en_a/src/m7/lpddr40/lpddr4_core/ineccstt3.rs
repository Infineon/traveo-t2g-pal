#[doc = "Register `INECCSTT3` reader"]
pub struct R(crate::R<INECCSTT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INECCSTT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INECCSTT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INECCSTT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC_ERR_BUFF_EMPTY` reader - ECC Information Buffer Empty Flag NO = 0 The FIFO is not empty, some data errors occur YES = 1 The FIFO is empty, no error occurs"]
pub type ECC_ERR_BUFF_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ECC_ERR_BUFF_FULL` reader - ECC Information Buffer Full Flag NO = 0 The FIFO is not full YES = 1 The FIFO is full, the FIFO cannot collect the information of new errors"]
pub type ECC_ERR_BUFF_FULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - ECC Information Buffer Empty Flag NO = 0 The FIFO is not empty, some data errors occur YES = 1 The FIFO is empty, no error occurs"]
    #[inline(always)]
    pub fn ecc_err_buff_empty(&self) -> ECC_ERR_BUFF_EMPTY_R {
        ECC_ERR_BUFF_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC Information Buffer Full Flag NO = 0 The FIFO is not full YES = 1 The FIFO is full, the FIFO cannot collect the information of new errors"]
    #[inline(always)]
    pub fn ecc_err_buff_full(&self) -> ECC_ERR_BUFF_FULL_R {
        ECC_ERR_BUFF_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Inline ECC Status Register 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ineccstt3](index.html) module"]
pub struct INECCSTT3_SPEC;
impl crate::RegisterSpec for INECCSTT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ineccstt3::R](R) reader structure"]
impl crate::Readable for INECCSTT3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INECCSTT3 to value 0x01"]
impl crate::Resettable for INECCSTT3_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
