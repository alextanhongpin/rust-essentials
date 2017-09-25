fn main() {
  let magician = "gandalf";
  match magician {
    "gandalf" => println!("a good magiciam"),
    "sauron" => println!("a magician gone bad!"),
    _ => println!("no magician turn up")
  };

  let meaning_of_line: i32 = 41;
  match meaning_of_line {
    1 => println!("unity!"),
    2 | 3 | 4 | 5 | 11 => println!("oookkkk"),
    num @ 40...42 => println!("close {}", num),
    _ => println!("nnoooo")
  }

  let loki = ("asd", false, 80u32);
  match loki {
    (name, demi, _) if demi => {
      println!("this is a demigod called {}", name);
    },
    (name, _, _) if name == "thor" => println!("this is thor"),
    (_, _, pow) if pow < 100 => {
      println!("this is a powerless god");
    },
    _ => println!("this is something else")
  }
}