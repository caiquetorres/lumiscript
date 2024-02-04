trait ToBool {
    fun toBool() -> bool;
}

trait Add {
    fun add(other: This) -> This;
}

trait Sub {
    fun sub(other: This) -> This;
}

impl ToBool for Bool {
    extern fun toBool() -> Bool;
}

impl ToBool for Num {
    fun toBool() -> Bool {
        return this != 0;
    }
}

impl Add for Num {
    extern fun add(other: Num) -> Num;
}

impl Sub for Num {
    extern fun sub(other: Num) -> Num;
}
