#[doc = "Register `CONF1` reader"]
pub struct R(crate::R<CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1` writer"]
pub struct W(crate::W<CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1_SPEC>;
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
impl From<crate::W<CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK_SUM_EN` reader - a"]
pub type CHECK_SUM_EN_R = crate::BitReader<bool>;
#[doc = "Field `CHECK_SUM_EN` writer - a"]
pub type CHECK_SUM_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `CHECK_SEQ_EN` reader - a"]
pub type CHECK_SEQ_EN_R = crate::BitReader<bool>;
#[doc = "Field `CHECK_SEQ_EN` writer - a"]
pub type CHECK_SEQ_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `CRC_DISABLE` reader - a"]
pub type CRC_DISABLE_R = crate::BitReader<bool>;
#[doc = "Field `CRC_DISABLE` writer - a"]
pub type CRC_DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `SAVE_HEAD` reader - a"]
pub type SAVE_HEAD_R = crate::BitReader<bool>;
#[doc = "Field `SAVE_HEAD` writer - a"]
pub type SAVE_HEAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `TX_CHECK_SUM_RE` reader - a"]
pub type TX_CHECK_SUM_RE_R = crate::BitReader<bool>;
#[doc = "Field `TX_CHECK_SUM_RE` writer - a"]
pub type TX_CHECK_SUM_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `TX_ACK_NUM_RE` reader - a"]
pub type TX_ACK_NUM_RE_R = crate::BitReader<bool>;
#[doc = "Field `TX_ACK_NUM_RE` writer - a"]
pub type TX_ACK_NUM_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `WAIT_SW_START` reader - a"]
pub type WAIT_SW_START_R = crate::BitReader<bool>;
#[doc = "Field `WAIT_SW_START` writer - a"]
pub type WAIT_SW_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
#[doc = "Field `SW_START` reader - a"]
pub type SW_START_R = crate::BitReader<bool>;
#[doc = "Field `SW_START` writer - a"]
pub type SW_START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn check_sum_en(&self) -> CHECK_SUM_EN_R {
        CHECK_SUM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn check_seq_en(&self) -> CHECK_SEQ_EN_R {
        CHECK_SEQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn crc_disable(&self) -> CRC_DISABLE_R {
        CRC_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn save_head(&self) -> SAVE_HEAD_R {
        SAVE_HEAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn tx_check_sum_re(&self) -> TX_CHECK_SUM_RE_R {
        TX_CHECK_SUM_RE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn tx_ack_num_re(&self) -> TX_ACK_NUM_RE_R {
        TX_ACK_NUM_RE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn wait_sw_start(&self) -> WAIT_SW_START_R {
        WAIT_SW_START_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - a"]
    #[inline(always)]
    pub fn sw_start(&self) -> SW_START_R {
        SW_START_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - a"]
    #[inline(always)]
    pub fn check_sum_en(&mut self) -> CHECK_SUM_EN_W<0> {
        CHECK_SUM_EN_W::new(self)
    }
    #[doc = "Bit 1 - a"]
    #[inline(always)]
    pub fn check_seq_en(&mut self) -> CHECK_SEQ_EN_W<1> {
        CHECK_SEQ_EN_W::new(self)
    }
    #[doc = "Bit 2 - a"]
    #[inline(always)]
    pub fn crc_disable(&mut self) -> CRC_DISABLE_W<2> {
        CRC_DISABLE_W::new(self)
    }
    #[doc = "Bit 3 - a"]
    #[inline(always)]
    pub fn save_head(&mut self) -> SAVE_HEAD_W<3> {
        SAVE_HEAD_W::new(self)
    }
    #[doc = "Bit 4 - a"]
    #[inline(always)]
    pub fn tx_check_sum_re(&mut self) -> TX_CHECK_SUM_RE_W<4> {
        TX_CHECK_SUM_RE_W::new(self)
    }
    #[doc = "Bit 5 - a"]
    #[inline(always)]
    pub fn tx_ack_num_re(&mut self) -> TX_ACK_NUM_RE_W<5> {
        TX_ACK_NUM_RE_W::new(self)
    }
    #[doc = "Bit 7 - a"]
    #[inline(always)]
    pub fn wait_sw_start(&mut self) -> WAIT_SW_START_W<7> {
        WAIT_SW_START_W::new(self)
    }
    #[doc = "Bit 8 - a"]
    #[inline(always)]
    pub fn sw_start(&mut self) -> SW_START_W<8> {
        SW_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1](index.html) module"]
pub struct CONF1_SPEC;
impl crate::RegisterSpec for CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1::R](R) reader structure"]
impl crate::Readable for CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1::W](W) writer structure"]
impl crate::Writable for CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1 to value 0x33"]
impl crate::Resettable for CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x33
    }
}