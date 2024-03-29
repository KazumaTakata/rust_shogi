use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub enum PieceType {
    King,
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
    ProRook,
    ProBishop,
    ProSilver,
    ProKnight,
    ProLance,
    ProPawn,
}

impl PieceType {
    pub fn is_promoted(&self) -> bool {
        if [
            PieceType::ProPawn,
            PieceType::ProKnight,
            PieceType::Lance,
            PieceType::ProSilver,
            PieceType::ProBishop,
            PieceType::ProRook,
        ]
        .contains(self)
        {
            return true;
        }

        return false;
    }
}

impl FromStr for PieceType {
    type Err = ();

    fn from_str(input: &str) -> Result<PieceType, Self::Err> {
        match input {
            "OU" => Ok(PieceType::King),
            "HI" => Ok(PieceType::Rook),
            "KA" => Ok(PieceType::Bishop),
            "KI" => Ok(PieceType::Gold),
            "GI" => Ok(PieceType::Silver),
            "KE" => Ok(PieceType::Knight),
            "KY" => Ok(PieceType::Lance),
            "FU" => Ok(PieceType::Pawn),
            "RY" => Ok(PieceType::ProRook),
            "UM" => Ok(PieceType::ProBishop),
            "NG" => Ok(PieceType::ProSilver),
            "NK" => Ok(PieceType::ProKnight),
            "NY" => Ok(PieceType::ProLance),
            "TO" => Ok(PieceType::ProPawn),

            _ => Err(()),
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PieceType::King => write!(f, "OU"),
            &PieceType::Rook => write!(f, "HI"),
            &PieceType::Bishop => write!(f, "KA"),
            &PieceType::Gold => write!(f, "KI"),
            &PieceType::Silver => write!(f, "GI"),
            &PieceType::Knight => write!(f, "KE"),
            &PieceType::Lance => write!(f, "KY"),
            &PieceType::Pawn => write!(f, "FU"),
            &PieceType::ProRook => write!(f, "RY"),
            &PieceType::ProBishop => write!(f, "UM"),
            &PieceType::ProSilver => write!(f, "NG"),
            &PieceType::ProKnight => write!(f, "NK"),
            &PieceType::ProLance => write!(f, "NY"),
            &PieceType::ProPawn => write!(f, "TO"),
            _ => write!(f, "*"),
        }
    }
}
