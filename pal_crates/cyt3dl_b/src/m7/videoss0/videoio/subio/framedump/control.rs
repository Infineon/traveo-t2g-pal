#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUTSELECT` reader - Index of display stream that is dumped."]
pub type INPUTSELECT_R = crate::BitReader<bool>;
#[doc = "Field `INPUTSELECT` writer - Index of display stream that is dumped."]
pub type INPUTSELECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `SETFIELD` reader - The field property of the generated frame is set to this value. If field toggle is used, this is the start value."]
pub type SETFIELD_R = crate::BitReader<bool>;
#[doc = "Field `SETFIELD` writer - The field property of the generated frame is set to this value. If field toggle is used, this is the start value."]
pub type SETFIELD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
#[doc = "Field `TOGGLEFIELD` reader - Enable toggling of the field property from frame to frame. Set to '1' to enable."]
pub type TOGGLEFIELD_R = crate::BitReader<bool>;
#[doc = "Field `TOGGLEFIELD` writer - Enable toggling of the field property from frame to frame. Set to '1' to enable."]
pub type TOGGLEFIELD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Index of display stream that is dumped."]
    #[inline(always)]
    pub fn inputselect(&self) -> INPUTSELECT_R {
        INPUTSELECT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 24 - The field property of the generated frame is set to this value. If field toggle is used, this is the start value."]
    #[inline(always)]
    pub fn setfield(&self) -> SETFIELD_R {
        SETFIELD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable toggling of the field property from frame to frame. Set to '1' to enable."]
    #[inline(always)]
    pub fn togglefield(&self) -> TOGGLEFIELD_R {
        TOGGLEFIELD_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index of display stream that is dumped."]
    #[inline(always)]
    #[must_use]
    pub fn inputselect(&mut self) -> INPUTSELECT_W<0> {
        INPUTSELECT_W::new(self)
    }
    #[doc = "Bit 24 - The field property of the generated frame is set to this value. If field toggle is used, this is the start value."]
    #[inline(always)]
    #[must_use]
    pub fn setfield(&mut self) -> SETFIELD_W<24> {
        SETFIELD_W::new(self)
    }
    #[doc = "Bit 25 - Enable toggling of the field property from frame to frame. Set to '1' to enable."]
    #[inline(always)]
    #[must_use]
    pub fn togglefield(&mut self) -> TOGGLEFIELD_W<25> {
        TOGGLEFIELD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
