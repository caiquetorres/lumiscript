trait Add {
    fun add(other: This) -> This;
}

impl Add for Num {
    extern fun add(other: This) -> This;
}

fun main() {
    let res = fib(10);
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

main();
