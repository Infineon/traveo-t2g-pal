#[doc = "Register `TR_CTL1` reader"]
pub struct R(crate::R<TR_CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR_CTL1` writer"]
pub struct W(crate::W<TR_CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_CTL1_SPEC>;
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
impl From<crate::W<TR_CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RO11_EN` reader - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
pub type RO11_EN_R = crate::BitReader<bool>;
#[doc = "Field `RO11_EN` writer - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
pub type RO11_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
#[doc = "Field `RO15_EN` reader - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
pub type RO15_EN_R = crate::BitReader<bool>;
#[doc = "Field `RO15_EN` writer - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
pub type RO15_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
#[doc = "Field `GARO15_EN` reader - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
pub type GARO15_EN_R = crate::BitReader<bool>;
#[doc = "Field `GARO15_EN` writer - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
pub type GARO15_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
#[doc = "Field `GARO31_EN` reader - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
pub type GARO31_EN_R = crate::BitReader<bool>;
#[doc = "Field `GARO31_EN` writer - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
pub type GARO31_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
#[doc = "Field `FIRO15_EN` reader - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
pub type FIRO15_EN_R = crate::BitReader<bool>;
#[doc = "Field `FIRO15_EN` writer - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
pub type FIRO15_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
#[doc = "Field `FIRO31_EN` reader - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
pub type FIRO31_EN_R = crate::BitReader<bool>;
#[doc = "Field `FIRO31_EN` writer - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
pub type FIRO31_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_CTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
    #[inline(always)]
    pub fn ro11_en(&self) -> RO11_EN_R {
        RO11_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn ro15_en(&self) -> RO15_EN_R {
        RO15_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn garo15_en(&self) -> GARO15_EN_R {
        GARO15_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    pub fn garo31_en(&self) -> GARO31_EN_R {
        GARO31_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
    #[inline(always)]
    pub fn firo15_en(&self) -> FIRO15_EN_R {
        FIRO15_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    pub fn firo31_en(&self) -> FIRO31_EN_R {
        FIRO31_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FW sets this field to '1' to enable the ring oscillator with 11 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn ro11_en(&mut self) -> RO11_EN_W<0> {
        RO11_EN_W::new(self)
    }
    #[doc = "Bit 1 - FW sets this field to '1' to enable the ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn ro15_en(&mut self) -> RO15_EN_W<1> {
        RO15_EN_W::new(self)
    }
    #[doc = "Bit 2 - FW sets this field to '1' to enable the fixed Galois ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn garo15_en(&mut self) -> GARO15_EN_W<2> {
        GARO15_EN_W::new(self)
    }
    #[doc = "Bit 3 - FW sets this field to '1' to enable the programmable Galois ring oscillator with up to 31 inverters. The TR_GARO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn garo31_en(&mut self) -> GARO31_EN_W<3> {
        GARO31_EN_W::new(self)
    }
    #[doc = "Bit 4 - FW sets this field to '1' to enable the fixed Fibonacci ring oscillator with 15 inverters."]
    #[inline(always)]
    #[must_use]
    pub fn firo15_en(&mut self) -> FIRO15_EN_W<4> {
        FIRO15_EN_W::new(self)
    }
    #[doc = "Bit 5 - FW sets this field to '1' to enable the programmable Fibonacci ring oscillator with up to 31 inverters. The TR_FIRO_CTL register specifies the programmable polynomial."]
    #[inline(always)]
    #[must_use]
    pub fn firo31_en(&mut self) -> FIRO31_EN_W<5> {
        FIRO31_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "True random control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr_ctl1](index.html) module"]
pub struct TR_CTL1_SPEC;
impl crate::RegisterSpec for TR_CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr_ctl1::R](R) reader structure"]
impl crate::Readable for TR_CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr_ctl1::W](W) writer structure"]
impl crate::Writable for TR_CTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TR_CTL1 to value 0"]
impl crate::Resettable for TR_CTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
