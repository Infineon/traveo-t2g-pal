#[doc = "Register `STATICCONTROL` reader"]
pub struct R(crate::R<STATICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATICCONTROL` writer"]
pub struct W(crate::W<STATICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATICCONTROL_SPEC>;
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
impl From<crate::W<STATICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHDEN` reader - Enables shadow registers for RWS type fields (0 = write through, 1 = shadowed)."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enables shadow registers for RWS type fields (0 = write through, 1 = shadowed)."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `SHDLDSEL` reader - Source select for events that will load shadow registers into the active configuration."]
pub type SHDLDSEL_R = crate::BitReader<SHDLDSEL_A>;
#[doc = "Source select for events that will load shadow registers into the active configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHDLDSEL_A {
    #[doc = "0: Shadows are loaded at start of frame for each evaluation window for which ShdLdReq has been set."]
    LOCAL = 0,
    #[doc = "1: Shadows of all evaluation windows are loaded synchronous to the display stream (shadow load token received on frame input port)."]
    GLOBAL = 1,
}
impl From<SHDLDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SHDLDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SHDLDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHDLDSEL_A {
        match self.bits {
            false => SHDLDSEL_A::LOCAL,
            true => SHDLDSEL_A::GLOBAL,
        }
    }
    #[doc = "Checks if the value of the field is `LOCAL`"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == SHDLDSEL_A::LOCAL
    }
    #[doc = "Checks if the value of the field is `GLOBAL`"]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == SHDLDSEL_A::GLOBAL
    }
}
#[doc = "Field `SHDLDSEL` writer - Source select for events that will load shadow registers into the active configuration."]
pub type SHDLDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, SHDLDSEL_A, O>;
impl<'a, const O: u8> SHDLDSEL_W<'a, O> {
    #[doc = "Shadows are loaded at start of frame for each evaluation window for which ShdLdReq has been set."]
    #[inline(always)]
    pub fn local(self) -> &'a mut W {
        self.variant(SHDLDSEL_A::LOCAL)
    }
    #[doc = "Shadows of all evaluation windows are loaded synchronous to the display stream (shadow load token received on frame input port)."]
    #[inline(always)]
    pub fn global(self) -> &'a mut W {
        self.variant(SHDLDSEL_A::GLOBAL)
    }
}
#[doc = "Field `ERRTHRES` reader - Number of frames with signature violation before StsSigError is set for an evaluation window. These frames do not need to be consecutive. 0 means that each frame with violation will set the status. The internal counter compared against this threshold is reset when 'ErrThreshReset' consecutive frames without signature violation occur."]
pub type ERRTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRTHRES` writer - Number of frames with signature violation before StsSigError is set for an evaluation window. These frames do not need to be consecutive. 0 means that each frame with violation will set the status. The internal counter compared against this threshold is reset when 'ErrThreshReset' consecutive frames without signature violation occur."]
pub type ERRTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, u8, 8, O>;
#[doc = "Field `ERRTHRESRESET` reader - ErrThresReset must be programmed to value less than 255. Number of consecutive frames without signature violation before StsSigError is reset for an evaluation window. 0 means that each frame without violation will reset the status. The internal counter compared against this threshold is reset with each frame that has violation."]
pub type ERRTHRESRESET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRTHRESRESET` writer - ErrThresReset must be programmed to value less than 255. Number of consecutive frames without signature violation before StsSigError is reset for an evaluation window. 0 means that each frame without violation will reset the status. The internal counter compared against this threshold is reset with each frame that has violation."]
pub type ERRTHRESRESET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables shadow registers for RWS type fields (0 = write through, 1 = shadowed)."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Source select for events that will load shadow registers into the active configuration."]
    #[inline(always)]
    pub fn shdldsel(&self) -> SHDLDSEL_R {
        SHDLDSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of frames with signature violation before StsSigError is set for an evaluation window. These frames do not need to be consecutive. 0 means that each frame with violation will set the status. The internal counter compared against this threshold is reset when 'ErrThreshReset' consecutive frames without signature violation occur."]
    #[inline(always)]
    pub fn errthres(&self) -> ERRTHRES_R {
        ERRTHRES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ErrThresReset must be programmed to value less than 255. Number of consecutive frames without signature violation before StsSigError is reset for an evaluation window. 0 means that each frame without violation will reset the status. The internal counter compared against this threshold is reset with each frame that has violation."]
    #[inline(always)]
    pub fn errthresreset(&self) -> ERRTHRESRESET_R {
        ERRTHRESRESET_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enables shadow registers for RWS type fields (0 = write through, 1 = shadowed)."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bit 4 - Source select for events that will load shadow registers into the active configuration."]
    #[inline(always)]
    #[must_use]
    pub fn shdldsel(&mut self) -> SHDLDSEL_W<4> {
        SHDLDSEL_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of frames with signature violation before StsSigError is set for an evaluation window. These frames do not need to be consecutive. 0 means that each frame with violation will set the status. The internal counter compared against this threshold is reset when 'ErrThreshReset' consecutive frames without signature violation occur."]
    #[inline(always)]
    #[must_use]
    pub fn errthres(&mut self) -> ERRTHRES_W<16> {
        ERRTHRES_W::new(self)
    }
    #[doc = "Bits 24:31 - ErrThresReset must be programmed to value less than 255. Number of consecutive frames without signature violation before StsSigError is reset for an evaluation window. 0 means that each frame without violation will reset the status. The internal counter compared against this threshold is reset with each frame that has violation."]
    #[inline(always)]
    #[must_use]
    pub fn errthresreset(&mut self) -> ERRTHRESRESET_W<24> {
        ERRTHRESRESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global configuration shared by all evaluation windows.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
pub struct STATICCONTROL_SPEC;
impl crate::RegisterSpec for STATICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [staticcontrol::R](R) reader structure"]
impl crate::Readable for STATICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [staticcontrol::W](W) writer structure"]
impl crate::Writable for STATICCONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATICCONTROL to value 0x0800_0000"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800_0000;
}
