trait Add {
    fun add(other: Self) -> Self;
}

impl Add for Num {
    extern fun add(other: Self) -> Self;
}

class Point {
    x: Num,
    y: Num
}

impl Add for Point {
    fun add(other: This) -> This {
        This {
            x: this.x + other.x,
            y: this.y + other.y
        }
    }
}

let p1 = Point { x: 1, y: 1 };
let p2 = Point { x: 2, y: 3 };
let sum = p1 + p2;

println sum.x;
println sum.y;
