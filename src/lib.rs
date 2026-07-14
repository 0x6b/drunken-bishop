mod render;
mod world;

pub use world::{ParseHexError, World};

#[cfg(test)]
mod test {
    use crate::{ParseHexError, World};

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
        assert_eq!(World::try_from("xyz").err().unwrap(), ParseHexError::OddLength { length: 3 });
        assert!(World::try_from("0g").is_err());
    }

    #[test]
    fn explains_how_to_fix_odd_length_input() {
        let error = World::try_from("abc").err().unwrap();

        assert_eq!(
            error.to_string(),
            "hexadecimal input has 3 digits; add or remove one digit to make complete byte pairs"
        );
    }

    #[test]
    fn saturates_frequently_visited_cells() {
        let input = "1b".repeat(100);
        let drawing = World::try_from(input.as_str()).unwrap().to_string();

        assert_eq!(drawing.lines().nth(6).unwrap().chars().nth(10), Some('^'));
    }
}
