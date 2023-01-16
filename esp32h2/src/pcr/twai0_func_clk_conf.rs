#[doc = "Register `TWAI0_FUNC_CLK_CONF` reader"]
pub struct R(crate::R<TWAI0_FUNC_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWAI0_FUNC_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWAI0_FUNC_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWAI0_FUNC_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWAI0_FUNC_CLK_CONF` writer"]
pub struct W(crate::W<TWAI0_FUNC_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWAI0_FUNC_CLK_CONF_SPEC>;
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
impl From<crate::W<TWAI0_FUNC_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWAI0_FUNC_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWAI0_FUNC_CLK_SEL` reader - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI0_FUNC_CLK_SEL_R = crate::BitReader<bool>;
#[doc = "Field `TWAI0_FUNC_CLK_SEL` writer - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
pub type TWAI0_FUNC_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWAI0_FUNC_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `TWAI0_FUNC_CLK_EN` reader - Set 1 to enable twai0 function clock"]
pub type TWAI0_FUNC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TWAI0_FUNC_CLK_EN` writer - Set 1 to enable twai0 function clock"]
pub type TWAI0_FUNC_CLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TWAI0_FUNC_CLK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    pub fn twai0_func_clk_sel(&self) -> TWAI0_FUNC_CLK_SEL_R {
        TWAI0_FUNC_CLK_SEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Set 1 to enable twai0 function clock"]
    #[inline(always)]
    pub fn twai0_func_clk_en(&self) -> TWAI0_FUNC_CLK_EN_R {
        TWAI0_FUNC_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - set this field to select clock-source. 0(default): XTAL, 1: FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_sel(&mut self) -> TWAI0_FUNC_CLK_SEL_W<20> {
        TWAI0_FUNC_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 22 - Set 1 to enable twai0 function clock"]
    #[inline(always)]
    #[must_use]
    pub fn twai0_func_clk_en(&mut self) -> TWAI0_FUNC_CLK_EN_W<22> {
        TWAI0_FUNC_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TWAI0_FUNC_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twai0_func_clk_conf](index.html) module"]
pub struct TWAI0_FUNC_CLK_CONF_SPEC;
impl crate::RegisterSpec for TWAI0_FUNC_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [twai0_func_clk_conf::R](R) reader structure"]
impl crate::Readable for TWAI0_FUNC_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twai0_func_clk_conf::W](W) writer structure"]
impl crate::Writable for TWAI0_FUNC_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWAI0_FUNC_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for TWAI0_FUNC_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}