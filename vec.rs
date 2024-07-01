pub fn nest_vector<T>(vec: Vec<T>, every_n: usize) -> Vec<Vec<T>> {
    let mut nested_vec = Vec::new();
    let mut counter = 1;

    for item in vec {
        if nested_vec.is_empty() {
            nested_vec.push(Vec::new());
        }

        if counter > every_n {
            counter = 1;
            nested_vec.push(Vec::new());
        }

        nested_vec.last_mut().unwrap().push(item);

        counter += 1;
    }

    nested_vec
}
