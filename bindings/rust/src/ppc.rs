#![allow(non_camel_case_types)]
// For Unicorn Engine. AUTO-GENERATED FILE, DO NOT EDIT

// PowerPC registers
#[repr(C)]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum RegisterPPC {
    INVALID = 0,
    PC = 1,
    R0 = 2,
    R1 = 3,
    R2 = 4,
    R3 = 5,
    R4 = 6,
    R5 = 7,
    R6 = 8,
    R7 = 9,
    R8 = 10,
    R9 = 11,
    R10 = 12,
    R11 = 13,
    R12 = 14,
    R13 = 15,
    R14 = 16,
    R15 = 17,
    R16 = 18,
    R17 = 19,
    R18 = 20,
    R19 = 21,
    R20 = 22,
    R21 = 23,
    R22 = 24,
    R23 = 25,
    R24 = 26,
    R25 = 27,
    R26 = 28,
    R27 = 29,
    R28 = 30,
    R29 = 31,
    R30 = 32,
    R31 = 33,
    CR0 = 34,
    CR1 = 35,
    CR2 = 36,
    CR3 = 37,
    CR4 = 38,
    CR5 = 39,
    CR6 = 40,
    CR7 = 41,
    FPR0 = 42,
    FPR1 = 43,
    FPR2 = 44,
    FPR3 = 45,
    FPR4 = 46,
    FPR5 = 47,
    FPR6 = 48,
    FPR7 = 49,
    FPR8 = 50,
    FPR9 = 51,
    FPR10 = 52,
    FPR11 = 53,
    FPR12 = 54,
    FPR13 = 55,
    FPR14 = 56,
    FPR15 = 57,
    FPR16 = 58,
    FPR17 = 59,
    FPR18 = 60,
    FPR19 = 61,
    FPR20 = 62,
    FPR21 = 63,
    FPR22 = 64,
    FPR23 = 65,
    FPR24 = 66,
    FPR25 = 67,
    FPR26 = 68,
    FPR27 = 69,
    FPR28 = 70,
    FPR29 = 71,
    FPR30 = 72,
    FPR31 = 73,
    LR = 74,
    XER = 75,
    CTR = 76,
    MSR = 77,
    FPSCR = 78,
    CR = 79,
    ENDING = 80,
}

impl From<RegisterPPC> for i32 {
    fn from(r: RegisterPPC) -> Self {
        r as i32
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PpcCpuModel {
    UC_CPU_PPC32_401 = 0,
    UC_CPU_PPC32_401A1,
    UC_CPU_PPC32_401B2,
    UC_CPU_PPC32_401C2,
    UC_CPU_PPC32_401D2,
    UC_CPU_PPC32_401E2,
    UC_CPU_PPC32_401F2,
    UC_CPU_PPC32_401G2,
    UC_CPU_PPC32_IOP480,
    UC_CPU_PPC32_COBRA,
    UC_CPU_PPC32_403GA,
    UC_CPU_PPC32_403GB,
    UC_CPU_PPC32_403GC,
    UC_CPU_PPC32_403GCX,
    UC_CPU_PPC32_405D2,
    UC_CPU_PPC32_405D4,
    UC_CPU_PPC32_405CRA,
    UC_CPU_PPC32_405CRB,
    UC_CPU_PPC32_405CRC,
    UC_CPU_PPC32_405EP,
    UC_CPU_PPC32_405EZ,
    UC_CPU_PPC32_405GPA,
    UC_CPU_PPC32_405GPB,
    UC_CPU_PPC32_405GPC,
    UC_CPU_PPC32_405GPD,
    UC_CPU_PPC32_405GPR,
    UC_CPU_PPC32_405LP,
    UC_CPU_PPC32_NPE405H,
    UC_CPU_PPC32_NPE405H2,
    UC_CPU_PPC32_NPE405L,
    UC_CPU_PPC32_NPE4GS3,
    UC_CPU_PPC32_STB03,
    UC_CPU_PPC32_STB04,
    UC_CPU_PPC32_STB25,
    UC_CPU_PPC32_X2VP4,
    UC_CPU_PPC32_X2VP20,
    UC_CPU_PPC32_440_XILINX,
    UC_CPU_PPC32_440_XILINX_W_DFPU,
    UC_CPU_PPC32_440EPA,
    UC_CPU_PPC32_440EPB,
    UC_CPU_PPC32_440EPX,
    UC_CPU_PPC32_460EXB,
    UC_CPU_PPC32_G2,
    UC_CPU_PPC32_G2H4,
    UC_CPU_PPC32_G2GP,
    UC_CPU_PPC32_G2LS,
    UC_CPU_PPC32_G2HIP3,
    UC_CPU_PPC32_G2HIP4,
    UC_CPU_PPC32_MPC603,
    UC_CPU_PPC32_G2LE,
    UC_CPU_PPC32_G2LEGP,
    UC_CPU_PPC32_G2LELS,
    UC_CPU_PPC32_G2LEGP1,
    UC_CPU_PPC32_G2LEGP3,
    UC_CPU_PPC32_MPC5200_V10,
    UC_CPU_PPC32_MPC5200_V11,
    UC_CPU_PPC32_MPC5200_V12,
    UC_CPU_PPC32_MPC5200B_V20,
    UC_CPU_PPC32_MPC5200B_V21,
    UC_CPU_PPC32_E200Z5,
    UC_CPU_PPC32_E200Z6,
    UC_CPU_PPC32_E300C1,
    UC_CPU_PPC32_E300C2,
    UC_CPU_PPC32_E300C3,
    UC_CPU_PPC32_E300C4,
    UC_CPU_PPC32_MPC8343,
    UC_CPU_PPC32_MPC8343A,
    UC_CPU_PPC32_MPC8343E,
    UC_CPU_PPC32_MPC8343EA,
    UC_CPU_PPC32_MPC8347T,
    UC_CPU_PPC32_MPC8347P,
    UC_CPU_PPC32_MPC8347AT,
    UC_CPU_PPC32_MPC8347AP,
    UC_CPU_PPC32_MPC8347ET,
    UC_CPU_PPC32_MPC8347EP,
    UC_CPU_PPC32_MPC8347EAT,
    UC_CPU_PPC32_MPC8347EAP,
    UC_CPU_PPC32_MPC8349,
    UC_CPU_PPC32_MPC8349A,
    UC_CPU_PPC32_MPC8349E,
    UC_CPU_PPC32_MPC8349EA,
    UC_CPU_PPC32_MPC8377,
    UC_CPU_PPC32_MPC8377E,
    UC_CPU_PPC32_MPC8378,
    UC_CPU_PPC32_MPC8378E,
    UC_CPU_PPC32_MPC8379,
    UC_CPU_PPC32_MPC8379E,
    UC_CPU_PPC32_E500_V10,
    UC_CPU_PPC32_E500_V20,
    UC_CPU_PPC32_E500V2_V10,
    UC_CPU_PPC32_E500V2_V20,
    UC_CPU_PPC32_E500V2_V21,
    UC_CPU_PPC32_E500V2_V22,
    UC_CPU_PPC32_E500V2_V30,
    UC_CPU_PPC32_E500MC,
    UC_CPU_PPC32_MPC8533_V10,
    UC_CPU_PPC32_MPC8533_V11,
    UC_CPU_PPC32_MPC8533E_V10,
    UC_CPU_PPC32_MPC8533E_V11,
    UC_CPU_PPC32_MPC8540_V10,
    UC_CPU_PPC32_MPC8540_V20,
    UC_CPU_PPC32_MPC8540_V21,
    UC_CPU_PPC32_MPC8541_V10,
    UC_CPU_PPC32_MPC8541_V11,
    UC_CPU_PPC32_MPC8541E_V10,
    UC_CPU_PPC32_MPC8541E_V11,
    UC_CPU_PPC32_MPC8543_V10,
    UC_CPU_PPC32_MPC8543_V11,
    UC_CPU_PPC32_MPC8543_V20,
    UC_CPU_PPC32_MPC8543_V21,
    UC_CPU_PPC32_MPC8543E_V10,
    UC_CPU_PPC32_MPC8543E_V11,
    UC_CPU_PPC32_MPC8543E_V20,
    UC_CPU_PPC32_MPC8543E_V21,
    UC_CPU_PPC32_MPC8544_V10,
    UC_CPU_PPC32_MPC8544_V11,
    UC_CPU_PPC32_MPC8544E_V10,
    UC_CPU_PPC32_MPC8544E_V11,
    UC_CPU_PPC32_MPC8545_V20,
    UC_CPU_PPC32_MPC8545_V21,
    UC_CPU_PPC32_MPC8545E_V20,
    UC_CPU_PPC32_MPC8545E_V21,
    UC_CPU_PPC32_MPC8547E_V20,
    UC_CPU_PPC32_MPC8547E_V21,
    UC_CPU_PPC32_MPC8548_V10,
    UC_CPU_PPC32_MPC8548_V11,
    UC_CPU_PPC32_MPC8548_V20,
    UC_CPU_PPC32_MPC8548_V21,
    UC_CPU_PPC32_MPC8548E_V10,
    UC_CPU_PPC32_MPC8548E_V11,
    UC_CPU_PPC32_MPC8548E_V20,
    UC_CPU_PPC32_MPC8548E_V21,
    UC_CPU_PPC32_MPC8555_V10,
    UC_CPU_PPC32_MPC8555_V11,
    UC_CPU_PPC32_MPC8555E_V10,
    UC_CPU_PPC32_MPC8555E_V11,
    UC_CPU_PPC32_MPC8560_V10,
    UC_CPU_PPC32_MPC8560_V20,
    UC_CPU_PPC32_MPC8560_V21,
    UC_CPU_PPC32_MPC8567,
    UC_CPU_PPC32_MPC8567E,
    UC_CPU_PPC32_MPC8568,
    UC_CPU_PPC32_MPC8568E,
    UC_CPU_PPC32_MPC8572,
    UC_CPU_PPC32_MPC8572E,
    UC_CPU_PPC32_E600,
    UC_CPU_PPC32_MPC8610,
    UC_CPU_PPC32_MPC8641,
    UC_CPU_PPC32_MPC8641D,
    UC_CPU_PPC32_601_V0,
    UC_CPU_PPC32_601_V1,
    UC_CPU_PPC32_601_V2,
    UC_CPU_PPC32_602,
    UC_CPU_PPC32_603,
    UC_CPU_PPC32_603E_V1_1,
    UC_CPU_PPC32_603E_V1_2,
    UC_CPU_PPC32_603E_V1_3,
    UC_CPU_PPC32_603E_V1_4,
    UC_CPU_PPC32_603E_V2_2,
    UC_CPU_PPC32_603E_V3,
    UC_CPU_PPC32_603E_V4,
    UC_CPU_PPC32_603E_V4_1,
    UC_CPU_PPC32_603E7,
    UC_CPU_PPC32_603E7T,
    UC_CPU_PPC32_603E7V,
    UC_CPU_PPC32_603E7V1,
    UC_CPU_PPC32_603E7V2,
    UC_CPU_PPC32_603P,
    UC_CPU_PPC32_604,
    UC_CPU_PPC32_604E_V1_0,
    UC_CPU_PPC32_604E_V2_2,
    UC_CPU_PPC32_604E_V2_4,
    UC_CPU_PPC32_604R,
    UC_CPU_PPC32_740_V1_0,
    UC_CPU_PPC32_750_V1_0,
    UC_CPU_PPC32_740_V2_0,
    UC_CPU_PPC32_750_V2_0,
    UC_CPU_PPC32_740_V2_1,
    UC_CPU_PPC32_750_V2_1,
    UC_CPU_PPC32_740_V2_2,
    UC_CPU_PPC32_750_V2_2,
    UC_CPU_PPC32_740_V3_0,
    UC_CPU_PPC32_750_V3_0,
    UC_CPU_PPC32_740_V3_1,
    UC_CPU_PPC32_750_V3_1,
    UC_CPU_PPC32_740E,
    UC_CPU_PPC32_750E,
    UC_CPU_PPC32_740P,
    UC_CPU_PPC32_750P,
    UC_CPU_PPC32_750CL_V1_0,
    UC_CPU_PPC32_750CL_V2_0,
    UC_CPU_PPC32_750CX_V1_0,
    UC_CPU_PPC32_750CX_V2_0,
    UC_CPU_PPC32_750CX_V2_1,
    UC_CPU_PPC32_750CX_V2_2,
    UC_CPU_PPC32_750CXE_V2_1,
    UC_CPU_PPC32_750CXE_V2_2,
    UC_CPU_PPC32_750CXE_V2_3,
    UC_CPU_PPC32_750CXE_V2_4,
    UC_CPU_PPC32_750CXE_V2_4B,
    UC_CPU_PPC32_750CXE_V3_0,
    UC_CPU_PPC32_750CXE_V3_1,
    UC_CPU_PPC32_750CXE_V3_1B,
    UC_CPU_PPC32_750CXR,
    UC_CPU_PPC32_750FL,
    UC_CPU_PPC32_750FX_V1_0,
    UC_CPU_PPC32_750FX_V2_0,
    UC_CPU_PPC32_750FX_V2_1,
    UC_CPU_PPC32_750FX_V2_2,
    UC_CPU_PPC32_750FX_V2_3,
    UC_CPU_PPC32_750GL,
    UC_CPU_PPC32_750GX_V1_0,
    UC_CPU_PPC32_750GX_V1_1,
    UC_CPU_PPC32_750GX_V1_2,
    UC_CPU_PPC32_750L_V2_0,
    UC_CPU_PPC32_750L_V2_1,
    UC_CPU_PPC32_750L_V2_2,
    UC_CPU_PPC32_750L_V3_0,
    UC_CPU_PPC32_750L_V3_2,
    UC_CPU_PPC32_745_V1_0,
    UC_CPU_PPC32_755_V1_0,
    UC_CPU_PPC32_745_V1_1,
    UC_CPU_PPC32_755_V1_1,
    UC_CPU_PPC32_745_V2_0,
    UC_CPU_PPC32_755_V2_0,
    UC_CPU_PPC32_745_V2_1,
    UC_CPU_PPC32_755_V2_1,
    UC_CPU_PPC32_745_V2_2,
    UC_CPU_PPC32_755_V2_2,
    UC_CPU_PPC32_745_V2_3,
    UC_CPU_PPC32_755_V2_3,
    UC_CPU_PPC32_745_V2_4,
    UC_CPU_PPC32_755_V2_4,
    UC_CPU_PPC32_745_V2_5,
    UC_CPU_PPC32_755_V2_5,
    UC_CPU_PPC32_745_V2_6,
    UC_CPU_PPC32_755_V2_6,
    UC_CPU_PPC32_745_V2_7,
    UC_CPU_PPC32_755_V2_7,
    UC_CPU_PPC32_745_V2_8,
    UC_CPU_PPC32_755_V2_8,
    UC_CPU_PPC32_7400_V1_0,
    UC_CPU_PPC32_7400_V1_1,
    UC_CPU_PPC32_7400_V2_0,
    UC_CPU_PPC32_7400_V2_1,
    UC_CPU_PPC32_7400_V2_2,
    UC_CPU_PPC32_7400_V2_6,
    UC_CPU_PPC32_7400_V2_7,
    UC_CPU_PPC32_7400_V2_8,
    UC_CPU_PPC32_7400_V2_9,
    UC_CPU_PPC32_7410_V1_0,
    UC_CPU_PPC32_7410_V1_1,
    UC_CPU_PPC32_7410_V1_2,
    UC_CPU_PPC32_7410_V1_3,
    UC_CPU_PPC32_7410_V1_4,
    UC_CPU_PPC32_7448_V1_0,
    UC_CPU_PPC32_7448_V1_1,
    UC_CPU_PPC32_7448_V2_0,
    UC_CPU_PPC32_7448_V2_1,
    UC_CPU_PPC32_7450_V1_0,
    UC_CPU_PPC32_7450_V1_1,
    UC_CPU_PPC32_7450_V1_2,
    UC_CPU_PPC32_7450_V2_0,
    UC_CPU_PPC32_7450_V2_1,
    UC_CPU_PPC32_7441_V2_1,
    UC_CPU_PPC32_7441_V2_3,
    UC_CPU_PPC32_7451_V2_3,
    UC_CPU_PPC32_7441_V2_10,
    UC_CPU_PPC32_7451_V2_10,
    UC_CPU_PPC32_7445_V1_0,
    UC_CPU_PPC32_7455_V1_0,
    UC_CPU_PPC32_7445_V2_1,
    UC_CPU_PPC32_7455_V2_1,
    UC_CPU_PPC32_7445_V3_2,
    UC_CPU_PPC32_7455_V3_2,
    UC_CPU_PPC32_7445_V3_3,
    UC_CPU_PPC32_7455_V3_3,
    UC_CPU_PPC32_7445_V3_4,
    UC_CPU_PPC32_7455_V3_4,
    UC_CPU_PPC32_7447_V1_0,
    UC_CPU_PPC32_7457_V1_0,
    UC_CPU_PPC32_7447_V1_1,
    UC_CPU_PPC32_7457_V1_1,
    UC_CPU_PPC32_7457_V1_2,
    UC_CPU_PPC32_7447A_V1_0,
    UC_CPU_PPC32_7457A_V1_0,
    UC_CPU_PPC32_7447A_V1_1,
    UC_CPU_PPC32_7457A_V1_1,
    UC_CPU_PPC32_7447A_V1_2,
    UC_CPU_PPC32_7457A_V1_2,
}

impl From<PpcCpuModel> for i32 {
    fn from(value: PpcCpuModel) -> Self {
        value as i32
    }
}

impl From<&PpcCpuModel> for i32 {
    fn from(value: &PpcCpuModel) -> Self {
        (*value) as i32
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ppc64CpuModel {
    UC_CPU_PPC64_E5500 = 0,
    UC_CPU_PPC64_E6500,
    UC_CPU_PPC64_970_V2_2,
    UC_CPU_PPC64_970FX_V1_0,
    UC_CPU_PPC64_970FX_V2_0,
    UC_CPU_PPC64_970FX_V2_1,
    UC_CPU_PPC64_970FX_V3_0,
    UC_CPU_PPC64_970FX_V3_1,
    UC_CPU_PPC64_970MP_V1_0,
    UC_CPU_PPC64_970MP_V1_1,
    UC_CPU_PPC64_POWER5_V2_1,
    UC_CPU_PPC64_POWER7_V2_3,
    UC_CPU_PPC64_POWER7_V2_1,
    UC_CPU_PPC64_POWER8E_V2_1,
    UC_CPU_PPC64_POWER8_V2_0,
    UC_CPU_PPC64_POWER8NVL_V1_0,
    UC_CPU_PPC64_POWER9_V1_0,
    UC_CPU_PPC64_POWER9_V2_0,
    UC_CPU_PPC64_POWER10_V1_0,
}

impl From<Ppc64CpuModel> for i32 {
    fn from(value: Ppc64CpuModel) -> Self {
        value as i32
    }
}

impl From<&Ppc64CpuModel> for i32 {
    fn from(value: &Ppc64CpuModel) -> Self {
        (*value) as i32
    }
}
