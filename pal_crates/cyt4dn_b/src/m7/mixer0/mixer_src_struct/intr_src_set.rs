#[doc = "Register `INTR_SRC_SET` reader"]
pub struct R(crate::R<INTR_SRC_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SRC_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SRC_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SRC_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR_SRC_SET` writer"]
pub struct W(crate::W<INTR_SRC_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SRC_SET_SPEC>;
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
impl From<crate::W<INTR_SRC_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SRC_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFO_TRIGGER` reader - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FIFO_TRIGGER_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_TRIGGER` writer - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FIFO_TRIGGER_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SET_SPEC, bool, O>;
#[doc = "Field `FIFO_OVERFLOW` reader - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FIFO_OVERFLOW_R = crate::BitReader<bool>;
#[doc = "Field `FIFO_OVERFLOW` writer - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FIFO_OVERFLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SET_SPEC, bool, O>;
#[doc = "Field `FADED_OUT` reader - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FADED_OUT_R = crate::BitReader<bool>;
#[doc = "Field `FADED_OUT` writer - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
pub type FADED_OUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTR_SRC_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_trigger(&self) -> FIFO_TRIGGER_R {
        FIFO_TRIGGER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn fifo_overflow(&self) -> FIFO_OVERFLOW_R {
        FIFO_OVERFLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    pub fn faded_out(&self) -> FADED_OUT_R {
        FADED_OUT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_trigger(&mut self) -> FIFO_TRIGGER_W<0> {
        FIFO_TRIGGER_W::new(self)
    }
    #[doc = "Bit 1 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn fifo_overflow(&mut self) -> FIFO_OVERFLOW_W<1> {
        FIFO_OVERFLOW_W::new(self)
    }
    #[doc = "Bit 8 - Write this field with '1' to set corresponding INTR_SRC field (a write of '0' has no effect)."]
    #[inline(always)]
    #[must_use]
    pub fn faded_out(&mut self) -> FADED_OUT_W<8> {
        FADED_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr_src_set](index.html) module"]
pub struct INTR_SRC_SET_SPEC;
impl crate::RegisterSpec for INTR_SRC_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr_src_set::R](R) reader structure"]
impl crate::Readable for INTR_SRC_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr_src_set::W](W) writer structure"]
impl crate::Writable for INTR_SRC_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTR_SRC_SET to value 0"]
impl crate::Resettable for INTR_SRC_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
