#[doc = "Register `FILTER` reader"]
pub struct R(crate::R<FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER` writer"]
pub struct W(crate::W<FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_SPEC>;
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
impl From<crate::W<FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MID` reader - This field sets the filter for the Master ID. Only bits i for which FILTER_MASK.MID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT."]
pub type MID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MID` writer - This field sets the filter for the Master ID. Only bits i for which FILTER_MASK.MID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT."]
pub type MID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `TID` reader - This field sets the filter for the upper transaction ID. Only bits i for which FILTER_MASK.TID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR.CNT, ADDR_STALL.CNT, DATA.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
pub type TID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TID` writer - This field sets the filter for the upper transaction ID. Only bits i for which FILTER_MASK.TID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR.CNT, ADDR_STALL.CNT, DATA.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
pub type TID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRITE` reader - This field sets the filter for the direction. If WRITE = 1, then only write transactions are counted, and if WRITE = 0, then only read transactions are counted. This field affects the following counters: OT_AC.CNT, ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - This field sets the filter for the direction. If WRITE = 1, then only write transactions are counted, and if WRITE = 0, then only read transactions are counted. This field affects the following counters: OT_AC.CNT, ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FILTER_SPEC, bool, O>;
#[doc = "Field `LEN_MIN` reader - The value of this field + 1 defines the minimum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type LEN_MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN_MIN` writer - The value of this field + 1 defines the minimum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type LEN_MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `LEN_MAX` reader - The value of this field + 1 defines the maximum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type LEN_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEN_MAX` writer - The value of this field + 1 defines the maximum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
pub type LEN_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - This field sets the filter for the Master ID. Only bits i for which FILTER_MASK.MID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT."]
    #[inline(always)]
    pub fn mid(&self) -> MID_R {
        MID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - This field sets the filter for the upper transaction ID. Only bits i for which FILTER_MASK.TID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR.CNT, ADDR_STALL.CNT, DATA.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
    #[inline(always)]
    pub fn tid(&self) -> TID_R {
        TID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 23 - This field sets the filter for the direction. If WRITE = 1, then only write transactions are counted, and if WRITE = 0, then only read transactions are counted. This field affects the following counters: OT_AC.CNT, ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - The value of this field + 1 defines the minimum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    pub fn len_min(&self) -> LEN_MIN_R {
        LEN_MIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - The value of this field + 1 defines the maximum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    pub fn len_max(&self) -> LEN_MAX_R {
        LEN_MAX_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - This field sets the filter for the Master ID. Only bits i for which FILTER_MASK.MID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT."]
    #[inline(always)]
    #[must_use]
    pub fn mid(&mut self) -> MID_W<0> {
        MID_W::new(self)
    }
    #[doc = "Bits 8:15 - This field sets the filter for the upper transaction ID. Only bits i for which FILTER_MASK.TID\\[i\\]
is 1 are compared. This field affects the following counters: ADDR.CNT, ADDR_STALL.CNT, DATA.CNT, DATA_STALL.CNT. Note: If WRITE = 1, this field has no effect on DATA_CNT and DATA_STALL_CNT. Note: TID denotes bits \\[ID_WIDTH-1:8\\]
of the transaction ID. Different ports can have different ID widths. ID_WIDTH is the ID width of the port with the largest ID width. For ports with smaller ID widths, the MSBs of TID are '0'."]
    #[inline(always)]
    #[must_use]
    pub fn tid(&mut self) -> TID_W<8> {
        TID_W::new(self)
    }
    #[doc = "Bit 23 - This field sets the filter for the direction. If WRITE = 1, then only write transactions are counted, and if WRITE = 0, then only read transactions are counted. This field affects the following counters: OT_AC.CNT, ADDR_CNT.CNT, ADDR_STALL.CNT, DATA_CNT.CNT, DATA_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<23> {
        WRITE_W::new(self)
    }
    #[doc = "Bits 24:27 - The value of this field + 1 defines the minimum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    #[must_use]
    pub fn len_min(&mut self) -> LEN_MIN_W<24> {
        LEN_MIN_W::new(self)
    }
    #[doc = "Bits 28:31 - The value of this field + 1 defines the maximum burst length to be counted. This field affects the following counters: ADDR_CNT.CNT, ADDR_STALL.CNT. Note that this field does not have a corresponding field in the FILTER_MASK register."]
    #[inline(always)]
    #[must_use]
    pub fn len_max(&mut self) -> LEN_MAX_W<28> {
        LEN_MAX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transaction filter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](index.html) module"]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter::R](R) reader structure"]
impl crate::Readable for FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter::W](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER to value 0xf000_0000"]
impl crate::Resettable for FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000_0000;
}
