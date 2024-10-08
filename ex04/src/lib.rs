// Alternative solution:
// 1. Sort by width
// 2. Sort within each width by height and remember the lowest height
// 3. Remember the lowest height

#[allow(dead_code)]
fn sort_boxes(boxes: &mut [[u32; 2]]) {
    let mut amount = boxes.len();
    let mut sorted = false;

    if amount < 2 {
        return;
    }

    while !sorted && amount > 1 {
        sorted = true;
        let mut i = 1;
        while i < amount {
            if !is_fit(&boxes[i - 1], &boxes[i]) {
                boxes.swap(i - 1, i);
                sorted = false;
            }
            i += 1;
        }
        amount -= 1;
    }

    amount = boxes.len();
    let mut i = 1;
    while i < amount {
        if !is_fit(&boxes[i - 1], &boxes[i]) {
            panic!();
        }
        i += 1;
    }

    // for (i, cur_box) in boxes.iter_mut().enumerate() {
    //     while cur_box.iter().next() != None {
    //         if !is_fit(&boxes[i + 1], cur_box) {
    //             boxes.swap(i, i + 1);
    //         }
    //     }
    // }
}

fn is_fit(big: &[u32; 2], small: &[u32; 2]) -> bool {
    big[0] >= small[0] && big[1] >= small[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut boxes = [[3, 3], [4, 3], [1, 0], [5, 7], [3, 3]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }

    #[test]
    fn empty_array() {
        let mut boxes: [[u32; 2]; 0] = [];
        let boxes_res: [[u32; 2]; 0] = [];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, boxes_res);
    }

    #[test]
    fn single_element() {
        let mut boxes = [[1, 2]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[1, 2]]);
    }

    #[test]
    fn already_sorted() {
        let mut boxes = [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }

    #[test]
    fn reverse_sorted() {
        let mut boxes = [[1, 0], [3, 3], [3, 3], [4, 3], [5, 7]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[5, 7], [4, 3], [3, 3], [3, 3], [1, 0]]);
    }

    #[test]
    #[should_panic]
    fn mixed_order() {
        let mut boxes = [[2, 2], [1, 1], [3, 3], [2, 1], [1, 2]];
        sort_boxes(&mut boxes);
        assert_eq!(boxes, [[3, 3], [2, 2], [2, 1], [1, 2], [1, 1]]);
    }

    #[test]
    #[should_panic]
    fn panic_case_1() {
        let mut boxes = [[5, 3], [5, 2], [8, 5], [2, 2], [1, 2], [2, 1]];
        sort_boxes(&mut boxes);
    }

    #[test]
    #[should_panic]
    fn panic_case_2() {
        let mut boxes = [[2, 1], [5, 3], [5, 2], [1, 2], [8, 5], [2, 2]];
        sort_boxes(&mut boxes);
    }
}
