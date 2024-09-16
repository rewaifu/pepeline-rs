//Rec.2020
pub const KB_2020: f32 = 0.0593;
pub const KR_2020: f32 = 0.2627;
pub const KG_2020: f32 = 1f32 - KR_2020 - KB_2020;
pub const KE_2020: f32 = (1_f32 - KR_2020) / 0.5;
pub const KD_2020: f32 = (1_f32 - KB_2020) / 0.5;
pub const KCRG_2020: f32 = KR_2020 * KE_2020 / KD_2020;
pub const KCBG_2020: f32 = KB_2020 * KD_2020 / KG_2020;
//Rec.601
pub const KB_601: f32 = 0.114;
pub const KR_601: f32 = 0.299;
pub const KG_601: f32 = 1f32 - KR_601 - KB_601;
pub const KE_601: f32 = (1_f32 - KR_601) / 0.5;
pub const KD_601: f32 = (1_f32 - KB_601) / 0.5;
pub const KCRG_601: f32 = KR_601 * KE_601 / KD_601;
pub const KCBG_601: f32 = KB_601 * KD_601 / KG_601;
//Rec.709
pub const KB_709: f32 = 0.0722;
pub const KR_709: f32 = 0.2126;
pub const KG_709: f32 = 1_f32 - KR_709 - KB_709;
pub const KE_709: f32 = (1_f32 - KR_709) / 0.5;
pub const KD_709: f32 = (1_f32 - KB_709) / 0.5;
pub const KCRG_709: f32 = KR_709 * KE_709 / KD_709;
pub const KCBG_709: f32 = KB_709 * KD_709 / KG_709;
//Average
pub const AVERAGE: f32 = 1_f32 / 3_f32;
