use std::env;

use handlebars::Handlebars;
use serde_json::json;

use crate::act::actor::*;

pub fn act() {
    snap()
}

fn snap() {
    let mut tpl = Handlebars::new();

    let sp = _rng_pick(&PRE).unwrap();
    //let mut rng = rand::thread_rng();
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

/* 
    let mut _actor = pick_multi(&[
            &LBROW,
            &RBROW,
            &CHAIR,
            &LNIP,
            &RNIP,
            &LEYE,
            &REYE,
            &MOUTH,
            &LLEG,
            &RLEG,
            &CHIN,
            ]);

    let [   lb,
            rb,
            cr,
            ln,
            rn,
            le,
            re,
            mo,
            ll,
            rl,
            cn,
        ] = vec_to_tuple(_actor);


    let lb = LBROW[ rng.gen_range(0..=LBROW.len()-1) ];
    let rb = RBROW[ rng.gen_range(0..=RBROW.len()-1) ];
    let cr = CHAIR[ rng.gen_range(0..=CHAIR.len()-1) ]; //Center Hair
    let ln =  LNIP[ rng.gen_range(0..=LNIP.len()-1) ];
    let rn =  RNIP[ rng.gen_range(0..=RNIP.len()-1) ];
    let le =  LEYE[ rng.gen_range(0..=LEYE.len()-1) ];
    let re =  REYE[ rng.gen_range(0..=REYE.len()-1) ];
    let mo = MOUTH[ rng.gen_range(0..=MOUTH.len()-1) ];
    let ll =  LLEG[ rng.gen_range(0..=LLEG.len()-1) ];
    let rl =  RLEG[ rng.gen_range(0..=RLEG.len()-1) ];
    let cn =  CHIN[ rng.gen_range(0..=CHIN.len()-1) ];

 */
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
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    println!("{}" 
        , format!(
        "```\n{}\n...act by ferris-actor v{}\n```\n"
            , ferris 
            , version)
    );

}

/* 
PS: got author from Cargo.toml
cargo metadata --format-version 1 | jq -r '.packages[0].authors[0]'

or ~>

use std::env;
use serde_json::Value;

fn main() {
    let metadata = env::var("CARGO_MANIFEST_DIR").unwrap() + "/Cargo.toml";
    let contents = std::fs::read_to_string(metadata).unwrap();
    let value: Value = toml::from_str(&contents).unwrap();
    let authors = value["package"]["authors"].as_array().unwrap();
    let first_author = authors[0].as_str().unwrap();
    println!("First author: {}", first_author);
}

*/
