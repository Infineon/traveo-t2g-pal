#[doc = "Register `EVALCONTROL6` reader"]
pub struct R(crate::R<EVALCONTROL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL6` writer"]
pub struct W(crate::W<EVALCONTROL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL6_SPEC>;
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
impl From<crate::W<EVALCONTROL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN6` reader - See EnEvalWin0."]
pub type ENEVALWIN6_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN6` writer - See EnEvalWin0."]
pub type ENEVALWIN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
#[doc = "Field `ENCRC6` reader - See EnCRC0."]
pub type ENCRC6_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC6` writer - See EnCRC0."]
pub type ENCRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK6` reader - See AlphaMask0."]
pub type ALPHAMASK6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK6` writer - See AlphaMask0."]
pub type ALPHAMASK6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
#[doc = "Field `ALPHAINV6` reader - See AlphaInv0."]
pub type ALPHAINV6_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV6` writer - See AlphaInv0."]
pub type ALPHAINV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC6` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC6_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC6` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC6` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC6_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC6` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin6(&self) -> ENEVALWIN6_R {
        ENEVALWIN6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc6(&self) -> ENCRC6_R {
        ENCRC6_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask6(&self) -> ALPHAMASK6_R {
        ALPHAMASK6_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv6(&self) -> ALPHAINV6_R {
        ALPHAINV6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic6(&self) -> ENLOCALPANIC6_R {
        ENLOCALPANIC6_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic6(&self) -> ENGLOBALPANIC6_R {
        ENGLOBALPANIC6_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin6(&mut self) -> ENEVALWIN6_W<0> {
        ENEVALWIN6_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc6(&mut self) -> ENCRC6_W<1> {
        ENCRC6_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask6(&mut self) -> ALPHAMASK6_W<8> {
        ALPHAMASK6_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv6(&mut self) -> ALPHAINV6_W<9> {
        ALPHAINV6_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic6(&mut self) -> ENLOCALPANIC6_W<16> {
        ENLOCALPANIC6_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic6(&mut self) -> ENGLOBALPANIC6_W<17> {
        ENGLOBALPANIC6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 6.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol6](index.html) module"]
pub struct EVALCONTROL6_SPEC;
impl crate::RegisterSpec for EVALCONTROL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol6::R](R) reader structure"]
impl crate::Readable for EVALCONTROL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol6::W](W) writer structure"]
impl crate::Writable for EVALCONTROL6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL6 to value 0"]
impl crate::Resettable for EVALCONTROL6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
