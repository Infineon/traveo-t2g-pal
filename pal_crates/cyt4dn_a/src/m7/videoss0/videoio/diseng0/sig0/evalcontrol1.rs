#[doc = "Register `EVALCONTROL1` reader"]
pub struct R(crate::R<EVALCONTROL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL1` writer"]
pub struct W(crate::W<EVALCONTROL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL1_SPEC>;
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
impl From<crate::W<EVALCONTROL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN1` reader - See EnEvalWin0."]
pub type ENEVALWIN1_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN1` writer - See EnEvalWin0."]
pub type ENEVALWIN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
#[doc = "Field `ENCRC1` reader - See EnCRC0."]
pub type ENCRC1_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC1` writer - See EnCRC0."]
pub type ENCRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK1` reader - See AlphaMask0."]
pub type ALPHAMASK1_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK1` writer - See AlphaMask0."]
pub type ALPHAMASK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
#[doc = "Field `ALPHAINV1` reader - See AlphaInv0."]
pub type ALPHAINV1_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV1` writer - See AlphaInv0."]
pub type ALPHAINV1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC1` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC1_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC1` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC1` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC1_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC1` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin1(&self) -> ENEVALWIN1_R {
        ENEVALWIN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc1(&self) -> ENCRC1_R {
        ENCRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask1(&self) -> ALPHAMASK1_R {
        ALPHAMASK1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv1(&self) -> ALPHAINV1_R {
        ALPHAINV1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic1(&self) -> ENLOCALPANIC1_R {
        ENLOCALPANIC1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic1(&self) -> ENGLOBALPANIC1_R {
        ENGLOBALPANIC1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin1(&mut self) -> ENEVALWIN1_W<0> {
        ENEVALWIN1_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc1(&mut self) -> ENCRC1_W<1> {
        ENCRC1_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask1(&mut self) -> ALPHAMASK1_W<8> {
        ALPHAMASK1_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv1(&mut self) -> ALPHAINV1_W<9> {
        ALPHAINV1_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic1(&mut self) -> ENLOCALPANIC1_W<16> {
        ENLOCALPANIC1_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic1(&mut self) -> ENGLOBALPANIC1_W<17> {
        ENGLOBALPANIC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol1](index.html) module"]
pub struct EVALCONTROL1_SPEC;
impl crate::RegisterSpec for EVALCONTROL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol1::R](R) reader structure"]
impl crate::Readable for EVALCONTROL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol1::W](W) writer structure"]
impl crate::Writable for EVALCONTROL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL1 to value 0"]
impl crate::Resettable for EVALCONTROL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
