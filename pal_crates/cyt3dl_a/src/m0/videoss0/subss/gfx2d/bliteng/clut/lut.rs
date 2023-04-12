#[doc = "Register `LUT[%s]` reader"]
pub struct R(crate::R<LUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT[%s]` writer"]
pub struct W(crate::W<LUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT_SPEC>;
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
impl From<crate::W<LUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLUE` reader - Blue component"]
pub type BLUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLUE` writer - Blue component"]
pub type BLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `GREEN` reader - Green component"]
pub type GREEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GREEN` writer - Green component"]
pub type GREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u16, u16, 10, O>;
#[doc = "Field `RED` reader - Red component"]
pub type RED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RED` writer - Red component"]
pub type RED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Blue component"]
    #[inline(always)]
    pub fn blue(&self) -> BLUE_R {
        BLUE_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Green component"]
    #[inline(always)]
    pub fn green(&self) -> GREEN_R {
        GREEN_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Red component"]
    #[inline(always)]
    pub fn red(&self) -> RED_R {
        RED_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Blue component"]
    #[inline(always)]
    #[must_use]
    pub fn blue(&mut self) -> BLUE_W<0> {
        BLUE_W::new(self)
    }
    #[doc = "Bits 10:19 - Green component"]
    #[inline(always)]
    #[must_use]
    pub fn green(&mut self) -> GREEN_W<10> {
        GREEN_W::new(self)
    }
    #[doc = "Bits 20:29 - Red component"]
    #[inline(always)]
    #[must_use]
    pub fn red(&mut self) -> RED_W<20> {
        RED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Look Up Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut](index.html) module"]
pub struct LUT_SPEC;
impl crate::RegisterSpec for LUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut::R](R) reader structure"]
impl crate::Readable for LUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut::W](W) writer structure"]
impl crate::Writable for LUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUT[%s]
to value 0"]
impl crate::Resettable for LUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
