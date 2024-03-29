use std::str::FromStr;

use crate::board::Teban;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Position {
    SQ_00,
    SQ_1A,
    SQ_1B,
    SQ_1C,
    SQ_1D,
    SQ_1E,
    SQ_1F,
    SQ_1G,
    SQ_1H,
    SQ_1I,
    SQ_2A,
    SQ_2B,
    SQ_2C,
    SQ_2D,
    SQ_2E,
    SQ_2F,
    SQ_2G,
    SQ_2H,
    SQ_2I,
    SQ_3A,
    SQ_3B,
    SQ_3C,
    SQ_3D,
    SQ_3E,
    SQ_3F,
    SQ_3G,
    SQ_3H,
    SQ_3I,
    SQ_4A,
    SQ_4B,
    SQ_4C,
    SQ_4D,
    SQ_4E,
    SQ_4F,
    SQ_4G,
    SQ_4H,
    SQ_4I,
    SQ_5A,
    SQ_5B,
    SQ_5C,
    SQ_5D,
    SQ_5E,
    SQ_5F,
    SQ_5G,
    SQ_5H,
    SQ_5I,
    SQ_6A,
    SQ_6B,
    SQ_6C,
    SQ_6D,
    SQ_6E,
    SQ_6F,
    SQ_6G,
    SQ_6H,
    SQ_6I,
    SQ_7A,
    SQ_7B,
    SQ_7C,
    SQ_7D,
    SQ_7E,
    SQ_7F,
    SQ_7G,
    SQ_7H,
    SQ_7I,
    SQ_8A,
    SQ_8B,
    SQ_8C,
    SQ_8D,
    SQ_8E,
    SQ_8F,
    SQ_8G,
    SQ_8H,
    SQ_8I,
    SQ_9A,
    SQ_9B,
    SQ_9C,
    SQ_9D,
    SQ_9E,
    SQ_9F,
    SQ_9G,
    SQ_9H,
    SQ_9I,
}

impl Position {
    pub fn to_tensor_index_with_teban(&self, teban: &Teban) -> (i32, i32) {
        let (x, y) = self.to_tensor_index();

        if x == 0 && y == 0 {
            return (0, 0)
        }

        return match teban {
            Teban::Sente => (x, y),
            Teban::Gote => (10 - x, 10 - y),
        };
    }

    pub fn to_tensor_index(&self) -> (i32, i32) {
        return match self {
            Position::SQ_00 => (0, 0),

            Position::SQ_1A => (1, 1),
            Position::SQ_1B => (1, 2),
            Position::SQ_1C => (1, 3),
            Position::SQ_1D => (1, 4),
            Position::SQ_1E => (1, 5),
            Position::SQ_1F => (1, 6),
            Position::SQ_1G => (1, 7),
            Position::SQ_1H => (1, 8),
            Position::SQ_1I => (1, 9),

            Position::SQ_2A => (2, 1),
            Position::SQ_2B => (2, 2),
            Position::SQ_2C => (2, 3),
            Position::SQ_2D => (2, 4),
            Position::SQ_2E => (2, 5),
            Position::SQ_2F => (2, 6),
            Position::SQ_2G => (2, 7),
            Position::SQ_2H => (2, 8),
            Position::SQ_2I => (2, 9),

            Position::SQ_3A => (3, 1),
            Position::SQ_3B => (3, 2),
            Position::SQ_3C => (3, 3),
            Position::SQ_3D => (3, 4),
            Position::SQ_3E => (3, 5),
            Position::SQ_3F => (3, 6),
            Position::SQ_3G => (3, 7),
            Position::SQ_3H => (3, 8),
            Position::SQ_3I => (3, 9),

            Position::SQ_4A => (4, 1),
            Position::SQ_4B => (4, 2),
            Position::SQ_4C => (4, 3),
            Position::SQ_4D => (4, 4),
            Position::SQ_4E => (4, 5),
            Position::SQ_4F => (4, 6),
            Position::SQ_4G => (4, 7),
            Position::SQ_4H => (4, 8),
            Position::SQ_4I => (4, 9),

            Position::SQ_5A => (5, 1),
            Position::SQ_5B => (5, 2),
            Position::SQ_5C => (5, 3),
            Position::SQ_5D => (5, 4),
            Position::SQ_5E => (5, 5),
            Position::SQ_5F => (5, 6),
            Position::SQ_5G => (5, 7),
            Position::SQ_5H => (5, 8),
            Position::SQ_5I => (5, 9),

            Position::SQ_6A => (6, 1),
            Position::SQ_6B => (6, 2),
            Position::SQ_6C => (6, 3),
            Position::SQ_6D => (6, 4),
            Position::SQ_6E => (6, 5),
            Position::SQ_6F => (6, 6),
            Position::SQ_6G => (6, 7),
            Position::SQ_6H => (6, 8),
            Position::SQ_6I => (6, 9),

            Position::SQ_7A => (7, 1),
            Position::SQ_7B => (7, 2),
            Position::SQ_7C => (7, 3),
            Position::SQ_7D => (7, 4),
            Position::SQ_7E => (7, 5),
            Position::SQ_7F => (7, 6),
            Position::SQ_7G => (7, 7),
            Position::SQ_7H => (7, 8),
            Position::SQ_7I => (7, 9),

            Position::SQ_8A => (8, 1),
            Position::SQ_8B => (8, 2),
            Position::SQ_8C => (8, 3),
            Position::SQ_8D => (8, 4),
            Position::SQ_8E => (8, 5),
            Position::SQ_8F => (8, 6),
            Position::SQ_8G => (8, 7),
            Position::SQ_8H => (8, 8),
            Position::SQ_8I => (8, 9),

            Position::SQ_9A => (9, 1),
            Position::SQ_9B => (9, 2),
            Position::SQ_9C => (9, 3),
            Position::SQ_9D => (9, 4),
            Position::SQ_9E => (9, 5),
            Position::SQ_9F => (9, 6),
            Position::SQ_9G => (9, 7),
            Position::SQ_9H => (9, 8),
            Position::SQ_9I => (9, 9),
        };
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(input: &str) -> Result<Position, Self::Err> {
        match input {
            "00" => Ok(Position::SQ_00),

            "11" => Ok(Position::SQ_1A),
            "12" => Ok(Position::SQ_1B),
            "13" => Ok(Position::SQ_1C),
            "14" => Ok(Position::SQ_1D),
            "15" => Ok(Position::SQ_1E),
            "16" => Ok(Position::SQ_1F),
            "17" => Ok(Position::SQ_1G),
            "18" => Ok(Position::SQ_1H),
            "19" => Ok(Position::SQ_1I),

            "21" => Ok(Position::SQ_2A),
            "22" => Ok(Position::SQ_2B),
            "23" => Ok(Position::SQ_2C),
            "24" => Ok(Position::SQ_2D),
            "25" => Ok(Position::SQ_2E),
            "26" => Ok(Position::SQ_2F),
            "27" => Ok(Position::SQ_2G),
            "28" => Ok(Position::SQ_2H),
            "29" => Ok(Position::SQ_2I),

            "31" => Ok(Position::SQ_3A),
            "32" => Ok(Position::SQ_3B),
            "33" => Ok(Position::SQ_3C),
            "34" => Ok(Position::SQ_3D),
            "35" => Ok(Position::SQ_3E),
            "36" => Ok(Position::SQ_3F),
            "37" => Ok(Position::SQ_3G),
            "38" => Ok(Position::SQ_3H),
            "39" => Ok(Position::SQ_3I),

            "41" => Ok(Position::SQ_4A),
            "42" => Ok(Position::SQ_4B),
            "43" => Ok(Position::SQ_4C),
            "44" => Ok(Position::SQ_4D),
            "45" => Ok(Position::SQ_4E),
            "46" => Ok(Position::SQ_4F),
            "47" => Ok(Position::SQ_4G),
            "48" => Ok(Position::SQ_4H),
            "49" => Ok(Position::SQ_4I),

            "51" => Ok(Position::SQ_5A),
            "52" => Ok(Position::SQ_5B),
            "53" => Ok(Position::SQ_5C),
            "54" => Ok(Position::SQ_5D),
            "55" => Ok(Position::SQ_5E),
            "56" => Ok(Position::SQ_5F),
            "57" => Ok(Position::SQ_5G),
            "58" => Ok(Position::SQ_5H),
            "59" => Ok(Position::SQ_5I),

            "61" => Ok(Position::SQ_6A),
            "62" => Ok(Position::SQ_6B),
            "63" => Ok(Position::SQ_6C),
            "64" => Ok(Position::SQ_6D),
            "65" => Ok(Position::SQ_6E),
            "66" => Ok(Position::SQ_6F),
            "67" => Ok(Position::SQ_6G),
            "68" => Ok(Position::SQ_6H),
            "69" => Ok(Position::SQ_6I),

            "71" => Ok(Position::SQ_7A),
            "72" => Ok(Position::SQ_7B),
            "73" => Ok(Position::SQ_7C),
            "74" => Ok(Position::SQ_7D),
            "75" => Ok(Position::SQ_7E),
            "76" => Ok(Position::SQ_7F),
            "77" => Ok(Position::SQ_7G),
            "78" => Ok(Position::SQ_7H),
            "79" => Ok(Position::SQ_7I),

            "81" => Ok(Position::SQ_8A),
            "82" => Ok(Position::SQ_8B),
            "83" => Ok(Position::SQ_8C),
            "84" => Ok(Position::SQ_8D),
            "85" => Ok(Position::SQ_8E),
            "86" => Ok(Position::SQ_8F),
            "87" => Ok(Position::SQ_8G),
            "88" => Ok(Position::SQ_8H),
            "89" => Ok(Position::SQ_8I),

            "91" => Ok(Position::SQ_9A),
            "92" => Ok(Position::SQ_9B),
            "93" => Ok(Position::SQ_9C),
            "94" => Ok(Position::SQ_9D),
            "95" => Ok(Position::SQ_9E),
            "96" => Ok(Position::SQ_9F),
            "97" => Ok(Position::SQ_9G),
            "98" => Ok(Position::SQ_9H),
            "99" => Ok(Position::SQ_9I),

            _ => Err(()),
        }
    }
}
