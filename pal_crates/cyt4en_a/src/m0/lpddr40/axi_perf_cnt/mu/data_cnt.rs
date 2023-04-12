#[doc = "Register `DATA_CNT` reader"]
pub struct R(crate::R<DATA_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the value of the data transfer counter. This counter shows the number of filtered data transfers during the measurement window. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented by each data transfer that passes the filter defined by FILTER and FILTER_MASK. It should be noted that if FILTER.WRITE = 1, the write data are not filtered by master ID and upper transaction ID. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the value of the data transfer counter. This counter shows the number of filtered data transfers during the measurement window. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented by each data transfer that passes the filter defined by FILTER and FILTER_MASK. It should be noted that if FILTER.WRITE = 1, the write data are not filtered by master ID and upper transaction ID. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "Data transfer counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_cnt](index.html) module"]
pub struct DATA_CNT_SPEC;
impl crate::RegisterSpec for DATA_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_cnt::R](R) reader structure"]
impl crate::Readable for DATA_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DATA_CNT to value 0"]
impl crate::Resettable for DATA_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
