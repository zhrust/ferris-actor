use std::{thread, time};

extern crate termion;
use termion::{clear,  cursor};

use handlebars::Handlebars;
use serde_json::json;

use crate::act::actor::*;

pub fn act() {
    commie()
}


fn commie() {
    let mut tpl = Handlebars::new();

    let mut i = 0;
    loop {
        i+=1;

        let sp = PRE[i % PRE.len()];
        //let sp = PRE[rng.gen_range(0..=PRE.len()-1)];
        let lb = _rng_pick(&LBROW   ).unwrap();
        let rb = _rng_pick(&RBROW   ).unwrap();
        let cr = _rng_pick(&CHAIR   ).unwrap(); //Center Hair
        let ln = _rng_pick(&LNIP    ).unwrap();
        let rn = _rng_pick(&RNIP    ).unwrap();
        let le = _rng_pick(&LEYE    ).unwrap();
        let re = _rng_pick(&REYE    ).unwrap();
        let mo = _rng_pick(&MOUTH   ).unwrap();
        let ll = _rng_pick(&LLEG    ).unwrap();
        let rl = _rng_pick(&RLEG    ).unwrap();
        let cn = _rng_pick(&CHIN    ).unwrap();

        if let Err(e) = tpl.register_template_string("ferr", CARB) {
                // 处理错误
                eprintln!("注册模板失败：{}", e);
            }

        let ferris = tpl.render("ferr", 
                    &json!({"sp": sp, 
                            "lb": lb,
                            "rb": rb,
                            "cr": cr,
                            "ln": ln,
                            "rn": rn,
                            "le": le,
                            "re": re,
                            "mo": mo,
                            "ll": ll,
                            "rl": rl,
                            "cn": cn,
                        }))
        .map_err(|e| {
            eprintln!("渲染模板失败：{}", e);
        })
        .map(|result| result.to_string())
        .unwrap_or_else(|_| {
            eprintln!("渲染模板失败：未知错误");
            String::new()
        });

        println!(
            //"\n{}{}{}{}{}{}",
            "\n{}{}{}{}",
            //cursor::Hide,
            cursor::Show,
            clear::All,
            cursor::Goto(1, 1),
            //color::Fg(color::Black),
            //color::Bg(color::Red),
            //COMMUNISM
            format!("{}", ferris)
        );

        thread::sleep(time::Duration::from_millis(242));
    }
}

/*我们首先使用 \x1B[2J 终端控制码清空屏幕，然后使用 \x1B[1;1H 将光标移动到左上角。接下来，我们迭代 frames 中的每一帧，并将其逐行输出到屏幕上。在输出完所有行之后，我们使用 stdout().flush().unwrap() 刷新输出缓冲区。然后，我们使用 \x1B[<n>A 将光标向上移动到刚刚输出的所有行的开头。最后，我们重复这个过程来显示下一帧。
*/
/* 
fn clippy() {
    //let nose = vec!["-", "\\", "|", "/","<",">","v","^"];

    let frames0 = vec![
        "  _______ ",
        " /       \\",
        "/         \\",
        "|         |",
        "|  O   O  |",
    ];
    let frames1 = vec![
        "|    <    |",
        "|    ^    |",
        "|    >    |",
        "|    v    |",
        "|    .    |",
        "|    -    |",
        "|    \\    |",
        "|    |    |",
        "|    /    |",
        "|    o    |",
    ];
    let frames2 = vec![
        "|         |",
        "| \\     / |",
        "|  \\___/  |",
        " \\_______/",
    ];

    let mut i = 0;
    loop {
        i += 1;
        print!("\x1B[2J"); // clear the screen
        print!("\x1B[1;1H"); // move the cursor to the top-left corner

        for frame in &frames0 {
            print!("{}\n", frame);
        }
        //for frame in &frames1 {
        //    print!("{}\n", frame);
        //}
        let frame = frames1[i % frames1.len()];
        print!("{}\n", frame);
        
        for frame in &frames2 {
            print!("{}\n", frame);
        }
        stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(100));

        //print!("\x1B[{}A", frames.len()); // move the cursor up
        print!("\x1B[{}A", frames0.len()+frames1.len()+frames2.len()); // move the cursor up
    }
} */

/*
const COMMUNISM: &'static str = r#"
              !#########       #
            !########!          ##!
         !########!               ###
      !##########                  ####
    ######### #####                ######
     !###!      !####!              ######
       !           #####            ######!
                     !####!         #######
                        #####       #######
                          !####!   #######!
                             ####!########
          ##                   ##########
        ,######!          !#############
      ,#### ########################!####!
    ,####'     ##################!'    #####
  ,####'            #######              !####!
 ####'                                      #####
 ~##                                          ##~
"#;
*/




