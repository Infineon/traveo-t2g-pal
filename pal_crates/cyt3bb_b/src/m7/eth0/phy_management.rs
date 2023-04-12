#[doc = "Register `PHY_MANAGEMENT` reader"]
pub struct R(crate::R<PHY_MANAGEMENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PHY_MANAGEMENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PHY_MANAGEMENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PHY_MANAGEMENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PHY_MANAGEMENT` writer"]
pub struct W(crate::W<PHY_MANAGEMENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PHY_MANAGEMENT_SPEC>;
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
impl From<crate::W<PHY_MANAGEMENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PHY_MANAGEMENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PHY_WRITE_READ_DATA` reader - For a write operation this is written with the data to be written to the PHY. After a read operation this contains the data read from the PHY."]
pub type PHY_WRITE_READ_DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PHY_WRITE_READ_DATA` writer - For a write operation this is written with the data to be written to the PHY. After a read operation this contains the data read from the PHY."]
pub type PHY_WRITE_READ_DATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_MANAGEMENT_SPEC, u16, u16, 16, O>;
#[doc = "Field `WRITE10` reader - Must be written with 10."]
pub type WRITE10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRITE10` writer - Must be written with 10."]
pub type WRITE10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_MANAGEMENT_SPEC, u8, u8, 2, O>;
#[doc = "Field `REGISTER_ADDRESS` reader - Register address - specifies the register in the PHY to access."]
pub type REGISTER_ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REGISTER_ADDRESS` writer - Register address - specifies the register in the PHY to access."]
pub type REGISTER_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_MANAGEMENT_SPEC, u8, u8, 5, O>;
#[doc = "Field `PHY_ADDRESS` reader - PHY address."]
pub type PHY_ADDRESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_ADDRESS` writer - PHY address."]
pub type PHY_ADDRESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_MANAGEMENT_SPEC, u8, u8, 5, O>;
#[doc = "Field `OPERATION` reader - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OPERATION` writer - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
pub type OPERATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PHY_MANAGEMENT_SPEC, u8, u8, 2, O>;
#[doc = "Field `WRITE1` reader - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_R = crate::BitReader<bool>;
#[doc = "Field `WRITE1` writer - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
pub type WRITE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_MANAGEMENT_SPEC, bool, O>;
#[doc = "Field `WRITE0` reader - Must be written with 0."]
pub type WRITE0_R = crate::BitReader<bool>;
#[doc = "Field `WRITE0` writer - Must be written with 0."]
pub type WRITE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PHY_MANAGEMENT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - For a write operation this is written with the data to be written to the PHY. After a read operation this contains the data read from the PHY."]
    #[inline(always)]
    pub fn phy_write_read_data(&self) -> PHY_WRITE_READ_DATA_R {
        PHY_WRITE_READ_DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&self) -> WRITE10_R {
        WRITE10_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn register_address(&self) -> REGISTER_ADDRESS_R {
        REGISTER_ADDRESS_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phy_address(&self) -> PHY_ADDRESS_R {
        PHY_ADDRESS_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&self) -> WRITE1_R {
        WRITE1_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&self) -> WRITE0_R {
        WRITE0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - For a write operation this is written with the data to be written to the PHY. After a read operation this contains the data read from the PHY."]
    #[inline(always)]
    #[must_use]
    pub fn phy_write_read_data(&mut self) -> PHY_WRITE_READ_DATA_W<0> {
        PHY_WRITE_READ_DATA_W::new(self)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    #[must_use]
    pub fn write10(&mut self) -> WRITE10_W<16> {
        WRITE10_W::new(self)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    #[must_use]
    pub fn register_address(&mut self) -> REGISTER_ADDRESS_W<18> {
        REGISTER_ADDRESS_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    #[must_use]
    pub fn phy_address(&mut self) -> PHY_ADDRESS_W<23> {
        PHY_ADDRESS_W::new(self)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    #[must_use]
    pub fn operation(&mut self) -> OPERATION_W<28> {
        OPERATION_W::new(self)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    #[must_use]
    pub fn write1(&mut self) -> WRITE1_W<30> {
        WRITE1_W::new(self)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    #[must_use]
    pub fn write0(&mut self) -> WRITE0_W<31> {
        WRITE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The PHY maintenance register is implemented as a shift register. Writing to the register starts a shift operation which is signalled as complete when bit-2 is set in the network status register. It takes about 2000 pclk cycles to complete, when MDC is set for pclk divide by 32 in the network configuration register. An interrupt is generated upon completion. During this time, the MSB of the register is output on the MDIO pin and the LSB updated from the MDIO pin with each MDC cycle. This causes transmission of a PHY management frame on MDIO. See Section 22.2.4.5 of the IEEE 802.3 standard. Reading during the shift operation will return the current contents of the shift register. At the end of management operation, the bits will have shifted back to their original locations. For a read operation, the data bits will be updated with data read from the PHY. It is important to write the correct values to the register to ensure a valid PHY management frame is produced. The MDIO interface can read IEEE 802.3 clause 45 PHYs as well as clause 22 PHYs. To read clause 45 PHYs, bit 30 should be written with a 0 rather than a 1. For a description of MDC generation, see Network Configuration Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [phy_management](index.html) module"]
pub struct PHY_MANAGEMENT_SPEC;
impl crate::RegisterSpec for PHY_MANAGEMENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [phy_management::R](R) reader structure"]
impl crate::Readable for PHY_MANAGEMENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [phy_management::W](W) writer structure"]
impl crate::Writable for PHY_MANAGEMENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHY_MANAGEMENT to value 0"]
impl crate::Resettable for PHY_MANAGEMENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
