#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal
    } else if first_list.len() == 0 {
        return Comparison::Sublist
    } else if second_list.len() == 0 {
        return Comparison::Superlist
    } else if first_list.len() > second_list.len() && is_superlist(first_list, second_list) {
        return Comparison::Superlist
    } else if is_superlist(second_list, first_list) {
        return Comparison::Sublist
    }

    return Comparison::Unequal
}

fn is_superlist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    let sublist_start_indices : Vec<usize> = first_list
        .iter()
        .enumerate()
        .filter(|(_, val)| **val == second_list[0])
        .map(|(i,_)| i)
        .collect();

    for start_index in sublist_start_indices {
        let sublist_end_index = start_index + second_list.len() - 1;
        if (sublist_end_index <= first_list.len() - 1) && (&first_list[start_index..(start_index + second_list.len())] == second_list) {
            return true;
        }
    }
    return false;
}
