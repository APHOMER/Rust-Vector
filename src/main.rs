
// Creating a new, empty vector to hold values of type i32
// fn main() {
//     let v: Vec<i32> = Vec::new();
// }


// // Creating a new vector containing values
// fn main() {
//     let v = vec![1, 2, 3];
// }


// fn main() {
//     let mut v: Vec<i32> = Vec::new();

//     // let mut v = Vec::new();


//     v.push(5);
//     v.push(6);
//     v.push(7);
//     v.push(8);

//     // println!("The vector list is {:?}", v);

//     let third: &i32 = &v[2];

//     println!("The third element on the list is {:?}", third);


//     let fourth: Option<&i32> = v.get(3);
//     match fourth {
//         Some(fourth) => println!("The fourth element is {fourth}"),
//         None => println!("There is no fourth element."),
//     }


//     let v = vec![1, 2, 3, 4, 5];

//     let does_not_exist = &v[100]; // Program will panic
//     let does_not_exist = v.get(100); // This return None

// }


// ITERATING THROUGH IMMUTABLE ELEMENT

// fn main() {

//     let v = vec![100, 32, 57, 6, 8];

//     for i in &v {
//         println!("{i}");   
//     }

    // ADDING TO ELEMENTS

//     for i in &mut v {
//        // *i += 50;
//     }

    
//     println!("{v}"); 
// }


// USING AN ENUM TO STORE MULTIPLE TYPES

// fn main() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }


// fn main() {
//     let data = "initial contents";

//     let s = data.to_string();

//     // the method also works on a literal directly:
//     let s = "initial contents".to_string();
// }


// fn main() {
//     let hello = String::from("السلام عليكم");
//     let hello = String::from("Dobrý den");
//     let hello = String::from("Hello");
//     let hello = String::from("שלום");
//     let hello = String::from("नमस्ते");
//     let hello = String::from("こんにちは");
//     let hello = String::from("안녕하세요");
//     let hello = String::from("你好");
//     let hello = String::from("Olá");
//     let hello = String::from("Здравствуйте");
//     let hello = String::from("Hola");

// }

// fn main() {
//     // let mut s = String::from("foo");
//     // s.push_str(" bar");

//     // println!("{s}")

//     // let s1 = String::from("Hello, ");
//     // let s2 = String::from("world!");
//     // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

//     // println!("{s3}")

//     let s1 = String::from("tic");
//     let s2 = String::from("tac");
//     let s3 = String::from("toe");

//     // let s = s1 + "-" + &s2 + "-" + &s3;
//     let s = format!("{s1}-{s2}-{s3}");

//     println!("{s}");

//     let hello = String::from("Здравствуйте");

//     let l = hello.len();
//     println!("{l}");
    
//     // println!(`"{}", {&hello[0]}`);
//     println!("{}", {&hello[0..4]}); // firts 4 bytes of the str

//     // ....................
//     for c in "Зд".chars() {
//     println!("{c}");
// }
// }

// #![allow(unused)]
// fn main() {
// for c in "Зд".chars() {
//     println!("{c}");
// }
// }

    // FOR BYTES

// #![allow(unused)]
// fn main() {
// for b in "Зд".bytes() {
//     println!("{b}");
// }
// }




// Storing Keys with Associated Values in Hash Maps

// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Yellow"), 50);
//     scores.insert(String::from("Blue"), 25);

//     // scores.insert(String::from("Name"), "Abraham");
//     // scores.insert(String::from("Nickname"), "Abgadget");
//     // scores.insert(String::from("Department"), "Systems");
    
//     scores.insert(String::from("Level"), 500);


//     println!("{:?}", scores);
    
//     let team_name = String::from("Blue");
//     let score = scores.get(&team_name).copied().unwrap_or(0);

//     // println!("{score}");

//     for (key, value) in &scores {
//         if(*value > 10) {
//             println!("{key}: {value}");
//         } else {
//             println!("{value}");
//         }
//         // println!("{key}: {value}");
//     }

// }


// fn main() {
//     use std::collections::HashMap;

//     let mut scores = HashMap::new();

//     scores.insert(String::from("Blue"), 10);
//     scores.insert(String::from("Blue"), 25);

//     println!("{scores:?}");
// }


fn main() {
    use std::collections::HashMap;

    let text = "hello Mercy hope you are doing great today Mercy";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}






