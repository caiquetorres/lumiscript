# LumiScript

In the past few months, I've developed a quirky obsession with compilers and programming languages. It all started with a university course that dove deep into compiler concepts like LL1 compilers and hands-on syntax testing. Surprisingly, I found myself totally hooked, realizing the immense knowledge hidden within programming languages.

Then I stumbled upon this [book](https://craftinginterpreters.com/), and bam! Everything changed. It might just be one of the best technical yet easy-to-follow books I've ever come across. It brilliantly explained stuff like lexing and parsing code, setting up a virtual machine, and even garbage collection, making the learning process a breeze. But, I wasn't satisfied yet. While the book used Java and C for examples, I was itching to get my hands dirty with Rust and do something big, like creating a programming language.

Fueled by this newfound passion, I kicked off this repository dedicated to a programming language built solely for learning purposes. Though not meant for real-world use, I'm tackling it with a practical mindset. Leveraging my 4 years of experience with TypeScript, JavaScript, and C#, I'm weaving in intriguing elements from these languages. But at its core, this language is all about Rust and Kotlin.

## Current Status

We are actively working on building the foundational components of the programming, including:

- **While statements**
- **Else-if statements**
- **Chars and Strings**
- **Runtime error stack traces**

Stay tuned for updates as we make progress on the project!

## Getting started

To begin using `LumiScript`, follow the commands provided below:

```bash
git clone https://github.com/caiquetorres/lumiscript
cd lumiscript
cargo run -- --file path/to/file.ls
```

> The repository includes a folder named `samples`, where you can find some files that can be compiled and executed. Feel free to check them out by running the provided commands.

Now, let's get into the nitty-gritty, starting with the grammar.

## Types

Every programming language is built on data, like numbers, true/false values, and characters. That's where my language starts.

Right now, my language supports three main basic data types: `Nil`, `Bool`, and `Num`. These are the building blocks for making fancier data stuff in the language.

> By the way, I haven't gotten around to `Chars` and `Strings` yet, but I know they're important. For now, I'm focusing on getting collections up and running.

One thing that's always struck me as odd is how programming languages let you write expressions as standalone statements, like:

```js
2;
```

It's weird because you're not really doing anything with the value, but hey, it's totally accepted. So, I've decided to roll with this feature in my language too.

Sure thing, let's delve into some specific types.

### Num

In `LumiScript`, all numbers are treated as floats. The aim is to shape this language into a scripting language similar to JavaScript and Python, simplifying the process by avoiding the complexity of managing multiple numeric types.

For now, numbers can be added and subtracted (multiplication and division are not implemented yet, so let's take it one step at a time, shall we?). Below is a simple example of how we can do that:

```
println 1 + 2;
```

The same applies to subtractions.

```
println 1 - 2;
```

### Bool

When it comes to booleans, `LumiScript` keeps it simple: there's `true` and `false`, and that's it.

### Nil

I've got mixed feelings about nulls. They tend to complicate things—null pointer exceptions, "cannot read property length of null"... you know the drill. But I've decided to include them in `LumiScript` because, well, simplicity. But fear not! In the future, the type checker will have your back, making sure you don't mess up your code.

The syntax for nulls is straightforward: it's represented by the word `nil`.

Now, you might wonder, why `nil` instead of `null`? Well, the answer is simple: because I want to.

## Functions

We know functions are crucial—we really do. In `LumiScript`, functions are declared using the keyword `fun`. Check out an example below:

```
fun two() {
  println 2;
}
two(); // invocation
```

They can also receive parameters and return values, as shown in the example below.

```
fun sum(a: Num, b: Num) -> Num {
  a + b
}
```

> Now, here's an important point: there are two ways to return values from functions. The first one is using the keyword `return`, just like in any other programming language. The other way is by not using this keyword and omitting the semicolon at the end of the line (you can thank Rust and Kotlin for that).

## Classes

Alright, let's get down to business, shall we?

Classes are a crucial concept that empowers you to craft your own data types, as complex as you desire. Taking cues from Rust (except for the name—we're going with `class` instead of `struct`), classes in `LumiScript` are implemented to represent just their data, like so:

```
class Person {
  age: Num,
  isSingle: Bool
}
```

To create an instance of a class, you can use the following syntax:

```
Person {
  age: 23,
  isSingle: true
}
```

Now, let's dive into methods...

Rust introduces the fantastic concept of separating the class/struct from its methods, and I absolutely loved it the first time I saw it. So, naturally, I'm following suit (fingers crossed it works as well as I hope).

```
impl Person {
  fun shoutTheAge() {
    println this.age;
  }
}
```

Isn't that cool? Now we can define methods separately and keep our classes clean and focused.

Oh, and there's another thing—those implementations are scoped. Check out the code below to get a better understanding:

```
let person = Person { age: 23, isSingle: true };
{
  impl Person {
    fun shoutTheAge() {
      println this.age;
    }
  }
  person.shoutTheAge(); // works
}
person.shoutTheAge(); // doesn't work
```

With scoped implementations, like in the example above, you can see that methods defined within a certain scope are only accessible within that scope. Neat, huh?

### Special methods

Let's talk about something I've been itching to implement from the get-go: special methods.

To explain this concept, let's work with the class defined below:

```
class Point {
  x: Num,
  y: Num
}
```

In some languages, I can implement a special method that allows me to perform operations like addition and subtraction between values. We commonly see this with numbers—in basically all programming languages, we can add 1 + 2. But in my language, I want to take it further.

```
impl Add for Point {
  fun add(other: This) -> This {
    This {
      x: this.x + other.x,
      y: this.y + other.y
    }
  }
}
```

> Note that the code uses `This` and `this`. These symbols essentially refer to the class type and the instance, respectively.

The snippet above represents this behavior. By implementing the trait Add, my Point class is now capable of being added to another Point class using a simple + character (if you want to see that happening, check out samples/point.ls).

```
let p1 = Point { x: 1, y: 2 };
let p2 = Point { x: 3, y: 4 };
let p3 = p1 + p2;
```

Exciting, isn't it?

Currently, the traits that can be implemented are:

- `Add` - for addition
- `Sub` - for subtractions
- `Eq` - for comparisons
- `Not` - for negations

Keep in mind that I'm working on adding new ones. These basic calculations are just the beginning!
