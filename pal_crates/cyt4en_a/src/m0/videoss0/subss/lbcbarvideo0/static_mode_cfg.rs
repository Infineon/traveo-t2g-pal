#[doc = "Register `STATIC_MODE_CFG` reader"]
pub struct R(crate::R<STATIC_MODE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATIC_MODE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATIC_MODE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATIC_MODE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATIC_MODE_CFG` writer"]
pub struct W(crate::W<STATIC_MODE_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATIC_MODE_CFG_SPEC>;
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
impl From<crate::W<STATIC_MODE_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATIC_MODE_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLV0_MST` reader - Destination for requests received on slave port 0; value is a master port ID."]
pub type SLV0_MST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV0_MST` writer - Destination for requests received on slave port 0; value is a master port ID."]
pub type SLV0_MST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATIC_MODE_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLV1_MST` reader - Destination for requests received on slave port 1; value is a master port ID."]
pub type SLV1_MST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV1_MST` writer - Destination for requests received on slave port 1; value is a master port ID."]
pub type SLV1_MST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATIC_MODE_CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SLV2_MST` reader - Destination for requests received on slave port 2; value is a master port ID."]
pub type SLV2_MST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLV2_MST` writer - Destination for requests received on slave port 2; value is a master port ID."]
pub type SLV2_MST_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STATIC_MODE_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Destination for requests received on slave port 0; value is a master port ID."]
    #[inline(always)]
    pub fn slv0_mst(&self) -> SLV0_MST_R {
        SLV0_MST_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Destination for requests received on slave port 1; value is a master port ID."]
    #[inline(always)]
    pub fn slv1_mst(&self) -> SLV1_MST_R {
        SLV1_MST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Destination for requests received on slave port 2; value is a master port ID."]
    #[inline(always)]
    pub fn slv2_mst(&self) -> SLV2_MST_R {
        SLV2_MST_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Destination for requests received on slave port 0; value is a master port ID."]
    #[inline(always)]
    #[must_use]
    pub fn slv0_mst(&mut self) -> SLV0_MST_W<0> {
        SLV0_MST_W::new(self)
    }
    #[doc = "Bits 4:5 - Destination for requests received on slave port 1; value is a master port ID."]
    #[inline(always)]
    #[must_use]
    pub fn slv1_mst(&mut self) -> SLV1_MST_W<4> {
        SLV1_MST_W::new(self)
    }
    #[doc = "Bits 8:9 - Destination for requests received on slave port 2; value is a master port ID."]
    #[inline(always)]
    #[must_use]
    pub fn slv2_mst(&mut self) -> SLV2_MST_W<8> {
        SLV2_MST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for static arbitration mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [static_mode_cfg](index.html) module"]
pub struct STATIC_MODE_CFG_SPEC;
impl crate::RegisterSpec for STATIC_MODE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [static_mode_cfg::R](R) reader structure"]
impl crate::Readable for STATIC_MODE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [static_mode_cfg::W](W) writer structure"]
impl crate::Writable for STATIC_MODE_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATIC_MODE_CFG to value 0x0210"]
impl crate::Resettable for STATIC_MODE_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210;
}
