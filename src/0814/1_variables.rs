fn main() {
    {
        // TODO: Add missing keyword.
        let x:i64 = 5;

        println!("x has the value {x}");
    }

    {
        let x:i64 = 22;

        if x == 10 {
            println!("x is ten!");
        } else {
            println!("x is not ten!");
        }
    }

    {
        let x: i32 = 22;

        println!("Number {x}");
    }

    {
        let mut x = 3;
        println!("Number {x}");

        x = 5;
        println!("Number {x}");
    }

    {
        let number = "T-H-R-E-E";
        println!("Spell a number: {}", number);

        let number = 3;
        println!("Number plus two is: {}", number + 2);
    }
}
