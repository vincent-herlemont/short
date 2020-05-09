pub trait NewCfg {
    type T;
    fn new() -> Self::T;
}
