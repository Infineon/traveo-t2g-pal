#[doc = "Register `ROT_CTRL` reader"]
pub struct R(crate::R<ROT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROT_CTRL` writer"]
pub struct W(crate::W<ROT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROT_CTRL_SPEC>;
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
impl From<crate::W<ROT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUTO_INC` reader - Auto-increment BLK_IDX by 1 for each read/write of ROT_BLK_LUT"]
pub type AUTO_INC_R = crate::BitReader<bool>;
#[doc = "Field `AUTO_INC` writer - Auto-increment BLK_IDX by 1 for each read/write of ROT_BLK_LUT"]
pub type AUTO_INC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROT_CTRL_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Security lockdown for the root-of-trust configuration registers. Software can set this bit but not clear it once set. When set, write operations to ROT_BLK_LUT are not possible. Write is ignored."]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Security lockdown for the root-of-trust configuration registers. Software can set this bit but not clear it once set. When set, write operations to ROT_BLK_LUT are not possible. Write is ignored."]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 8 - Auto-increment BLK_IDX by 1 for each read/write of ROT_BLK_LUT"]
    #[inline(always)]
    pub fn auto_inc(&self) -> AUTO_INC_R {
        AUTO_INC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Security lockdown for the root-of-trust configuration registers. Software can set this bit but not clear it once set. When set, write operations to ROT_BLK_LUT are not possible. Write is ignored."]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Auto-increment BLK_IDX by 1 for each read/write of ROT_BLK_LUT"]
    #[inline(always)]
    #[must_use]
    pub fn auto_inc(&mut self) -> AUTO_INC_W<8> {
        AUTO_INC_W::new(self)
    }
    #[doc = "Bit 31 - Security lockdown for the root-of-trust configuration registers. Software can set this bit but not clear it once set. When set, write operations to ROT_BLK_LUT are not possible. Write is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<31> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register with lock bit and auto-increment only\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rot_ctrl](index.html) module"]
pub struct ROT_CTRL_SPEC;
impl crate::RegisterSpec for ROT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rot_ctrl::R](R) reader structure"]
impl crate::Readable for ROT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rot_ctrl::W](W) writer structure"]
impl crate::Writable for ROT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROT_CTRL to value 0x0100"]
impl crate::Resettable for ROT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
