use enum_derived::Rand;
use rand::{prelude::Distribution, Rng};

trait Expr<T> {
    /// Builds an instance of given expression
    fn build() -> T;
    /// Serializes the expression to a string representing Elixir code
    fn serialize(&self) -> String;
}

#[derive(Debug)]
struct NonNegativeFloat(f64);

#[derive(Debug)]
struct NonNegativeInt(u64);

impl Rand for NonNegativeFloat {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        NonNegativeFloat(rng.gen_range(0.0..1000000.0))
    }
}

impl Rand for NonNegativeInt {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        NonNegativeInt(rng.gen_range(0..1000000))
    }
}

#[derive(Rand, Debug)]
enum NumberValue {
    Integer(NonNegativeInt),
    Float(NonNegativeFloat),
}

#[derive(Rand, Debug)]
enum NumberNotation {
    Decimal,
    Hex,
    Octal,
    Binary,
}

#[derive(Debug)]
struct Number {
    value: NumberValue,
    has_underscores: bool,
    notation: NumberNotation,
}

#[derive(Debug)]
enum Literal {
    Number(Number), // 101.2021, 0x10d, 0b01_0101_1
    Atom(String), // :foo, :"Foo.Bar"
    String(String), // "hey there"
    Binary(String), // <<"hey" :: utf8, _rest>>, <<"hey">>
}

enum DataStructures {
    List(Vec<Literal>), // [1, 2, 3, "foo"]
    Map(Vec<(Literal, Literal)>), // %{"a" => 10, %{10 => 10}}
    Tuple(Vec<Literal>), // {"hey there", :bar, [1, 2, 4]}
}

impl Expr<Literal> for Number {
    fn build() -> Literal {
        // TODO: dsl for better representing and solving constraints
        // in this case, a value of type float implies our notation should be decimal
        let value = NumberValue::rand();
        let notation = match value {
            NumberValue::Float(_) => NumberNotation::Decimal,
            _ => NumberNotation::rand()
        };

        let mut rng = rand::thread_rng();
        let has_underscores = rng.gen_bool(0.2);

        Literal::Number(Number {
            value,
            notation,
            has_underscores,
        })
    }

    fn serialize(&self) -> String {
        "".to_owned()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decimal_integers() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn octal() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn hex() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn binary() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn decimal_floats() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn underscores_integer() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn underscores_float() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn main() {
    let n = Number::build();
    dbg!(n);
}
