mod direction;
mod position;
mod symbols;
mod world;

pub(crate) use direction::{
    Direction,
    Direction::{DownLeft, DownRight, UpLeft, UpRight},
};
pub(crate) use position::Position;
pub(crate) use symbols::Symbols;
pub use world::{ParseHexError, World};

const WIDTH: usize = 17;
const HEIGHT: usize = 9;

#[cfg(test)]
mod test {
    use crate::World;

    #[test]
    fn test1() {
        assert_eq!(
            World::try_from("fc94b0c1e5b0987c5843997697ee9fb7")
                .unwrap()
                .to_string(),
            r#"+-----------------+
|       .=o.  .   |
|     . *+*. o    |
|      =.*..o     |
|       o + ..    |
|        S o.     |
|         o  .    |
|          .  . . |
|              o .|
|               E.|
+-----------------+"#
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            World::try_from("37e46a2d48381a0af3726dd9176bbd5e")
                .unwrap()
                .to_string(),
            r#"+-----------------+
|                 |
|                 |
|          .      |
|     .   o       |
|o . o . S +      |
|.+ + = . B .     |
|o + + o B o E    |
| o .   + . o     |
|         .o      |
+-----------------+"#
        );
    }
    #[test]
    fn test3() {
        assert_eq!(
            World::try_from("6b").unwrap().to_string(),
            r#"+-----------------+
|                 |
|                 |
|                 |
|                 |
|        S        |
|         .       |
|        E        |
|       .         |
|                 |
+-----------------+"#
        );
    }

    #[test]
    fn rejects_invalid_hexadecimal_input() {
        assert!(World::try_from("xyz").is_err());
        assert!(World::try_from("0g").is_err());
    }

    #[test]
    fn saturates_frequently_visited_cells() {
        let input = "1b".repeat(100);
        let drawing = World::try_from(input.as_str()).unwrap().to_string();

        assert_eq!(drawing.lines().nth(6).unwrap().chars().nth(10), Some('^'));
    }
}
