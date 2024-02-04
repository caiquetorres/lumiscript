fun main() {
    let res = fib(9);
    println res;
}

fun fib(n: Num) -> Num {
    let a = 0;
    let b = 1;
    let c = 0;

    if n == 0 {
        return a;
    }

    for i in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    return b;
}
