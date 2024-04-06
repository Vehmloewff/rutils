pub struct ConsumedVec<'a, T>(&'a mut Vec<T>);

impl<'a, T> ConsumedVec<'_, T> {
    pub fn new(vec: &mut Vec<T>) -> ConsumedVec<'_, T> {
        vec.reverse();
        ConsumedVec(vec)
    }
}

impl<T> Iterator for ConsumedVec<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub trait ConsumableVec<T> {
    fn consume(&mut self) -> ConsumedVec<'_, T>;
}

impl<T> ConsumableVec<T> for Vec<T> {
    fn consume(&mut self) -> ConsumedVec<'_, T> {
        ConsumedVec::new(self)
    }
}

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
