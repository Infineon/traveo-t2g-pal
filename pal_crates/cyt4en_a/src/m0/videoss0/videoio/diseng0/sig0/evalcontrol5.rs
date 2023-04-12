#[doc = "Register `EVALCONTROL5` reader"]
pub struct R(crate::R<EVALCONTROL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCONTROL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCONTROL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCONTROL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCONTROL5` writer"]
pub struct W(crate::W<EVALCONTROL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCONTROL5_SPEC>;
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
impl From<crate::W<EVALCONTROL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCONTROL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEVALWIN5` reader - See EnEvalWin0."]
pub type ENEVALWIN5_R = crate::BitReader<bool>;
#[doc = "Field `ENEVALWIN5` writer - See EnEvalWin0."]
pub type ENEVALWIN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
#[doc = "Field `ENCRC5` reader - See EnCRC0."]
pub type ENCRC5_R = crate::BitReader<bool>;
#[doc = "Field `ENCRC5` writer - See EnCRC0."]
pub type ENCRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
#[doc = "Field `ALPHAMASK5` reader - See AlphaMask0."]
pub type ALPHAMASK5_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAMASK5` writer - See AlphaMask0."]
pub type ALPHAMASK5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
#[doc = "Field `ALPHAINV5` reader - See AlphaInv0."]
pub type ALPHAINV5_R = crate::BitReader<bool>;
#[doc = "Field `ALPHAINV5` writer - See AlphaInv0."]
pub type ALPHAINV5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
#[doc = "Field `ENLOCALPANIC5` reader - See EnLocalPanic0."]
pub type ENLOCALPANIC5_R = crate::BitReader<bool>;
#[doc = "Field `ENLOCALPANIC5` writer - See EnLocalPanic0."]
pub type ENLOCALPANIC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
#[doc = "Field `ENGLOBALPANIC5` reader - See EnGlobalPanic0."]
pub type ENGLOBALPANIC5_R = crate::BitReader<bool>;
#[doc = "Field `ENGLOBALPANIC5` writer - See EnGlobalPanic0."]
pub type ENGLOBALPANIC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVALCONTROL5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    pub fn enevalwin5(&self) -> ENEVALWIN5_R {
        ENEVALWIN5_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    pub fn encrc5(&self) -> ENCRC5_R {
        ENCRC5_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    pub fn alphamask5(&self) -> ALPHAMASK5_R {
        ALPHAMASK5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    pub fn alphainv5(&self) -> ALPHAINV5_R {
        ALPHAINV5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    pub fn enlocalpanic5(&self) -> ENLOCALPANIC5_R {
        ENLOCALPANIC5_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    pub fn englobalpanic5(&self) -> ENGLOBALPANIC5_R {
        ENGLOBALPANIC5_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See EnEvalWin0."]
    #[inline(always)]
    #[must_use]
    pub fn enevalwin5(&mut self) -> ENEVALWIN5_W<0> {
        ENEVALWIN5_W::new(self)
    }
    #[doc = "Bit 1 - See EnCRC0."]
    #[inline(always)]
    #[must_use]
    pub fn encrc5(&mut self) -> ENCRC5_W<1> {
        ENCRC5_W::new(self)
    }
    #[doc = "Bit 8 - See AlphaMask0."]
    #[inline(always)]
    #[must_use]
    pub fn alphamask5(&mut self) -> ALPHAMASK5_W<8> {
        ALPHAMASK5_W::new(self)
    }
    #[doc = "Bit 9 - See AlphaInv0."]
    #[inline(always)]
    #[must_use]
    pub fn alphainv5(&mut self) -> ALPHAINV5_W<9> {
        ALPHAINV5_W::new(self)
    }
    #[doc = "Bit 16 - See EnLocalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn enlocalpanic5(&mut self) -> ENLOCALPANIC5_W<16> {
        ENLOCALPANIC5_W::new(self)
    }
    #[doc = "Bit 17 - See EnGlobalPanic0."]
    #[inline(always)]
    #[must_use]
    pub fn englobalpanic5(&mut self) -> ENGLOBALPANIC5_W<17> {
        ENGLOBALPANIC5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control settings for evaluation window 5.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalcontrol5](index.html) module"]
pub struct EVALCONTROL5_SPEC;
impl crate::RegisterSpec for EVALCONTROL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalcontrol5::R](R) reader structure"]
impl crate::Readable for EVALCONTROL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalcontrol5::W](W) writer structure"]
impl crate::Writable for EVALCONTROL5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVALCONTROL5 to value 0"]
impl crate::Resettable for EVALCONTROL5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
