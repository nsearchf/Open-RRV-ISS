// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


const CSR_FFLAGS: u16 = 0x1;
const CSR_FRM: u16 = 0x2;
const CSR_FCSR: u16 = 0x3;
const CSR_VSTART: u16 = 0x8;
const CSR_VXSAT: u16 = 0x9;
const CSR_VXRM: u16 = 0xa;
const CSR_VCSR: u16 = 0xf;
const CSR_SSP: u16 = 0x11;
const CSR_SEED: u16 = 0x15;
const CSR_JVT: u16 = 0x17;
const CSR_CYCLE: u16 = 0xc00;
const CSR_TIME: u16 = 0xc01;
const CSR_INSTRET: u16 = 0xc02;
const CSR_HPMCOUNTER3: u16 = 0xc03;
const CSR_HPMCOUNTER4: u16 = 0xc04;
const CSR_HPMCOUNTER5: u16 = 0xc05;
const CSR_HPMCOUNTER6: u16 = 0xc06;
const CSR_HPMCOUNTER7: u16 = 0xc07;
const CSR_HPMCOUNTER8: u16 = 0xc08;
const CSR_HPMCOUNTER9: u16 = 0xc09;
const CSR_HPMCOUNTER10: u16 = 0xc0a;
const CSR_HPMCOUNTER11: u16 = 0xc0b;
const CSR_HPMCOUNTER12: u16 = 0xc0c;
const CSR_HPMCOUNTER13: u16 = 0xc0d;
const CSR_HPMCOUNTER14: u16 = 0xc0e;
const CSR_HPMCOUNTER15: u16 = 0xc0f;
const CSR_HPMCOUNTER16: u16 = 0xc10;
const CSR_HPMCOUNTER17: u16 = 0xc11;
const CSR_HPMCOUNTER18: u16 = 0xc12;
const CSR_HPMCOUNTER19: u16 = 0xc13;
const CSR_HPMCOUNTER20: u16 = 0xc14;
const CSR_HPMCOUNTER21: u16 = 0xc15;
const CSR_HPMCOUNTER22: u16 = 0xc16;
const CSR_HPMCOUNTER23: u16 = 0xc17;
const CSR_HPMCOUNTER24: u16 = 0xc18;
const CSR_HPMCOUNTER25: u16 = 0xc19;
const CSR_HPMCOUNTER26: u16 = 0xc1a;
const CSR_HPMCOUNTER27: u16 = 0xc1b;
const CSR_HPMCOUNTER28: u16 = 0xc1c;
const CSR_HPMCOUNTER29: u16 = 0xc1d;
const CSR_HPMCOUNTER30: u16 = 0xc1e;
const CSR_HPMCOUNTER31: u16 = 0xc1f;
const CSR_VL: u16 = 0xc20;
const CSR_VTYPE: u16 = 0xc21;
const CSR_VLENB: u16 = 0xc22;
const CSR_SSTATUS: u16 = 0x100;
const CSR_SEDELEG: u16 = 0x102;
const CSR_SIDELEG: u16 = 0x103;
const CSR_SIE: u16 = 0x104;
const CSR_STVEC: u16 = 0x105;
const CSR_SCOUNTEREN: u16 = 0x106;
const CSR_SENVCFG: u16 = 0x10a;
const CSR_SSTATEEN0: u16 = 0x10c;
const CSR_SSTATEEN1: u16 = 0x10d;
const CSR_SSTATEEN2: u16 = 0x10e;
const CSR_SSTATEEN3: u16 = 0x10f;
const CSR_SCOUNTINHIBIT: u16 = 0x120;
const CSR_SSCRATCH: u16 = 0x140;
const CSR_SEPC: u16 = 0x141;
const CSR_SCAUSE: u16 = 0x142;
const CSR_STVAL: u16 = 0x143;
const CSR_SIP: u16 = 0x144;
const CSR_STIMECMP: u16 = 0x14d;
const CSR_SCTRCTL: u16 = 0x14e;
const CSR_SCTRSTATUS: u16 = 0x14f;
const CSR_SISELECT: u16 = 0x150;
const CSR_SIREG: u16 = 0x151;
const CSR_SIREG2: u16 = 0x152;
const CSR_SIREG3: u16 = 0x153;
const CSR_SIREG4: u16 = 0x155;
const CSR_SIREG5: u16 = 0x156;
const CSR_SIREG6: u16 = 0x157;
const CSR_STOPEI: u16 = 0x15c;
const CSR_SCTRDEPTH: u16 = 0x15f;
const CSR_SATP: u16 = 0x180;
const CSR_SRMCFG: u16 = 0x181;
const CSR_SCONTEXT: u16 = 0x5a8;
const CSR_VSSTATUS: u16 = 0x200;
const CSR_VSIE: u16 = 0x204;
const CSR_VSTVEC: u16 = 0x205;
const CSR_VSSCRATCH: u16 = 0x240;
const CSR_VSEPC: u16 = 0x241;
const CSR_VSCAUSE: u16 = 0x242;
const CSR_VSTVAL: u16 = 0x243;
const CSR_VSIP: u16 = 0x244;
const CSR_VSTIMECMP: u16 = 0x24d;
const CSR_VSCTRCTL: u16 = 0x24e;
const CSR_VSISELECT: u16 = 0x250;
const CSR_VSIREG: u16 = 0x251;
const CSR_VSIREG2: u16 = 0x252;
const CSR_VSIREG3: u16 = 0x253;
const CSR_VSIREG4: u16 = 0x255;
const CSR_VSIREG5: u16 = 0x256;
const CSR_VSIREG6: u16 = 0x257;
const CSR_VSTOPEI: u16 = 0x25c;
const CSR_VSATP: u16 = 0x280;
const CSR_HSTATUS: u16 = 0x600;
const CSR_HEDELEG: u16 = 0x602;
const CSR_HIDELEG: u16 = 0x603;
const CSR_HIE: u16 = 0x604;
const CSR_HTIMEDELTA: u16 = 0x605;
const CSR_HCOUNTEREN: u16 = 0x606;
const CSR_HGEIE: u16 = 0x607;
const CSR_HVIEN: u16 = 0x608;
const CSR_HVICTL: u16 = 0x609;
const CSR_HENVCFG: u16 = 0x60a;
const CSR_HSTATEEN0: u16 = 0x60c;
const CSR_HSTATEEN1: u16 = 0x60d;
const CSR_HSTATEEN2: u16 = 0x60e;
const CSR_HSTATEEN3: u16 = 0x60f;
const CSR_HTVAL: u16 = 0x643;
const CSR_HIP: u16 = 0x644;
const CSR_HVIP: u16 = 0x645;
const CSR_HVIPRIO1: u16 = 0x646;
const CSR_HVIPRIO2: u16 = 0x647;
const CSR_HTINST: u16 = 0x64a;
const CSR_HGATP: u16 = 0x680;
const CSR_HCONTEXT: u16 = 0x6a8;
const CSR_HGEIP: u16 = 0xe12;
const CSR_VSTOPI: u16 = 0xeb0;
const CSR_SCOUNTOVF: u16 = 0xda0;
const CSR_STOPI: u16 = 0xdb0;
const CSR_UTVT: u16 = 0x7;
const CSR_UNXTI: u16 = 0x45;
const CSR_UINTSTATUS: u16 = 0x46;
const CSR_USCRATCHCSW: u16 = 0x48;
const CSR_USCRATCHCSWL: u16 = 0x49;
const CSR_STVT: u16 = 0x107;
const CSR_SNXTI: u16 = 0x145;
const CSR_SINTSTATUS: u16 = 0x146;
const CSR_SSCRATCHCSW: u16 = 0x148;
const CSR_SSCRATCHCSWL: u16 = 0x149;
const CSR_MTVT: u16 = 0x307;
const CSR_MNXTI: u16 = 0x345;
const CSR_MINTSTATUS: u16 = 0x346;
const CSR_MSCRATCHCSW: u16 = 0x348;
const CSR_MSCRATCHCSWL: u16 = 0x349;
const CSR_MSTATUS: u16 = 0x300;
const CSR_MISA: u16 = 0x301;
const CSR_MEDELEG: u16 = 0x302;
const CSR_MIDELEG: u16 = 0x303;
const CSR_MIE: u16 = 0x304;
const CSR_MTVEC: u16 = 0x305;
const CSR_MCOUNTEREN: u16 = 0x306;
const CSR_MVIEN: u16 = 0x308;
const CSR_MVIP: u16 = 0x309;
const CSR_MENVCFG: u16 = 0x30a;
const CSR_MSTATEEN0: u16 = 0x30c;
const CSR_MSTATEEN1: u16 = 0x30d;
const CSR_MSTATEEN2: u16 = 0x30e;
const CSR_MSTATEEN3: u16 = 0x30f;
const CSR_MCOUNTINHIBIT: u16 = 0x320;
const CSR_MSCRATCH: u16 = 0x340;
const CSR_MEPC: u16 = 0x341;
const CSR_MCAUSE: u16 = 0x342;
const CSR_MTVAL: u16 = 0x343;
const CSR_MIP: u16 = 0x344;
const CSR_MTINST: u16 = 0x34a;
const CSR_MTVAL2: u16 = 0x34b;
const CSR_MCTRCTL: u16 = 0x34e;
const CSR_MISELECT: u16 = 0x350;
const CSR_MIREG: u16 = 0x351;
const CSR_MIREG2: u16 = 0x352;
const CSR_MIREG3: u16 = 0x353;
const CSR_MIREG4: u16 = 0x355;
const CSR_MIREG5: u16 = 0x356;
const CSR_MIREG6: u16 = 0x357;
const CSR_MTOPEI: u16 = 0x35c;
const CSR_PMPCFG0: u16 = 0x3a0;
const CSR_PMPCFG1: u16 = 0x3a1;
const CSR_PMPCFG2: u16 = 0x3a2;
const CSR_PMPCFG3: u16 = 0x3a3;
const CSR_PMPCFG4: u16 = 0x3a4;
const CSR_PMPCFG5: u16 = 0x3a5;
const CSR_PMPCFG6: u16 = 0x3a6;
const CSR_PMPCFG7: u16 = 0x3a7;
const CSR_PMPCFG8: u16 = 0x3a8;
const CSR_PMPCFG9: u16 = 0x3a9;
const CSR_PMPCFG10: u16 = 0x3aa;
const CSR_PMPCFG11: u16 = 0x3ab;
const CSR_PMPCFG12: u16 = 0x3ac;
const CSR_PMPCFG13: u16 = 0x3ad;
const CSR_PMPCFG14: u16 = 0x3ae;
const CSR_PMPCFG15: u16 = 0x3af;
const CSR_PMPADDR0: u16 = 0x3b0;
const CSR_PMPADDR1: u16 = 0x3b1;
const CSR_PMPADDR2: u16 = 0x3b2;
const CSR_PMPADDR3: u16 = 0x3b3;
const CSR_PMPADDR4: u16 = 0x3b4;
const CSR_PMPADDR5: u16 = 0x3b5;
const CSR_PMPADDR6: u16 = 0x3b6;
const CSR_PMPADDR7: u16 = 0x3b7;
const CSR_PMPADDR8: u16 = 0x3b8;
const CSR_PMPADDR9: u16 = 0x3b9;
const CSR_PMPADDR10: u16 = 0x3ba;
const CSR_PMPADDR11: u16 = 0x3bb;
const CSR_PMPADDR12: u16 = 0x3bc;
const CSR_PMPADDR13: u16 = 0x3bd;
const CSR_PMPADDR14: u16 = 0x3be;
const CSR_PMPADDR15: u16 = 0x3bf;
const CSR_PMPADDR16: u16 = 0x3c0;
const CSR_PMPADDR17: u16 = 0x3c1;
const CSR_PMPADDR18: u16 = 0x3c2;
const CSR_PMPADDR19: u16 = 0x3c3;
const CSR_PMPADDR20: u16 = 0x3c4;
const CSR_PMPADDR21: u16 = 0x3c5;
const CSR_PMPADDR22: u16 = 0x3c6;
const CSR_PMPADDR23: u16 = 0x3c7;
const CSR_PMPADDR24: u16 = 0x3c8;
const CSR_PMPADDR25: u16 = 0x3c9;
const CSR_PMPADDR26: u16 = 0x3ca;
const CSR_PMPADDR27: u16 = 0x3cb;
const CSR_PMPADDR28: u16 = 0x3cc;
const CSR_PMPADDR29: u16 = 0x3cd;
const CSR_PMPADDR30: u16 = 0x3ce;
const CSR_PMPADDR31: u16 = 0x3cf;
const CSR_PMPADDR32: u16 = 0x3d0;
const CSR_PMPADDR33: u16 = 0x3d1;
const CSR_PMPADDR34: u16 = 0x3d2;
const CSR_PMPADDR35: u16 = 0x3d3;
const CSR_PMPADDR36: u16 = 0x3d4;
const CSR_PMPADDR37: u16 = 0x3d5;
const CSR_PMPADDR38: u16 = 0x3d6;
const CSR_PMPADDR39: u16 = 0x3d7;
const CSR_PMPADDR40: u16 = 0x3d8;
const CSR_PMPADDR41: u16 = 0x3d9;
const CSR_PMPADDR42: u16 = 0x3da;
const CSR_PMPADDR43: u16 = 0x3db;
const CSR_PMPADDR44: u16 = 0x3dc;
const CSR_PMPADDR45: u16 = 0x3dd;
const CSR_PMPADDR46: u16 = 0x3de;
const CSR_PMPADDR47: u16 = 0x3df;
const CSR_PMPADDR48: u16 = 0x3e0;
const CSR_PMPADDR49: u16 = 0x3e1;
const CSR_PMPADDR50: u16 = 0x3e2;
const CSR_PMPADDR51: u16 = 0x3e3;
const CSR_PMPADDR52: u16 = 0x3e4;
const CSR_PMPADDR53: u16 = 0x3e5;
const CSR_PMPADDR54: u16 = 0x3e6;
const CSR_PMPADDR55: u16 = 0x3e7;
const CSR_PMPADDR56: u16 = 0x3e8;
const CSR_PMPADDR57: u16 = 0x3e9;
const CSR_PMPADDR58: u16 = 0x3ea;
const CSR_PMPADDR59: u16 = 0x3eb;
const CSR_PMPADDR60: u16 = 0x3ec;
const CSR_PMPADDR61: u16 = 0x3ed;
const CSR_PMPADDR62: u16 = 0x3ee;
const CSR_PMPADDR63: u16 = 0x3ef;
const CSR_MSECCFG: u16 = 0x747;
const CSR_TSELECT: u16 = 0x7a0;
const CSR_TDATA1: u16 = 0x7a1;
const CSR_TDATA2: u16 = 0x7a2;
const CSR_TDATA3: u16 = 0x7a3;
const CSR_TINFO: u16 = 0x7a4;
const CSR_TCONTROL: u16 = 0x7a5;
const CSR_MCONTEXT: u16 = 0x7a8;
const CSR_MSCONTEXT: u16 = 0x7aa;
const CSR_DCSR: u16 = 0x7b0;
const CSR_DPC: u16 = 0x7b1;
const CSR_DSCRATCH0: u16 = 0x7b2;
const CSR_DSCRATCH1: u16 = 0x7b3;
const CSR_MCYCLE: u16 = 0xb00;
const CSR_MINSTRET: u16 = 0xb02;
const CSR_MHPMCOUNTER3: u16 = 0xb03;
const CSR_MHPMCOUNTER4: u16 = 0xb04;
const CSR_MHPMCOUNTER5: u16 = 0xb05;
const CSR_MHPMCOUNTER6: u16 = 0xb06;
const CSR_MHPMCOUNTER7: u16 = 0xb07;
const CSR_MHPMCOUNTER8: u16 = 0xb08;
const CSR_MHPMCOUNTER9: u16 = 0xb09;
const CSR_MHPMCOUNTER10: u16 = 0xb0a;
const CSR_MHPMCOUNTER11: u16 = 0xb0b;
const CSR_MHPMCOUNTER12: u16 = 0xb0c;
const CSR_MHPMCOUNTER13: u16 = 0xb0d;
const CSR_MHPMCOUNTER14: u16 = 0xb0e;
const CSR_MHPMCOUNTER15: u16 = 0xb0f;
const CSR_MHPMCOUNTER16: u16 = 0xb10;
const CSR_MHPMCOUNTER17: u16 = 0xb11;
const CSR_MHPMCOUNTER18: u16 = 0xb12;
const CSR_MHPMCOUNTER19: u16 = 0xb13;
const CSR_MHPMCOUNTER20: u16 = 0xb14;
const CSR_MHPMCOUNTER21: u16 = 0xb15;
const CSR_MHPMCOUNTER22: u16 = 0xb16;
const CSR_MHPMCOUNTER23: u16 = 0xb17;
const CSR_MHPMCOUNTER24: u16 = 0xb18;
const CSR_MHPMCOUNTER25: u16 = 0xb19;
const CSR_MHPMCOUNTER26: u16 = 0xb1a;
const CSR_MHPMCOUNTER27: u16 = 0xb1b;
const CSR_MHPMCOUNTER28: u16 = 0xb1c;
const CSR_MHPMCOUNTER29: u16 = 0xb1d;
const CSR_MHPMCOUNTER30: u16 = 0xb1e;
const CSR_MHPMCOUNTER31: u16 = 0xb1f;
const CSR_MCYCLECFG: u16 = 0x321;
const CSR_MINSTRETCFG: u16 = 0x322;
const CSR_MHPMEVENT3: u16 = 0x323;
const CSR_MHPMEVENT4: u16 = 0x324;
const CSR_MHPMEVENT5: u16 = 0x325;
const CSR_MHPMEVENT6: u16 = 0x326;
const CSR_MHPMEVENT7: u16 = 0x327;
const CSR_MHPMEVENT8: u16 = 0x328;
const CSR_MHPMEVENT9: u16 = 0x329;
const CSR_MHPMEVENT10: u16 = 0x32a;
const CSR_MHPMEVENT11: u16 = 0x32b;
const CSR_MHPMEVENT12: u16 = 0x32c;
const CSR_MHPMEVENT13: u16 = 0x32d;
const CSR_MHPMEVENT14: u16 = 0x32e;
const CSR_MHPMEVENT15: u16 = 0x32f;
const CSR_MHPMEVENT16: u16 = 0x330;
const CSR_MHPMEVENT17: u16 = 0x331;
const CSR_MHPMEVENT18: u16 = 0x332;
const CSR_MHPMEVENT19: u16 = 0x333;
const CSR_MHPMEVENT20: u16 = 0x334;
const CSR_MHPMEVENT21: u16 = 0x335;
const CSR_MHPMEVENT22: u16 = 0x336;
const CSR_MHPMEVENT23: u16 = 0x337;
const CSR_MHPMEVENT24: u16 = 0x338;
const CSR_MHPMEVENT25: u16 = 0x339;
const CSR_MHPMEVENT26: u16 = 0x33a;
const CSR_MHPMEVENT27: u16 = 0x33b;
const CSR_MHPMEVENT28: u16 = 0x33c;
const CSR_MHPMEVENT29: u16 = 0x33d;
const CSR_MHPMEVENT30: u16 = 0x33e;
const CSR_MHPMEVENT31: u16 = 0x33f;
const CSR_MVENDORID: u16 = 0xf11;
const CSR_MARCHID: u16 = 0xf12;
const CSR_MIMPID: u16 = 0xf13;
const CSR_MHARTID: u16 = 0xf14;
const CSR_MCONFIGPTR: u16 = 0xf15;
const CSR_MTOPI: u16 = 0xfb0;
const CSR_SIEH: u16 = 0x114;
const CSR_SIPH: u16 = 0x154;
const CSR_STIMECMPH: u16 = 0x15d;
const CSR_VSIEH: u16 = 0x214;
const CSR_VSIPH: u16 = 0x254;
const CSR_VSTIMECMPH: u16 = 0x25d;
const CSR_HTIMEDELTAH: u16 = 0x615;
const CSR_HIDELEGH: u16 = 0x613;
const CSR_HVIENH: u16 = 0x618;
const CSR_HENVCFGH: u16 = 0x61a;
const CSR_HVIPH: u16 = 0x655;
const CSR_HVIPRIO1H: u16 = 0x656;
const CSR_HVIPRIO2H: u16 = 0x657;
const CSR_HSTATEEN0H: u16 = 0x61c;
const CSR_HSTATEEN1H: u16 = 0x61d;
const CSR_HSTATEEN2H: u16 = 0x61e;
const CSR_HSTATEEN3H: u16 = 0x61f;
const CSR_CYCLEH: u16 = 0xc80;
const CSR_TIMEH: u16 = 0xc81;
const CSR_INSTRETH: u16 = 0xc82;
const CSR_HPMCOUNTER3H: u16 = 0xc83;
const CSR_HPMCOUNTER4H: u16 = 0xc84;
const CSR_HPMCOUNTER5H: u16 = 0xc85;
const CSR_HPMCOUNTER6H: u16 = 0xc86;
const CSR_HPMCOUNTER7H: u16 = 0xc87;
const CSR_HPMCOUNTER8H: u16 = 0xc88;
const CSR_HPMCOUNTER9H: u16 = 0xc89;
const CSR_HPMCOUNTER10H: u16 = 0xc8a;
const CSR_HPMCOUNTER11H: u16 = 0xc8b;
const CSR_HPMCOUNTER12H: u16 = 0xc8c;
const CSR_HPMCOUNTER13H: u16 = 0xc8d;
const CSR_HPMCOUNTER14H: u16 = 0xc8e;
const CSR_HPMCOUNTER15H: u16 = 0xc8f;
const CSR_HPMCOUNTER16H: u16 = 0xc90;
const CSR_HPMCOUNTER17H: u16 = 0xc91;
const CSR_HPMCOUNTER18H: u16 = 0xc92;
const CSR_HPMCOUNTER19H: u16 = 0xc93;
const CSR_HPMCOUNTER20H: u16 = 0xc94;
const CSR_HPMCOUNTER21H: u16 = 0xc95;
const CSR_HPMCOUNTER22H: u16 = 0xc96;
const CSR_HPMCOUNTER23H: u16 = 0xc97;
const CSR_HPMCOUNTER24H: u16 = 0xc98;
const CSR_HPMCOUNTER25H: u16 = 0xc99;
const CSR_HPMCOUNTER26H: u16 = 0xc9a;
const CSR_HPMCOUNTER27H: u16 = 0xc9b;
const CSR_HPMCOUNTER28H: u16 = 0xc9c;
const CSR_HPMCOUNTER29H: u16 = 0xc9d;
const CSR_HPMCOUNTER30H: u16 = 0xc9e;
const CSR_HPMCOUNTER31H: u16 = 0xc9f;
const CSR_MSTATUSH: u16 = 0x310;
const CSR_MIDELEGH: u16 = 0x313;
const CSR_MIEH: u16 = 0x314;
const CSR_MVIENH: u16 = 0x318;
const CSR_MVIPH: u16 = 0x319;
const CSR_MENVCFGH: u16 = 0x31a;
const CSR_MSTATEEN0H: u16 = 0x31c;
const CSR_MSTATEEN1H: u16 = 0x31d;
const CSR_MSTATEEN2H: u16 = 0x31e;
const CSR_MSTATEEN3H: u16 = 0x31f;
const CSR_MIPH: u16 = 0x354;
const CSR_MCYCLECFGH: u16 = 0x721;
const CSR_MINSTRETCFGH: u16 = 0x722;
const CSR_MHPMEVENT3H: u16 = 0x723;
const CSR_MHPMEVENT4H: u16 = 0x724;
const CSR_MHPMEVENT5H: u16 = 0x725;
const CSR_MHPMEVENT6H: u16 = 0x726;
const CSR_MHPMEVENT7H: u16 = 0x727;
const CSR_MHPMEVENT8H: u16 = 0x728;
const CSR_MHPMEVENT9H: u16 = 0x729;
const CSR_MHPMEVENT10H: u16 = 0x72a;
const CSR_MHPMEVENT11H: u16 = 0x72b;
const CSR_MHPMEVENT12H: u16 = 0x72c;
const CSR_MHPMEVENT13H: u16 = 0x72d;
const CSR_MHPMEVENT14H: u16 = 0x72e;
const CSR_MHPMEVENT15H: u16 = 0x72f;
const CSR_MHPMEVENT16H: u16 = 0x730;
const CSR_MHPMEVENT17H: u16 = 0x731;
const CSR_MHPMEVENT18H: u16 = 0x732;
const CSR_MHPMEVENT19H: u16 = 0x733;
const CSR_MHPMEVENT20H: u16 = 0x734;
const CSR_MHPMEVENT21H: u16 = 0x735;
const CSR_MHPMEVENT22H: u16 = 0x736;
const CSR_MHPMEVENT23H: u16 = 0x737;
const CSR_MHPMEVENT24H: u16 = 0x738;
const CSR_MHPMEVENT25H: u16 = 0x739;
const CSR_MHPMEVENT26H: u16 = 0x73a;
const CSR_MHPMEVENT27H: u16 = 0x73b;
const CSR_MHPMEVENT28H: u16 = 0x73c;
const CSR_MHPMEVENT29H: u16 = 0x73d;
const CSR_MHPMEVENT30H: u16 = 0x73e;
const CSR_MHPMEVENT31H: u16 = 0x73f;
const CSR_MNSCRATCH: u16 = 0x740;
const CSR_MNEPC: u16 = 0x741;
const CSR_MNCAUSE: u16 = 0x742;
const CSR_MNSTATUS: u16 = 0x744;
const CSR_MSECCFGH: u16 = 0x757;
const CSR_MCYCLEH: u16 = 0xb80;
const CSR_MINSTRETH: u16 = 0xb82;
const CSR_MHPMCOUNTER3H: u16 = 0xb83;
const CSR_MHPMCOUNTER4H: u16 = 0xb84;
const CSR_MHPMCOUNTER5H: u16 = 0xb85;
const CSR_MHPMCOUNTER6H: u16 = 0xb86;
const CSR_MHPMCOUNTER7H: u16 = 0xb87;
const CSR_MHPMCOUNTER8H: u16 = 0xb88;
const CSR_MHPMCOUNTER9H: u16 = 0xb89;
const CSR_MHPMCOUNTER10H: u16 = 0xb8a;
const CSR_MHPMCOUNTER11H: u16 = 0xb8b;
const CSR_MHPMCOUNTER12H: u16 = 0xb8c;
const CSR_MHPMCOUNTER13H: u16 = 0xb8d;
const CSR_MHPMCOUNTER14H: u16 = 0xb8e;
const CSR_MHPMCOUNTER15H: u16 = 0xb8f;
const CSR_MHPMCOUNTER16H: u16 = 0xb90;
const CSR_MHPMCOUNTER17H: u16 = 0xb91;
const CSR_MHPMCOUNTER18H: u16 = 0xb92;
const CSR_MHPMCOUNTER19H: u16 = 0xb93;
const CSR_MHPMCOUNTER20H: u16 = 0xb94;
const CSR_MHPMCOUNTER21H: u16 = 0xb95;
const CSR_MHPMCOUNTER22H: u16 = 0xb96;
const CSR_MHPMCOUNTER23H: u16 = 0xb97;
const CSR_MHPMCOUNTER24H: u16 = 0xb98;
const CSR_MHPMCOUNTER25H: u16 = 0xb99;
const CSR_MHPMCOUNTER26H: u16 = 0xb9a;
const CSR_MHPMCOUNTER27H: u16 = 0xb9b;
const CSR_MHPMCOUNTER28H: u16 = 0xb9c;
const CSR_MHPMCOUNTER29H: u16 = 0xb9d;
const CSR_MHPMCOUNTER30H: u16 = 0xb9e;
const CSR_MHPMCOUNTER31H: u16 = 0xb9f;
