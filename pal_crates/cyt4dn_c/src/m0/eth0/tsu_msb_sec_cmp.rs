#[doc = "Register `TSU_MSB_SEC_CMP` reader"]
pub struct R(crate::R<TSU_MSB_SEC_CMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSU_MSB_SEC_CMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSU_MSB_SEC_CMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSU_MSB_SEC_CMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSU_MSB_SEC_CMP` writer"]
pub struct W(crate::W<TSU_MSB_SEC_CMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSU_MSB_SEC_CMP_SPEC>;
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
impl From<crate::W<TSU_MSB_SEC_CMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSU_MSB_SEC_CMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARISON_MSB_SEC` reader - TSU timer comparison value (s). Value is compared to the top 16 bits (most significant 16-bits {47:32\\]
of seconds value) of the TSU timer count value."]
pub type COMPARISON_MSB_SEC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARISON_MSB_SEC` writer - TSU timer comparison value (s). Value is compared to the top 16 bits (most significant 16-bits {47:32\\]
of seconds value) of the TSU timer count value."]
pub type COMPARISON_MSB_SEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TSU_MSB_SEC_CMP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TSU timer comparison value (s). Value is compared to the top 16 bits (most significant 16-bits {47:32\\]
of seconds value) of the TSU timer count value."]
    #[inline(always)]
    pub fn comparison_msb_sec(&self) -> COMPARISON_MSB_SEC_R {
        COMPARISON_MSB_SEC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSU timer comparison value (s). Value is compared to the top 16 bits (most significant 16-bits {47:32\\]
of seconds value) of the TSU timer count value."]
    #[inline(always)]
    #[must_use]
    pub fn comparison_msb_sec(&mut self) -> COMPARISON_MSB_SEC_W<0> {
        COMPARISON_MSB_SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TSU timer comparison value seconds (47 to 32 bits)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tsu_msb_sec_cmp](index.html) module"]
pub struct TSU_MSB_SEC_CMP_SPEC;
impl crate::RegisterSpec for TSU_MSB_SEC_CMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tsu_msb_sec_cmp::R](R) reader structure"]
impl crate::Readable for TSU_MSB_SEC_CMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tsu_msb_sec_cmp::W](W) writer structure"]
impl crate::Writable for TSU_MSB_SEC_CMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSU_MSB_SEC_CMP to value 0"]
impl crate::Resettable for TSU_MSB_SEC_CMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
