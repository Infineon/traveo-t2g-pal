#[doc = "Register `TID_CAPACITY_STATUS` reader"]
pub struct R(crate::R<TID_CAPACITY_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TID_CAPACITY_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TID_CAPACITY_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TID_CAPACITY_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TID_CAPACITY_STATUS` writer"]
pub struct W(crate::W<TID_CAPACITY_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TID_CAPACITY_STATUS_SPEC>;
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
impl From<crate::W<TID_CAPACITY_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TID_CAPACITY_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TID0_MAX` reader - When the limit for outstanding transactions is outreached for channel 0, the bit is set. It is cleared by writing a '1' to it."]
pub type TID0_MAX_R = crate::BitReader<bool>;
#[doc = "Field `TID0_MAX` writer - When the limit for outstanding transactions is outreached for channel 0, the bit is set. It is cleared by writing a '1' to it."]
pub type TID0_MAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TID_CAPACITY_STATUS_SPEC, bool, O>;
#[doc = "Field `TID1_MAX` reader - When the limit for outstanding transactions is outreached for channel 1, the bit is set. It is cleared by writing a '1' to it."]
pub type TID1_MAX_R = crate::BitReader<bool>;
#[doc = "Field `TID1_MAX` writer - When the limit for outstanding transactions is outreached for channel 1, the bit is set. It is cleared by writing a '1' to it."]
pub type TID1_MAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TID_CAPACITY_STATUS_SPEC, bool, O>;
#[doc = "Field `TID2_MAX` reader - When the limit for outstanding transactions is outreached for channel 2, the bit is set. It is cleared by writing a '1' to it."]
pub type TID2_MAX_R = crate::BitReader<bool>;
#[doc = "Field `TID2_MAX` writer - When the limit for outstanding transactions is outreached for channel 2, the bit is set. It is cleared by writing a '1' to it."]
pub type TID2_MAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TID_CAPACITY_STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When the limit for outstanding transactions is outreached for channel 0, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    pub fn tid0_max(&self) -> TID0_MAX_R {
        TID0_MAX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - When the limit for outstanding transactions is outreached for channel 1, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    pub fn tid1_max(&self) -> TID1_MAX_R {
        TID1_MAX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - When the limit for outstanding transactions is outreached for channel 2, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    pub fn tid2_max(&self) -> TID2_MAX_R {
        TID2_MAX_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When the limit for outstanding transactions is outreached for channel 0, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    #[must_use]
    pub fn tid0_max(&mut self) -> TID0_MAX_W<0> {
        TID0_MAX_W::new(self)
    }
    #[doc = "Bit 4 - When the limit for outstanding transactions is outreached for channel 1, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    #[must_use]
    pub fn tid1_max(&mut self) -> TID1_MAX_W<4> {
        TID1_MAX_W::new(self)
    }
    #[doc = "Bit 8 - When the limit for outstanding transactions is outreached for channel 2, the bit is set. It is cleared by writing a '1' to it."]
    #[inline(always)]
    #[must_use]
    pub fn tid2_max(&mut self) -> TID2_MAX_W<8> {
        TID2_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Outstanding transaction capacity monitor.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tid_capacity_status](index.html) module"]
pub struct TID_CAPACITY_STATUS_SPEC;
impl crate::RegisterSpec for TID_CAPACITY_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tid_capacity_status::R](R) reader structure"]
impl crate::Readable for TID_CAPACITY_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tid_capacity_status::W](W) writer structure"]
impl crate::Writable for TID_CAPACITY_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TID_CAPACITY_STATUS to value 0"]
impl crate::Resettable for TID_CAPACITY_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
