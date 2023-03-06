#[doc = "Register `PANICCOLOR` reader"]
pub struct R(crate::R<PANICCOLOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PANICCOLOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PANICCOLOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PANICCOLOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PANICCOLOR` writer"]
pub struct W(crate::W<PANICCOLOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PANICCOLOR_SPEC>;
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
impl From<crate::W<PANICCOLOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PANICCOLOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PANICALPHA` reader - Alpha mask bit."]
pub type PANICALPHA_R = crate::BitReader<bool>;
#[doc = "Field `PANICALPHA` writer - Alpha mask bit."]
pub type PANICALPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PANICCOLOR_SPEC, bool, O>;
#[doc = "Field `PANICBLUE` reader - Blue color component."]
pub type PANICBLUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PANICBLUE` writer - Blue color component."]
pub type PANICBLUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PANICCOLOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PANICGREEN` reader - Green color component."]
pub type PANICGREEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PANICGREEN` writer - Green color component."]
pub type PANICGREEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PANICCOLOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PANICRED` reader - Red color component."]
pub type PANICRED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PANICRED` writer - Red color component."]
pub type PANICRED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PANICCOLOR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 7 - Alpha mask bit."]
    #[inline(always)]
    pub fn panicalpha(&self) -> PANICALPHA_R {
        PANICALPHA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Blue color component."]
    #[inline(always)]
    pub fn panicblue(&self) -> PANICBLUE_R {
        PANICBLUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Green color component."]
    #[inline(always)]
    pub fn panicgreen(&self) -> PANICGREEN_R {
        PANICGREEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Red color component."]
    #[inline(always)]
    pub fn panicred(&self) -> PANICRED_R {
        PANICRED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Alpha mask bit."]
    #[inline(always)]
    #[must_use]
    pub fn panicalpha(&mut self) -> PANICALPHA_W<7> {
        PANICALPHA_W::new(self)
    }
    #[doc = "Bits 8:15 - Blue color component."]
    #[inline(always)]
    #[must_use]
    pub fn panicblue(&mut self) -> PANICBLUE_W<8> {
        PANICBLUE_W::new(self)
    }
    #[doc = "Bits 16:23 - Green color component."]
    #[inline(always)]
    #[must_use]
    pub fn panicgreen(&mut self) -> PANICGREEN_W<16> {
        PANICGREEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Red color component."]
    #[inline(always)]
    #[must_use]
    pub fn panicred(&mut self) -> PANICRED_W<24> {
        PANICRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Overlay color for evaluation windows in panic mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paniccolor](index.html) module"]
pub struct PANICCOLOR_SPEC;
impl crate::RegisterSpec for PANICCOLOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paniccolor::R](R) reader structure"]
impl crate::Readable for PANICCOLOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paniccolor::W](W) writer structure"]
impl crate::Writable for PANICCOLOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PANICCOLOR to value 0"]
impl crate::Resettable for PANICCOLOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
