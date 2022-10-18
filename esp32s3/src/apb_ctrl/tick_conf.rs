#[doc = "Register `TICK_CONF` reader"]
pub struct R(crate::R<TICK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TICK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TICK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TICK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TICK_CONF` writer"]
pub struct W(crate::W<TICK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TICK_CONF_SPEC>;
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
impl From<crate::W<TICK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TICK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTAL_TICK_NUM` reader - ******* Description ***********"]
pub type XTAL_TICK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTAL_TICK_NUM` writer - ******* Description ***********"]
pub type XTAL_TICK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TICK_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `CK8M_TICK_NUM` reader - ******* Description ***********"]
pub type CK8M_TICK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_TICK_NUM` writer - ******* Description ***********"]
pub type CK8M_TICK_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TICK_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `TICK_ENABLE` reader - ******* Description ***********"]
pub type TICK_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TICK_ENABLE` writer - ******* Description ***********"]
pub type TICK_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TICK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    pub fn xtal_tick_num(&self) -> XTAL_TICK_NUM_R {
        XTAL_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - ******* Description ***********"]
    #[inline(always)]
    pub fn tick_enable(&self) -> TICK_ENABLE_R {
        TICK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - ******* Description ***********"]
    #[inline(always)]
    pub fn xtal_tick_num(&mut self) -> XTAL_TICK_NUM_W<0> {
        XTAL_TICK_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - ******* Description ***********"]
    #[inline(always)]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W<8> {
        CK8M_TICK_NUM_W::new(self)
    }
    #[doc = "Bit 16 - ******* Description ***********"]
    #[inline(always)]
    pub fn tick_enable(&mut self) -> TICK_ENABLE_W<16> {
        TICK_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tick_conf](index.html) module"]
pub struct TICK_CONF_SPEC;
impl crate::RegisterSpec for TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tick_conf::R](R) reader structure"]
impl crate::Readable for TICK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tick_conf::W](W) writer structure"]
impl crate::Writable for TICK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TICK_CONF to value 0x0001_0727"]
impl crate::Resettable for TICK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0727
    }
}