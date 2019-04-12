use super::*;

#[cfg(test)]
mod test {

    use super::GameField;

    #[test]
    fn create_field() {
        let GameField { field, .. } = super::GameField::new();
        assert_eq!(field, [[0; 10]; 10]);
    }
    #[test]
    fn get_ships() {
        let GameField { ships, .. } = super::GameField::new();
        let arr = [1, 2, 3, 4];
        let length = arr.len();
        println!("Ships {:?}", ships);
        for (i, &elem) in arr.iter().enumerate() {
            let index = length - (1 + i);
            let to_be = &arr[index];
            assert_eq!(ships.get(&elem).unwrap(), to_be);
        }
    }
}