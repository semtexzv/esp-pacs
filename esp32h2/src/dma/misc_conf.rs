#[doc = "Register `MISC_CONF` reader"]
pub struct R(crate::R<MISC_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_CONF` writer"]
pub struct W(crate::W<MISC_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_CONF_SPEC>;
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
impl From<crate::W<MISC_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHBM_RST_INTER` reader - Set this bit then clear this bit to reset the internal ahb FSM."]
pub type AHBM_RST_INTER_R = crate::BitReader;
#[doc = "Field `AHBM_RST_INTER` writer - Set this bit then clear this bit to reset the internal ahb FSM."]
pub type AHBM_RST_INTER_W<'a, const O: u8> = crate::BitWriter<'a, MISC_CONF_SPEC, O>;
#[doc = "Field `ARB_PRI_DIS` reader - Set this bit to disable priority arbitration function."]
pub type ARB_PRI_DIS_R = crate::BitReader;
#[doc = "Field `ARB_PRI_DIS` writer - Set this bit to disable priority arbitration function."]
pub type ARB_PRI_DIS_W<'a, const O: u8> = crate::BitWriter<'a, MISC_CONF_SPEC, O>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, MISC_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    pub fn ahbm_rst_inter(&self) -> AHBM_RST_INTER_R {
        AHBM_RST_INTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> ARB_PRI_DIS_R {
        ARB_PRI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CONF")
            .field(
                "ahbm_rst_inter",
                &format_args!("{}", self.ahbm_rst_inter().bit()),
            )
            .field("arb_pri_dis", &format_args!("{}", self.arb_pri_dis().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISC_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit then clear this bit to reset the internal ahb FSM."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst_inter(&mut self) -> AHBM_RST_INTER_W<0> {
        AHBM_RST_INTER_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to disable priority arbitration function."]
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_dis(&mut self) -> ARB_PRI_DIS_W<2> {
        ARB_PRI_DIS_W::new(self)
    }
    #[doc = "Bit 3 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<3> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MISC register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_conf](index.html) module"]
pub struct MISC_CONF_SPEC;
impl crate::RegisterSpec for MISC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_conf::R](R) reader structure"]
impl crate::Readable for MISC_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_conf::W](W) writer structure"]
impl crate::Writable for MISC_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_CONF to value 0"]
impl crate::Resettable for MISC_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}