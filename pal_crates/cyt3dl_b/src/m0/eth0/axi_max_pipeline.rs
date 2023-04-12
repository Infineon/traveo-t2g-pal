#[doc = "Register `AXI_MAX_PIPELINE` reader"]
pub struct R(crate::R<AXI_MAX_PIPELINE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXI_MAX_PIPELINE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXI_MAX_PIPELINE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXI_MAX_PIPELINE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AXI_MAX_PIPELINE` writer"]
pub struct W(crate::W<AXI_MAX_PIPELINE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AXI_MAX_PIPELINE_SPEC>;
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
impl From<crate::W<AXI_MAX_PIPELINE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AXI_MAX_PIPELINE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR2R_MAX_PIPELINE` reader - Defines the maximum number of outstanding AXI read requests that can be issued by the DMA via the AR channel."]
pub type AR2R_MAX_PIPELINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AR2R_MAX_PIPELINE` writer - Defines the maximum number of outstanding AXI read requests that can be issued by the DMA via the AR channel."]
pub type AR2R_MAX_PIPELINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_MAX_PIPELINE_SPEC, u8, u8, 8, O>;
#[doc = "Field `AW2W_MAX_PIPELINE` reader - Defines the maximum number of outstanding AXI write requests that can be issued by the DMA via the AW channel."]
pub type AW2W_MAX_PIPELINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AW2W_MAX_PIPELINE` writer - Defines the maximum number of outstanding AXI write requests that can be issued by the DMA via the AW channel."]
pub type AW2W_MAX_PIPELINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AXI_MAX_PIPELINE_SPEC, u8, u8, 8, O>;
#[doc = "Field `USE_AW2B_FILL` reader - For the write issuing capability as defined in bits 15:8 of this register, select whether the max number of transactions operates between the AW to W AXI channel or the AW to B channel. Set to 0 to operate between the AW and W channels. Set to 1 to operate between the AW and B channels."]
pub type USE_AW2B_FILL_R = crate::BitReader<bool>;
#[doc = "Field `USE_AW2B_FILL` writer - For the write issuing capability as defined in bits 15:8 of this register, select whether the max number of transactions operates between the AW to W AXI channel or the AW to B channel. Set to 0 to operate between the AW and W channels. Set to 1 to operate between the AW and B channels."]
pub type USE_AW2B_FILL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AXI_MAX_PIPELINE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Defines the maximum number of outstanding AXI read requests that can be issued by the DMA via the AR channel."]
    #[inline(always)]
    pub fn ar2r_max_pipeline(&self) -> AR2R_MAX_PIPELINE_R {
        AR2R_MAX_PIPELINE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Defines the maximum number of outstanding AXI write requests that can be issued by the DMA via the AW channel."]
    #[inline(always)]
    pub fn aw2w_max_pipeline(&self) -> AW2W_MAX_PIPELINE_R {
        AW2W_MAX_PIPELINE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - For the write issuing capability as defined in bits 15:8 of this register, select whether the max number of transactions operates between the AW to W AXI channel or the AW to B channel. Set to 0 to operate between the AW and W channels. Set to 1 to operate between the AW and B channels."]
    #[inline(always)]
    pub fn use_aw2b_fill(&self) -> USE_AW2B_FILL_R {
        USE_AW2B_FILL_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines the maximum number of outstanding AXI read requests that can be issued by the DMA via the AR channel."]
    #[inline(always)]
    #[must_use]
    pub fn ar2r_max_pipeline(&mut self) -> AR2R_MAX_PIPELINE_W<0> {
        AR2R_MAX_PIPELINE_W::new(self)
    }
    #[doc = "Bits 8:15 - Defines the maximum number of outstanding AXI write requests that can be issued by the DMA via the AW channel."]
    #[inline(always)]
    #[must_use]
    pub fn aw2w_max_pipeline(&mut self) -> AW2W_MAX_PIPELINE_W<8> {
        AW2W_MAX_PIPELINE_W::new(self)
    }
    #[doc = "Bit 16 - For the write issuing capability as defined in bits 15:8 of this register, select whether the max number of transactions operates between the AW to W AXI channel or the AW to B channel. Set to 0 to operate between the AW and W channels. Set to 1 to operate between the AW and B channels."]
    #[inline(always)]
    #[must_use]
    pub fn use_aw2b_fill(&mut self) -> USE_AW2B_FILL_W<16> {
        USE_AW2B_FILL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to set the maximum amount of outstanding transactions on the AXI bus between AR / R channels and AW / W channels. Cannot be more than the depth of the configured AXI pipeline FIFO (defined in verilog defs.v)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [axi_max_pipeline](index.html) module"]
pub struct AXI_MAX_PIPELINE_SPEC;
impl crate::RegisterSpec for AXI_MAX_PIPELINE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [axi_max_pipeline::R](R) reader structure"]
impl crate::Readable for AXI_MAX_PIPELINE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [axi_max_pipeline::W](W) writer structure"]
impl crate::Writable for AXI_MAX_PIPELINE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AXI_MAX_PIPELINE to value 0x0101"]
impl crate::Resettable for AXI_MAX_PIPELINE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
