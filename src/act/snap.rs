use handlebars::Handlebars;
use serde_json::json;

use crate::act::actor::*;
use crate::act::version;

pub fn act() {
    
    snap()
}

fn snap() {
    
    //let mut _ver = version::get_version();
    let mut _ver = version::VERSION;
    //log::info!("_ver:\n\t {}", _ver);
    let mut tpl = Handlebars::new();
    //// 初始化日志记录器
    //Builder::from_env(Env::default().default_filter_or("info")).init();
    //// 创建一个带有自定义日志级别的 Handlebars 对象
    //let mut tpl = Handlebars::with_logging(|level, message| {
    //    if level <= log::Level::Info {
    //        println!("Handlebars log: {}: {}", level, message);
    //    }
    //});

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

    println!("{}" 
        , format!(
        "```\n{}\n...act by ferris-actor v{}\n```\n"
        //"```\n{}\n...act by ferris-actor\n```\n"
            , ferris 
            , _ver
        )
        //    , VERSION)
    );

    println!("Version: 0.2.2");

}

