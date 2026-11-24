#![cfg(not(feature = "unchecked"))]
#![cfg(not(all(feature = "no_index", feature = "no_object")))]
use rhai::{Engine, EvalAltResult, INT};

#[track_caller]
fn too_deep(r: Result<impl std::fmt::Debug, Box<EvalAltResult>>) -> bool {
    matches!(*r.unwrap_err(), EvalAltResult::ErrorDataTooLarge(ref what, ..) if what == "Depth of nested arrays/maps")
}

#[test]
#[cfg(not(feature = "no_index"))]
fn test_max_data_depth_arrays() {
    let mut engine = Engine::new();
    engine.set_max_data_depth(3);
    assert_eq!(engine.max_data_depth(), 3);

    assert_eq!(engine.eval::<INT>("let a = [[[1]]]; a[0][0][0]").unwrap(), 1);
    assert!(too_deep(engine.eval::<INT>("let x = 1; let a = [[[[x]]]]; 0")));
    assert!(too_deep(engine.eval::<INT>("let a = []; for i in 0..10 { a = [a]; } 0")));
    assert!(too_deep(engine.eval::<INT>("let a = [[[]]]; a.push(a); 0")));
    assert!(too_deep(engine.eval::<INT>("let a = []; a.pad(2, [[[]]]); 0")));
    assert_eq!(engine.eval::<INT>("let a = []; a.pad(2, [[]]); a.len()").unwrap(), 2);
}

#[test]
#[cfg(not(feature = "no_object"))]
fn test_max_data_depth_maps() {
    let mut engine = Engine::new();
    engine.set_max_data_depth(3);

    assert_eq!(engine.eval::<INT>("let m = #{a: #{b: #{c: 1}}}; m.a.b.c").unwrap(), 1);
    assert!(too_deep(engine.eval::<INT>("let x = 1; let m = #{a: #{b: #{c: #{d: x}}}}; 0")));
    assert!(too_deep(engine.eval::<INT>("let m = #{}; for i in 0..10 { m = #{a: m}; } 0")));
    assert!(too_deep(engine.eval::<INT>("let m = #{a: #{b: #{}}}; m.c = m; 0")));
    #[cfg(not(feature = "no_index"))]
    assert_eq!(engine.eval::<INT>("let x = 1; let m = #{a: [#{b: x}]}; m.a[0].b").unwrap(), 1);
    #[cfg(not(feature = "no_index"))]
    assert!(too_deep(engine.eval::<INT>("let x = 1; let m = #{a: [#{b: [x]}]}; 0")));
}

/// Without a limit, nesting is as deep as ever; with one, a script nesting without end is
/// stopped by an error long before the native stack is at risk.
#[test]
#[cfg(not(feature = "no_index"))]
fn test_max_data_depth_keeps_the_stack() {
    let mut engine = Engine::new();
    assert_eq!(engine.max_data_depth(), 0);
    assert_eq!(engine.eval::<INT>("let a = []; for i in 0..50 { a = [a]; } 1").unwrap(), 1);

    let run = std::thread::Builder::new()
        .stack_size(1 << 20)
        .spawn(|| {
            let mut engine = Engine::new();
            engine.set_max_data_depth(64);
            too_deep(engine.eval::<INT>("let a = []; loop { a = [a]; }"))
        })
        .unwrap();
    assert!(run.join().unwrap());
}
