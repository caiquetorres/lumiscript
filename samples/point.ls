class Point {
    x: Num,
    y: Num
}

impl Point {
    fun getX() -> Num {
        return this.x;
    }

    fun getY() -> Num {
        return this.y;
    }
}

impl Add for Point {
    fun add(other: This) -> This {
        return Point {
            x: this.x + other.x,
            y: this.y + other.y
        };
    }
}

impl Sub for Point {
    fun sub(other: This) -> This {
        return Point {
            x: this.x - other.x,
            y: this.y - other.y
        };
    }
}

let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };

let res = p1 + p2;

println res.getX();
println res.getY();

let res = p1 - p2;

println res.getX();
println res.getY();