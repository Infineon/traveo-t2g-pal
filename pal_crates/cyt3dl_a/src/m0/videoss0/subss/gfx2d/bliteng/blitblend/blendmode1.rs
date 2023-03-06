#[doc = "Register `BLENDMODE1` reader"]
pub struct R(crate::R<BLENDMODE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLENDMODE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLENDMODE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLENDMODE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLENDMODE1` writer"]
pub struct W(crate::W<BLENDMODE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLENDMODE1_SPEC>;
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
impl From<crate::W<BLENDMODE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLENDMODE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDMODECOLORRED` reader - Red component blend mode"]
pub type BLENDMODECOLORRED_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDMODECOLORRED` writer - Red component blend mode"]
pub type BLENDMODECOLORRED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE1_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDMODECOLORGREEN` reader - Green component blend mode"]
pub type BLENDMODECOLORGREEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDMODECOLORGREEN` writer - Green component blend mode"]
pub type BLENDMODECOLORGREEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Red component blend mode"]
    #[inline(always)]
    pub fn blendmodecolorred(&self) -> BLENDMODECOLORRED_R {
        BLENDMODECOLORRED_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Green component blend mode"]
    #[inline(always)]
    pub fn blendmodecolorgreen(&self) -> BLENDMODECOLORGREEN_R {
        BLENDMODECOLORGREEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Red component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodecolorred(&mut self) -> BLENDMODECOLORRED_W<0> {
        BLENDMODECOLORRED_W::new(self)
    }
    #[doc = "Bits 16:31 - Green component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodecolorgreen(&mut self) -> BLENDMODECOLORGREEN_W<16> {
        BLENDMODECOLORGREEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL and Open VG blending modes for colors red and green\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blendmode1](index.html) module"]
pub struct BLENDMODE1_SPEC;
impl crate::RegisterSpec for BLENDMODE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blendmode1::R](R) reader structure"]
impl crate::Readable for BLENDMODE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blendmode1::W](W) writer structure"]
impl crate::Writable for BLENDMODE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLENDMODE1 to value 0x8006_8006"]
impl crate::Resettable for BLENDMODE1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8006_8006;
}
