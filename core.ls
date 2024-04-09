trait Eq {
    fun eq(other: This) -> Bool;
}

trait ToBool {
    fun toBool() -> Bool;
}

trait Add {
    fun add(other: This) -> This;
}

trait Sub {
    fun sub(other: This) -> This;
}

impl Eq for Num {
    extern fun eq(other: This) -> Bool;
}

impl ToBool for Num {
    fun toBool() -> Bool {
        return this != 0;
    }
}

impl Add for Num {
    extern fun add(other: This) -> This;
}

impl Sub for Num {
    extern fun sub(other: This) -> This;
}
