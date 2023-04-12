#[doc = "Register `MAPBIT3_0` reader"]
pub struct R(crate::R<MAPBIT3_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAPBIT3_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAPBIT3_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAPBIT3_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAPBIT3_0` writer"]
pub struct W(crate::W<MAPBIT3_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAPBIT3_0_SPEC>;
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
impl From<crate::W<MAPBIT3_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAPBIT3_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAPBIT0` reader - map bit\\[ 0 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit0 in \\[ 23..0 \\]
=> bit\\[ 0 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit0 \\]; if MapBit0 in \\[29 .. 24\\]
=> bit\\[ 0 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit0 = 31 => bit\\[ 0 \\]
= 0 ; if MapBit1= 30 => bit\\[ 0 \\]
= 1"]
pub type MAPBIT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT0` writer - map bit\\[ 0 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit0 in \\[ 23..0 \\]
=> bit\\[ 0 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit0 \\]; if MapBit0 in \\[29 .. 24\\]
=> bit\\[ 0 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit0 = 31 => bit\\[ 0 \\]
= 0 ; if MapBit1= 30 => bit\\[ 0 \\]
= 1"]
pub type MAPBIT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT1` reader - map bit\\[ 1 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit1 in \\[ 23..0 \\]
=> bit\\[ 1 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit1 \\]; if MapBit1 in \\[29 .. 24\\]
=> bit\\[ 1 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit1 = 31 => bit\\[ 1 \\]
= 0 ; if MapBit1= 30 => bit\\[ 1 \\]
= 1"]
pub type MAPBIT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT1` writer - map bit\\[ 1 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit1 in \\[ 23..0 \\]
=> bit\\[ 1 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit1 \\]; if MapBit1 in \\[29 .. 24\\]
=> bit\\[ 1 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit1 = 31 => bit\\[ 1 \\]
= 0 ; if MapBit1= 30 => bit\\[ 1 \\]
= 1"]
pub type MAPBIT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT2` reader - map bit\\[ 2 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit2 in \\[ 23..0 \\]
=> bit\\[ 2 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit2 \\]; if MapBit2 in \\[29 .. 24\\]
=> bit\\[ 2 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit2 = 31 => bit\\[ 2 \\]
= 0 ; if MapBit2= 30 => bit\\[ 2 \\]
= 1"]
pub type MAPBIT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT2` writer - map bit\\[ 2 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit2 in \\[ 23..0 \\]
=> bit\\[ 2 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit2 \\]; if MapBit2 in \\[29 .. 24\\]
=> bit\\[ 2 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit2 = 31 => bit\\[ 2 \\]
= 0 ; if MapBit2= 30 => bit\\[ 2 \\]
= 1"]
pub type MAPBIT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT3_0_SPEC, u8, u8, 5, O>;
#[doc = "Field `MAPBIT3` reader - map bit\\[ 3 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit3 in \\[ 23..0 \\]
=> bit\\[ 3 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit3 \\]; if MapBit3 in \\[29 .. 24\\]
=> bit\\[ 3 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit3 = 31 => bit\\[ 3 \\]
= 0 ; if MapBit3= 30 => bit\\[ 3 \\]
= 1"]
pub type MAPBIT3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAPBIT3` writer - map bit\\[ 3 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit3 in \\[ 23..0 \\]
=> bit\\[ 3 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit3 \\]; if MapBit3 in \\[29 .. 24\\]
=> bit\\[ 3 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit3 = 31 => bit\\[ 3 \\]
= 0 ; if MapBit3= 30 => bit\\[ 3 \\]
= 1"]
pub type MAPBIT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAPBIT3_0_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - map bit\\[ 0 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit0 in \\[ 23..0 \\]
=> bit\\[ 0 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit0 \\]; if MapBit0 in \\[29 .. 24\\]
=> bit\\[ 0 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit0 = 31 => bit\\[ 0 \\]
= 0 ; if MapBit1= 30 => bit\\[ 0 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit0(&self) -> MAPBIT0_R {
        MAPBIT0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - map bit\\[ 1 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit1 in \\[ 23..0 \\]
=> bit\\[ 1 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit1 \\]; if MapBit1 in \\[29 .. 24\\]
=> bit\\[ 1 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit1 = 31 => bit\\[ 1 \\]
= 0 ; if MapBit1= 30 => bit\\[ 1 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit1(&self) -> MAPBIT1_R {
        MAPBIT1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - map bit\\[ 2 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit2 in \\[ 23..0 \\]
=> bit\\[ 2 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit2 \\]; if MapBit2 in \\[29 .. 24\\]
=> bit\\[ 2 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit2 = 31 => bit\\[ 2 \\]
= 0 ; if MapBit2= 30 => bit\\[ 2 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit2(&self) -> MAPBIT2_R {
        MAPBIT2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - map bit\\[ 3 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit3 in \\[ 23..0 \\]
=> bit\\[ 3 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit3 \\]; if MapBit3 in \\[29 .. 24\\]
=> bit\\[ 3 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit3 = 31 => bit\\[ 3 \\]
= 0 ; if MapBit3= 30 => bit\\[ 3 \\]
= 1"]
    #[inline(always)]
    pub fn mapbit3(&self) -> MAPBIT3_R {
        MAPBIT3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - map bit\\[ 0 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit0 in \\[ 23..0 \\]
=> bit\\[ 0 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit0 \\]; if MapBit0 in \\[29 .. 24\\]
=> bit\\[ 0 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit0 = 31 => bit\\[ 0 \\]
= 0 ; if MapBit1= 30 => bit\\[ 0 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit0(&mut self) -> MAPBIT0_W<0> {
        MAPBIT0_W::new(self)
    }
    #[doc = "Bits 8:12 - map bit\\[ 1 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit1 in \\[ 23..0 \\]
=> bit\\[ 1 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit1 \\]; if MapBit1 in \\[29 .. 24\\]
=> bit\\[ 1 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit1 = 31 => bit\\[ 1 \\]
= 0 ; if MapBit1= 30 => bit\\[ 1 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit1(&mut self) -> MAPBIT1_W<8> {
        MAPBIT1_W::new(self)
    }
    #[doc = "Bits 16:20 - map bit\\[ 2 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit2 in \\[ 23..0 \\]
=> bit\\[ 2 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit2 \\]; if MapBit2 in \\[29 .. 24\\]
=> bit\\[ 2 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit2 = 31 => bit\\[ 2 \\]
= 0 ; if MapBit2= 30 => bit\\[ 2 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit2(&mut self) -> MAPBIT2_W<16> {
        MAPBIT2_W::new(self)
    }
    #[doc = "Bits 24:28 - map bit\\[ 3 \\]
from \\[ Blue, Green, Red \\]
or from TSig\\[5-0\\]. If MapBit3 in \\[ 23..0 \\]
=> bit\\[ 3 \\]
= \\[ Blue, Green, Red \\]
\\[ MapBit3 \\]; if MapBit3 in \\[29 .. 24\\]
=> bit\\[ 3 \\]
in { TSig\\[5\\]
.. TSig\\[0\\]
}; If MapBit3 = 31 => bit\\[ 3 \\]
= 0 ; if MapBit3= 30 => bit\\[ 3 \\]
= 1"]
    #[inline(always)]
    #[must_use]
    pub fn mapbit3(&mut self) -> MAPBIT3_W<24> {
        MAPBIT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mapping of 24 bit RGB or Timing Generator TSig\\[5-0\\]
to bit 0 .. 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mapbit3_0](index.html) module"]
pub struct MAPBIT3_0_SPEC;
impl crate::RegisterSpec for MAPBIT3_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mapbit3_0::R](R) reader structure"]
impl crate::Readable for MAPBIT3_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mapbit3_0::W](W) writer structure"]
impl crate::Writable for MAPBIT3_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAPBIT3_0 to value 0x0302_0100"]
impl crate::Resettable for MAPBIT3_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0302_0100;
}
