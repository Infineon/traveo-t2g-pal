#[doc = "Register `TSU_SEC_CMP` reader"]
pub struct R(crate::R<TSU_SEC_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_SEC_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_SEC_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_SEC_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_SEC_CMP` writer"]
pub struct W(crate::W<TSU_SEC_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_SEC_CMP_SPEC>;
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
impl From<crate::W<TSU_SEC_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_SEC_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_SEC` reader - TSU timer comparison value (s). Value is compared to seconds value bits \\[31:0\\]
of the TSU timer count value."]
pub type COMPARISON_SEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMPARISON_SEC` writer - TSU timer comparison value (s). Value is compared to seconds value bits \\[31:0\\]
of the TSU timer count value."]
pub type COMPARISON_SEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_SEC_CMP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - TSU timer comparison value (s). Value is compared to seconds value bits \\[31:0\\]
of the TSU timer count value."]
    #[inline(always)]
    pub fn comparison_sec(&self) -> COMPARISON_SEC_R {
        COMPARISON_SEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSU timer comparison value (s). Value is compared to seconds value bits \\[31:0\\]
of the TSU timer count value."]
    #[inline(always)]
    #[must_use]
    pub fn comparison_sec(&mut self) -> COMPARISON_SEC_W<0> {
        COMPARISON_SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSU timer comparison value seconds (31 to 0 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_sec_cmp](index.html) module"]
pub struct TSU_SEC_CMP_SPEC;
impl crate::RegisterSpec for TSU_SEC_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_sec_cmp::R](R) reader structure"]
impl crate::Readable for TSU_SEC_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_sec_cmp::W](W) writer structure"]
impl crate::Writable for TSU_SEC_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_SEC_CMP to value 0"]
impl crate::Resettable for TSU_SEC_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
