#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB_PRI_AHB_SMIF0` reader - If two AHB masters are both trying to access the physical SMIF0 space from the separate SMIF0 and SMIF1 XIP spaces as allowed through remap regions, the winner is determined by this bit: 0: the master accessing through the SMIF0 XIP space 1: the master accessing through the SMIF1 XIP space"]
pub type ARB_PRI_AHB_SMIF0_R = crate::BitReader<bool>;
#[doc = "Field `ARB_PRI_AHB_SMIF0` writer - If two AHB masters are both trying to access the physical SMIF0 space from the separate SMIF0 and SMIF1 XIP spaces as allowed through remap regions, the winner is determined by this bit: 0: the master accessing through the SMIF0 XIP space 1: the master accessing through the SMIF1 XIP space"]
pub type ARB_PRI_AHB_SMIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ARB_PRI_AHB_SMIF1` reader - Same as ARB_PRI_AHB_SMIF0 except for accessing the physical SMIF1 space."]
pub type ARB_PRI_AHB_SMIF1_R = crate::BitReader<bool>;
#[doc = "Field `ARB_PRI_AHB_SMIF1` writer - Same as ARB_PRI_AHB_SMIF0 except for accessing the physical SMIF1 space."]
pub type ARB_PRI_AHB_SMIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ARB_PRI_AXI_SMIF0` reader - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF0 space."]
pub type ARB_PRI_AXI_SMIF0_R = crate::BitReader<bool>;
#[doc = "Field `ARB_PRI_AXI_SMIF0` writer - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF0 space."]
pub type ARB_PRI_AXI_SMIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ARB_PRI_AXI_SMIF1` reader - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF1 space."]
pub type ARB_PRI_AXI_SMIF1_R = crate::BitReader<bool>;
#[doc = "Field `ARB_PRI_AXI_SMIF1` writer - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF1 space."]
pub type ARB_PRI_AXI_SMIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `ENABLED` reader - Determines whether the bridge is enabled or not: 0: Remapping bridge is disabled and bypassed. In that case the SMIF0 XIP region connects directly to the SMIF0 physical space, and the SMIF1 XIP region maps directly to the SMIF1 physical space. As such no extra latency or arbitration cycles are incurred. All remap region registers are ignored as are the ARB_PRI_* registers. 1: Remapping bridge is invoked which allows for the remap regions to be employed. However, an extra bus cycle of latency is incurred on all AHB and AXI address channel accesses to allow for remap calculations. Also there can be arbitration latency in the case where 2 masters are trying to access a single physical SMIF. This bit should not be changed if there is any ongoing or pending XIP activity. When this bit is 1, it is forbidden to change any bridge configuration registers (i.e., the rest of the bits in this CTL register and any bits in the REMAP_REGION registers)."]
pub type ENABLED_R = crate::BitReader<bool>;
#[doc = "Field `ENABLED` writer - Determines whether the bridge is enabled or not: 0: Remapping bridge is disabled and bypassed. In that case the SMIF0 XIP region connects directly to the SMIF0 physical space, and the SMIF1 XIP region maps directly to the SMIF1 physical space. As such no extra latency or arbitration cycles are incurred. All remap region registers are ignored as are the ARB_PRI_* registers. 1: Remapping bridge is invoked which allows for the remap regions to be employed. However, an extra bus cycle of latency is incurred on all AHB and AXI address channel accesses to allow for remap calculations. Also there can be arbitration latency in the case where 2 masters are trying to access a single physical SMIF. This bit should not be changed if there is any ongoing or pending XIP activity. When this bit is 1, it is forbidden to change any bridge configuration registers (i.e., the rest of the bits in this CTL register and any bits in the REMAP_REGION registers)."]
pub type ENABLED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - If two AHB masters are both trying to access the physical SMIF0 space from the separate SMIF0 and SMIF1 XIP spaces as allowed through remap regions, the winner is determined by this bit: 0: the master accessing through the SMIF0 XIP space 1: the master accessing through the SMIF1 XIP space"]
    #[inline(always)]
    pub fn arb_pri_ahb_smif0(&self) -> ARB_PRI_AHB_SMIF0_R {
        ARB_PRI_AHB_SMIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Same as ARB_PRI_AHB_SMIF0 except for accessing the physical SMIF1 space."]
    #[inline(always)]
    pub fn arb_pri_ahb_smif1(&self) -> ARB_PRI_AHB_SMIF1_R {
        ARB_PRI_AHB_SMIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF0 space."]
    #[inline(always)]
    pub fn arb_pri_axi_smif0(&self) -> ARB_PRI_AXI_SMIF0_R {
        ARB_PRI_AXI_SMIF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF1 space."]
    #[inline(always)]
    pub fn arb_pri_axi_smif1(&self) -> ARB_PRI_AXI_SMIF1_R {
        ARB_PRI_AXI_SMIF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Determines whether the bridge is enabled or not: 0: Remapping bridge is disabled and bypassed. In that case the SMIF0 XIP region connects directly to the SMIF0 physical space, and the SMIF1 XIP region maps directly to the SMIF1 physical space. As such no extra latency or arbitration cycles are incurred. All remap region registers are ignored as are the ARB_PRI_* registers. 1: Remapping bridge is invoked which allows for the remap regions to be employed. However, an extra bus cycle of latency is incurred on all AHB and AXI address channel accesses to allow for remap calculations. Also there can be arbitration latency in the case where 2 masters are trying to access a single physical SMIF. This bit should not be changed if there is any ongoing or pending XIP activity. When this bit is 1, it is forbidden to change any bridge configuration registers (i.e., the rest of the bits in this CTL register and any bits in the REMAP_REGION registers)."]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If two AHB masters are both trying to access the physical SMIF0 space from the separate SMIF0 and SMIF1 XIP spaces as allowed through remap regions, the winner is determined by this bit: 0: the master accessing through the SMIF0 XIP space 1: the master accessing through the SMIF1 XIP space"]
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_ahb_smif0(&mut self) -> ARB_PRI_AHB_SMIF0_W<0> {
        ARB_PRI_AHB_SMIF0_W::new(self)
    }
    #[doc = "Bit 1 - Same as ARB_PRI_AHB_SMIF0 except for accessing the physical SMIF1 space."]
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_ahb_smif1(&mut self) -> ARB_PRI_AHB_SMIF1_W<1> {
        ARB_PRI_AHB_SMIF1_W::new(self)
    }
    #[doc = "Bit 8 - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF0 space."]
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_axi_smif0(&mut self) -> ARB_PRI_AXI_SMIF0_W<8> {
        ARB_PRI_AXI_SMIF0_W::new(self)
    }
    #[doc = "Bit 9 - Same as ARB_PRI_AHB_SMIF0 except for two AXI masters accessing the physical SMIF1 space."]
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_axi_smif1(&mut self) -> ARB_PRI_AXI_SMIF1_W<9> {
        ARB_PRI_AXI_SMIF1_W::new(self)
    }
    #[doc = "Bit 31 - Determines whether the bridge is enabled or not: 0: Remapping bridge is disabled and bypassed. In that case the SMIF0 XIP region connects directly to the SMIF0 physical space, and the SMIF1 XIP region maps directly to the SMIF1 physical space. As such no extra latency or arbitration cycles are incurred. All remap region registers are ignored as are the ARB_PRI_* registers. 1: Remapping bridge is invoked which allows for the remap regions to be employed. However, an extra bus cycle of latency is incurred on all AHB and AXI address channel accesses to allow for remap calculations. Also there can be arbitration latency in the case where 2 masters are trying to access a single physical SMIF. This bit should not be changed if there is any ongoing or pending XIP activity. When this bit is 1, it is forbidden to change any bridge configuration registers (i.e., the rest of the bits in this CTL register and any bits in the REMAP_REGION registers)."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> ENABLED_W<31> {
        ENABLED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global control registers for the bridge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0x8000_0303"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0303;
}
