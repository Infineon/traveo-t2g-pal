#[doc = r"Register block"]
#[repr(C)]
pub struct LCDBUSIF {
    #[doc = "0x00 - Register to change the protection status of this address block."]
    pub lockunlocklcd: LOCKUNLOCKLCD,
    #[doc = "0x04 - Protection status of this address block."]
    pub lockstatuslcd: LOCKSTATUSLCD,
    #[doc = "0x08 - IP Identifier for this LCDBusIF derivate, needs to be unlocked."]
    pub ipidentifier: IPIDENTIFIER,
    #[doc = "0x0c - LCDBus interface configuration register."]
    pub interfaceconfig: INTERFACECONFIG,
    #[doc = "0x10 - Write timing configuration register. Sum of all fields is write cycle time in module clock cycles."]
    pub writetimingconfig: WRITETIMINGCONFIG,
    #[doc = "0x14 - Read timing configuration register. Sum of all fields is read cycle time in module clock cycles."]
    pub readtimingconfig: READTIMINGCONFIG,
    #[doc = "0x18 - Transfer mapping for command data on LCDBus."]
    pub commandtransfermapping0: COMMANDTRANSFERMAPPING0,
    #[doc = "0x1c - Transfer mapping for command data on LCDBus."]
    pub commandtransfermapping1: COMMANDTRANSFERMAPPING1,
    #[doc = "0x20 - Transfer mapping for command data on LCDBus."]
    pub commandtransfermapping2: COMMANDTRANSFERMAPPING2,
    #[doc = "0x24 - Transfer mapping for data/parameter data on LCDBus."]
    pub datatransfermapping0: DATATRANSFERMAPPING0,
    #[doc = "0x28 - Transfer mapping for data/parameter data on LCDBus."]
    pub datatransfermapping1: DATATRANSFERMAPPING1,
    #[doc = "0x2c - Transfer mapping for data/parameter data on LCDBus."]
    pub datatransfermapping2: DATATRANSFERMAPPING2,
    #[doc = "0x30 - Color component size for color format of LCD controller."]
    pub colorcomponentbits: COLORCOMPONENTBITS,
    #[doc = "0x34 - Color component offset for color format of LCD controller."]
    pub colorcomponentshift: COLORCOMPONENTSHIFT,
    #[doc = "0x38 - Configuration for packing unit."]
    pub destinationattributes: DESTINATIONATTRIBUTES,
    _reserved15: [u8; 0x44],
    #[doc = "0x80 - Register to change the protection status of this address block."]
    pub lockunlockcontrol: LOCKUNLOCKCONTROL,
    #[doc = "0x84 - Protection status of this address block."]
    pub lockstatuscontrol: LOCKSTATUSCONTROL,
    #[doc = "0x88 - LCD Bus Interface interrupt enable register"]
    pub interruptenable: INTERRUPTENABLE,
    #[doc = "0x8c - LCD Bus Interface interrupt preset register"]
    pub interruptpreset: INTERRUPTPRESET,
    #[doc = "0x90 - LCD Bus Interface interrupt clear register"]
    pub interruptclear: INTERRUPTCLEAR,
    #[doc = "0x94 - LCD Bus Interface interrupt status register"]
    pub interruptstatus: INTERRUPTSTATUS,
    #[doc = "0x98 - Sequencer configuration register."]
    pub sequencerconfig: SEQUENCERCONFIG,
    #[doc = "0x9c - Instruction fifo configuration register."]
    pub instructionfifoconfig: INSTRUCTIONFIFOCONFIG,
    #[doc = "0xa0 - Read Channel Configuration Register."]
    pub readchannelconfig: READCHANNELCONFIG,
    #[doc = "0xa4 - Read Channel Buffer Configuration Register."]
    pub readchannelbuffer: READCHANNELBUFFER,
    #[doc = "0xa8 - Read Channel Control Register."]
    pub readchannelcontrol: READCHANNELCONTROL,
    #[doc = "0xac - Write Channel Configuration Register."]
    pub writechannelconfig: WRITECHANNELCONFIG,
    #[doc = "0xb0 - Write Channel Buffer Configuration Register."]
    pub writechannelbuffer: WRITECHANNELBUFFER,
    #[doc = "0xb4 - Software Reset Register."]
    pub softwarereset: SOFTWARERESET,
    #[doc = "0xb8 - Sequencer synchronization register."]
    pub sequencersync: SEQUENCERSYNC,
    #[doc = "0xbc - Status of the internal sequencer."]
    pub sequencerstatus: SEQUENCERSTATUS,
    #[doc = "0xc0 - Transfer remaining of current executed command."]
    pub sequencertransferstatus: SEQUENCERTRANSFERSTATUS,
    #[doc = "0xc4 - Instruction fifo status register."]
    pub instrfifostatus: INSTRFIFOSTATUS,
    #[doc = "0xc8 - Read Channel Status Register."]
    pub readchannelstatus: READCHANNELSTATUS,
    #[doc = "0xcc - Write Channel Status Register."]
    pub writechannelstatus: WRITECHANNELSTATUS,
    _reserved35: [u8; 0x30],
    #[doc = "0x100 - Register to change the protection status of the fifo address blocks."]
    pub lockunlockfifo: LOCKUNLOCKFIFO,
    #[doc = "0x104 - Protection status of this address block."]
    pub lockstatusfifo: LOCKSTATUSFIFO,
    #[doc = "0x108 - Reception fifo configuration register."]
    pub receptionfifoconfig: RECEPTIONFIFOCONFIG,
    #[doc = "0x10c - Reception fifo control register."]
    pub rxfifocontrol: RXFIFOCONTROL,
    #[doc = "0x110 - Reception fifo status register."]
    pub rxfifostatus: RXFIFOSTATUS,
    _reserved40: [u8; 0xec],
    #[doc = "0x200..0x280 - Instruction Fifo."]
    pub instructionfifo: [INSTRUCTIONFIFO; 32],
    _reserved41: [u8; 0x80],
    #[doc = "0x300..0x340 - Reception Fifo."]
    pub rxfifo: [RXFIFO; 16],
    _reserved42: [u8; 0xc0],
    #[doc = "0x400..0x800 - Color lookup table memory."]
    pub colorlookuptable: [COLORLOOKUPTABLE; 256],
}
#[doc = "LOCKUNLOCKLCD (rw) register accessor: an alias for `Reg<LOCKUNLOCKLCD_SPEC>`"]
pub type LOCKUNLOCKLCD = crate::Reg<lockunlocklcd::LOCKUNLOCKLCD_SPEC>;
#[doc = "Register to change the protection status of this address block."]
pub mod lockunlocklcd;
#[doc = "LOCKSTATUSLCD (r) register accessor: an alias for `Reg<LOCKSTATUSLCD_SPEC>`"]
pub type LOCKSTATUSLCD = crate::Reg<lockstatuslcd::LOCKSTATUSLCD_SPEC>;
#[doc = "Protection status of this address block."]
pub mod lockstatuslcd;
#[doc = "IPIDENTIFIER (rw) register accessor: an alias for `Reg<IPIDENTIFIER_SPEC>`"]
pub type IPIDENTIFIER = crate::Reg<ipidentifier::IPIDENTIFIER_SPEC>;
#[doc = "IP Identifier for this LCDBusIF derivate, needs to be unlocked."]
pub mod ipidentifier;
#[doc = "INTERFACECONFIG (rw) register accessor: an alias for `Reg<INTERFACECONFIG_SPEC>`"]
pub type INTERFACECONFIG = crate::Reg<interfaceconfig::INTERFACECONFIG_SPEC>;
#[doc = "LCDBus interface configuration register."]
pub mod interfaceconfig;
#[doc = "WRITETIMINGCONFIG (rw) register accessor: an alias for `Reg<WRITETIMINGCONFIG_SPEC>`"]
pub type WRITETIMINGCONFIG = crate::Reg<writetimingconfig::WRITETIMINGCONFIG_SPEC>;
#[doc = "Write timing configuration register. Sum of all fields is write cycle time in module clock cycles."]
pub mod writetimingconfig;
#[doc = "READTIMINGCONFIG (rw) register accessor: an alias for `Reg<READTIMINGCONFIG_SPEC>`"]
pub type READTIMINGCONFIG = crate::Reg<readtimingconfig::READTIMINGCONFIG_SPEC>;
#[doc = "Read timing configuration register. Sum of all fields is read cycle time in module clock cycles."]
pub mod readtimingconfig;
#[doc = "COMMANDTRANSFERMAPPING0 (rw) register accessor: an alias for `Reg<COMMANDTRANSFERMAPPING0_SPEC>`"]
pub type COMMANDTRANSFERMAPPING0 =
    crate::Reg<commandtransfermapping0::COMMANDTRANSFERMAPPING0_SPEC>;
#[doc = "Transfer mapping for command data on LCDBus."]
pub mod commandtransfermapping0;
#[doc = "COMMANDTRANSFERMAPPING1 (rw) register accessor: an alias for `Reg<COMMANDTRANSFERMAPPING1_SPEC>`"]
pub type COMMANDTRANSFERMAPPING1 =
    crate::Reg<commandtransfermapping1::COMMANDTRANSFERMAPPING1_SPEC>;
#[doc = "Transfer mapping for command data on LCDBus."]
pub mod commandtransfermapping1;
#[doc = "COMMANDTRANSFERMAPPING2 (rw) register accessor: an alias for `Reg<COMMANDTRANSFERMAPPING2_SPEC>`"]
pub type COMMANDTRANSFERMAPPING2 =
    crate::Reg<commandtransfermapping2::COMMANDTRANSFERMAPPING2_SPEC>;
#[doc = "Transfer mapping for command data on LCDBus."]
pub mod commandtransfermapping2;
#[doc = "DATATRANSFERMAPPING0 (rw) register accessor: an alias for `Reg<DATATRANSFERMAPPING0_SPEC>`"]
pub type DATATRANSFERMAPPING0 = crate::Reg<datatransfermapping0::DATATRANSFERMAPPING0_SPEC>;
#[doc = "Transfer mapping for data/parameter data on LCDBus."]
pub mod datatransfermapping0;
#[doc = "DATATRANSFERMAPPING1 (rw) register accessor: an alias for `Reg<DATATRANSFERMAPPING1_SPEC>`"]
pub type DATATRANSFERMAPPING1 = crate::Reg<datatransfermapping1::DATATRANSFERMAPPING1_SPEC>;
#[doc = "Transfer mapping for data/parameter data on LCDBus."]
pub mod datatransfermapping1;
#[doc = "DATATRANSFERMAPPING2 (rw) register accessor: an alias for `Reg<DATATRANSFERMAPPING2_SPEC>`"]
pub type DATATRANSFERMAPPING2 = crate::Reg<datatransfermapping2::DATATRANSFERMAPPING2_SPEC>;
#[doc = "Transfer mapping for data/parameter data on LCDBus."]
pub mod datatransfermapping2;
#[doc = "COLORCOMPONENTBITS (rw) register accessor: an alias for `Reg<COLORCOMPONENTBITS_SPEC>`"]
pub type COLORCOMPONENTBITS = crate::Reg<colorcomponentbits::COLORCOMPONENTBITS_SPEC>;
#[doc = "Color component size for color format of LCD controller."]
pub mod colorcomponentbits;
#[doc = "COLORCOMPONENTSHIFT (rw) register accessor: an alias for `Reg<COLORCOMPONENTSHIFT_SPEC>`"]
pub type COLORCOMPONENTSHIFT = crate::Reg<colorcomponentshift::COLORCOMPONENTSHIFT_SPEC>;
#[doc = "Color component offset for color format of LCD controller."]
pub mod colorcomponentshift;
#[doc = "DESTINATIONATTRIBUTES (rw) register accessor: an alias for `Reg<DESTINATIONATTRIBUTES_SPEC>`"]
pub type DESTINATIONATTRIBUTES = crate::Reg<destinationattributes::DESTINATIONATTRIBUTES_SPEC>;
#[doc = "Configuration for packing unit."]
pub mod destinationattributes;
#[doc = "LOCKUNLOCKCONTROL (rw) register accessor: an alias for `Reg<LOCKUNLOCKCONTROL_SPEC>`"]
pub type LOCKUNLOCKCONTROL = crate::Reg<lockunlockcontrol::LOCKUNLOCKCONTROL_SPEC>;
#[doc = "Register to change the protection status of this address block."]
pub mod lockunlockcontrol;
#[doc = "LOCKSTATUSCONTROL (r) register accessor: an alias for `Reg<LOCKSTATUSCONTROL_SPEC>`"]
pub type LOCKSTATUSCONTROL = crate::Reg<lockstatuscontrol::LOCKSTATUSCONTROL_SPEC>;
#[doc = "Protection status of this address block."]
pub mod lockstatuscontrol;
#[doc = "INTERRUPTENABLE (rw) register accessor: an alias for `Reg<INTERRUPTENABLE_SPEC>`"]
pub type INTERRUPTENABLE = crate::Reg<interruptenable::INTERRUPTENABLE_SPEC>;
#[doc = "LCD Bus Interface interrupt enable register"]
pub mod interruptenable;
#[doc = "INTERRUPTPRESET (w) register accessor: an alias for `Reg<INTERRUPTPRESET_SPEC>`"]
pub type INTERRUPTPRESET = crate::Reg<interruptpreset::INTERRUPTPRESET_SPEC>;
#[doc = "LCD Bus Interface interrupt preset register"]
pub mod interruptpreset;
#[doc = "INTERRUPTCLEAR (w) register accessor: an alias for `Reg<INTERRUPTCLEAR_SPEC>`"]
pub type INTERRUPTCLEAR = crate::Reg<interruptclear::INTERRUPTCLEAR_SPEC>;
#[doc = "LCD Bus Interface interrupt clear register"]
pub mod interruptclear;
#[doc = "INTERRUPTSTATUS (r) register accessor: an alias for `Reg<INTERRUPTSTATUS_SPEC>`"]
pub type INTERRUPTSTATUS = crate::Reg<interruptstatus::INTERRUPTSTATUS_SPEC>;
#[doc = "LCD Bus Interface interrupt status register"]
pub mod interruptstatus;
#[doc = "SEQUENCERCONFIG (rw) register accessor: an alias for `Reg<SEQUENCERCONFIG_SPEC>`"]
pub type SEQUENCERCONFIG = crate::Reg<sequencerconfig::SEQUENCERCONFIG_SPEC>;
#[doc = "Sequencer configuration register."]
pub mod sequencerconfig;
#[doc = "INSTRUCTIONFIFOCONFIG (rw) register accessor: an alias for `Reg<INSTRUCTIONFIFOCONFIG_SPEC>`"]
pub type INSTRUCTIONFIFOCONFIG = crate::Reg<instructionfifoconfig::INSTRUCTIONFIFOCONFIG_SPEC>;
#[doc = "Instruction fifo configuration register."]
pub mod instructionfifoconfig;
#[doc = "READCHANNELCONFIG (rw) register accessor: an alias for `Reg<READCHANNELCONFIG_SPEC>`"]
pub type READCHANNELCONFIG = crate::Reg<readchannelconfig::READCHANNELCONFIG_SPEC>;
#[doc = "Read Channel Configuration Register."]
pub mod readchannelconfig;
#[doc = "READCHANNELBUFFER (rw) register accessor: an alias for `Reg<READCHANNELBUFFER_SPEC>`"]
pub type READCHANNELBUFFER = crate::Reg<readchannelbuffer::READCHANNELBUFFER_SPEC>;
#[doc = "Read Channel Buffer Configuration Register."]
pub mod readchannelbuffer;
#[doc = "READCHANNELCONTROL (w) register accessor: an alias for `Reg<READCHANNELCONTROL_SPEC>`"]
pub type READCHANNELCONTROL = crate::Reg<readchannelcontrol::READCHANNELCONTROL_SPEC>;
#[doc = "Read Channel Control Register."]
pub mod readchannelcontrol;
#[doc = "WRITECHANNELCONFIG (rw) register accessor: an alias for `Reg<WRITECHANNELCONFIG_SPEC>`"]
pub type WRITECHANNELCONFIG = crate::Reg<writechannelconfig::WRITECHANNELCONFIG_SPEC>;
#[doc = "Write Channel Configuration Register."]
pub mod writechannelconfig;
#[doc = "WRITECHANNELBUFFER (rw) register accessor: an alias for `Reg<WRITECHANNELBUFFER_SPEC>`"]
pub type WRITECHANNELBUFFER = crate::Reg<writechannelbuffer::WRITECHANNELBUFFER_SPEC>;
#[doc = "Write Channel Buffer Configuration Register."]
pub mod writechannelbuffer;
#[doc = "SOFTWARERESET (w) register accessor: an alias for `Reg<SOFTWARERESET_SPEC>`"]
pub type SOFTWARERESET = crate::Reg<softwarereset::SOFTWARERESET_SPEC>;
#[doc = "Software Reset Register."]
pub mod softwarereset;
#[doc = "SEQUENCERSYNC (w) register accessor: an alias for `Reg<SEQUENCERSYNC_SPEC>`"]
pub type SEQUENCERSYNC = crate::Reg<sequencersync::SEQUENCERSYNC_SPEC>;
#[doc = "Sequencer synchronization register."]
pub mod sequencersync;
#[doc = "SEQUENCERSTATUS (r) register accessor: an alias for `Reg<SEQUENCERSTATUS_SPEC>`"]
pub type SEQUENCERSTATUS = crate::Reg<sequencerstatus::SEQUENCERSTATUS_SPEC>;
#[doc = "Status of the internal sequencer."]
pub mod sequencerstatus;
#[doc = "SEQUENCERTRANSFERSTATUS (r) register accessor: an alias for `Reg<SEQUENCERTRANSFERSTATUS_SPEC>`"]
pub type SEQUENCERTRANSFERSTATUS =
    crate::Reg<sequencertransferstatus::SEQUENCERTRANSFERSTATUS_SPEC>;
#[doc = "Transfer remaining of current executed command."]
pub mod sequencertransferstatus;
#[doc = "INSTRFIFOSTATUS (r) register accessor: an alias for `Reg<INSTRFIFOSTATUS_SPEC>`"]
pub type INSTRFIFOSTATUS = crate::Reg<instrfifostatus::INSTRFIFOSTATUS_SPEC>;
#[doc = "Instruction fifo status register."]
pub mod instrfifostatus;
#[doc = "READCHANNELSTATUS (r) register accessor: an alias for `Reg<READCHANNELSTATUS_SPEC>`"]
pub type READCHANNELSTATUS = crate::Reg<readchannelstatus::READCHANNELSTATUS_SPEC>;
#[doc = "Read Channel Status Register."]
pub mod readchannelstatus;
#[doc = "WRITECHANNELSTATUS (r) register accessor: an alias for `Reg<WRITECHANNELSTATUS_SPEC>`"]
pub type WRITECHANNELSTATUS = crate::Reg<writechannelstatus::WRITECHANNELSTATUS_SPEC>;
#[doc = "Write Channel Status Register."]
pub mod writechannelstatus;
#[doc = "LOCKUNLOCKFIFO (rw) register accessor: an alias for `Reg<LOCKUNLOCKFIFO_SPEC>`"]
pub type LOCKUNLOCKFIFO = crate::Reg<lockunlockfifo::LOCKUNLOCKFIFO_SPEC>;
#[doc = "Register to change the protection status of the fifo address blocks."]
pub mod lockunlockfifo;
#[doc = "LOCKSTATUSFIFO (r) register accessor: an alias for `Reg<LOCKSTATUSFIFO_SPEC>`"]
pub type LOCKSTATUSFIFO = crate::Reg<lockstatusfifo::LOCKSTATUSFIFO_SPEC>;
#[doc = "Protection status of this address block."]
pub mod lockstatusfifo;
#[doc = "RECEPTIONFIFOCONFIG (rw) register accessor: an alias for `Reg<RECEPTIONFIFOCONFIG_SPEC>`"]
pub type RECEPTIONFIFOCONFIG = crate::Reg<receptionfifoconfig::RECEPTIONFIFOCONFIG_SPEC>;
#[doc = "Reception fifo configuration register."]
pub mod receptionfifoconfig;
#[doc = "RXFIFOCONTROL (w) register accessor: an alias for `Reg<RXFIFOCONTROL_SPEC>`"]
pub type RXFIFOCONTROL = crate::Reg<rxfifocontrol::RXFIFOCONTROL_SPEC>;
#[doc = "Reception fifo control register."]
pub mod rxfifocontrol;
#[doc = "RXFIFOSTATUS (r) register accessor: an alias for `Reg<RXFIFOSTATUS_SPEC>`"]
pub type RXFIFOSTATUS = crate::Reg<rxfifostatus::RXFIFOSTATUS_SPEC>;
#[doc = "Reception fifo status register."]
pub mod rxfifostatus;
#[doc = "INSTRUCTIONFIFO (rw) register accessor: an alias for `Reg<INSTRUCTIONFIFO_SPEC>`"]
pub type INSTRUCTIONFIFO = crate::Reg<instructionfifo::INSTRUCTIONFIFO_SPEC>;
#[doc = "Instruction Fifo."]
pub mod instructionfifo;
#[doc = "RXFIFO (rw) register accessor: an alias for `Reg<RXFIFO_SPEC>`"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "Reception Fifo."]
pub mod rxfifo;
#[doc = "COLORLOOKUPTABLE (rw) register accessor: an alias for `Reg<COLORLOOKUPTABLE_SPEC>`"]
pub type COLORLOOKUPTABLE = crate::Reg<colorlookuptable::COLORLOOKUPTABLE_SPEC>;
#[doc = "Color lookup table memory."]
pub mod colorlookuptable;
