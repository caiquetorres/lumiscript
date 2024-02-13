trait Add {
    fun add(other: This) -> This;
}

trait Sub {
    fun sub(other: This) -> This;
}

impl Add for Num {
    extern fun add(other: This) -> This;
}

impl Sub for Num {
    extern fun sub(other: This) -> This;
}

fun main() {
    let res = fib(9);
    println res;
}

fun fib(n: Num) -> Num {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return fib(n - 1) + fib(n - 2);
}
