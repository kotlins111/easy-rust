#[cfg(test)]
mod test {
    use easy_rust::anti_null_exception::{check_if_five, give_result, take_fifth};
    use easy_rust::traits::EvenOddVec;
    use std::collections::BinaryHeap;

    #[test]
    fn test1() {
        let number = take_fifth(vec![1, 2, 3, 4]);
        let number2 = take_fifth(vec![1, 2, 3, 4, 5]).unwrap();
        let number3 = take_fifth(vec![1, 2, 3, 4, 5]).unwrap_or(0); // can use match
        assert_eq!(number, None);
        assert_eq!(number2, 5);
    }

    #[test]
    fn test2() {
        let new_vec = vec![1, 2];
        let bigger_vec = vec![1, 2, 3, 4, 5];
        let vec_of_vecs = vec![new_vec, bigger_vec];
        for vec in vec_of_vecs {
            let inside_number = take_fifth(vec);
            if inside_number.is_some() {
                assert_eq!(inside_number.unwrap(), 5);
            } else {
                assert_eq!(inside_number, None);
            }
        }
        assert!(1 < 2);
    }

    #[test]
    fn test3() {
        let result = give_result(5);
        if result.is_ok() {
            assert_eq!(result, Ok(()));
        } else {
            assert_eq!(result, Err(()));
        }
    }

    #[test]
    fn test_check_if_five() {
        let result1 = check_if_five(3);
        let result2 = check_if_five(5);
        assert_eq!(result1.err().unwrap(), "The number is not five");
        println!("{:?}", result2);
    }

    #[test]
    fn test_vec_get_option() {
        let my_vec = vec![2, 3, 4];
        for i in 0..10 {
            match my_vec.get(i) {
                None => {}
                Some(number) => {
                    println!("The number is {number}")
                }
            }
        }

        // use if let
        for index in 0..10 {
            if let Some(number) = my_vec.get(index) {
                println!("The number is {number}");
            }
        }
    }

    #[test]
    fn test_weather_parse() {
        let weather_vec = vec![
            vec!["Berlin", "cloudy", "5", "-7", "78"],
            vec!["Athens", "sunny", "not humid", "20", "10", "50"],
        ];
        for mut city in weather_vec {
            println!("For the city of {}:", city[0]);
            while let Some(information) = city.pop() {
                if let Ok(value) = information.parse::<i32>() {
                    println!("The number is {value}.")
                }
            }
        }
    }

    #[test]
    fn test_task_priority() {
        let mut jobs = BinaryHeap::with_capacity(10);
        jobs.push((100, "Write back to email from the ceo"));
        jobs.push((80, "Finish the report"));
        jobs.push((5, "Watch some youtube"));
        jobs.push((70, "Tell the team thanks for working hard"));
        jobs.push((30, "Plan who to hire next for the team"));

        while let Some(job) = jobs.pop() {
            println!("You need to: {}", job.1);
        }
    }

    #[test]
    fn test_bytes_to_chars() {
        let bytes = vec![240, 159, 146, 149];
        let s = String::from_utf8(bytes).expect("Found invalid utf-8");
        assert_eq!(s, "💕".to_string()); //emoji take 4 bytes
    }

    #[test]
    fn test_even_odd_vec_from() {
        let numbers = [1, 2, 8, 7, -1, -3, 9, 11].to_vec();
        let new_vec = EvenOddVec::from(numbers);
        assert_eq!(new_vec.0[0][0], 2);
        assert_eq!(new_vec.0[1][1], 7);
    }
}
