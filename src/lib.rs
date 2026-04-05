pub mod dublevec;
pub mod rectvec;
pub mod vec2;

#[cfg(test)]
mod tests {
    use super::*;
    use dublevec::DubleVec;
    use rectvec::RectVec;
    use vec2::Vec2;

    #[test]
    fn assign_works() {
        let mut vec: RectVec<i32> = RectVec::new(Vec2 { x: 5, y: 5 }, 0);
        vec.assign(5, Vec2 { x: 1, y: 1 }); // 5
    }
    #[test]
    fn access_works() -> Result<(), ()> {
        let vec: RectVec<i32> = RectVec::new(Vec2 { x: 2, y: 2 }, 0);
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
        let _rectvect_i32: RectVec<i32> = RectVec::from(vecvec);
    }
    #[test]
    fn fmt_works() {
        let mut dublevec: RectVec<char> = RectVec::new(Vec2 { x: 5, y: 5 }, '#');
        dublevec.assign('*', Vec2 { x: 2, y: 2 });
        //println!("{}", dublevec);
    }
    #[test]
    fn dublevec_works() {
        let mut dublevec: DubleVec<i32> = DubleVec::new();
        dublevec.push(vec![5, 2, 5]);
        dublevec.push(vec![1, 5]);

        if let Some(val) = dublevec.access(Vec2 { x: 0, y: 1 }) {
            println!("{}", val);
        } else {
            println!("out of range");
        }
        println!();
        dublevec.dbg();
    }
}
