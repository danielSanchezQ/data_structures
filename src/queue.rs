use std::collections::vec_deque::VecDeque;

type Queue<T> = VecDeque<T>;

pub fn simulate_package_processing(buffer_size: usize, packages: &[(usize, usize)]) -> Vec<isize> {
    let mut buff: Queue<(usize, usize)> = Queue::with_capacity(buffer_size);
    let mut results = vec![0; packages.len()];
    let mut t = 0usize;
    let mut current: Option<(usize, usize)> = None;
    let mut packages_index = 0;
    loop {
        if buff.is_empty() && packages_index >= packages.len() {
            break;
        }
        current = match current {
            Some((_, 0)) => None,
            otherwise => otherwise,
        };
        'push_to_buffer: loop {
            if packages_index < packages.len() {
                let (package_t, package_cost) = packages[packages_index];
                if package_t > t {
                    break 'push_to_buffer;
                }
                let diff = if current.is_none() { 0 } else { 1 };
                if buff.len() < buffer_size && current.is_none() && package_cost == 0 {
                    results[packages_index] = t as isize;
                } else if buff.len() < buffer_size - diff {
                    buff.push_back((packages_index, package_cost));
                } else {
                    results[packages_index] = -1;
                }
                packages_index += 1;
            } else {
                break 'push_to_buffer;
            }
        }
        'process: loop {
            if buff.is_empty() && current.is_none() {
                break 'process;
            }
            match current {
                None => {
                    current = buff.pop_front();
                    match current {
                        None => {}
                        Some((i, _)) => {
                            results[i] = t as isize;
                        }
                    }
                }
                Some((current_i, 0)) => {
                    results[current_i] = t as isize;
                    current = buff.pop_front();
                }
                Some((current_i, current_cost)) => {
                    let new_cost = current_cost - 1;
                    current = Some((current_i, new_cost));
                    break 'process;
                }
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
