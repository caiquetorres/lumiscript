trait Copy {
    fun copy() -> This;
}

impl Copy for Num {
    extern fun copy() -> This;
}

impl Copy for Bool {
    extern fun copy() -> This;
}

trait ToBool {
    fun toBool() -> bool;
}

impl ToBool for Bool {
    extern fun toBool() -> Bool;
}

impl ToBool for Num {
    fun toBool() -> Bool {
        return this != 0;
    }
}

trait Add {
    fun add(other: This) -> This;
}

trait Sub {
    fun sub(other: This) -> This;
}

impl Add for Num {
    extern fun add(other: Num) -> Num;
}

impl Sub for Num {
    extern fun sub(other: Num) -> Num;
}
