#[doc = "Register `RD_BOUND_CTL` reader"]
pub struct R(crate::R<RD_BOUND_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_BOUND_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_BOUND_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_BOUND_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RD_BOUND_CTL` writer"]
pub struct W(crate::W<RD_BOUND_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RD_BOUND_CTL_SPEC>;
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
impl From<crate::W<RD_BOUND_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RD_BOUND_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIZE5` reader - Number of base latency cycles (minus 1) used for calculating the number of fist page boundary crossing latency cycles: '0': base_latency = 1 cycles ... '31': base_latency = 32 cycles. The actual latency cycles when crossing the first page boundary depend on the start address of the transaction and is calculated as follows: if ((page_size - base_latency) &lt; Start_Addr &amp; (sub_page_size - 1)) { ((Start_Addr &amp; (sub_page_size - 1)) - page_size + base_latency) } else { 0 }"]
pub type SIZE5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE5` writer - Number of base latency cycles (minus 1) used for calculating the number of fist page boundary crossing latency cycles: '0': base_latency = 1 cycles ... '31': base_latency = 32 cycles. The actual latency cycles when crossing the first page boundary depend on the start address of the transaction and is calculated as follows: if ((page_size - base_latency) &lt; Start_Addr &amp; (sub_page_size - 1)) { ((Start_Addr &amp; (sub_page_size - 1)) - page_size + base_latency) } else { 0 }"]
pub type SIZE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RD_BOUND_CTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `SUB_PAGE_SIZE` reader - Specifies the size of a memory sub page 'sub_page_size'. '0': sub_page_size = 8 words = 16 bytes (default). '1': sub_page_size = 16 words = 32 bytes. '2': sub_page_size = 32 words = 64 bytes. '3': sub_page_size = 64 words = 128 bytes."]
pub type SUB_PAGE_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB_PAGE_SIZE` writer - Specifies the size of a memory sub page 'sub_page_size'. '0': sub_page_size = 8 words = 16 bytes (default). '1': sub_page_size = 16 words = 32 bytes. '2': sub_page_size = 32 words = 64 bytes. '3': sub_page_size = 64 words = 128 bytes."]
pub type SUB_PAGE_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RD_BOUND_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SUB_PAGE_NR` reader - Specifies the number of sub pages per page. '0': 1 sub pages per page, i.e. page_size = sub_page_size '1': 2 sub pages per page, i.e. page_size = 2 x sub_page_size '2': 4 sub pages per page, i.e. page_size = 4 x sub_page_size '3': 8 sub pages per page, i.e. page_size = 8 x sub_page_size"]
pub type SUB_PAGE_NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUB_PAGE_NR` writer - Specifies the number of sub pages per page. '0': 1 sub pages per page, i.e. page_size = sub_page_size '1': 2 sub pages per page, i.e. page_size = 2 x sub_page_size '2': 4 sub pages per page, i.e. page_size = 4 x sub_page_size '3': 8 sub pages per page, i.e. page_size = 8 x sub_page_size"]
pub type SUB_PAGE_NR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RD_BOUND_CTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `SUBSEQ_BOUND_EN` reader - Enable subsequent page boundary latency cycles. '0': Disabled. The page crossing latency cycles are only generated when crossing the first page boundary (i.e. the first time when the SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd sub page boundary is crossed). '1': Enabled. The page crossing latency cycles are generated when crossing the first and subsequent page boundaries (i.e. every time when a SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd, 4th, 6th, ... sub page boundary is crossed). Note: This only applies when the number of base latency cycles (SIZE5+1) is greater than the size of a page (base_latency > page_size). Must be set to 0 otherwise."]
pub type SUBSEQ_BOUND_EN_R = crate::BitReader<bool>;
#[doc = "Field `SUBSEQ_BOUND_EN` writer - Enable subsequent page boundary latency cycles. '0': Disabled. The page crossing latency cycles are only generated when crossing the first page boundary (i.e. the first time when the SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd sub page boundary is crossed). '1': Enabled. The page crossing latency cycles are generated when crossing the first and subsequent page boundaries (i.e. every time when a SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd, 4th, 6th, ... sub page boundary is crossed). Note: This only applies when the number of base latency cycles (SIZE5+1) is greater than the size of a page (base_latency > page_size). Must be set to 0 otherwise."]
pub type SUBSEQ_BOUND_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD_BOUND_CTL_SPEC, bool, O>;
#[doc = "Field `PRESENT` reader - Presence of first page boundary latency cycles: '0': not present '1': present"]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `PRESENT` writer - Presence of first page boundary latency cycles: '0': not present '1': present"]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, RD_BOUND_CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Number of base latency cycles (minus 1) used for calculating the number of fist page boundary crossing latency cycles: '0': base_latency = 1 cycles ... '31': base_latency = 32 cycles. The actual latency cycles when crossing the first page boundary depend on the start address of the transaction and is calculated as follows: if ((page_size - base_latency) &lt; Start_Addr &amp; (sub_page_size - 1)) { ((Start_Addr &amp; (sub_page_size - 1)) - page_size + base_latency) } else { 0 }"]
    #[inline(always)]
    pub fn size5(&self) -> SIZE5_R {
        SIZE5_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Specifies the size of a memory sub page 'sub_page_size'. '0': sub_page_size = 8 words = 16 bytes (default). '1': sub_page_size = 16 words = 32 bytes. '2': sub_page_size = 32 words = 64 bytes. '3': sub_page_size = 64 words = 128 bytes."]
    #[inline(always)]
    pub fn sub_page_size(&self) -> SUB_PAGE_SIZE_R {
        SUB_PAGE_SIZE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Specifies the number of sub pages per page. '0': 1 sub pages per page, i.e. page_size = sub_page_size '1': 2 sub pages per page, i.e. page_size = 2 x sub_page_size '2': 4 sub pages per page, i.e. page_size = 4 x sub_page_size '3': 8 sub pages per page, i.e. page_size = 8 x sub_page_size"]
    #[inline(always)]
    pub fn sub_page_nr(&self) -> SUB_PAGE_NR_R {
        SUB_PAGE_NR_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 28 - Enable subsequent page boundary latency cycles. '0': Disabled. The page crossing latency cycles are only generated when crossing the first page boundary (i.e. the first time when the SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd sub page boundary is crossed). '1': Enabled. The page crossing latency cycles are generated when crossing the first and subsequent page boundaries (i.e. every time when a SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd, 4th, 6th, ... sub page boundary is crossed). Note: This only applies when the number of base latency cycles (SIZE5+1) is greater than the size of a page (base_latency > page_size). Must be set to 0 otherwise."]
    #[inline(always)]
    pub fn subseq_bound_en(&self) -> SUBSEQ_BOUND_EN_R {
        SUBSEQ_BOUND_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Presence of first page boundary latency cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of base latency cycles (minus 1) used for calculating the number of fist page boundary crossing latency cycles: '0': base_latency = 1 cycles ... '31': base_latency = 32 cycles. The actual latency cycles when crossing the first page boundary depend on the start address of the transaction and is calculated as follows: if ((page_size - base_latency) &lt; Start_Addr &amp; (sub_page_size - 1)) { ((Start_Addr &amp; (sub_page_size - 1)) - page_size + base_latency) } else { 0 }"]
    #[inline(always)]
    #[must_use]
    pub fn size5(&mut self) -> SIZE5_W<0> {
        SIZE5_W::new(self)
    }
    #[doc = "Bits 16:17 - Specifies the size of a memory sub page 'sub_page_size'. '0': sub_page_size = 8 words = 16 bytes (default). '1': sub_page_size = 16 words = 32 bytes. '2': sub_page_size = 32 words = 64 bytes. '3': sub_page_size = 64 words = 128 bytes."]
    #[inline(always)]
    #[must_use]
    pub fn sub_page_size(&mut self) -> SUB_PAGE_SIZE_W<16> {
        SUB_PAGE_SIZE_W::new(self)
    }
    #[doc = "Bits 20:21 - Specifies the number of sub pages per page. '0': 1 sub pages per page, i.e. page_size = sub_page_size '1': 2 sub pages per page, i.e. page_size = 2 x sub_page_size '2': 4 sub pages per page, i.e. page_size = 4 x sub_page_size '3': 8 sub pages per page, i.e. page_size = 8 x sub_page_size"]
    #[inline(always)]
    #[must_use]
    pub fn sub_page_nr(&mut self) -> SUB_PAGE_NR_W<20> {
        SUB_PAGE_NR_W::new(self)
    }
    #[doc = "Bit 28 - Enable subsequent page boundary latency cycles. '0': Disabled. The page crossing latency cycles are only generated when crossing the first page boundary (i.e. the first time when the SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd sub page boundary is crossed). '1': Enabled. The page crossing latency cycles are generated when crossing the first and subsequent page boundaries (i.e. every time when a SUB_PAGE_NRth sub page boundary is crossed, e.g. with 2 sub pages per page when the 2nd, 4th, 6th, ... sub page boundary is crossed). Note: This only applies when the number of base latency cycles (SIZE5+1) is greater than the size of a page (base_latency > page_size). Must be set to 0 otherwise."]
    #[inline(always)]
    #[must_use]
    pub fn subseq_bound_en(&mut self) -> SUBSEQ_BOUND_EN_W<28> {
        SUBSEQ_BOUND_EN_W::new(self)
    }
    #[doc = "Bit 31 - Presence of first page boundary latency cycles: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<31> {
        PRESENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read boundary control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_bound_ctl](index.html) module"]
pub struct RD_BOUND_CTL_SPEC;
impl crate::RegisterSpec for RD_BOUND_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_bound_ctl::R](R) reader structure"]
impl crate::Readable for RD_BOUND_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rd_bound_ctl::W](W) writer structure"]
impl crate::Writable for RD_BOUND_CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RD_BOUND_CTL to value 0x0010_0000"]
impl crate::Resettable for RD_BOUND_CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
