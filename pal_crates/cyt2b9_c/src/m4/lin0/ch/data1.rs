#[doc = "Register `DATA1` reader"]
pub struct R(crate::R<DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA1` writer"]
pub struct W(crate::W<DATA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA1_SPEC>;
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
impl From<crate::W<DATA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA5` reader - Data field 5."]
pub type DATA5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA5` writer - Data field 5."]
pub type DATA5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA6` reader - Data field 6."]
pub type DATA6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA6` writer - Data field 6."]
pub type DATA6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA7` reader - Data field 7."]
pub type DATA7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA7` writer - Data field 7."]
pub type DATA7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA1_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA8` reader - Data field 8."]
pub type DATA8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA8` writer - Data field 8."]
pub type DATA8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data field 5."]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data field 6."]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data field 7."]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data field 8."]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data field 5."]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<0> {
        DATA5_W::new(self)
    }
    #[doc = "Bits 8:15 - Data field 6."]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<8> {
        DATA6_W::new(self)
    }
    #[doc = "Bits 16:23 - Data field 7."]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<16> {
        DATA7_W::new(self)
    }
    #[doc = "Bits 24:31 - Data field 8."]
    #[inline(always)]
    #[must_use]
    pub fn data8(&mut self) -> DATA8_W<24> {
        DATA8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Response data 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data1](index.html) module"]
pub struct DATA1_SPEC;
impl crate::RegisterSpec for DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data1::R](R) reader structure"]
impl crate::Readable for DATA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data1::W](W) writer structure"]
impl crate::Writable for DATA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA1 to value 0"]
impl crate::Resettable for DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
