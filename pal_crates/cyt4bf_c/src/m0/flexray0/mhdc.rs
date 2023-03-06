#[doc = "Register `MHDC` reader"]
pub struct R(crate::R<MHDC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MHDC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MHDC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MHDC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MHDC` writer"]
pub struct W(crate::W<MHDC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MHDC_SPEC>;
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
impl From<crate::W<MHDC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MHDC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFDL` reader - Static Frame Data Length (gPayloadLengthStatic) Configures the cluster-wide payload length for all frames sent in the static segment in double bytes. The payload length must be identical in all nodes of a cluster. Valid values are 0 to 127."]
pub type SFDL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFDL` writer - Static Frame Data Length (gPayloadLengthStatic) Configures the cluster-wide payload length for all frames sent in the static segment in double bytes. The payload length must be identical in all nodes of a cluster. Valid values are 0 to 127."]
pub type SFDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MHDC_SPEC, u8, u8, 7, O>;
#[doc = "Field `SLT` reader - Start of Latest Transmit (pLatestTx) Configures the maximum minislot value allowed before inhibiting frame transmission in the dynamic segment of the cycle. There is no transmission in dynamic segment if SLT\\[12:0\\]
is set to zero. Valid values are 0 to 7981 minislots."]
pub type SLT_R = crate::FieldReader<u16, SLT_A>;
#[doc = "Start of Latest Transmit (pLatestTx) Configures the maximum minislot value allowed before inhibiting frame transmission in the dynamic segment of the cycle. There is no transmission in dynamic segment if SLT\\[12:0\\]
is set to zero. Valid values are 0 to 7981 minislots.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SLT_A {
    #[doc = "7981: N/A"]
    MAX = 7981,
}
impl From<SLT_A> for u16 {
    #[inline(always)]
    fn from(variant: SLT_A) -> Self {
        variant as _
    }
}
impl SLT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLT_A> {
        match self.bits {
            7981 => Some(SLT_A::MAX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAX`"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == SLT_A::MAX
    }
}
#[doc = "Field `SLT` writer - Start of Latest Transmit (pLatestTx) Configures the maximum minislot value allowed before inhibiting frame transmission in the dynamic segment of the cycle. There is no transmission in dynamic segment if SLT\\[12:0\\]
is set to zero. Valid values are 0 to 7981 minislots."]
pub type SLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MHDC_SPEC, u16, SLT_A, 13, O>;
impl<'a, const O: u8> SLT_W<'a, O> {
    #[doc = "N/A"]
    #[inline(always)]
    pub fn max(self) -> &'a mut W {
        self.variant(SLT_A::MAX)
    }
}
impl R {
    #[doc = "Bits 0:6 - Static Frame Data Length (gPayloadLengthStatic) Configures the cluster-wide payload length for all frames sent in the static segment in double bytes. The payload length must be identical in all nodes of a cluster. Valid values are 0 to 127."]
    #[inline(always)]
    pub fn sfdl(&self) -> SFDL_R {
        SFDL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:28 - Start of Latest Transmit (pLatestTx) Configures the maximum minislot value allowed before inhibiting frame transmission in the dynamic segment of the cycle. There is no transmission in dynamic segment if SLT\\[12:0\\]
is set to zero. Valid values are 0 to 7981 minislots."]
    #[inline(always)]
    pub fn slt(&self) -> SLT_R {
        SLT_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Static Frame Data Length (gPayloadLengthStatic) Configures the cluster-wide payload length for all frames sent in the static segment in double bytes. The payload length must be identical in all nodes of a cluster. Valid values are 0 to 127."]
    #[inline(always)]
    #[must_use]
    pub fn sfdl(&mut self) -> SFDL_W<0> {
        SFDL_W::new(self)
    }
    #[doc = "Bits 16:28 - Start of Latest Transmit (pLatestTx) Configures the maximum minislot value allowed before inhibiting frame transmission in the dynamic segment of the cycle. There is no transmission in dynamic segment if SLT\\[12:0\\]
is set to zero. Valid values are 0 to 7981 minislots."]
    #[inline(always)]
    #[must_use]
    pub fn slt(&mut self) -> SLT_W<16> {
        SLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MHD Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mhdc](index.html) module"]
pub struct MHDC_SPEC;
impl crate::RegisterSpec for MHDC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mhdc::R](R) reader structure"]
impl crate::Readable for MHDC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mhdc::W](W) writer structure"]
impl crate::Writable for MHDC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MHDC to value 0"]
impl crate::Resettable for MHDC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
