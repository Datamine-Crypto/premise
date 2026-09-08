#[test]
#[should_panic(expected = "attempt to subtract with overflow")]
fn underflow_is_refused() {
    let _ = 0u32 - 1u32;
}

#[test]
#[ignore = "needs the desk to confirm the visit cycle"]
fn cycle_is_confirmed() {}
