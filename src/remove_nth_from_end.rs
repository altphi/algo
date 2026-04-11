use std::error;
use std::fmt::Display;
use std::io;
use std::str::FromStr;

type List<T> = Option<Box<Node<T>>>;

#[derive(Clone)]
pub struct Node<T> {
    pub val: T,
    pub next: List<T>,
}

pub fn remove_nth_from_end(mut head: Option<Box<Node<i32>>>, n: i32) -> Option<Box<Node<i32>>> {
    let mut fast = head.as_ref();
    let mut counter: i32 = 0;

    while fast.is_some() {
      fast = fast.and_then(|x| x.next.as_ref());
      counter += 1;
    }

    let index_to_motify = counter - n;

    if index_to_motify == 0 {
        return head.unwrap().next;
    }

    let mut slow = head.as_mut();
    let mut index = 0;

    while let Some(node) = slow {
        index+=1;

        if index == index_to_motify {
            node.next = node.next.take().and_then(|x| x.next);
            break;
        }
        slow = node.next.as_mut();
    }

    head
}

fn build_list<'a, T, I>(iter: &mut I) -> Result<List<T>, Box<dyn error::Error>>
where
    T: FromStr,
    I: Iterator<Item = &'a str>,
    <T as FromStr>::Err: 'static + error::Error,
{
    let val = match iter.next() {
        Some(val) => val.parse()?,
        None => return Ok(None),
    };
    let next = build_list(iter)?;
    Ok(Some(Box::new(Node { val, next })))
}

fn format_list<T: Display>(node: &List<T>, out: &mut Vec<String>) {
    if let Some(node) = node {
        out.push(format!("{}", node.val));
        format_list(&node.next, out);
    }
}

fn print_words<T: Display>(v: &Vec<T>) {
    let mut iter = v.iter();
    if let Some(val) = iter.next() {
        print!("{}", val);
        for val in iter {
            print!(" {}", val);
        }
    }
    println!();
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let head = build_list(&mut line.split_whitespace())?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let n = line.trim_end().parse()?;
    let res = remove_nth_from_end(head, n);
    let mut out = Vec::new();
    format_list(&res, &mut out);
    print_words(&out);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list_from_vec(values: &[i32]) -> Option<Box<Node<i32>>> {
        let mut head = None;
        for &v in values.iter().rev() {
            head = Some(Box::new(Node { val: v, next: head }));
        }
        head
    }

    fn vec_from_list(head: &Option<Box<Node<i32>>>) -> Vec<i32> {
        let mut out = Vec::new();
        let mut cur = head.as_ref();
        while let Some(node) = cur {
            out.push(node.val);
            cur = node.next.as_ref();
        }
        out
    }

    #[test]
    fn removes_from_middle() {
        let head = list_from_vec(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(vec_from_list(&result), vec![1, 2, 3, 5]);
    }

    #[test]
    fn removes_head_when_n_equals_length() {
        let head = list_from_vec(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 5);
        assert_eq!(vec_from_list(&result), vec![2, 3, 4, 5]);
    }

    #[test]
    fn removes_tail_when_n_is_one() {
        let head = list_from_vec(&[1, 2, 3, 4, 5]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(vec_from_list(&result), vec![1, 2, 3, 4]);
    }

    #[test]
    fn removes_only_element() {
        let head = list_from_vec(&[1]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(vec_from_list(&result), Vec::<i32>::new());
    }

    #[test]
    fn removes_first_of_two() {
        let head = list_from_vec(&[1, 2]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(vec_from_list(&result), vec![2]);
    }

    #[test]
    fn removes_second_of_two() {
        let head = list_from_vec(&[1, 2]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(vec_from_list(&result), vec![1]);
    }

    #[test]
    fn removes_middle_in_three() {
        let head = list_from_vec(&[1, 2, 3]);
        let result = remove_nth_from_end(head, 2);
        assert_eq!(vec_from_list(&result), vec![1, 3]);
    }

    #[test]
    fn empty_list_stays_empty_if_you_choose_to_allow_it() {
        let head = list_from_vec(&[]);
        let result = remove_nth_from_end(head, 1);
        assert_eq!(vec_from_list(&result), Vec::<i32>::new());
    }
}
