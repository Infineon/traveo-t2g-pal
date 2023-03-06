#[doc = "Register `EVALCONTROL2` reader"]
pub struct R(crate::R<EVALCONTROL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL2` writer"]
pub struct W(crate::W<EVALCONTROL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL2_SPEC>;
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
impl From<crate::W<EVALCONTROL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN2` reader - See EnEvalWin0."]
pub type ENEVALWIN2_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN2` writer - See EnEvalWin0."]
pub type ENEVALWIN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
#[doc = "Field `ENCRC2` reader - See EnCRC0."]
pub type ENCRC2_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC2` writer - See EnCRC0."]
pub type ENCRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK2` reader - See AlphaMask0."]
pub type ALPHAMASK2_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK2` writer - See AlphaMask0."]
pub type ALPHAMASK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
#[doc = "Field `ALPHAINV2` reader - See AlphaInv0."]
pub type ALPHAINV2_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV2` writer - See AlphaInv0."]
pub type ALPHAINV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC2` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC2_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC2` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC2` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC2_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC2` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin2(&self) -> ENEVALWIN2_R {
        ENEVALWIN2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc2(&self) -> ENCRC2_R {
        ENCRC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask2(&self) -> ALPHAMASK2_R {
        ALPHAMASK2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv2(&self) -> ALPHAINV2_R {
        ALPHAINV2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic2(&self) -> ENLOCALPANIC2_R {
        ENLOCALPANIC2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic2(&self) -> ENGLOBALPANIC2_R {
        ENGLOBALPANIC2_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin2(&mut self) -> ENEVALWIN2_W<0> {
        ENEVALWIN2_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc2(&mut self) -> ENCRC2_W<1> {
        ENCRC2_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask2(&mut self) -> ALPHAMASK2_W<8> {
        ALPHAMASK2_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv2(&mut self) -> ALPHAINV2_W<9> {
        ALPHAINV2_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic2(&mut self) -> ENLOCALPANIC2_W<16> {
        ENLOCALPANIC2_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic2(&mut self) -> ENGLOBALPANIC2_W<17> {
        ENGLOBALPANIC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol2](index.html) module"]
pub struct EVALCONTROL2_SPEC;
impl crate::RegisterSpec for EVALCONTROL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol2::R](R) reader structure"]
impl crate::Readable for EVALCONTROL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol2::W](W) writer structure"]
impl crate::Writable for EVALCONTROL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL2 to value 0"]
impl crate::Resettable for EVALCONTROL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
