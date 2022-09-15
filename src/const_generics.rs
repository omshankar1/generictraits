// Const Generics
// is_ordered for Array[T, N elements] -> [T, N]

pub trait IsOrdered {
    fn is_ordered(&self) -> bool;
}

impl<T, const N: usize> IsOrdered for [T; N]
where
    T: Copy + Ord,
{
    fn is_ordered(&self) -> bool {
        if matches!(self.len(), 0..=1) {
            return true;
        }
        let mut prev = None;
        for item in self {
            if Some(item) < prev {
                return false;
            }
            prev = Some(item);
        }
        true
    }
}

pub fn ordered_array() {
    // Const Generics

    let array1 = [1, 2, 3, 4];
    let array2 = [1, 3, 4, 2];
    println!("{}", array1.is_ordered());
    println!("{}", array2.is_ordered());
}
