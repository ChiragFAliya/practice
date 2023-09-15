// fn main() {
//     let x = 6;
//     println!("{x}",)
// }

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}",y);
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn main(){
//     let m = 5;

//     if m < 3 {
//         println!("godd");
//     } else {
//         println!("not good");
//     }
// }

// fn main() {
//     let number = 3;

//     if number <= 3 {
//         println!("number was three");

//     }
// }

// fn main() {
//     let number = 96;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }


// fn main() {
//     let condition = false;

//     let number = if condition { "kiu" } else { "six" };

//     println!("The value of number is: {number}");
// }

// fn main() {
//     loop {
//         println!("again!");
//     }
// }
  
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };
//     println!("The result is {result}");
// }

// fn main(){
//     let mut number = 5;

//     while number != 0 {
//         println!("{}",number);

//         number -= 1;
//     }
//     println!("liftoff")
// }

// fn main() {
//     let a: [i32; 10] = [5; 10];
//     let mut sum = 0;
//     for x in a {
//         sum += x;
//     }
//     println!("{sum}");
// }


// fn main() {
//     let mut x = 0;
//     'a: loop {
//         x += 1;
//         'b: loop {
//             if x > 10 {
//                 continue 'a;
//             } else {
//                 break 'b;
//             }      
//         }
//         break;       
//     }
// }

// fn main (){
//     let three = 20i8;
//     println!("{three}",)
// }

// fn main(){
//     println!("{}",! 13==13)
// }

// fn main (){
//     println!("my \nname \nis \nkhan");
//     println!("hello \nworld")
// }
// fn main(){
//     let x ="chirag";
//     match x {
//         "abhi"=> println!("one"),
//         "chirag" => println!("thiese"),
//         "faliya" => println!("big number"),
//         _ => println!("out of range"),
//     }
// }

// fn main(){
//     if 1 == 1{
//         if 3 < 5{
//             println!("jay ho");
//         }
//         else {println!("jay mataji")}
//     }
// }

// fn main(){
//     for i in 1..=10{
//         if i == 6{
//             continue;
//         }
//         println!("{}",i)
//     }
// }

// fn main(){
//     let mut x = 0;
//     while x != 10 {
//         println!("{x}");
//         x+=1;
//     }
// }

// fn main(){
//     let mut x = 1;
//     while x <= 10 {
//         if x == 6{
//             x += 1;
//             continue;
//         }  
//         println!("{x}");
//         x+=1;
    
//     }
// }

// fn main(){
//     for i in (1..=50).rev(){
//         println!("{}",i)
//     }
// }

// fn main(){
//     let a: (i32,&str,i32,i32,bool)=(25,"abd",554,24,false);
//     println!("{:?}",a)
//     println!("{:?}",a.3)
// }

// fn main(){
//     let _1 = [1,2,3,4];
//     println!("{:?}",_1);

//     let _2 = [25;4];
//     println!("{:?}",_2);

//     let mut _3 = [25;4];
//     _3[3] = 22;

//     println!("{:?}",_3)
// }

// fn main(){
//     let mut arr = [[1,2,3,],[11,12,13],[21,22,23],[55,56,58]];
//     println!("{:?}",arr[2][2]);
//     arr[2] = [55,55,55];
//     println!("{:?}",arr)
// }

// fn main(){
//     let arr = [1,2,3,4,5,6,7,8,9];
//     // for i in arr.iter(){
//     //     println!("{:?}",i)
//     // }
//     for i in 0..10{
//         println!("{:?}",arr[i])
//     }

// }

