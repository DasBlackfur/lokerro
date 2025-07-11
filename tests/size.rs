use lokerro::Result as LokerroResult;

#[test]
fn check_size() {
    assert_eq!(size_of::<LokerroResult<()>>(), size_of::<usize>())
}