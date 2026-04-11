type List<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Node<T> {
    pub val: T,
    pub next: List<T>,
}

#[allow(dead_code)]
fn middle_of_linked_list(head: List<i32>) -> i32 {
   let mut slow = head.as_ref();
   let mut fast = head.as_ref();

   while fast.is_some() {
       fast = fast
           .and_then(|x| {
               let next_ref = x.next.as_ref();
               if next_ref.is_some() {
                  slow = slow.and_then(|x| x.next.as_ref());
               }
               next_ref
           })
           .and_then(|x| x.next.as_ref());
   }

   match slow {
       Some(x) => x.val,
       None => 0,  // in "real world" we would prefer to return Option "all the way up" so we
                   // weren't forced to return meaningless/wrong defaults when no solution was
                   // found
   }
}


fn main() { }


#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_slice(values: &[i32]) -> List<i32> {
        let mut head: List<i32> = None;

        for &value in values.iter().rev() {
            head = Some(Box::new(Node {
                val: value,
                next: head,
            }));
        }

        head
    }

    #[test]
    fn test_single_element() {
        let input = list_from_slice(&[42]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_two_elements_returns_second() {
        let input = list_from_slice(&[1, 2]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_odd_length_list() {
        let input = list_from_slice(&[1, 2, 3, 4, 5]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_even_length_list_returns_second_middle() {
        let input = list_from_slice(&[1, 2, 3, 4, 5, 6]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_negative_values() {
        let input = list_from_slice(&[-5, -4, -3, -2, -1]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, -3);
    }

    #[test]
    fn test_duplicates() {
        let input = list_from_slice(&[7, 7, 7, 7, 7]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_longer_even_list() {
        let input = list_from_slice(&[10, 20, 30, 40, 50, 60, 70, 80]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_longer_odd_list() {
        let input = list_from_slice(&[9, 8, 7, 6, 5, 4, 3]);
        let result = middle_of_linked_list(input);
        assert_eq!(result, 6);
    }
}
