#[doc = "Register `GTUC5` reader"]
pub struct R(crate::R<GTUC5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GTUC5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GTUC5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GTUC5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GTUC5` writer"]
pub struct W(crate::W<GTUC5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GTUC5_SPEC>;
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
impl From<crate::W<GTUC5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GTUC5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCA` reader - Delay Compensation Channel A (pDelayCompensation\\[A\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
pub type DCA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCA` writer - Delay Compensation Channel A (pDelayCompensation\\[A\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
pub type DCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC5_SPEC, u8, u8, 8, O>;
#[doc = "Field `DCB` reader - Delay Compensation Channel B (pDelayCompensation\\[B\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
pub type DCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCB` writer - Delay Compensation Channel B (pDelayCompensation\\[B\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
pub type DCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC5_SPEC, u8, u8, 8, O>;
#[doc = "Field `CDD` reader - Cluster Drift Damping (pClusterDriftDamping) Configures the cluster drift damping value used in clock synchronization to minimize accumulation of rounding errors. Valid values are 0 to 20 uT."]
pub type CDD_R = crate::FieldReader<u8, CDD_A>;
#[doc = "Cluster Drift Damping (pClusterDriftDamping) Configures the cluster drift damping value used in clock synchronization to minimize accumulation of rounding errors. Valid values are 0 to 20 uT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDD_A {
    #[doc = "20: N/A"]
    MAX = 20,
}
impl From<CDD_A> for u8 {
    #[inline(always)]
    fn from(variant: CDD_A) -> Self {
        variant as _
    }
}
impl CDD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDD_A> {
        match self.bits {
            20 => Some(CDD_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == CDD_A::MAX
    }
}
#[doc = "Field `CDD` writer - Cluster Drift Damping (pClusterDriftDamping) Configures the cluster drift damping value used in clock synchronization to minimize accumulation of rounding errors. Valid values are 0 to 20 uT."]
pub type CDD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC5_SPEC, u8, CDD_A, 5, O>;
impl<'a, const O: u8> CDD_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(CDD_A::MAX)
    }
}
#[doc = "Field `DEC` reader - Decoding Correction (pDecodingCorrection) Configures the decoding correction value used to determine the primary time reference point. Valid values are 14 to 143 uT."]
pub type DEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEC` writer - Decoding Correction (pDecodingCorrection) Configures the decoding correction value used to determine the primary time reference point. Valid values are 14 to 143 uT."]
pub type DEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GTUC5_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Delay Compensation Channel A (pDelayCompensation\\[A\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
    #[inline(always)]
    pub fn dca(&self) -> DCA_R {
        DCA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Delay Compensation Channel B (pDelayCompensation\\[B\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
    #[inline(always)]
    pub fn dcb(&self) -> DCB_R {
        DCB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Cluster Drift Damping (pClusterDriftDamping) Configures the cluster drift damping value used in clock synchronization to minimize accumulation of rounding errors. Valid values are 0 to 20 uT."]
    #[inline(always)]
    pub fn cdd(&self) -> CDD_R {
        CDD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - Decoding Correction (pDecodingCorrection) Configures the decoding correction value used to determine the primary time reference point. Valid values are 14 to 143 uT."]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay Compensation Channel A (pDelayCompensation\\[A\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
    #[inline(always)]
    #[must_use]
    pub fn dca(&mut self) -> DCA_W<0> {
        DCA_W::new(self)
    }
    #[doc = "Bits 8:15 - Delay Compensation Channel B (pDelayCompensation\\[B\\]) Used to compensate for reception delays on the indicated channel. This covers assumed propagation delay up to cPropagationDelayMax for microticks in the range of 0.0125 to 0.05us. In practice, the minimum of the propagation delays of all sync nodes should be applied. Valid values are 0 to 200 uT."]
    #[inline(always)]
    #[must_use]
    pub fn dcb(&mut self) -> DCB_W<8> {
        DCB_W::new(self)
    }
    #[doc = "Bits 16:20 - Cluster Drift Damping (pClusterDriftDamping) Configures the cluster drift damping value used in clock synchronization to minimize accumulation of rounding errors. Valid values are 0 to 20 uT."]
    #[inline(always)]
    #[must_use]
    pub fn cdd(&mut self) -> CDD_W<16> {
        CDD_W::new(self)
    }
    #[doc = "Bits 24:31 - Decoding Correction (pDecodingCorrection) Configures the decoding correction value used to determine the primary time reference point. Valid values are 14 to 143 uT."]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<24> {
        DEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GTU Configuration Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gtuc5](index.html) module"]
pub struct GTUC5_SPEC;
impl crate::RegisterSpec for GTUC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gtuc5::R](R) reader structure"]
impl crate::Readable for GTUC5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gtuc5::W](W) writer structure"]
impl crate::Writable for GTUC5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GTUC5 to value 0x0e00_0000"]
impl crate::Resettable for GTUC5_SPEC {
    const RESET_VALUE: Self::Ux = 0x0e00_0000;
}
