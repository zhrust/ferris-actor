use indoc::indoc;

use rand::Rng;

pub fn _rng_pick<T>(slice: &[T]) -> Option<&T> {
    let mut _rng = rand::thread_rng();
    slice.get(_rng.gen_range(0..slice.len()))
}

pub const CARB: &str = indoc! {r#"
{{{sp}}}    _~{{{lb}}}{{{cr}}}{{{rb}}}~_
{{{sp}}}{{{ln}}} /  {{{le}}} {{{re}}}  \ {{{rn}}}
{{{sp}}}  '_   {{{mo}}}   _'
{{{sp}}}  {{{ll}}} '--{{{cn}}}--' {{{rl}}}
"#};

pub static PRE: &[&str] = &[
            " ", "  ", "   ", "    ", "     ", "      ", "       ","        ",
            "        ","       ","      ","     ", "    ","   ", "  ", " ",
                            ];

pub static LBROW: &[&str] = &["~", "∽", "-", "`", "^"];
pub static RBROW: &[&str] = &["~", "∽", "-", "`", "^"];
pub static CHAIR: &[&str] = &["-", "~", "|", "*", "&", "+"];

pub static LNIP: &[&str] = &["\\)", "\\/", "()"];
pub static RNIP: &[&str] = &["(/", "\\/", "()"];

pub static LEYE: &[&str] = &[
            "o", "O", "-", "+", "=", "*", ">", "^", "?", "#",
            "♡", "◕", "◴", "◵", "◶", "◷", "☉", "←", "→"
                            ];
pub static REYE: &[&str] = &[
            "o", "O", "-", "+", "=", "*", "<", "^", "?", "#",
            "♡", "◕", "◴", "◵", "◶", "◷", "☉", "←", "→"
                            ];

pub static MOUTH: &[&str] = &[
            "⎵", "⏡", "⏝", "⌐", "⌄", "v", "V", "∧", "△", "▽", 
            "♢", "⩌", "𝟂", "⎕"
                            ];

pub static CHIN: &[&str] = &[
            ".", "~", "∽", "+", "-", "#", "⌄"
                            ];

pub static LLEG: &[&str] = &["(","/", "|", "\\", ">"];
pub static RLEG: &[&str] = &[")","/", "|", "\\", "<"];


/*
orig.
    _~^~^~_
\) /  o o  \ (/
  '_   ¬   _'
  \ '-----' /
*/

/* 
fn pick_one<T>(items: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    &items[rng.gen_range(0..items.len())]
}

pub fn pick_multi<'a, T>(items: &[&'a [T]]) -> Vec<String>
where
    T: std::fmt::Display,
{
    items
        .iter()
        .map(|x| pick_one(x).to_string())
        .collect::<Vec<String>>()
}

pub fn vec_to_tuple<T>(vec: Vec<T>) -> Option<Box<dyn FnOnce() -> T>>
where
    T: std::marker::Sized,
{
    match vec.into_iter().collect::<Vec<_>>().as_slice() {
        [] => None,
        [x] => Some(Box::new(move || x.clone())),
        xs => Some(Box::new(move || xs.clone().into())),
    }
}
 */
