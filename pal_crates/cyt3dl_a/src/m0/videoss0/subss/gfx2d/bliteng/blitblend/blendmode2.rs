#[doc = "Register `BLENDMODE2` reader"]
pub struct R(crate::R<BLENDMODE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLENDMODE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLENDMODE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLENDMODE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLENDMODE2` writer"]
pub struct W(crate::W<BLENDMODE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLENDMODE2_SPEC>;
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
impl From<crate::W<BLENDMODE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLENDMODE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLENDMODECOLORBLUE` reader - Blue component blend mode"]
pub type BLENDMODECOLORBLUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDMODECOLORBLUE` writer - Blue component blend mode"]
pub type BLENDMODECOLORBLUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE2_SPEC, u16, u16, 16, O>;
#[doc = "Field `BLENDMODEALPHA` reader - Alpha component blend mode"]
pub type BLENDMODEALPHA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLENDMODEALPHA` writer - Alpha component blend mode"]
pub type BLENDMODEALPHA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLENDMODE2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Blue component blend mode"]
    #[inline(always)]
    pub fn blendmodecolorblue(&self) -> BLENDMODECOLORBLUE_R {
        BLENDMODECOLORBLUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Alpha component blend mode"]
    #[inline(always)]
    pub fn blendmodealpha(&self) -> BLENDMODEALPHA_R {
        BLENDMODEALPHA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Blue component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodecolorblue(&mut self) -> BLENDMODECOLORBLUE_W<0> {
        BLENDMODECOLORBLUE_W::new(self)
    }
    #[doc = "Bits 16:31 - Alpha component blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn blendmodealpha(&mut self) -> BLENDMODEALPHA_W<16> {
        BLENDMODEALPHA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Open GL and Open VG blending modes for color blue and alpha\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blendmode2](index.html) module"]
pub struct BLENDMODE2_SPEC;
impl crate::RegisterSpec for BLENDMODE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blendmode2::R](R) reader structure"]
impl crate::Readable for BLENDMODE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blendmode2::W](W) writer structure"]
impl crate::Writable for BLENDMODE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLENDMODE2 to value 0x8006_8006"]
impl crate::Resettable for BLENDMODE2_SPEC {
    const RESET_VALUE: Self::Ux = 0x8006_8006;
}
