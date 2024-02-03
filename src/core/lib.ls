trait Add {
    fun add(other: Self) -> Self;
}

trait ToStr {
    fun toStr() -> str;
}

impl Add for num {
    fun add(other: num) -> num {
        /* native code */
    }
}

impl ToString for str {
    fun toStr() -> str {
        return this;
    }
}

impl ToString for num {
    fun toStr() -> str {
        /* native code */
    }
}

impl ToString for bool {
    fun toStr() -> str {
        /* native code */
    }
}
