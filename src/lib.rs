pub use types::world::World; // re-export World type
mod types;

const WIDTH: usize = 17;
const HEIGHT: usize = 9;

#[cfg(test)]
mod test {
    use crate::World;

    #[test]
    fn test1() {
        assert_eq!(
            World::from("fc94b0c1e5b0987c5843997697ee9fb7").to_string(),
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
            World::from("37e46a2d48381a0af3726dd9176bbd5e").to_string(),
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
            World::from("6b").to_string(),
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
}
