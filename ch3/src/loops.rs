n main() {
    // пример бесконечного цикла
    //  loop {
    //     println!("again!");
    // }

    // возвращающий цикл
    // let mut counter = 0;
    //
    // let result = loop {
    //     counter += 1;
    //
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    //
    // println!("The result is {result}");

    // пример применения меток цикла
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;
    //
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //
    //     count += 1;
    // }
    // println!("End count = {count}");

    //  цикл с условием
    // let mut number = 3;
    //
    // while number != 0 {
    //     println!("{number}!");
    //
    //     number -= 1;
    // }
    //
    // println!("LIFTOFF!!!");

    //перебор значений при помощи while
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    //
    // while index < 5 {
    //     println!("the value is: {}", a[index]);
    //
    //     index += 1;
    // }

    // перебор значений при помощи  for
    // let a = [10, 20, 30, 40, 50];
    //
    // for element in a {
    //     println!("the value is: {element}");
    // }

    // пример for с использованием range
     for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
