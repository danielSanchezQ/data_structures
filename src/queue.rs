use std::collections::vec_deque::VecDeque;

type Queue<T> = VecDeque<T>;

pub fn simulate_package_processing(buffer_size: usize, packages: &[(usize, usize)]) -> Vec<isize> {
    let mut buff: Queue<(usize, usize, usize)> = Queue::with_capacity(buffer_size);
    let mut results = vec![0; packages.len()];
    let mut t = 0;
    let mut packages_index = 0;

    loop {
        if buff.is_empty() && packages_index >= packages.len() {
            break;
        }
        // removed 0 from buff
        'process_zero: while !buff.is_empty() {
            match buff.front() {
                Some((i, 0, original_cost)) => {
                    results[*i] = (t - original_cost) as isize;
                    buff.pop_front();
                }
                _ => {
                    break 'process_zero;
                }
            }
        }

        // add items to buff
        'add_items: loop {
            if packages_index >= packages.len() {
                break 'add_items;
            }
            let (t1, cost) = packages[packages_index];
            if t1 > t {
                break 'add_items;
            }
            if buff.is_empty() && cost == 0 {
                results[packages_index] = t as isize;
            } else if buff.len() >= buffer_size {
                results[packages_index] = -1;
            } else {
                buff.push_back((packages_index, cost, cost));
            }
            packages_index += 1;
        }

        // process las package
        if let Some(item) = buff.front_mut() {
            if item.1 > 0 {
                item.1 -= 1;
            }
        }
        t += 1;
    }

    results
}

#[cfg(test)]
mod test {
    use crate::queue::simulate_package_processing;

    #[test]
    fn test_package_simulation_examples() {
        assert_eq!(simulate_package_processing(1, &[]), vec![]);
        assert_eq!(simulate_package_processing(1, &[(0, 0)]), vec![0]);
        assert_eq!(
            simulate_package_processing(1, &[(0, 1), (0, 1)]),
            vec![0, -1]
        );
        assert_eq!(
            simulate_package_processing(1, &[(0, 1), (1, 1)]),
            vec![0, 1]
        );
        assert_eq!(
            simulate_package_processing(1, &[(0, 0), (0, 0)]),
            vec![0, 0]
        );
        assert_eq!(
            simulate_package_processing(1, &[(0, 1), (0, 0)]),
            vec![0, -1]
        );
        assert_eq!(
            simulate_package_processing(2, &[(0, 1), (0, 1)]),
            vec![0, 1]
        );
        assert_eq!(
            simulate_package_processing(1, &[(0, 2), (1, 4), (5, 3)]),
            vec![0, -1, 5]
        );
    }
}
