#[doc = "Register `TERTCONTROLWORD` reader"]
pub struct R(crate::R<TERTCONTROLWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TERTCONTROLWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TERTCONTROLWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TERTCONTROLWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `T_VAL` reader - Value of last received control word on the tertiary input. If a 39 bit pixel channel is connected, the mapping is as follows: t_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
pub type T_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of last received control word on the tertiary input. If a 39 bit pixel channel is connected, the mapping is as follows: t_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
    #[inline(always)]
    pub fn t_val(&self) -> T_VAL_R {
        T_VAL_R::new(self.bits)
    }
}
#[doc = "Value of last received tertiary control word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tertcontrolword](index.html) module"]
pub struct TERTCONTROLWORD_SPEC;
impl crate::RegisterSpec for TERTCONTROLWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tertcontrolword::R](R) reader structure"]
impl crate::Readable for TERTCONTROLWORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TERTCONTROLWORD to value 0"]
impl crate::Resettable for TERTCONTROLWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
