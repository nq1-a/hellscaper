#[derive(Default)]
pub struct WRoll<'a> {
    pub init_bar: i32,
    pub crit_msg: &'a str,
    pub succ_msg: &'a str,
    pub fail_msg: &'a str,
    pub fumb_msg: &'a str,
    pub tail_msg: &'a str,
    pub pre_bias: i32,
    pub n1_bar_d: i32,
}
