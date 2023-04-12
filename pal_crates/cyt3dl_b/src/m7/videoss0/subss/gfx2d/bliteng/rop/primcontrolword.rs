#[doc = "Register `PRIMCONTROLWORD` reader"]
pub struct R(crate::R<PRIMCONTROLWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIMCONTROLWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIMCONTROLWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIMCONTROLWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P_VAL` reader - Value of last received control word on the primary input. If a 39 bit pixel channel is connected, the mapping is as follows: p_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
pub type P_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of last received control word on the primary input. If a 39 bit pixel channel is connected, the mapping is as follows: p_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
    #[inline(always)]
    pub fn p_val(&self) -> P_VAL_R {
        P_VAL_R::new(self.bits)
    }
}
#[doc = "Value of last received primary control word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [primcontrolword](index.html) module"]
pub struct PRIMCONTROLWORD_SPEC;
impl crate::RegisterSpec for PRIMCONTROLWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [primcontrolword::R](R) reader structure"]
impl crate::Readable for PRIMCONTROLWORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRIMCONTROLWORD to value 0"]
impl crate::Resettable for PRIMCONTROLWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
