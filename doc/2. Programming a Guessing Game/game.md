# 编写一个猜谜游戏

本章将通过一些真实的代码向你介绍一些 Rust 的基本概念。你将会学到`let`，`match`，methods，associated functions，使用 external crates 等等！

我们将要实现一个经典的初始编程问题：一个猜谜游戏。游戏规则：程序生成一个 1 到 100 之间的随机整数。让用户输入一个猜想，然后程序告诉用户猜想是高或是低。如果猜想正确，游戏打印恭喜并退出。

## 创建一个新的项目

创建一个新的项目：

```bash
cargo new guessing_game
cd guessing_game
```

我们看到生成的`Cargo.toml`文件：

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

## 编写猜谜游戏

第一步:

```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

让我们来一步一步的来看这段代码。为了获取用户的输入以及输出结果，我们需要引入`io`这个库。`io`这个库是一个基础库，即`std`：

```null
use std::io
```

接下来就是`main`函数：

```null
fn main() {
```

`fn`声明了一个新函数，而小括号`()`表明没有入参，花括号`{`则开始函数的本体。

在第一章里我们知道了`println!`是一个向屏幕输出的宏：

```null
    println!("Guess the number!");

    println!("Please input your guess.");
```

### 使用变量存储值

接下来我们创建一个地方用于储存用户的输入：

```null
    let mut guess = String::new();
```

注意这里是一个`let`的声明，这是用于创建一个*变量*。这里是另一个例子：

```null
let foo = bar;
```

这一行代码创建了一个新变量`foo`并绑定了`bar`变量的值。在 Rust 中，变量默认是不可变的。我们将会在第三章的"Variables and Mutability"小结里再进行详细的讨论。下面的案例中展示，在一个变量名前使用`mut`将使改变量变得不可变：

```null
let foo = 5; // immutable
let mut bar = 5; // mutable
```

回到猜谜项目中来。你现在知道了`let mut guess`将创建一个变量`guess`。在等号`=`的另一边则是`guess`所绑定的值，这里是`String::new`这个函数的值。这个函数返回一个新的`String`实例。`String`是一个由标准库提供的字符串类型。

语义`::`在`::new`代表`new`是一个`String`类型的关联函数（associated function）。关联函数是一个在类型上的实现，而不是一个确切的`String`实例。在别的一些语言中这叫静态方法（static method）。

`new`函数创建了一个新的空字符串。你将会发现在很多类型上都有一个`new`函数，因为这是一个很普遍的函数专门用作于在创建一个某种类型的值。

总结一下，`let mut guess = String::new();` 创建了一个可变的变量并且绑定了一个新的空的字符串实例。

我们从标准库`use std::io;`中获取了 input/output 的功能，现在我们从`io`模块中调用`stdin`函数：

```null
    io::stdin()
        .read_line(&mut guess)
```

如果我们没有在项目起始写`use std::io;`，我们也可以`std::io::stdin`来调用该函数。`stdin`函数返回一个`std::io::Stdin`的实例，用于处理终端的输入。

接下来`.read_line(&mut guess)`，调用`read_line`方法来接收用户的输入。同事我们也传入了一个参数`&mut guess`给`read_line`。

`read_line`的作用是接收任意用户的输入类型并转换为字符串。字符串参数需要是可变的，这样才能由用户输入来改变字符串的内容。

`&`表示这个参数是一个引用，意味着你可以在你的代码中多次引用它而不需要多次复制该数据进内存。引用是一个复杂的特性，Rust 的一个主要优势就是安全和简单的使用引用。现在我们只需要知道引用和变量类似，而且引用默认是不可变的。因此我们需要写`&mut guess`而不是`&guess`。

### 处理失败的结果类型

```null
    .expect("Failed to read line");
```

当类似`.foo()`这样的语义出现时，另起一行便于阅读。

`read_line`接受用户输入转换成字符串，并且返回一个值。这里的话是一个`io::Result`。Rust 的标准库中有很多类型的`Result`：一个泛型的`Result`同样拥有很多个版本的子模块，例如：`io::Result`。

这个`Result`类型是枚举类。枚举类是一个类型可以拥有有限集合的值，这些值被称为枚举的变量（variants）。第六章将会展示更多的细节。

`Result`的变量是`Ok`或者`Err`。`Ok`表示运算成功，并且返回的值会储存于`Ok`中。`Err`表示运算失败，并且返回的错误信息会被存储于其中。

这些`Result`类型设计出来的目的是编码错误信息。`Result`类型的值，与其它各种类型的值一样都有各自的方法。一个`io::Result`实例有一个`expect`的方法可以被调用。如果这个`io::Result`实例是一个`Err`值，`expect`将会导致程序崩溃并展示你之前输入进`expect`的值。如果`read_line`方法返回一个`Err`，它很有可能是因为操作系统带来的错误。如果`io::Result`的实例是`Ok`值，`expect`则会返回`Ok`的值。如果不使用`expect`，程序可以被编译，但是你会得到如下警告：

```null
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust 将警告你还没有使用从`read_line`返回的`Result`值，这表明程序还没有正确的处理一个可能发生的错误。

正确的避免警告的方式是写一个错误处理，但是如果你本意就是想当一个错误产生时让程序崩溃，你可以使用`expect`。你将在第九章学到错误恢复的内容。

### 从`println!`句柄中打印值

```null
println!("You guessed: {}", guess);
```

这一行负责打印用户输入的保存为字符串的值。花括号`{}`为占位符。可以如下使用更多的占位符：

```rust
let x = 5;
let y = 10;

println!("x = {} and y = {}", x, y);
```

## 生成一个秘密数字

接下来我们需要生成一个数字让用户来猜。这个秘密数字需要每次运行的时候都不一样。由于 Rust 的标准库里没有一个随机数生成的函数，我们需要引入一个外部的 crate。一个名为`rand`的 crate。

### 使用一个 Crate 来获取更多的功能

回忆一下，一个 Crate 代表着一个 Rust 源代码的集。我们正在构建的项目是一个可执行的二进制 crate。`rand`这个 crate 是一个库，其包含了可供别的项目使用的代码。

在使用`rand`这个 crate 之前，我们需要修改一下`Cargo.toml`文件:

```toml
[dependencies]
rand = "0.5.5"
```

Cargo 明白 Semantic Versioning（也叫 SemVer）,这是一种版本命名标准。 `0.5.5`其实是`^0.5.5`的缩写，意味着接受介于`0.5.5`和`0.6.0`之间的任何版本。

```null
$ cargo build
    Updating crates.io index
  Downloaded rand v0.5.5
  Downloaded libc v0.2.62
  Downloaded rand_core v0.2.2
  Downloaded rand_core v0.3.1
  Downloaded rand_core v0.4.2
   Compiling rand_core v0.4.2
   Compiling libc v0.2.62
   Compiling rand_core v0.3.1
   Compiling rand_core v0.2.2
   Compiling rand v0.5.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

你或许看到了不同的版本号（但是他们是可以被编译的，感谢 SemVer！），展示的内容取决于操作系统。

这样我们有了一个外部依赖。Cargo 从`registry`中获取最新的版本，也就是从`Crates.io`中获取一份拷贝数据。Crates.io 是人们用于构建 Rust 生态圈的地方。

在更新完 registry 后，Cargo 检查`[dependencies]`部分然后下载你还没有的 crates。在这里，虽然我们只列了`rand`是一个依赖，Cargo 还是会抓取`libc`和`rand_core`，因为`rand`依赖它们俩。下载完这些 crates，Rust 编译它们然后再编译我们的项目。

...

### 根据`Cargo.lock`文件确保项目可被重复生产

...

### 更新一个 Crate

...

## 生成一个随机数

现在修改一下`src/main.rs`文件：

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

首先，我们加了一行`use`：`use rand::Rng`。`Rng`这个 trait 特性定义了生成随机数的方法，同事这个特性必须在作用域中为我们服务。第十章节将会详细讲解 traits。

其次，我们又加了两行代码。`rand::thread_rng`函数给予了我们特定的随机数生成器来使用：在当前线程下执行并且有操作系统生成。接着我们调用随机数生成器中的`gen_range`方法。这个方法被定义在`Rng`特性里。`gen_range`方法接受两个数作为入参，然后生成一个介于其中的随机数。

...

## 猜想数字和秘密数字的比较

接下来我们写一段暂时不要编译的代码：

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

首先，这里又使用了一个`use`声明，从标准库中带来的`std::cmp::Ordering`。与`Result`一样，`Ordering`是另一个枚举，而它的枚举变量是`Less`，`Greater`和`Equal`。

我们加了五行关于`Ordering`的代码。`cmp`方法用于比较两个值，当然也可以比较任意可被比较的值。它接收一个引用：这里用`guess`去和`secret_number`作比较。接着它会返回一个`Ordering`的枚举值。我们用`match`表达式得出的`Ordering`枚举值来决定下一步该做什么。

一个 match 表达式使用 arms 所组成的。一个 arm 由一个 pattern 和它的执行代码所组成，当匹配上以后，该代码将会被执行。Rust 获取值后交给`match`然后逐个检查 arm 的 pattern。细节将会在第六和第十八章节讲到。

...

如果现在就编译现在的代码的话我们会发现：

```null
$ cargo build
   Compiling libc v0.2.51
   Compiling rand_core v0.4.0
   Compiling rand_core v0.3.1
   Compiling rand v0.5.6
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game`

To learn more, run the command again with --verbose.
```

这个错误主要的地方在于不匹配的类型。Rust 是一个强类型的静态语言。因此不能比较字符串和数字类型。

最终我们希望转换`String`成一个数字类型。如下：

```rust
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

我们看一下：

```null
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

我们创建了一个叫`guess`的变量。这里我们看到之前`guess`已经被声明过了。Rust 让我们 shadow 重影之前的`guess`值（用同一个名字重新代表另一个变量实体）。这个特性经常被用于转换一个值的类型。重影让我们复用`guess`变量名，而不用强制创建两个独立的变量。细节将会在第三章节讲到。

我们绑定`guess`到表达式`guess.trim().parse()`。在表达式中的`guess`是原来`String`类型的`guess`。在`String`实例上的`trim`方法则会剪除字符串前后所有的空格。尽管`u32`只能存放数字，但是用户需要输入回车来满足`read_line`。当用户按下回车，一个新的字符便加入到了改字符串。比方说，用户输入 5 然后按下回车，`guess`将会像这样：`5\n`。`\n`代表着“新起一行”。`trim`方法剪除了`\n`，只会返回`5`。

字符串的`parse`方法则是转换一个字符串为某一种数字类型。因为这个方法可以转换很多种数字类型，所以我们用`let guess: u32`来告诉 Rust 确切的类型。这里的在`guess`后的冒号`:`告诉 Rust 我们注解了变量的类型。

调用`parse`很容易产生错误，比如我们输入的不是数字。因此我们接上`expect`。...

现在运行程序：

```null
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!
```

## 允许循环的猜测

`loop`这个关键字创建了一个无限循环：

```rust
    // --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

我们把猜测这一部分的代码放入了 loop 里面，注意 loop 里面的缩进也是四个空格。一个新的问题产生了：程序永远在问下一个猜想，用户没法退出！

用户当然是可以用 ctrl-c 退出程序。另一个办法是触发错误：例如输入一个非数字的答案。

### 在正确的猜想后退出

我们声明`break`用于退出程序：

```rust
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

在`You win!`后加上`break`可以让程序退出循环。退出循环也意味着退出程序，因为循环是`main`的最后部分。

### 处理非法的输入

当用户输入非数字时也能确保程序不崩溃并继续运行。我们可以让`guess`的类型从`String`变为`u32`：

```rust
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // --snip--
```

从之前调用`expect`换成了`match`表达式，这让我们可以从错误崩溃变为错误处理。`parse`返回的是一个`Result`类型，而`Result`类型是一个枚举包含了`Ok`和`Err`。这里的`match`表达式正如之前处理`cmp`方法的`Ordering`一样。

如果`parse`成功的转换一个字符串成数字，它会返回一个`Ok`值并包含着这个数字，最终`guess`会变为这个数字。

如果`parse`的转换不成功，它会返回一个`Err`值并包含着错误信息。这里的下划线`_`意思是捕获所有值，在这个例子中我们捕获的是所有的异常值。接下来程序将会执行`continue`，这是告诉程序进入下一个`loop`循环。综上所述，程序就忽略了所有`parse`可能带来的错误了。

最后来看一下我们的代码：

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## 总结

这个项目向你介绍了 Rust 的很多新概念：`let`，`match`，方法，关联函数，外部 crates 的使用，等。下一个章节里你将会学到更多的细节。第三章节里包含了其他语言里也有的内容：变量，数据类型，函数。第四章节讲解所有权，这是一个让 Rust 与其它语言不一样的特性。第五章节讨论结构和方法，第六章节解释枚举。
