use super::*;
use helpers::{Coordinates, ShipDirection, LEN};

#[cfg(test)]
mod test {

    use super::{Coordinates, GameField, ShipDirection, LEN};

    #[test]
    fn create_field() {
        let GameField { field, .. } = super::GameField::new();
        assert_eq!(field, [[0; 12]; 12]);
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
    #[test]
    fn reduce_ships() {
        let mut field = super::GameField::new();
        for index in 1..5 {
            field.reduce_ships(&index);
        }
        assert_eq!(*field.ships.get(&1).unwrap(), 3);
        assert_eq!(*field.ships.get(&2).unwrap(), 2);
        assert_eq!(*field.ships.get(&3).unwrap(), 1);
        assert_eq!(*field.ships.get(&4).unwrap(), 0);
    }
    #[test]
    fn create_permission1() {
        let mut field = super::GameField::new();
        let permission = field.create_permission(&4);
        assert_eq!(permission, true);

    }
    #[test]
    fn create_permission2() {
        let mut field = super::GameField::new();
        field.reduce_ships(&4);
        let permission = field.create_permission(&4);
        assert_eq!(permission, false);
    }
    #[test]
    fn random_coordinates() {
        let field = super::GameField::new();
        let size = 4;
        let Coordinates {
            dynamic,
            stat,
            range,
        } = field.random_coordinates(&size);
        let expect_dynamic = dynamic >= 1 && dynamic <= 7;
        let expect_stat = stat >= 1 && stat <= 10;

        assert_eq!(expect_dynamic, true);
        assert_eq!(expect_stat, true);
        assert_eq!(range, dynamic..dynamic + size);
    }

    #[test]
    fn create_ship_1() {
        let mut field = super::GameField::new();
        let size = 2;
        field.create_ship(size, ShipDirection::Vertical);
   
        let expect: Vec<&[u8; LEN as usize]> = field
            .field
            .iter()
            .filter(|el| el.into_iter().any(|v| *v == 1))
            .collect();

        assert_eq!(expect.len(), size as usize);
    }
    #[test]
       fn create_ship_2() {
        let mut field = super::GameField::new();
        let size = 2;
        field.create_ship(size, ShipDirection::Vertical);
   
        let expect: Vec<&[u8; LEN as usize]> = field
            .field
            .iter()
            .filter(|el| el.into_iter().any(|v| *v == 2))
            .collect();
            println!("Expect {:?}",expect);
        let rows_contain_bounds = 4;

        assert_eq!(expect.len(), rows_contain_bounds);
    }
}