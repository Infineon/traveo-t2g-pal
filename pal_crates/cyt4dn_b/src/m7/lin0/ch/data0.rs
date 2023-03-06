#[doc = "Register `DATA0` reader"]
pub struct R(crate::R<DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA0` writer"]
pub struct W(crate::W<DATA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA0_SPEC>;
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
impl From<crate::W<DATA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA1` reader - Data field 1. Transmission: To be transmitted data field. SW provides data field. Reception: Received data field. SW uses the data field."]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` writer - Data field 1. Transmission: To be transmitted data field. SW provides data field. Reception: Received data field. SW uses the data field."]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA2` reader - Data field 2."]
pub type DATA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA2` writer - Data field 2."]
pub type DATA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA3` reader - Data field 3."]
pub type DATA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA3` writer - Data field 3."]
pub type DATA3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA0_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA4` reader - Data field 4."]
pub type DATA4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA4` writer - Data field 4."]
pub type DATA4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data field 1. Transmission: To be transmitted data field. SW provides data field. Reception: Received data field. SW uses the data field."]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data field 2."]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data field 3."]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data field 4."]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data field 1. Transmission: To be transmitted data field. SW provides data field. Reception: Received data field. SW uses the data field."]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<0> {
        DATA1_W::new(self)
    }
    #[doc = "Bits 8:15 - Data field 2."]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<8> {
        DATA2_W::new(self)
    }
    #[doc = "Bits 16:23 - Data field 3."]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<16> {
        DATA3_W::new(self)
    }
    #[doc = "Bits 24:31 - Data field 4."]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<24> {
        DATA4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response data 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data0](index.html) module"]
pub struct DATA0_SPEC;
impl crate::RegisterSpec for DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data0::R](R) reader structure"]
impl crate::Readable for DATA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data0::W](W) writer structure"]
impl crate::Writable for DATA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA0 to value 0"]
impl crate::Resettable for DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
