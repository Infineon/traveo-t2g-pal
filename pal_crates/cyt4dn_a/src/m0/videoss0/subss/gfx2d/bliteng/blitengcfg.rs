#[doc = r"Register block"]
#[repr(C)]
pub struct BLITENGCFG {
    #[doc = "0x00 - Static control register."]
    pub staticcontrol: STATICCONTROL,
    #[doc = "0x04 - Task configuration register."]
    pub tasksetup: TASKSETUP,
    #[doc = "0x08 - Dimension of final frame in memory."]
    pub lrframedimension: LRFRAMEDIMENSION,
    #[doc = "0x0c - Setup for rendering operation."]
    pub operationsetup: OPERATIONSETUP,
    #[doc = "0x10 - Dimension of object. Only possible in LBO mode"]
    pub lrobjectdimension: LROBJECTDIMENSION,
    #[doc = "0x14 - Position of current object in frame coordinate system. Only possible in LBO mode"]
    pub lrobjectposition: LROBJECTPOSITION,
    #[doc = "0x18 - Constant alpha used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set. Only possible in LBO mode."]
    pub lrconstalpha: LRCONSTALPHA,
    #[doc = "0x1c - Constant color used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set and BufferSelect==RGBA. Only possible in LBO mode"]
    pub lrconstrgb: LRCONSTRGB,
    #[doc = "0x20 - Sync ID request register."]
    pub syncidrequest0: SYNCIDREQUEST0,
    #[doc = "0x24 - Sync ID request register."]
    pub syncidrequest1: SYNCIDREQUEST1,
    #[doc = "0x28 - Sync ID request register."]
    pub syncidrequest2: SYNCIDREQUEST2,
    #[doc = "0x2c - Sync ID request register."]
    pub syncidrequest3: SYNCIDREQUEST3,
    #[doc = "0x30 - Sync ID request register."]
    pub syncidrequest4: SYNCIDREQUEST4,
    #[doc = "0x34 - Sync ID request register."]
    pub syncidrequest5: SYNCIDREQUEST5,
    #[doc = "0x38 - Sync ID request register."]
    pub syncidrequest6: SYNCIDREQUEST6,
    #[doc = "0x3c - Sync ID request register."]
    pub syncidrequest7: SYNCIDREQUEST7,
    #[doc = "0x40 - Triggers interrupts for sequence complete of specific TB of global sequence complete interrupt."]
    pub synccontrol: SYNCCONTROL,
    #[doc = "0x44 - Line rendering operation control register."]
    pub linerenderingcontrol: LINERENDERINGCONTROL,
    #[doc = "0x48 - Control register of the performance measurement counters ClockCounter and IdleCounter"]
    pub perfmeasurementcontrol: PERFMEASUREMENTCONTROL,
    #[doc = "0x4c - Sync ID Status register for task 0."]
    pub syncidstatus0: SYNCIDSTATUS0,
    #[doc = "0x50 - Sync ID Status register for task 1."]
    pub syncidstatus1: SYNCIDSTATUS1,
    #[doc = "0x54 - Sync ID Status register for task 2."]
    pub syncidstatus2: SYNCIDSTATUS2,
    #[doc = "0x58 - Sync ID Status register for task 3."]
    pub syncidstatus3: SYNCIDSTATUS3,
    #[doc = "0x5c - Sync ID Status register for task 4."]
    pub syncidstatus4: SYNCIDSTATUS4,
    #[doc = "0x60 - Sync ID Status register for task 5."]
    pub syncidstatus5: SYNCIDSTATUS5,
    #[doc = "0x64 - Sync ID Status register for task 6."]
    pub syncidstatus6: SYNCIDSTATUS6,
    #[doc = "0x68 - Sync ID Status register for task 7."]
    pub syncidstatus7: SYNCIDSTATUS7,
    #[doc = "0x6c - This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design."]
    pub designattributes0: DESIGNATTRIBUTES0,
    #[doc = "0x70 - This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design."]
    pub designattributes1: DESIGNATTRIBUTES1,
    #[doc = "0x74 - Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set."]
    pub perfmeasurementclockcounterrunning: PERFMEASUREMENTCLOCKCOUNTERRUNNING,
    #[doc = "0x78 - Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set."]
    pub perfmeasurementclockcounter: PERFMEASUREMENTCLOCKCOUNTER,
    #[doc = "0x7c - Performance measurement register for BlitEng containing idle cycles. Measurement is active only if PerfMeasurementEn is set."]
    pub perfmeasurementidlecounter: PERFMEASUREMENTIDLECOUNTER,
    #[doc = "0x80 - Performance measurement register for BlitEng containing idle cycles. Measurement is active only if SlicePerfMeasurementEn is set."]
    pub slicemeasurementstatus: SLICEMEASUREMENTSTATUS,
    #[doc = "0x84 - Performance measurement for each slice. Measurement is active only if SlicePerfMeasurementEn is set. This register can be always read,"]
    pub slicemeasurementvalues: SLICEMEASUREMENTVALUES,
    _reserved34: [u8; 0x78],
    #[doc = "0x100 - Settings common for all line-buffer handshake connections."]
    pub lbhglobal: LBHGLOBAL,
    #[doc = "0x104 - Global status register 0 for the line buffer handshake."]
    pub lbhglobalstatus0: LBHGLOBALSTATUS0,
    #[doc = "0x108 - Global status register 1 for the line buffer handshake."]
    pub lbhglobalstatus1: LBHGLOBALSTATUS1,
    #[doc = "0x10c - Global status register 2 for the line buffer handshake."]
    pub lbhglobalstatus2: LBHGLOBALSTATUS2,
    _reserved38: [u8; 0xf0],
    #[doc = "0x200 - Line buffer configuration register 0."]
    pub lbhlink0bufferconfig0: LBHLINK0BUFFERCONFIG0,
    #[doc = "0x204 - Line buffer configuration register 1."]
    pub lbhlink0bufferconfig1: LBHLINK0BUFFERCONFIG1,
    #[doc = "0x208 - Buffer status register 0 for the line buffer handshake."]
    pub lbhlink0bufferstatus0: LBHLINK0BUFFERSTATUS0,
    #[doc = "0x20c - Buffer status register 1 for the line buffer handshake."]
    pub lbhlink0bufferstatus1: LBHLINK0BUFFERSTATUS1,
    #[doc = "0x210 - Buffer status register 2 for the line buffer handshake."]
    pub lbhlink0bufferstatus2: LBHLINK0BUFFERSTATUS2,
    #[doc = "0x214 - Buffer status register 3 for the line buffer handshake."]
    pub lbhlink0bufferstatus3: LBHLINK0BUFFERSTATUS3,
    #[doc = "0x218 - Buffer status register 4 for the line buffer handshake."]
    pub lbhlink0bufferstatus4: LBHLINK0BUFFERSTATUS4,
    _reserved45: [u8; 0x24],
    #[doc = "0x240 - Line buffer configuration register 0."]
    pub lbhlink1bufferconfig0: LBHLINK1BUFFERCONFIG0,
    #[doc = "0x244 - Line buffer configuration register 1."]
    pub lbhlink1bufferconfig1: LBHLINK1BUFFERCONFIG1,
    #[doc = "0x248 - Buffer status register 0 for the line buffer handshake."]
    pub lbhlink1bufferstatus0: LBHLINK1BUFFERSTATUS0,
    #[doc = "0x24c - Buffer status register 1 for the line buffer handshake."]
    pub lbhlink1bufferstatus1: LBHLINK1BUFFERSTATUS1,
    #[doc = "0x250 - Buffer status register 2 for the line buffer handshake."]
    pub lbhlink1bufferstatus2: LBHLINK1BUFFERSTATUS2,
    #[doc = "0x254 - Buffer status register 3 for the line buffer handshake."]
    pub lbhlink1bufferstatus3: LBHLINK1BUFFERSTATUS3,
    #[doc = "0x258 - Buffer status register 4 for the line buffer handshake."]
    pub lbhlink1bufferstatus4: LBHLINK1BUFFERSTATUS4,
    _reserved52: [u8; 0x24],
    #[doc = "0x280 - Line buffer configuration register 0."]
    pub lbhlink2bufferconfig0: LBHLINK2BUFFERCONFIG0,
    #[doc = "0x284 - Line buffer configuration register 1."]
    pub lbhlink2bufferconfig1: LBHLINK2BUFFERCONFIG1,
    #[doc = "0x288 - Buffer status register 0 for the line buffer handshake."]
    pub lbhlink2bufferstatus0: LBHLINK2BUFFERSTATUS0,
    #[doc = "0x28c - Buffer status register 1 for the line buffer handshake."]
    pub lbhlink2bufferstatus1: LBHLINK2BUFFERSTATUS1,
    #[doc = "0x290 - Buffer status register 2 for the line buffer handshake."]
    pub lbhlink2bufferstatus2: LBHLINK2BUFFERSTATUS2,
    #[doc = "0x294 - Buffer status register 3 for the line buffer handshake."]
    pub lbhlink2bufferstatus3: LBHLINK2BUFFERSTATUS3,
    #[doc = "0x298 - Buffer status register 4 for the line buffer handshake."]
    pub lbhlink2bufferstatus4: LBHLINK2BUFFERSTATUS4,
    _reserved59: [u8; 0x24],
    #[doc = "0x2c0 - Line buffer configuration register 0."]
    pub lbhlink3bufferconfig0: LBHLINK3BUFFERCONFIG0,
    #[doc = "0x2c4 - Line buffer configuration register 1."]
    pub lbhlink3bufferconfig1: LBHLINK3BUFFERCONFIG1,
    #[doc = "0x2c8 - Buffer status register 0 for the line buffer handshake."]
    pub lbhlink3bufferstatus0: LBHLINK3BUFFERSTATUS0,
    #[doc = "0x2cc - Buffer status register 1 for the line buffer handshake."]
    pub lbhlink3bufferstatus1: LBHLINK3BUFFERSTATUS1,
    #[doc = "0x2d0 - Buffer status register 2 for the line buffer handshake."]
    pub lbhlink3bufferstatus2: LBHLINK3BUFFERSTATUS2,
    #[doc = "0x2d4 - Buffer status register 3 for the line buffer handshake."]
    pub lbhlink3bufferstatus3: LBHLINK3BUFFERSTATUS3,
    #[doc = "0x2d8 - Buffer status register 4 for the line buffer handshake."]
    pub lbhlink3bufferstatus4: LBHLINK3BUFFERSTATUS4,
    _reserved66: [u8; 0x24],
    #[doc = "0x300 - Line buffer configuration register 0."]
    pub lbhlink4bufferconfig0: LBHLINK4BUFFERCONFIG0,
    #[doc = "0x304 - Line buffer configuration register 1."]
    pub lbhlink4bufferconfig1: LBHLINK4BUFFERCONFIG1,
    #[doc = "0x308 - Buffer status register 0 for the line buffer handshake."]
    pub lbhlink4bufferstatus0: LBHLINK4BUFFERSTATUS0,
    #[doc = "0x30c - Buffer status register 1 for the line buffer handshake."]
    pub lbhlink4bufferstatus1: LBHLINK4BUFFERSTATUS1,
    #[doc = "0x310 - Buffer status register 2 for the line buffer handshake."]
    pub lbhlink4bufferstatus2: LBHLINK4BUFFERSTATUS2,
    #[doc = "0x314 - Buffer status register 3 for the line buffer handshake."]
    pub lbhlink4bufferstatus3: LBHLINK4BUFFERSTATUS3,
    #[doc = "0x318 - Buffer status register 4 for the line buffer handshake."]
    pub lbhlink4bufferstatus4: LBHLINK4BUFFERSTATUS4,
    _reserved73: [u8; 0x1ce4],
    #[doc = "0x2000..0x4000 - Shared Palette Memory."]
    pub palette: [PALETTE; 2048],
}
#[doc = "STATICCONTROL (rw) register accessor: an alias for `Reg<STATICCONTROL_SPEC>`"]
pub type STATICCONTROL = crate::Reg<staticcontrol::STATICCONTROL_SPEC>;
#[doc = "Static control register."]
pub mod staticcontrol;
#[doc = "TASKSETUP (rw) register accessor: an alias for `Reg<TASKSETUP_SPEC>`"]
pub type TASKSETUP = crate::Reg<tasksetup::TASKSETUP_SPEC>;
#[doc = "Task configuration register."]
pub mod tasksetup;
#[doc = "LRFRAMEDIMENSION (rw) register accessor: an alias for `Reg<LRFRAMEDIMENSION_SPEC>`"]
pub type LRFRAMEDIMENSION = crate::Reg<lrframedimension::LRFRAMEDIMENSION_SPEC>;
#[doc = "Dimension of final frame in memory."]
pub mod lrframedimension;
#[doc = "OPERATIONSETUP (rw) register accessor: an alias for `Reg<OPERATIONSETUP_SPEC>`"]
pub type OPERATIONSETUP = crate::Reg<operationsetup::OPERATIONSETUP_SPEC>;
#[doc = "Setup for rendering operation."]
pub mod operationsetup;
#[doc = "LROBJECTDIMENSION (rw) register accessor: an alias for `Reg<LROBJECTDIMENSION_SPEC>`"]
pub type LROBJECTDIMENSION = crate::Reg<lrobjectdimension::LROBJECTDIMENSION_SPEC>;
#[doc = "Dimension of object. Only possible in LBO mode"]
pub mod lrobjectdimension;
#[doc = "LROBJECTPOSITION (rw) register accessor: an alias for `Reg<LROBJECTPOSITION_SPEC>`"]
pub type LROBJECTPOSITION = crate::Reg<lrobjectposition::LROBJECTPOSITION_SPEC>;
#[doc = "Position of current object in frame coordinate system. Only possible in LBO mode"]
pub mod lrobjectposition;
#[doc = "LRCONSTALPHA (rw) register accessor: an alias for `Reg<LRCONSTALPHA_SPEC>`"]
pub type LRCONSTALPHA = crate::Reg<lrconstalpha::LRCONSTALPHA_SPEC>;
#[doc = "Constant alpha used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set. Only possible in LBO mode."]
pub mod lrconstalpha;
#[doc = "LRCONSTRGB (rw) register accessor: an alias for `Reg<LRCONSTRGB_SPEC>`"]
pub type LRCONSTRGB = crate::Reg<lrconstrgb::LRCONSTRGB_SPEC>;
#[doc = "Constant color used to fill buffer w/o fetch unit. Has affect only if ConstantColorFill is set and BufferSelect==RGBA. Only possible in LBO mode"]
pub mod lrconstrgb;
#[doc = "SYNCIDREQUEST0 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST0_SPEC>`"]
pub type SYNCIDREQUEST0 = crate::Reg<syncidrequest0::SYNCIDREQUEST0_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest0;
#[doc = "SYNCIDREQUEST1 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST1_SPEC>`"]
pub type SYNCIDREQUEST1 = crate::Reg<syncidrequest1::SYNCIDREQUEST1_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest1;
#[doc = "SYNCIDREQUEST2 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST2_SPEC>`"]
pub type SYNCIDREQUEST2 = crate::Reg<syncidrequest2::SYNCIDREQUEST2_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest2;
#[doc = "SYNCIDREQUEST3 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST3_SPEC>`"]
pub type SYNCIDREQUEST3 = crate::Reg<syncidrequest3::SYNCIDREQUEST3_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest3;
#[doc = "SYNCIDREQUEST4 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST4_SPEC>`"]
pub type SYNCIDREQUEST4 = crate::Reg<syncidrequest4::SYNCIDREQUEST4_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest4;
#[doc = "SYNCIDREQUEST5 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST5_SPEC>`"]
pub type SYNCIDREQUEST5 = crate::Reg<syncidrequest5::SYNCIDREQUEST5_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest5;
#[doc = "SYNCIDREQUEST6 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST6_SPEC>`"]
pub type SYNCIDREQUEST6 = crate::Reg<syncidrequest6::SYNCIDREQUEST6_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest6;
#[doc = "SYNCIDREQUEST7 (rw) register accessor: an alias for `Reg<SYNCIDREQUEST7_SPEC>`"]
pub type SYNCIDREQUEST7 = crate::Reg<syncidrequest7::SYNCIDREQUEST7_SPEC>;
#[doc = "Sync ID request register."]
pub mod syncidrequest7;
#[doc = "SYNCCONTROL (w) register accessor: an alias for `Reg<SYNCCONTROL_SPEC>`"]
pub type SYNCCONTROL = crate::Reg<synccontrol::SYNCCONTROL_SPEC>;
#[doc = "Triggers interrupts for sequence complete of specific TB of global sequence complete interrupt."]
pub mod synccontrol;
#[doc = "LINERENDERINGCONTROL (w) register accessor: an alias for `Reg<LINERENDERINGCONTROL_SPEC>`"]
pub type LINERENDERINGCONTROL = crate::Reg<linerenderingcontrol::LINERENDERINGCONTROL_SPEC>;
#[doc = "Line rendering operation control register."]
pub mod linerenderingcontrol;
#[doc = "PERFMEASUREMENTCONTROL (w) register accessor: an alias for `Reg<PERFMEASUREMENTCONTROL_SPEC>`"]
pub type PERFMEASUREMENTCONTROL = crate::Reg<perfmeasurementcontrol::PERFMEASUREMENTCONTROL_SPEC>;
#[doc = "Control register of the performance measurement counters ClockCounter and IdleCounter"]
pub mod perfmeasurementcontrol;
#[doc = "SYNCIDSTATUS0 (r) register accessor: an alias for `Reg<SYNCIDSTATUS0_SPEC>`"]
pub type SYNCIDSTATUS0 = crate::Reg<syncidstatus0::SYNCIDSTATUS0_SPEC>;
#[doc = "Sync ID Status register for task 0."]
pub mod syncidstatus0;
#[doc = "SYNCIDSTATUS1 (r) register accessor: an alias for `Reg<SYNCIDSTATUS1_SPEC>`"]
pub type SYNCIDSTATUS1 = crate::Reg<syncidstatus1::SYNCIDSTATUS1_SPEC>;
#[doc = "Sync ID Status register for task 1."]
pub mod syncidstatus1;
#[doc = "SYNCIDSTATUS2 (r) register accessor: an alias for `Reg<SYNCIDSTATUS2_SPEC>`"]
pub type SYNCIDSTATUS2 = crate::Reg<syncidstatus2::SYNCIDSTATUS2_SPEC>;
#[doc = "Sync ID Status register for task 2."]
pub mod syncidstatus2;
#[doc = "SYNCIDSTATUS3 (r) register accessor: an alias for `Reg<SYNCIDSTATUS3_SPEC>`"]
pub type SYNCIDSTATUS3 = crate::Reg<syncidstatus3::SYNCIDSTATUS3_SPEC>;
#[doc = "Sync ID Status register for task 3."]
pub mod syncidstatus3;
#[doc = "SYNCIDSTATUS4 (r) register accessor: an alias for `Reg<SYNCIDSTATUS4_SPEC>`"]
pub type SYNCIDSTATUS4 = crate::Reg<syncidstatus4::SYNCIDSTATUS4_SPEC>;
#[doc = "Sync ID Status register for task 4."]
pub mod syncidstatus4;
#[doc = "SYNCIDSTATUS5 (r) register accessor: an alias for `Reg<SYNCIDSTATUS5_SPEC>`"]
pub type SYNCIDSTATUS5 = crate::Reg<syncidstatus5::SYNCIDSTATUS5_SPEC>;
#[doc = "Sync ID Status register for task 5."]
pub mod syncidstatus5;
#[doc = "SYNCIDSTATUS6 (r) register accessor: an alias for `Reg<SYNCIDSTATUS6_SPEC>`"]
pub type SYNCIDSTATUS6 = crate::Reg<syncidstatus6::SYNCIDSTATUS6_SPEC>;
#[doc = "Sync ID Status register for task 6."]
pub mod syncidstatus6;
#[doc = "SYNCIDSTATUS7 (r) register accessor: an alias for `Reg<SYNCIDSTATUS7_SPEC>`"]
pub type SYNCIDSTATUS7 = crate::Reg<syncidstatus7::SYNCIDSTATUS7_SPEC>;
#[doc = "Sync ID Status register for task 7."]
pub mod syncidstatus7;
#[doc = "DESIGNATTRIBUTES0 (r) register accessor: an alias for `Reg<DESIGNATTRIBUTES0_SPEC>`"]
pub type DESIGNATTRIBUTES0 = crate::Reg<designattributes0::DESIGNATTRIBUTES0_SPEC>;
#[doc = "This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design."]
pub mod designattributes0;
#[doc = "DESIGNATTRIBUTES1 (r) register accessor: an alias for `Reg<DESIGNATTRIBUTES1_SPEC>`"]
pub type DESIGNATTRIBUTES1 = crate::Reg<designattributes1::DESIGNATTRIBUTES1_SPEC>;
#[doc = "This register contains attributes of Bliteng. They are not parameters since they cannot be easily changed, however here we still have them to provide some static status on design."]
pub mod designattributes1;
#[doc = "PERFMEASUREMENTCLOCKCOUNTERRUNNING (r) register accessor: an alias for `Reg<PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>`"]
pub type PERFMEASUREMENTCLOCKCOUNTERRUNNING =
    crate::Reg<perfmeasurementclockcounterrunning::PERFMEASUREMENTCLOCKCOUNTERRUNNING_SPEC>;
#[doc = "Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set."]
pub mod perfmeasurementclockcounterrunning;
#[doc = "PERFMEASUREMENTCLOCKCOUNTER (r) register accessor: an alias for `Reg<PERFMEASUREMENTCLOCKCOUNTER_SPEC>`"]
pub type PERFMEASUREMENTCLOCKCOUNTER =
    crate::Reg<perfmeasurementclockcounter::PERFMEASUREMENTCLOCKCOUNTER_SPEC>;
#[doc = "Performance measurement register for BlitEng containing clock cycle counter. Measurement is active only if PerfMeasurementEn is set."]
pub mod perfmeasurementclockcounter;
#[doc = "PERFMEASUREMENTIDLECOUNTER (r) register accessor: an alias for `Reg<PERFMEASUREMENTIDLECOUNTER_SPEC>`"]
pub type PERFMEASUREMENTIDLECOUNTER =
    crate::Reg<perfmeasurementidlecounter::PERFMEASUREMENTIDLECOUNTER_SPEC>;
#[doc = "Performance measurement register for BlitEng containing idle cycles. Measurement is active only if PerfMeasurementEn is set."]
pub mod perfmeasurementidlecounter;
#[doc = "SLICEMEASUREMENTSTATUS (r) register accessor: an alias for `Reg<SLICEMEASUREMENTSTATUS_SPEC>`"]
pub type SLICEMEASUREMENTSTATUS = crate::Reg<slicemeasurementstatus::SLICEMEASUREMENTSTATUS_SPEC>;
#[doc = "Performance measurement register for BlitEng containing idle cycles. Measurement is active only if SlicePerfMeasurementEn is set."]
pub mod slicemeasurementstatus;
#[doc = "SLICEMEASUREMENTVALUES (rw) register accessor: an alias for `Reg<SLICEMEASUREMENTVALUES_SPEC>`"]
pub type SLICEMEASUREMENTVALUES = crate::Reg<slicemeasurementvalues::SLICEMEASUREMENTVALUES_SPEC>;
#[doc = "Performance measurement for each slice. Measurement is active only if SlicePerfMeasurementEn is set. This register can be always read,"]
pub mod slicemeasurementvalues;
#[doc = "LBHGLOBAL (rw) register accessor: an alias for `Reg<LBHGLOBAL_SPEC>`"]
pub type LBHGLOBAL = crate::Reg<lbhglobal::LBHGLOBAL_SPEC>;
#[doc = "Settings common for all line-buffer handshake connections."]
pub mod lbhglobal;
#[doc = "LBHGLOBALSTATUS0 (r) register accessor: an alias for `Reg<LBHGLOBALSTATUS0_SPEC>`"]
pub type LBHGLOBALSTATUS0 = crate::Reg<lbhglobalstatus0::LBHGLOBALSTATUS0_SPEC>;
#[doc = "Global status register 0 for the line buffer handshake."]
pub mod lbhglobalstatus0;
#[doc = "LBHGLOBALSTATUS1 (r) register accessor: an alias for `Reg<LBHGLOBALSTATUS1_SPEC>`"]
pub type LBHGLOBALSTATUS1 = crate::Reg<lbhglobalstatus1::LBHGLOBALSTATUS1_SPEC>;
#[doc = "Global status register 1 for the line buffer handshake."]
pub mod lbhglobalstatus1;
#[doc = "LBHGLOBALSTATUS2 (r) register accessor: an alias for `Reg<LBHGLOBALSTATUS2_SPEC>`"]
pub type LBHGLOBALSTATUS2 = crate::Reg<lbhglobalstatus2::LBHGLOBALSTATUS2_SPEC>;
#[doc = "Global status register 2 for the line buffer handshake."]
pub mod lbhglobalstatus2;
#[doc = "LBHLINK0BUFFERCONFIG0 (rw) register accessor: an alias for `Reg<LBHLINK0BUFFERCONFIG0_SPEC>`"]
pub type LBHLINK0BUFFERCONFIG0 = crate::Reg<lbhlink0bufferconfig0::LBHLINK0BUFFERCONFIG0_SPEC>;
#[doc = "Line buffer configuration register 0."]
pub mod lbhlink0bufferconfig0;
#[doc = "LBHLINK0BUFFERCONFIG1 (rw) register accessor: an alias for `Reg<LBHLINK0BUFFERCONFIG1_SPEC>`"]
pub type LBHLINK0BUFFERCONFIG1 = crate::Reg<lbhlink0bufferconfig1::LBHLINK0BUFFERCONFIG1_SPEC>;
#[doc = "Line buffer configuration register 1."]
pub mod lbhlink0bufferconfig1;
#[doc = "LBHLINK0BUFFERSTATUS0 (rw) register accessor: an alias for `Reg<LBHLINK0BUFFERSTATUS0_SPEC>`"]
pub type LBHLINK0BUFFERSTATUS0 = crate::Reg<lbhlink0bufferstatus0::LBHLINK0BUFFERSTATUS0_SPEC>;
#[doc = "Buffer status register 0 for the line buffer handshake."]
pub mod lbhlink0bufferstatus0;
#[doc = "LBHLINK0BUFFERSTATUS1 (r) register accessor: an alias for `Reg<LBHLINK0BUFFERSTATUS1_SPEC>`"]
pub type LBHLINK0BUFFERSTATUS1 = crate::Reg<lbhlink0bufferstatus1::LBHLINK0BUFFERSTATUS1_SPEC>;
#[doc = "Buffer status register 1 for the line buffer handshake."]
pub mod lbhlink0bufferstatus1;
#[doc = "LBHLINK0BUFFERSTATUS2 (r) register accessor: an alias for `Reg<LBHLINK0BUFFERSTATUS2_SPEC>`"]
pub type LBHLINK0BUFFERSTATUS2 = crate::Reg<lbhlink0bufferstatus2::LBHLINK0BUFFERSTATUS2_SPEC>;
#[doc = "Buffer status register 2 for the line buffer handshake."]
pub mod lbhlink0bufferstatus2;
#[doc = "LBHLINK0BUFFERSTATUS3 (r) register accessor: an alias for `Reg<LBHLINK0BUFFERSTATUS3_SPEC>`"]
pub type LBHLINK0BUFFERSTATUS3 = crate::Reg<lbhlink0bufferstatus3::LBHLINK0BUFFERSTATUS3_SPEC>;
#[doc = "Buffer status register 3 for the line buffer handshake."]
pub mod lbhlink0bufferstatus3;
#[doc = "LBHLINK0BUFFERSTATUS4 (r) register accessor: an alias for `Reg<LBHLINK0BUFFERSTATUS4_SPEC>`"]
pub type LBHLINK0BUFFERSTATUS4 = crate::Reg<lbhlink0bufferstatus4::LBHLINK0BUFFERSTATUS4_SPEC>;
#[doc = "Buffer status register 4 for the line buffer handshake."]
pub mod lbhlink0bufferstatus4;
#[doc = "LBHLINK1BUFFERCONFIG0 (rw) register accessor: an alias for `Reg<LBHLINK1BUFFERCONFIG0_SPEC>`"]
pub type LBHLINK1BUFFERCONFIG0 = crate::Reg<lbhlink1bufferconfig0::LBHLINK1BUFFERCONFIG0_SPEC>;
#[doc = "Line buffer configuration register 0."]
pub mod lbhlink1bufferconfig0;
#[doc = "LBHLINK1BUFFERCONFIG1 (rw) register accessor: an alias for `Reg<LBHLINK1BUFFERCONFIG1_SPEC>`"]
pub type LBHLINK1BUFFERCONFIG1 = crate::Reg<lbhlink1bufferconfig1::LBHLINK1BUFFERCONFIG1_SPEC>;
#[doc = "Line buffer configuration register 1."]
pub mod lbhlink1bufferconfig1;
#[doc = "LBHLINK1BUFFERSTATUS0 (rw) register accessor: an alias for `Reg<LBHLINK1BUFFERSTATUS0_SPEC>`"]
pub type LBHLINK1BUFFERSTATUS0 = crate::Reg<lbhlink1bufferstatus0::LBHLINK1BUFFERSTATUS0_SPEC>;
#[doc = "Buffer status register 0 for the line buffer handshake."]
pub mod lbhlink1bufferstatus0;
#[doc = "LBHLINK1BUFFERSTATUS1 (r) register accessor: an alias for `Reg<LBHLINK1BUFFERSTATUS1_SPEC>`"]
pub type LBHLINK1BUFFERSTATUS1 = crate::Reg<lbhlink1bufferstatus1::LBHLINK1BUFFERSTATUS1_SPEC>;
#[doc = "Buffer status register 1 for the line buffer handshake."]
pub mod lbhlink1bufferstatus1;
#[doc = "LBHLINK1BUFFERSTATUS2 (r) register accessor: an alias for `Reg<LBHLINK1BUFFERSTATUS2_SPEC>`"]
pub type LBHLINK1BUFFERSTATUS2 = crate::Reg<lbhlink1bufferstatus2::LBHLINK1BUFFERSTATUS2_SPEC>;
#[doc = "Buffer status register 2 for the line buffer handshake."]
pub mod lbhlink1bufferstatus2;
#[doc = "LBHLINK1BUFFERSTATUS3 (r) register accessor: an alias for `Reg<LBHLINK1BUFFERSTATUS3_SPEC>`"]
pub type LBHLINK1BUFFERSTATUS3 = crate::Reg<lbhlink1bufferstatus3::LBHLINK1BUFFERSTATUS3_SPEC>;
#[doc = "Buffer status register 3 for the line buffer handshake."]
pub mod lbhlink1bufferstatus3;
#[doc = "LBHLINK1BUFFERSTATUS4 (r) register accessor: an alias for `Reg<LBHLINK1BUFFERSTATUS4_SPEC>`"]
pub type LBHLINK1BUFFERSTATUS4 = crate::Reg<lbhlink1bufferstatus4::LBHLINK1BUFFERSTATUS4_SPEC>;
#[doc = "Buffer status register 4 for the line buffer handshake."]
pub mod lbhlink1bufferstatus4;
#[doc = "LBHLINK2BUFFERCONFIG0 (rw) register accessor: an alias for `Reg<LBHLINK2BUFFERCONFIG0_SPEC>`"]
pub type LBHLINK2BUFFERCONFIG0 = crate::Reg<lbhlink2bufferconfig0::LBHLINK2BUFFERCONFIG0_SPEC>;
#[doc = "Line buffer configuration register 0."]
pub mod lbhlink2bufferconfig0;
#[doc = "LBHLINK2BUFFERCONFIG1 (rw) register accessor: an alias for `Reg<LBHLINK2BUFFERCONFIG1_SPEC>`"]
pub type LBHLINK2BUFFERCONFIG1 = crate::Reg<lbhlink2bufferconfig1::LBHLINK2BUFFERCONFIG1_SPEC>;
#[doc = "Line buffer configuration register 1."]
pub mod lbhlink2bufferconfig1;
#[doc = "LBHLINK2BUFFERSTATUS0 (rw) register accessor: an alias for `Reg<LBHLINK2BUFFERSTATUS0_SPEC>`"]
pub type LBHLINK2BUFFERSTATUS0 = crate::Reg<lbhlink2bufferstatus0::LBHLINK2BUFFERSTATUS0_SPEC>;
#[doc = "Buffer status register 0 for the line buffer handshake."]
pub mod lbhlink2bufferstatus0;
#[doc = "LBHLINK2BUFFERSTATUS1 (r) register accessor: an alias for `Reg<LBHLINK2BUFFERSTATUS1_SPEC>`"]
pub type LBHLINK2BUFFERSTATUS1 = crate::Reg<lbhlink2bufferstatus1::LBHLINK2BUFFERSTATUS1_SPEC>;
#[doc = "Buffer status register 1 for the line buffer handshake."]
pub mod lbhlink2bufferstatus1;
#[doc = "LBHLINK2BUFFERSTATUS2 (r) register accessor: an alias for `Reg<LBHLINK2BUFFERSTATUS2_SPEC>`"]
pub type LBHLINK2BUFFERSTATUS2 = crate::Reg<lbhlink2bufferstatus2::LBHLINK2BUFFERSTATUS2_SPEC>;
#[doc = "Buffer status register 2 for the line buffer handshake."]
pub mod lbhlink2bufferstatus2;
#[doc = "LBHLINK2BUFFERSTATUS3 (r) register accessor: an alias for `Reg<LBHLINK2BUFFERSTATUS3_SPEC>`"]
pub type LBHLINK2BUFFERSTATUS3 = crate::Reg<lbhlink2bufferstatus3::LBHLINK2BUFFERSTATUS3_SPEC>;
#[doc = "Buffer status register 3 for the line buffer handshake."]
pub mod lbhlink2bufferstatus3;
#[doc = "LBHLINK2BUFFERSTATUS4 (r) register accessor: an alias for `Reg<LBHLINK2BUFFERSTATUS4_SPEC>`"]
pub type LBHLINK2BUFFERSTATUS4 = crate::Reg<lbhlink2bufferstatus4::LBHLINK2BUFFERSTATUS4_SPEC>;
#[doc = "Buffer status register 4 for the line buffer handshake."]
pub mod lbhlink2bufferstatus4;
#[doc = "LBHLINK3BUFFERCONFIG0 (rw) register accessor: an alias for `Reg<LBHLINK3BUFFERCONFIG0_SPEC>`"]
pub type LBHLINK3BUFFERCONFIG0 = crate::Reg<lbhlink3bufferconfig0::LBHLINK3BUFFERCONFIG0_SPEC>;
#[doc = "Line buffer configuration register 0."]
pub mod lbhlink3bufferconfig0;
#[doc = "LBHLINK3BUFFERCONFIG1 (rw) register accessor: an alias for `Reg<LBHLINK3BUFFERCONFIG1_SPEC>`"]
pub type LBHLINK3BUFFERCONFIG1 = crate::Reg<lbhlink3bufferconfig1::LBHLINK3BUFFERCONFIG1_SPEC>;
#[doc = "Line buffer configuration register 1."]
pub mod lbhlink3bufferconfig1;
#[doc = "LBHLINK3BUFFERSTATUS0 (rw) register accessor: an alias for `Reg<LBHLINK3BUFFERSTATUS0_SPEC>`"]
pub type LBHLINK3BUFFERSTATUS0 = crate::Reg<lbhlink3bufferstatus0::LBHLINK3BUFFERSTATUS0_SPEC>;
#[doc = "Buffer status register 0 for the line buffer handshake."]
pub mod lbhlink3bufferstatus0;
#[doc = "LBHLINK3BUFFERSTATUS1 (r) register accessor: an alias for `Reg<LBHLINK3BUFFERSTATUS1_SPEC>`"]
pub type LBHLINK3BUFFERSTATUS1 = crate::Reg<lbhlink3bufferstatus1::LBHLINK3BUFFERSTATUS1_SPEC>;
#[doc = "Buffer status register 1 for the line buffer handshake."]
pub mod lbhlink3bufferstatus1;
#[doc = "LBHLINK3BUFFERSTATUS2 (r) register accessor: an alias for `Reg<LBHLINK3BUFFERSTATUS2_SPEC>`"]
pub type LBHLINK3BUFFERSTATUS2 = crate::Reg<lbhlink3bufferstatus2::LBHLINK3BUFFERSTATUS2_SPEC>;
#[doc = "Buffer status register 2 for the line buffer handshake."]
pub mod lbhlink3bufferstatus2;
#[doc = "LBHLINK3BUFFERSTATUS3 (r) register accessor: an alias for `Reg<LBHLINK3BUFFERSTATUS3_SPEC>`"]
pub type LBHLINK3BUFFERSTATUS3 = crate::Reg<lbhlink3bufferstatus3::LBHLINK3BUFFERSTATUS3_SPEC>;
#[doc = "Buffer status register 3 for the line buffer handshake."]
pub mod lbhlink3bufferstatus3;
#[doc = "LBHLINK3BUFFERSTATUS4 (r) register accessor: an alias for `Reg<LBHLINK3BUFFERSTATUS4_SPEC>`"]
pub type LBHLINK3BUFFERSTATUS4 = crate::Reg<lbhlink3bufferstatus4::LBHLINK3BUFFERSTATUS4_SPEC>;
#[doc = "Buffer status register 4 for the line buffer handshake."]
pub mod lbhlink3bufferstatus4;
#[doc = "LBHLINK4BUFFERCONFIG0 (rw) register accessor: an alias for `Reg<LBHLINK4BUFFERCONFIG0_SPEC>`"]
pub type LBHLINK4BUFFERCONFIG0 = crate::Reg<lbhlink4bufferconfig0::LBHLINK4BUFFERCONFIG0_SPEC>;
#[doc = "Line buffer configuration register 0."]
pub mod lbhlink4bufferconfig0;
#[doc = "LBHLINK4BUFFERCONFIG1 (rw) register accessor: an alias for `Reg<LBHLINK4BUFFERCONFIG1_SPEC>`"]
pub type LBHLINK4BUFFERCONFIG1 = crate::Reg<lbhlink4bufferconfig1::LBHLINK4BUFFERCONFIG1_SPEC>;
#[doc = "Line buffer configuration register 1."]
pub mod lbhlink4bufferconfig1;
#[doc = "LBHLINK4BUFFERSTATUS0 (rw) register accessor: an alias for `Reg<LBHLINK4BUFFERSTATUS0_SPEC>`"]
pub type LBHLINK4BUFFERSTATUS0 = crate::Reg<lbhlink4bufferstatus0::LBHLINK4BUFFERSTATUS0_SPEC>;
#[doc = "Buffer status register 0 for the line buffer handshake."]
pub mod lbhlink4bufferstatus0;
#[doc = "LBHLINK4BUFFERSTATUS1 (r) register accessor: an alias for `Reg<LBHLINK4BUFFERSTATUS1_SPEC>`"]
pub type LBHLINK4BUFFERSTATUS1 = crate::Reg<lbhlink4bufferstatus1::LBHLINK4BUFFERSTATUS1_SPEC>;
#[doc = "Buffer status register 1 for the line buffer handshake."]
pub mod lbhlink4bufferstatus1;
#[doc = "LBHLINK4BUFFERSTATUS2 (r) register accessor: an alias for `Reg<LBHLINK4BUFFERSTATUS2_SPEC>`"]
pub type LBHLINK4BUFFERSTATUS2 = crate::Reg<lbhlink4bufferstatus2::LBHLINK4BUFFERSTATUS2_SPEC>;
#[doc = "Buffer status register 2 for the line buffer handshake."]
pub mod lbhlink4bufferstatus2;
#[doc = "LBHLINK4BUFFERSTATUS3 (r) register accessor: an alias for `Reg<LBHLINK4BUFFERSTATUS3_SPEC>`"]
pub type LBHLINK4BUFFERSTATUS3 = crate::Reg<lbhlink4bufferstatus3::LBHLINK4BUFFERSTATUS3_SPEC>;
#[doc = "Buffer status register 3 for the line buffer handshake."]
pub mod lbhlink4bufferstatus3;
#[doc = "LBHLINK4BUFFERSTATUS4 (r) register accessor: an alias for `Reg<LBHLINK4BUFFERSTATUS4_SPEC>`"]
pub type LBHLINK4BUFFERSTATUS4 = crate::Reg<lbhlink4bufferstatus4::LBHLINK4BUFFERSTATUS4_SPEC>;
#[doc = "Buffer status register 4 for the line buffer handshake."]
pub mod lbhlink4bufferstatus4;
#[doc = "PALETTE (rw) register accessor: an alias for `Reg<PALETTE_SPEC>`"]
pub type PALETTE = crate::Reg<palette::PALETTE_SPEC>;
#[doc = "Shared Palette Memory."]
pub mod palette;
