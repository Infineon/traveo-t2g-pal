#[doc = "Register `SECCONTROLWORD` reader"]
pub struct R(crate::R<SECCONTROLWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCONTROLWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCONTROLWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCONTROLWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `S_VAL` reader - Value of last received control word on secondary input. If a 39 bit pixel channel is connected, the mapping is as follows: s_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
pub type S_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Value of last received control word on secondary input. If a 39 bit pixel channel is connected, the mapping is as follows: s_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
    #[inline(always)]
    pub fn s_val(&self) -> S_VAL_R {
        S_VAL_R::new(self.bits)
    }
}
#[doc = "Value of last received secondary control word\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccontrolword](index.html) module"]
pub struct SECCONTROLWORD_SPEC;
impl crate::RegisterSpec for SECCONTROLWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccontrolword::R](R) reader structure"]
impl crate::Readable for SECCONTROLWORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SECCONTROLWORD to value 0"]
impl crate::Resettable for SECCONTROLWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
