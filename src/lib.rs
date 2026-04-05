pub mod dublevec;
pub mod vec2;

#[cfg(test)]
mod tests {
    use super::*;
    use dublevec::DubleVec;
    use vec2::Vec2;

    #[test]
    fn assign_works() {
        let mut vec: DubleVec<i32> = DubleVec::new(Vec2 { x: 5, y: 5 }, 0);
        vec.assign(5, Vec2 { x: 1, y: 1 }); // 5
    }
    #[test]
    fn access_works() -> Result<(), ()> {
        let vec: DubleVec<i32> = DubleVec::new(Vec2 { x: 2, y: 2 }, 0);
        if vec.access(Vec2 { x: 0, y: 1 }).is_some() {
            Ok(())
        } else {
            Err(())
        }
    }
    #[test]
    fn vec2_works() {
        assert_eq!(
            Vec2 { x: 5, y: 5 } * Vec2 { x: 2, y: 2 },
            Vec2 { x: 10, y: 10 }
        );
    }
    #[test]
    fn from_works() {
        let vecvec: Vec<Vec<i32>> = vec![
            vec![5, 0, 2, 0, 0],
            vec![1, 2, 0, 0, 0],
            vec![7, 8, 9, 1, 0],
            vec![3, 0, 0, 0, 0],
            vec![4, 4, 4, 0, 3],
        ];
        let _dublevec_i32: DubleVec<i32> = DubleVec::from(vecvec);
    }
}
