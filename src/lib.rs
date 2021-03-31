mod fundamental;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_vec() {
        let mut test_data = (0..100).collect::<Vec<usize>>();
        println!("{:?}", test_data);
        let pid = test_data[11];
        let qid = test_data[22];
        for val in test_data.iter_mut() {
            if *val == pid {
                *val = qid
            }
        }
        println!("{:?}", test_data)
    }
}
