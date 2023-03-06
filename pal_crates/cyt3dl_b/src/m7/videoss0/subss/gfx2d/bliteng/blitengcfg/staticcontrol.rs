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
#[doc = "Field `SHDEN` reader - Enable shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
pub type SHDEN_R = crate::BitReader<bool>;
#[doc = "Field `SHDEN` writer - Enable shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
pub type SHDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `LRSTATUSSELECT` reader - Select pipeline from which to access status register information in line-rendering mode."]
pub type LRSTATUSSELECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LRSTATUSSELECT` writer - Select pipeline from which to access status register information in line-rendering mode."]
pub type LRSTATUSSELECT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATICCONTROL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BLITENGUTILIZATIONMEASUREMENTEN` reader - Enable utilization measurement of LBO render mode. Corresponding registers are IBOCounter and LBOCounter."]
pub type BLITENGUTILIZATIONMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `BLITENGUTILIZATIONMEASUREMENTEN` writer - Enable utilization measurement of LBO render mode. Corresponding registers are IBOCounter and LBOCounter."]
pub type BLITENGUTILIZATIONMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `LBOUTILIZATIONMEASUREMENTEN` reader - Enable utilization measurement of LBO render mode. Corresponding registers are LBOAllFetchesActiveCounter and LBOWaitBlitEngRessourcesCounter."]
pub type LBOUTILIZATIONMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `LBOUTILIZATIONMEASUREMENTEN` writer - Enable utilization measurement of LBO render mode. Corresponding registers are LBOAllFetchesActiveCounter and LBOWaitBlitEngRessourcesCounter."]
pub type LBOUTILIZATIONMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `TBUTILIZATIONMEASUREMENTEN` reader - Enable utilization measurement of CmdSeq task buffers (active time for each TB). Corresponding registers are TB\\[x\\]UtilizationCounter."]
pub type TBUTILIZATIONMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `TBUTILIZATIONMEASUREMENTEN` writer - Enable utilization measurement of CmdSeq task buffers (active time for each TB). Corresponding registers are TB\\[x\\]UtilizationCounter."]
pub type TBUTILIZATIONMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `LBOSOURCEPIXELMEASUREMENTEN` reader - Enable source pixel counting for LBO. Corresponding registers are LBOSourcePixelRGBACounter and LBOSourcePixelAlphaCounter."]
pub type LBOSOURCEPIXELMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `LBOSOURCEPIXELMEASUREMENTEN` writer - Enable source pixel counting for LBO. Corresponding registers are LBOSourcePixelRGBACounter and LBOSourcePixelAlphaCounter."]
pub type LBOSOURCEPIXELMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
#[doc = "Field `DESTINATIONPIXELMEASUREMENTEN` reader - Enable destination pixel counting for LBO and LBO. Corresponding registers are LBODestinationPixelCounter and IBODestinationPixelCounter."]
pub type DESTINATIONPIXELMEASUREMENTEN_R = crate::BitReader<bool>;
#[doc = "Field `DESTINATIONPIXELMEASUREMENTEN` writer - Enable destination pixel counting for LBO and LBO. Corresponding registers are LBODestinationPixelCounter and IBODestinationPixelCounter."]
pub type DESTINATIONPIXELMEASUREMENTEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, STATICCONTROL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
    #[inline(always)]
    pub fn shden(&self) -> SHDEN_R {
        SHDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select pipeline from which to access status register information in line-rendering mode."]
    #[inline(always)]
    pub fn lrstatusselect(&self) -> LRSTATUSSELECT_R {
        LRSTATUSSELECT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Enable utilization measurement of LBO render mode. Corresponding registers are IBOCounter and LBOCounter."]
    #[inline(always)]
    pub fn blitengutilizationmeasurementen(&self) -> BLITENGUTILIZATIONMEASUREMENTEN_R {
        BLITENGUTILIZATIONMEASUREMENTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable utilization measurement of LBO render mode. Corresponding registers are LBOAllFetchesActiveCounter and LBOWaitBlitEngRessourcesCounter."]
    #[inline(always)]
    pub fn lboutilizationmeasurementen(&self) -> LBOUTILIZATIONMEASUREMENTEN_R {
        LBOUTILIZATIONMEASUREMENTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable utilization measurement of CmdSeq task buffers (active time for each TB). Corresponding registers are TB\\[x\\]UtilizationCounter."]
    #[inline(always)]
    pub fn tbutilizationmeasurementen(&self) -> TBUTILIZATIONMEASUREMENTEN_R {
        TBUTILIZATIONMEASUREMENTEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable source pixel counting for LBO. Corresponding registers are LBOSourcePixelRGBACounter and LBOSourcePixelAlphaCounter."]
    #[inline(always)]
    pub fn lbosourcepixelmeasurementen(&self) -> LBOSOURCEPIXELMEASUREMENTEN_R {
        LBOSOURCEPIXELMEASUREMENTEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable destination pixel counting for LBO and LBO. Corresponding registers are LBODestinationPixelCounter and IBODestinationPixelCounter."]
    #[inline(always)]
    pub fn destinationpixelmeasurementen(&self) -> DESTINATIONPIXELMEASUREMENTEN_R {
        DESTINATIONPIXELMEASUREMENTEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable shadowing of all RWS type registers (0=write_through, 1=shadowed). For LBO this ShdEn and ShdEn fields of all modules inside Bliteng have to be 1."]
    #[inline(always)]
    #[must_use]
    pub fn shden(&mut self) -> SHDEN_W<0> {
        SHDEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Select pipeline from which to access status register information in line-rendering mode."]
    #[inline(always)]
    #[must_use]
    pub fn lrstatusselect(&mut self) -> LRSTATUSSELECT_W<4> {
        LRSTATUSSELECT_W::new(self)
    }
    #[doc = "Bit 8 - Enable utilization measurement of LBO render mode. Corresponding registers are IBOCounter and LBOCounter."]
    #[inline(always)]
    #[must_use]
    pub fn blitengutilizationmeasurementen(&mut self) -> BLITENGUTILIZATIONMEASUREMENTEN_W<8> {
        BLITENGUTILIZATIONMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bit 9 - Enable utilization measurement of LBO render mode. Corresponding registers are LBOAllFetchesActiveCounter and LBOWaitBlitEngRessourcesCounter."]
    #[inline(always)]
    #[must_use]
    pub fn lboutilizationmeasurementen(&mut self) -> LBOUTILIZATIONMEASUREMENTEN_W<9> {
        LBOUTILIZATIONMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bit 10 - Enable utilization measurement of CmdSeq task buffers (active time for each TB). Corresponding registers are TB\\[x\\]UtilizationCounter."]
    #[inline(always)]
    #[must_use]
    pub fn tbutilizationmeasurementen(&mut self) -> TBUTILIZATIONMEASUREMENTEN_W<10> {
        TBUTILIZATIONMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bit 11 - Enable source pixel counting for LBO. Corresponding registers are LBOSourcePixelRGBACounter and LBOSourcePixelAlphaCounter."]
    #[inline(always)]
    #[must_use]
    pub fn lbosourcepixelmeasurementen(&mut self) -> LBOSOURCEPIXELMEASUREMENTEN_W<11> {
        LBOSOURCEPIXELMEASUREMENTEN_W::new(self)
    }
    #[doc = "Bit 12 - Enable destination pixel counting for LBO and LBO. Corresponding registers are LBODestinationPixelCounter and IBODestinationPixelCounter."]
    #[inline(always)]
    #[must_use]
    pub fn destinationpixelmeasurementen(&mut self) -> DESTINATIONPIXELMEASUREMENTEN_W<12> {
        DESTINATIONPIXELMEASUREMENTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Static control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [staticcontrol](index.html) module"]
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
#[doc = "`reset()` method sets STATICCONTROL to value 0"]
impl crate::Resettable for STATICCONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
