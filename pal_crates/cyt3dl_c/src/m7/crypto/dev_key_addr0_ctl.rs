#[doc = "Register `DEV_KEY_ADDR0_CTL` reader"]
pub struct R(crate::R<DEV_KEY_ADDR0_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_KEY_ADDR0_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_KEY_ADDR0_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_KEY_ADDR0_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_KEY_ADDR0_CTL` writer"]
pub struct W(crate::W<DEV_KEY_ADDR0_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_KEY_ADDR0_CTL_SPEC>;
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
impl From<crate::W<DEV_KEY_ADDR0_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_KEY_ADDR0_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALID` reader - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEV_KEY_ADDR0_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<31> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device key address 0 control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_key_addr0_ctl](index.html) module"]
pub struct DEV_KEY_ADDR0_CTL_SPEC;
impl crate::RegisterSpec for DEV_KEY_ADDR0_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_key_addr0_ctl::R](R) reader structure"]
impl crate::Readable for DEV_KEY_ADDR0_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_key_addr0_ctl::W](W) writer structure"]
impl crate::Writable for DEV_KEY_ADDR0_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEV_KEY_ADDR0_CTL to value 0"]
impl crate::Resettable for DEV_KEY_ADDR0_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
