use std::str::FromStr;


#[derive(Debug)]
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
