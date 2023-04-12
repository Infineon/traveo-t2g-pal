#[doc = "Register `ADDR_STALL_CNT` reader"]
pub struct R(crate::R<ADDR_STALL_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_STALL_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_STALL_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_STALL_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the value of the address stall counter. This counter shows the number of filtered address stall cycles during the measurement window. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented by each address stall cycle that passes the filter defined by FILTER and FILTER_MASK. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the value of the address stall counter. This counter shows the number of filtered address stall cycles during the measurement window. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented by each address stall cycle that passes the filter defined by FILTER and FILTER_MASK. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "Address stall counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_stall_cnt](index.html) module"]
pub struct ADDR_STALL_CNT_SPEC;
impl crate::RegisterSpec for ADDR_STALL_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_stall_cnt::R](R) reader structure"]
impl crate::Readable for ADDR_STALL_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDR_STALL_CNT to value 0"]
impl crate::Resettable for ADDR_STALL_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
