use serde::{Deserialize, Serialize};
use strum::{EnumString, Display};

#[derive(Display, Debug, EnumString, Clone, Copy, Serialize, Deserialize, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Job {
    PLD = 0,
    WAR,
    DRK,
    GNB,
    WHM,
    SCH,
    AST,
    SGE,
    DRG,
    MNK,
    NIN,
    SAM,
    RPR,
    BLM,
    SMN,
    RDM,
    BRD,
    MCH,
    DNC,
}
