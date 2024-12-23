/*

Problem A:
The first section specifies the page ordering rules, one per line. The first rule, 47|53, means that if an update
includes both page number 47 and page number 53, then page number 47 must be printed at some point before page number 53.
(47 doesn't necessarily need to be immediately before 53; other pages are allowed to be between them.)

The second section specifies the page numbers of each update. Because most safety manuals are different, the
pages needed in the updates are different too. The first update, 75,47,61,53,29, means that the update consists of page
numbers 75, 47, 61, 53, and 29.

To get the printers going as soon as possible, start by identifying which updates are already in the right order.

For some reason, the Elves also need to know the middle page number of each update being printed. Because you are
currently only printing the correctly-ordered updates, you will need to find the middle page number of each
correctly-ordered update.

What do you get if you add up the middle page number from those correctly-ordered updates?

*/

use super::AOC;
use super::{read_input_file, PATH};

use std::collections::HashMap;

static FILE: &'static str = "aoc_5.txt";

#[allow(dead_code)]
impl AOC {
    pub fn aoc_5_a() -> i32 {
        let data = read_input_file(format!("{PATH}/{FILE}"));
        let (orders, page_list) = Self::transform_data_5(data);

        let mut hm = HashMap::new();

        let mut total = 0;

        for order in orders {
            let ref_hm = hm.entry(order.0).or_insert(HashMap::new());
            ref_hm.insert(order.1, true);
        }

        for pages in page_list {
            let mut is_valid = true;
            for i in 0..pages.len() - 1 {
                if let Some(order_map) = hm.get(&pages[i]) {
                    // If it is in the HashMap, then things should be after it
                    // it it isn't in the HashMap, then things should not be after it.
                    if let None = order_map.get(&pages[i + 1]) {
                        is_valid = false;
                    }
                } else {
                    // I expect nothing else in the list to be in the HashMap.
                    is_valid = false;
                }
            }

            if is_valid {
                total += pages[pages.len() / 2];
            }
        }

        total
    }

    pub fn aoc_5_b() -> i32 {
        let data = read_input_file(format!("{PATH}/{FILE}"));
        let (orders, page_list) = Self::transform_data_5(data);

        let mut hm = HashMap::new();

        let mut total = 0;

        for order in orders {
            let ref_hm = hm.entry(order.0).or_insert(HashMap::new());
            ref_hm.insert(order.1, true);
        }

        for pages in page_list {
            let mut is_error = false;

            for i in 0..pages.len() - 1 {
                if let Some(order_map) = hm.get(&pages[i]) {
                    if let None = order_map.get(&pages[i + 1]) {
                        is_error = true;
                    }
                } else {
                    is_error = true;
                }
            }

            if is_error {
                let mut new_pages: Vec<i32> = Vec::new();
                for page in &pages {
                    let curr_len = new_pages.len();

                    if new_pages.is_empty() {
                        new_pages.push(page.clone());
                        continue;
                    }

                    if let Some(new_order_map) = hm.get(&page) {
                        for j in 0..new_pages.len() {
                            if let Some(_) = new_order_map.get(&new_pages[j]) {
                                new_pages.insert(j, page.clone());
                                break;
                            }
                        }

                        if curr_len == new_pages.len() {
                            new_pages.push(page.clone());
                        }
                    } else {
                        new_pages.push(page.clone());
                    }
                }

                total += new_pages[new_pages.len() / 2];
            }
        }

        total
    }

    fn transform_data_5(data: Vec<String>) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
        let mut orders: Vec<(i32, i32)> = Vec::new();
        let mut page_list: Vec<Vec<i32>> = Vec::new();

        for row in data {
            match row.len() {
                5 => {
                    let number: Vec<i32> = row
                        .split("|")
                        .map(|nums| nums.parse::<i32>().unwrap())
                        .collect();
                    orders.push((number[0], number[1]));
                }
                0 => {}
                _ => {
                    let pages = row
                        .split(",")
                        .map(|page| page.parse::<i32>().unwrap())
                        .collect();
                    page_list.push(pages);
                }
            }
        }

        (orders, page_list)
    }
}
