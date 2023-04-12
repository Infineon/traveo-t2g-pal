#[doc = "Register `POSITION` reader"]
pub struct R(crate::R<POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POSITION` writer"]
pub struct W(crate::W<POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSITION_SPEC>;
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
impl From<crate::W<POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XPOS` reader - horizontal position, first pixel is at 0, format s15 (twos complement)"]
pub type XPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XPOS` writer - horizontal position, first pixel is at 0, format s15 (twos complement)"]
pub type XPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POSITION_SPEC, u16, u16, 16, O>;
#[doc = "Field `YPOS` reader - vertical position, first pixel is at 0, format s15 (twos complement)"]
pub type YPOS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `YPOS` writer - vertical position, first pixel is at 0, format s15 (twos complement)"]
pub type YPOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POSITION_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - horizontal position, first pixel is at 0, format s15 (twos complement)"]
    #[inline(always)]
    pub fn xpos(&self) -> XPOS_R {
        XPOS_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - vertical position, first pixel is at 0, format s15 (twos complement)"]
    #[inline(always)]
    pub fn ypos(&self) -> YPOS_R {
        YPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - horizontal position, first pixel is at 0, format s15 (twos complement)"]
    #[inline(always)]
    #[must_use]
    pub fn xpos(&mut self) -> XPOS_W<0> {
        XPOS_W::new(self)
    }
    #[doc = "Bits 16:31 - vertical position, first pixel is at 0, format s15 (twos complement)"]
    #[inline(always)]
    #[must_use]
    pub fn ypos(&mut self) -> YPOS_W<16> {
        YPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Position of secondary (overlay) input frame\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](index.html) module"]
pub struct POSITION_SPEC;
impl crate::RegisterSpec for POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [position::R](R) reader structure"]
impl crate::Readable for POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [position::W](W) writer structure"]
impl crate::Writable for POSITION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for POSITION_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
