#[doc = "Register `LASTCONTROLWORD` reader"]
pub struct R(crate::R<LASTCONTROLWORD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LASTCONTROLWORD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LASTCONTROLWORD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LASTCONTROLWORD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L_VAL` reader - Shows the last control word received from the pixel engine. If a 39 bit pixel channel is connected, the mapping is as follows: l_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
pub type L_VAL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shows the last control word received from the pixel engine. If a 39 bit pixel channel is connected, the mapping is as follows: l_val\\[31:0\\]
= { data\\[37:22\\], data\\[19:12\\], data\\[9:2\\]
}. For debug purposes only, read when stable only, otherwise read data might be corrupted."]
    #[inline(always)]
    pub fn l_val(&self) -> L_VAL_R {
        L_VAL_R::new(self.bits)
    }
}
#[doc = "Shows the last control word received\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lastcontrolword](index.html) module"]
pub struct LASTCONTROLWORD_SPEC;
impl crate::RegisterSpec for LASTCONTROLWORD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lastcontrolword::R](R) reader structure"]
impl crate::Readable for LASTCONTROLWORD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LASTCONTROLWORD to value 0"]
impl crate::Resettable for LASTCONTROLWORD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
