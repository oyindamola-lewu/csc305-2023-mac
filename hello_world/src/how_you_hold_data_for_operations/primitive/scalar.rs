use crate::how_you_hold_data_for_operations::derived::user_defined::Comp;

pub fn compare(x: i32, y: i32, comparison: Comp) -> bool {
    match comparison {
        Comp::LessThan => x<y,
        Comp::GreaterThan => x>y,
        Comp::Equals => x==y
    }
}