#[doc = "Register `FILTER_MASK` reader"]
pub struct R(crate::R<FILTER_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER_MASK` writer"]
pub struct W(crate::W<FILTER_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_MASK_SPEC>;
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
impl From<crate::W<FILTER_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MID` reader - This field sets the filter mask for the Master ID. Only bits i for which MID\\[i\\]
is 1 are compared with FILTER.MID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT."]
pub type MID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MID` writer - This field sets the filter mask for the Master ID. Only bits i for which MID\\[i\\]
is 1 are compared with FILTER.MID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT."]
pub type MID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_MASK_SPEC, u8, u8, 4, O>;
#[doc = "Field `TID` reader - This field sets the filter for the upper transaction ID. Only bits i for which TID\\[i\\]
is 1 are compared with FILTER.TID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
pub type TID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TID` writer - This field sets the filter for the upper transaction ID. Only bits i for which TID\\[i\\]
is 1 are compared with FILTER.TID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
pub type TID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_MASK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - This field sets the filter mask for the Master ID. Only bits i for which MID\\[i\\]
is 1 are compared with FILTER.MID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT."]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - This field sets the filter for the upper transaction ID. Only bits i for which TID\\[i\\]
is 1 are compared with FILTER.TID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field sets the filter mask for the Master ID. Only bits i for which MID\\[i\\]
is 1 are compared with FILTER.MID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT."]
    #[inline(always)]
    #[must_use]
    pub fn mid(&mut self) -> MID_W<0> {
        MID_W::new(self)
    }
    #[doc = "Bits 8:15 - This field sets the filter for the upper transaction ID. Only bits i for which TID\\[i\\]
is 1 are compared with FILTER.TID\\[i\\]. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If FILTER.WRITE = 1, this field has no effect on DATA_CNT.CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<8> {
        TID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transaction filter mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter_mask](index.html) module"]
pub struct FILTER_MASK_SPEC;
impl crate::RegisterSpec for FILTER_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter_mask::R](R) reader structure"]
impl crate::Readable for FILTER_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter_mask::W](W) writer structure"]
impl crate::Writable for FILTER_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER_MASK to value 0"]
impl crate::Resettable for FILTER_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
