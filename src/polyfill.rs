use crate::js_sys::{global, Object, Reflect};

// workaround: execute before generate_token
// see: https://stackoverflow.com/questions/70341840/how-to-polyfill-performance-now-in-rust-webassembly
pub fn set_performance() {
    let global = global();
    //let performance = {}
    let performance = Object::new();
    //performance.now = global.Date.now
    Reflect::set(
        &performance,
        &"now".into(),
        &Reflect::get(
            &Reflect::get(&global, &"Date".into()).unwrap(),
            &"now".into(),
        )
        .unwrap(),
    )
    .unwrap();
    //global.performance = performance
    Reflect::set(&global, &"performance".into(), &performance).unwrap();
}
