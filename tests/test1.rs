#[cfg(test)]
mod test{
    use easy_rust::anti_null_exception::take_fifth;

    #[test]
    fn test1(){
        let number = take_fifth(vec![1,2,3,4]);
        let number2 = take_fifth(vec![1,2,3,4,5]).unwrap();
        let number3 = take_fifth(vec![1, 2, 3, 4, 5]).unwrap_or(0); // can use match
        assert_eq!(number,None);
        assert_eq!(number2,5);
    }

    #[test]
    fn test2(){
        let new_vec = vec![1,2];
        let bigger_vec = vec![1,2,3,4,5];
        let vec_of_vecs = vec![new_vec,bigger_vec];
        for vec in vec_of_vecs {
            let inside_number = take_fifth(vec);
            if inside_number.is_some() {
                assert_eq!(inside_number.unwrap(),5);
            }else {
                assert_eq!(inside_number,None);
            }
        }
    }
}