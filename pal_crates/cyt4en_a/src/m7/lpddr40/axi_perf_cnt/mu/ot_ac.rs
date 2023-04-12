#[doc = "Register `OT_AC` reader"]
pub struct R(crate::R<OT_AC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OT_AC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OT_AC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OT_AC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the accumulated number of outstanding read or write transactions during the measurement window, depending on FILTER.WRITE. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented in each AXI clock cycle by the current number of outstanding read or write transactions. If FILTER.WRITE = 0, this counter accumulates the outstanding read transactions. If FILTER.WRITE = 1, this counter accumulates the outstanding write transactions. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
pub type CNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - While TMR_STATUS.MEAS is 1, this field reads 0. While TMR_STATUS.MEAS is 0, this field reads the accumulated number of outstanding read or write transactions during the measurement window, depending on FILTER.WRITE. This counter is cleared by TMR_CMD.START. During the measurement window, this counter is incremented in each AXI clock cycle by the current number of outstanding read or write transactions. If FILTER.WRITE = 0, this counter accumulates the outstanding read transactions. If FILTER.WRITE = 1, this counter accumulates the outstanding write transactions. Instead of overflowing, this counter is clipped at 0xFFFFFFFF. This means that if at the end of the measurement window, a count of 0xFFFFFFFF is read, then it is very likely that this counter has overflowed. In this case, a shorter measurement window should be used."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
#[doc = "Accumulated outstanding transactions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ot_ac](index.html) module"]
pub struct OT_AC_SPEC;
impl crate::RegisterSpec for OT_AC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ot_ac::R](R) reader structure"]
impl crate::Readable for OT_AC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OT_AC to value 0"]
impl crate::Resettable for OT_AC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
