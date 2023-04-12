#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRITE_TIMEOUT` reader - Timeout detected when writing to the LUT"]
pub type WRITE_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_TIMEOUT` writer - Timeout detected when writing to the LUT"]
pub type WRITE_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `READ_TIMEOUT` reader - Timeout detected when reading from the LUT"]
pub type READ_TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `READ_TIMEOUT` writer - Timeout detected when reading from the LUT"]
pub type READ_TIMEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timeout detected when writing to the LUT"]
    #[inline(always)]
    pub fn write_timeout(&self) -> WRITE_TIMEOUT_R {
        WRITE_TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout detected when reading from the LUT"]
    #[inline(always)]
    pub fn read_timeout(&self) -> READ_TIMEOUT_R {
        READ_TIMEOUT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timeout detected when writing to the LUT"]
    #[inline(always)]
    #[must_use]
    pub fn write_timeout(&mut self) -> WRITE_TIMEOUT_W<0> {
        WRITE_TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - Timeout detected when reading from the LUT"]
    #[inline(always)]
    #[must_use]
    pub fn read_timeout(&mut self) -> READ_TIMEOUT_W<4> {
        READ_TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLUT status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
