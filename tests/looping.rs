use rhai::{Engine, EvalAltResult, ParseErrorType, INT};

#[test]
fn test_loop() -> Result<(), Box<EvalAltResult>> {
    let engine = Engine::new();

    assert_eq!(
        engine.eval::<INT>(
            "
				let x = 0;
				let i = 0;

				loop {
					if i < 10 {
						i += 1;
						if x > 20 { continue; }
						x += i;
					} else {
						break;
					}
				}

				return x;
		    "
        )?,
        21
    );

    assert_eq!(
        *engine
            .compile("let x = 0; break;")
            .expect_err("should error")
            .err_type(),
        ParseErrorType::LoopBreak
    );

    #[cfg(not(feature = "no_function"))]
    assert_eq!(
        *engine
            .compile("loop { let f = || { break;  } }")
            .expect_err("should error")
            .err_type(),
        ParseErrorType::LoopBreak
    );

    assert_eq!(
        *engine
            .compile("let x = 0; if x > 0 { continue; }")
            .expect_err("should error")
            .err_type(),
        ParseErrorType::LoopBreak
    );

    Ok(())
}
