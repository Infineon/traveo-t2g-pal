#[doc = "Register `WOL_REGISTER` reader"]
pub struct R(crate::R<WOL_REGISTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WOL_REGISTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WOL_REGISTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WOL_REGISTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WOL_REGISTER` writer"]
pub struct W(crate::W<WOL_REGISTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WOL_REGISTER_SPEC>;
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
impl From<crate::W<WOL_REGISTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WOL_REGISTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WOL_REGISTER_SPEC, u16, u16, 16, O>;
#[doc = "Field `WOL_MASK_0` reader - Wake on LAN magic packet event enable. When set magic packet events will cause the wol output to be asserted."]
pub type WOL_MASK_0_R = crate::BitReader<bool>;
#[doc = "Field `WOL_MASK_0` writer - Wake on LAN magic packet event enable. When set magic packet events will cause the wol output to be asserted."]
pub type WOL_MASK_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_REGISTER_SPEC, bool, O>;
#[doc = "Field `WOL_MASK_1` reader - Wake on LAN ARP request event enable. When set ARP request events will cause the wol output to be asserted."]
pub type WOL_MASK_1_R = crate::BitReader<bool>;
#[doc = "Field `WOL_MASK_1` writer - Wake on LAN ARP request event enable. When set ARP request events will cause the wol output to be asserted."]
pub type WOL_MASK_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_REGISTER_SPEC, bool, O>;
#[doc = "Field `WOL_MASK_2` reader - Wake on LAN specific address register 1 event enable. When set specific address 1 events will cause the wol output to be asserted."]
pub type WOL_MASK_2_R = crate::BitReader<bool>;
#[doc = "Field `WOL_MASK_2` writer - Wake on LAN specific address register 1 event enable. When set specific address 1 events will cause the wol output to be asserted."]
pub type WOL_MASK_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_REGISTER_SPEC, bool, O>;
#[doc = "Field `WOL_MASK_3` reader - Wake on LAN multicast hash event enable. When set multicast hash events will cause the wol output to be asserted."]
pub type WOL_MASK_3_R = crate::BitReader<bool>;
#[doc = "Field `WOL_MASK_3` writer - Wake on LAN multicast hash event enable. When set multicast hash events will cause the wol output to be asserted."]
pub type WOL_MASK_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WOL_REGISTER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable. When set magic packet events will cause the wol output to be asserted."]
    #[inline(always)]
    pub fn wol_mask_0(&self) -> WOL_MASK_0_R {
        WOL_MASK_0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable. When set ARP request events will cause the wol output to be asserted."]
    #[inline(always)]
    pub fn wol_mask_1(&self) -> WOL_MASK_1_R {
        WOL_MASK_1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable. When set specific address 1 events will cause the wol output to be asserted."]
    #[inline(always)]
    pub fn wol_mask_2(&self) -> WOL_MASK_2_R {
        WOL_MASK_2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable. When set multicast hash events will cause the wol output to be asserted."]
    #[inline(always)]
    pub fn wol_mask_3(&self) -> WOL_MASK_3_R {
        WOL_MASK_3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wake on LAN ARP request IP address. Written to define the least significant 16 bits of the target IP address that is matched to generate a Wake on LAN event. A value of zero will not generate an event, even if this is matched by the received frame."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 16 - Wake on LAN magic packet event enable. When set magic packet events will cause the wol output to be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wol_mask_0(&mut self) -> WOL_MASK_0_W<16> {
        WOL_MASK_0_W::new(self)
    }
    #[doc = "Bit 17 - Wake on LAN ARP request event enable. When set ARP request events will cause the wol output to be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wol_mask_1(&mut self) -> WOL_MASK_1_W<17> {
        WOL_MASK_1_W::new(self)
    }
    #[doc = "Bit 18 - Wake on LAN specific address register 1 event enable. When set specific address 1 events will cause the wol output to be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wol_mask_2(&mut self) -> WOL_MASK_2_W<18> {
        WOL_MASK_2_W::new(self)
    }
    #[doc = "Bit 19 - Wake on LAN multicast hash event enable. When set multicast hash events will cause the wol output to be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn wol_mask_3(&mut self) -> WOL_MASK_3_W<19> {
        WOL_MASK_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake on LAN Register. Presents in design, but feature is not supported.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wol_register](index.html) module"]
pub struct WOL_REGISTER_SPEC;
impl crate::RegisterSpec for WOL_REGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wol_register::R](R) reader structure"]
impl crate::Readable for WOL_REGISTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wol_register::W](W) writer structure"]
impl crate::Writable for WOL_REGISTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WOL_REGISTER to value 0"]
impl crate::Resettable for WOL_REGISTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
