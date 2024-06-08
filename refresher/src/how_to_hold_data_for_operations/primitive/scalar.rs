// getting this from main
use crate::how_to_hold_data_for_operations::derived::user_defined::Comp;

pub fn compare(x: i32, y: i32, comp: Comp) -> bool{
    match comp {
        Comp::LessThan => x<y,
        Comp::GreaterThan => x>y,
        Comp::Equals => x==y,
    }
}