#[doc = r"Register block"]
#[repr(C)]
pub struct GPSCALER4 {
    #[doc = "0x00 - Static control settings that must typically be setup once only."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Global operation control."]
    pub control: CONTROL,
    #[doc = "0x08 - Dimensions of the output frame or fame slice. This values are required for both FIR filter and scaler, since their output may have other dimension than input. The dimension of scaler's output is defined by the scale factor. The dimension of FIR filter's output depends on the tiling at the begin and the end of the slice."]
    pub outputdimension: OUTPUTDIMENSION,
    #[doc = "0x0c - Scaler operation control."]
    pub scalercontrol: SCALERCONTROL,
    #[doc = "0x10 - Phase interpolator setup."]
    pub scalersetuph0: SCALERSETUPH0,
    #[doc = "0x14 - Phase interpolator setup."]
    pub scalersetuph1: SCALERSETUPH1,
    #[doc = "0x18 - Phase interpolator setup."]
    pub scalersetupv0: SCALERSETUPV0,
    #[doc = "0x1c - Phase interpolator setup, selected if input and output field polarity is 0."]
    pub scalersetupv1: SCALERSETUPV1,
    #[doc = "0x20 - Phase interpolator setup, selected if input field polarity is 1 and output field polarity is 0."]
    pub scalersetupv2: SCALERSETUPV2,
    #[doc = "0x24 - Phase interpolator setup, selected if input field polarity is 0 and output field polarity is 1."]
    pub scalersetupv3: SCALERSETUPV3,
    #[doc = "0x28 - Phase interpolator setup, selected if input and output field polarity is 1."]
    pub scalersetupv4: SCALERSETUPV4,
    #[doc = "0x2c - Filter operation control."]
    pub filtercontrol: FILTERCONTROL,
    #[doc = "0x30 - Color for outside of the slice."]
    pub tilecolorrgb: TILECOLORRGB,
    #[doc = "0x34 - Horizontal coefficients for filter_mode FIR."]
    pub coefficientsh0: COEFFICIENTSH0,
    #[doc = "0x38 - Horizontal coefficients for filter_mode FIR."]
    pub coefficientsh1: COEFFICIENTSH1,
    #[doc = "0x3c - Horizontal coefficients for filter_mode FIR."]
    pub coefficientsh2: COEFFICIENTSH2,
    #[doc = "0x40 - Horizontal coefficients for filter_mode FIR."]
    pub coefficientsh3: COEFFICIENTSH3,
    #[doc = "0x44 - Vertical coefficients for filter_mode FIR."]
    pub coefficientsv0: COEFFICIENTSV0,
    #[doc = "0x48 - Vertical coefficients for filter_mode FIR."]
    pub coefficientsv1: COEFFICIENTSV1,
    #[doc = "0x4c - Vertical coefficients for filter_mode FIR."]
    pub coefficientsv2: COEFFICIENTSV2,
    #[doc = "0x50 - Vertical coefficients for filter_mode FIR."]
    pub coefficientsv3: COEFFICIENTSV3,
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Static control settings that must typically be setup once only."]
pub mod staticcontrol;
#[doc = "CONTROL (rw) register accessor: an alias for `Reg<CONTROL_SPEC>`"]
pub type CONTROL = crate::Reg<control::CONTROL_SPEC>;
#[doc = "Global operation control."]
pub mod control;
#[doc = "OUTPUTDIMENSION (rw) register accessor: an alias for `Reg<OUTPUTDIMENSION_SPEC>`"]
pub type OUTPUTDIMENSION = crate::Reg<outputdimension::OUTPUTDIMENSION_SPEC>;
#[doc = "Dimensions of the output frame or fame slice. This values are required for both FIR filter and scaler, since their output may have other dimension than input. The dimension of scaler's output is defined by the scale factor. The dimension of FIR filter's output depends on the tiling at the begin and the end of the slice."]
pub mod outputdimension;
#[doc = "SCALERCONTROL (rw) register accessor: an alias for `Reg<SCALERCONTROL_SPEC>`"]
pub type SCALERCONTROL = crate::Reg<scalercontrol::SCALERCONTROL_SPEC>;
#[doc = "Scaler operation control."]
pub mod scalercontrol;
#[doc = "SCALERSETUPH0 (rw) register accessor: an alias for `Reg<SCALERSETUPH0_SPEC>`"]
pub type SCALERSETUPH0 = crate::Reg<scalersetuph0::SCALERSETUPH0_SPEC>;
#[doc = "Phase interpolator setup."]
pub mod scalersetuph0;
#[doc = "SCALERSETUPH1 (rw) register accessor: an alias for `Reg<SCALERSETUPH1_SPEC>`"]
pub type SCALERSETUPH1 = crate::Reg<scalersetuph1::SCALERSETUPH1_SPEC>;
#[doc = "Phase interpolator setup."]
pub mod scalersetuph1;
#[doc = "SCALERSETUPV0 (rw) register accessor: an alias for `Reg<SCALERSETUPV0_SPEC>`"]
pub type SCALERSETUPV0 = crate::Reg<scalersetupv0::SCALERSETUPV0_SPEC>;
#[doc = "Phase interpolator setup."]
pub mod scalersetupv0;
#[doc = "SCALERSETUPV1 (rw) register accessor: an alias for `Reg<SCALERSETUPV1_SPEC>`"]
pub type SCALERSETUPV1 = crate::Reg<scalersetupv1::SCALERSETUPV1_SPEC>;
#[doc = "Phase interpolator setup, selected if input and output field polarity is 0."]
pub mod scalersetupv1;
#[doc = "SCALERSETUPV2 (rw) register accessor: an alias for `Reg<SCALERSETUPV2_SPEC>`"]
pub type SCALERSETUPV2 = crate::Reg<scalersetupv2::SCALERSETUPV2_SPEC>;
#[doc = "Phase interpolator setup, selected if input field polarity is 1 and output field polarity is 0."]
pub mod scalersetupv2;
#[doc = "SCALERSETUPV3 (rw) register accessor: an alias for `Reg<SCALERSETUPV3_SPEC>`"]
pub type SCALERSETUPV3 = crate::Reg<scalersetupv3::SCALERSETUPV3_SPEC>;
#[doc = "Phase interpolator setup, selected if input field polarity is 0 and output field polarity is 1."]
pub mod scalersetupv3;
#[doc = "SCALERSETUPV4 (rw) register accessor: an alias for `Reg<SCALERSETUPV4_SPEC>`"]
pub type SCALERSETUPV4 = crate::Reg<scalersetupv4::SCALERSETUPV4_SPEC>;
#[doc = "Phase interpolator setup, selected if input and output field polarity is 1."]
pub mod scalersetupv4;
#[doc = "FILTERCONTROL (rw) register accessor: an alias for `Reg<FILTERCONTROL_SPEC>`"]
pub type FILTERCONTROL = crate::Reg<filtercontrol::FILTERCONTROL_SPEC>;
#[doc = "Filter operation control."]
pub mod filtercontrol;
#[doc = "TILECOLORRGB (rw) register accessor: an alias for `Reg<TILECOLORRGB_SPEC>`"]
pub type TILECOLORRGB = crate::Reg<tilecolorrgb::TILECOLORRGB_SPEC>;
#[doc = "Color for outside of the slice."]
pub mod tilecolorrgb;
#[doc = "COEFFICIENTSH0 (rw) register accessor: an alias for `Reg<COEFFICIENTSH0_SPEC>`"]
pub type COEFFICIENTSH0 = crate::Reg<coefficientsh0::COEFFICIENTSH0_SPEC>;
#[doc = "Horizontal coefficients for filter_mode FIR."]
pub mod coefficientsh0;
#[doc = "COEFFICIENTSH1 (rw) register accessor: an alias for `Reg<COEFFICIENTSH1_SPEC>`"]
pub type COEFFICIENTSH1 = crate::Reg<coefficientsh1::COEFFICIENTSH1_SPEC>;
#[doc = "Horizontal coefficients for filter_mode FIR."]
pub mod coefficientsh1;
#[doc = "COEFFICIENTSH2 (rw) register accessor: an alias for `Reg<COEFFICIENTSH2_SPEC>`"]
pub type COEFFICIENTSH2 = crate::Reg<coefficientsh2::COEFFICIENTSH2_SPEC>;
#[doc = "Horizontal coefficients for filter_mode FIR."]
pub mod coefficientsh2;
#[doc = "COEFFICIENTSH3 (rw) register accessor: an alias for `Reg<COEFFICIENTSH3_SPEC>`"]
pub type COEFFICIENTSH3 = crate::Reg<coefficientsh3::COEFFICIENTSH3_SPEC>;
#[doc = "Horizontal coefficients for filter_mode FIR."]
pub mod coefficientsh3;
#[doc = "COEFFICIENTSV0 (rw) register accessor: an alias for `Reg<COEFFICIENTSV0_SPEC>`"]
pub type COEFFICIENTSV0 = crate::Reg<coefficientsv0::COEFFICIENTSV0_SPEC>;
#[doc = "Vertical coefficients for filter_mode FIR."]
pub mod coefficientsv0;
#[doc = "COEFFICIENTSV1 (rw) register accessor: an alias for `Reg<COEFFICIENTSV1_SPEC>`"]
pub type COEFFICIENTSV1 = crate::Reg<coefficientsv1::COEFFICIENTSV1_SPEC>;
#[doc = "Vertical coefficients for filter_mode FIR."]
pub mod coefficientsv1;
#[doc = "COEFFICIENTSV2 (rw) register accessor: an alias for `Reg<COEFFICIENTSV2_SPEC>`"]
pub type COEFFICIENTSV2 = crate::Reg<coefficientsv2::COEFFICIENTSV2_SPEC>;
#[doc = "Vertical coefficients for filter_mode FIR."]
pub mod coefficientsv2;
#[doc = "COEFFICIENTSV3 (rw) register accessor: an alias for `Reg<COEFFICIENTSV3_SPEC>`"]
pub type COEFFICIENTSV3 = crate::Reg<coefficientsv3::COEFFICIENTSV3_SPEC>;
#[doc = "Vertical coefficients for filter_mode FIR."]
pub mod coefficientsv3;
