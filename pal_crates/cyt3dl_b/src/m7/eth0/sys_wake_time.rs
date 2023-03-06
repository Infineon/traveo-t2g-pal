#[doc = "Register `SYS_WAKE_TIME` reader"]
pub struct R(crate::R<SYS_WAKE_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_WAKE_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_WAKE_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_WAKE_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_WAKE_TIME` writer"]
pub struct W(crate::W<SYS_WAKE_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_WAKE_TIME_SPEC>;
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
impl From<crate::W<SYS_WAKE_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_WAKE_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYS_WAKE_TIME` reader - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en (each interval is equivalent to eight tx_clk periods and so varies with data rate)."]
pub type SYS_WAKE_TIME_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SYS_WAKE_TIME` writer - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en (each interval is equivalent to eight tx_clk periods and so varies with data rate)."]
pub type SYS_WAKE_TIME_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SYS_WAKE_TIME_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en (each interval is equivalent to eight tx_clk periods and so varies with data rate)."]
    #[inline(always)]
    pub fn sys_wake_time(&self) -> SYS_WAKE_TIME_R {
        SYS_WAKE_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en (each interval is equivalent to eight tx_clk periods and so varies with data rate)."]
    #[inline(always)]
    #[must_use]
    pub fn sys_wake_time(&mut self) -> SYS_WAKE_TIME_W<0> {
        SYS_WAKE_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to pause transmission after deassertion of tx_lpi_en. Each unit in this register corresponds to 64ns in gigabit mode, 320ns in 100M mode and 3200ns at 10M. After tx_lpi_en is deasserted transmission will pause for the set time.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_wake_time](index.html) module"]
pub struct SYS_WAKE_TIME_SPEC;
impl crate::RegisterSpec for SYS_WAKE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_wake_time::R](R) reader structure"]
impl crate::Readable for SYS_WAKE_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_wake_time::W](W) writer structure"]
impl crate::Writable for SYS_WAKE_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYS_WAKE_TIME to value 0"]
impl crate::Resettable for SYS_WAKE_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
