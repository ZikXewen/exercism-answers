#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    if _first_list == _second_list {
        Equal
    } else if _first_list.len() < _second_list.len() {
        if _first_list.is_empty()
            || _second_list
                .windows(_first_list.len())
                .any(|x| x == _first_list)
        {
            Sublist
        } else {
            Unequal
        }
    } else {
        if _second_list.is_empty()
            || _first_list
                .windows(_second_list.len())
                .any(|x| x == _second_list)
        {
            Superlist
        } else {
            Unequal
        }
    }
}
