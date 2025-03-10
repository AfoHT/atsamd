#[doc = "Register `APBCMASK` reader"]
pub struct R(crate::R<APBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APBCMASK` writer"]
pub struct W(crate::W<APBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APBCMASK_SPEC>;
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
impl From<crate::W<APBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCC2_` reader - TCC2 APB Clock Enable"]
pub struct TCC2__R(crate::FieldReader<bool, bool>);
impl TCC2__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC2_` writer - TCC2 APB Clock Enable"]
pub struct TCC2__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC2__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TCC3_` reader - TCC3 APB Clock Enable"]
pub struct TCC3__R(crate::FieldReader<bool, bool>);
impl TCC3__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TCC3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCC3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCC3_` writer - TCC3 APB Clock Enable"]
pub struct TCC3__W<'a> {
    w: &'a mut W,
}
impl<'a> TCC3__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub struct TC4__R(crate::FieldReader<bool, bool>);
impl TC4__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC4__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC4__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub struct TC4__W<'a> {
    w: &'a mut W,
}
impl<'a> TC4__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `TC5_` reader - TC5 APB Clock Enable"]
pub struct TC5__R(crate::FieldReader<bool, bool>);
impl TC5__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TC5__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC5__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC5_` writer - TC5 APB Clock Enable"]
pub struct TC5__W<'a> {
    w: &'a mut W,
}
impl<'a> TC5__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PDEC_` reader - PDEC APB Clock Enable"]
pub struct PDEC__R(crate::FieldReader<bool, bool>);
impl PDEC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PDEC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEC_` writer - PDEC APB Clock Enable"]
pub struct PDEC__W<'a> {
    w: &'a mut W,
}
impl<'a> PDEC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub struct AC__R(crate::FieldReader<bool, bool>);
impl AC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub struct AC__W<'a> {
    w: &'a mut W,
}
impl<'a> AC__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `AES_` reader - AES APB Clock Enable"]
pub struct AES__R(crate::FieldReader<bool, bool>);
impl AES__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AES__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AES_` writer - AES APB Clock Enable"]
pub struct AES__W<'a> {
    w: &'a mut W,
}
impl<'a> AES__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TRNG_` reader - TRNG APB Clock Enable"]
pub struct TRNG__R(crate::FieldReader<bool, bool>);
impl TRNG__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRNG__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRNG__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRNG_` writer - TRNG APB Clock Enable"]
pub struct TRNG__W<'a> {
    w: &'a mut W,
}
impl<'a> TRNG__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ICM_` reader - ICM APB Clock Enable"]
pub struct ICM__R(crate::FieldReader<bool, bool>);
impl ICM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ICM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICM_` writer - ICM APB Clock Enable"]
pub struct ICM__W<'a> {
    w: &'a mut W,
}
impl<'a> ICM__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `QSPI_` reader - QSPI APB Clock Enable"]
pub struct QSPI__R(crate::FieldReader<bool, bool>);
impl QSPI__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QSPI__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPI__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPI_` writer - QSPI APB Clock Enable"]
pub struct QSPI__W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub struct CCL__R(crate::FieldReader<bool, bool>);
impl CCL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub struct CCL__W<'a> {
    w: &'a mut W,
}
impl<'a> CCL__W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> TCC2__R {
        TCC2__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TCC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc3_(&self) -> TCC3__R {
        TCC3__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> TC4__R {
        TC4__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> TC5__R {
        TC5__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    pub fn pdec_(&self) -> PDEC__R {
        PDEC__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> AC__R {
        AC__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&self) -> AES__R {
        AES__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&self) -> TRNG__R {
        TRNG__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    pub fn icm_(&self) -> ICM__R {
        ICM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    pub fn qspi_(&self) -> QSPI__R {
        QSPI__R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> CCL__R {
        CCL__R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&mut self) -> TCC2__W {
        TCC2__W { w: self }
    }
    #[doc = "Bit 4 - TCC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc3_(&mut self) -> TCC3__W {
        TCC3__W { w: self }
    }
    #[doc = "Bit 5 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&mut self) -> TC4__W {
        TC4__W { w: self }
    }
    #[doc = "Bit 6 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&mut self) -> TC5__W {
        TC5__W { w: self }
    }
    #[doc = "Bit 7 - PDEC APB Clock Enable"]
    #[inline(always)]
    pub fn pdec_(&mut self) -> PDEC__W {
        PDEC__W { w: self }
    }
    #[doc = "Bit 8 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&mut self) -> AC__W {
        AC__W { w: self }
    }
    #[doc = "Bit 9 - AES APB Clock Enable"]
    #[inline(always)]
    pub fn aes_(&mut self) -> AES__W {
        AES__W { w: self }
    }
    #[doc = "Bit 10 - TRNG APB Clock Enable"]
    #[inline(always)]
    pub fn trng_(&mut self) -> TRNG__W {
        TRNG__W { w: self }
    }
    #[doc = "Bit 11 - ICM APB Clock Enable"]
    #[inline(always)]
    pub fn icm_(&mut self) -> ICM__W {
        ICM__W { w: self }
    }
    #[doc = "Bit 13 - QSPI APB Clock Enable"]
    #[inline(always)]
    pub fn qspi_(&mut self) -> QSPI__W {
        QSPI__W { w: self }
    }
    #[doc = "Bit 14 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&mut self) -> CCL__W {
        CCL__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apbcmask](index.html) module"]
pub struct APBCMASK_SPEC;
impl crate::RegisterSpec for APBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apbcmask::R](R) reader structure"]
impl crate::Readable for APBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apbcmask::W](W) writer structure"]
impl crate::Writable for APBCMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APBCMASK to value 0x2000"]
impl crate::Resettable for APBCMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
