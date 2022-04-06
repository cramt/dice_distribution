pub mod dice;
pub mod distribution;
pub mod posibility_space;

#[cfg(test)]
mod tests {
    use shoulda::Shoulda;

    use crate::dice::Dice;
    use crate::distribution::Distribution;
    use crate::posibility_space::PosibilitySpace;

    #[test]
    fn destribution_of_5d20kh3() {
        let d20: PosibilitySpace = Dice(20).into();
        let fived20kh3 = d20.multiply(5).keep_highest(3);
        let dist: Distribution = fived20kh3.into();
        let expected = Distribution(
            [
                (3, 1),
                (4, 5),
                (5, 15),
                (6, 41),
                (7, 90),
                (8, 170),
                (9, 301),
                (10, 495),
                (11, 765),
                (12, 1141),
                (13, 1640),
                (14, 2280),
                (15, 3101),
                (16, 4125),
                (17, 5375),
                (18, 6901),
                (19, 8730),
                (20, 10890),
                (21, 13441),
                (22, 16415),
                (23, 19840),
                (24, 23776),
                (25, 28220),
                (26, 33180),
                (27, 38656),
                (28, 44640),
                (29, 51055),
                (30, 57921),
                (31, 65125),
                (32, 72625),
                (33, 80321),
                (34, 88155),
                (35, 95940),
                (36, 103656),
                (37, 111080),
                (38, 118120),
                (39, 124576),
                (40, 130340),
                (41, 135115),
                (42, 138841),
                (43, 141195),
                (44, 142095),
                (45, 141361),
                (46, 139015),
                (47, 134890),
                (48, 129186),
                (49, 121820),
                (50, 113020),
                (51, 102866),
                (52, 91690),
                (53, 79575),
                (54, 67041),
                (55, 54255),
                (56, 41755),
                (57, 29881),
                (58, 19275),
                (59, 10270),
                (60, 3706),
            ]
            .into(),
        );
        dist.should().eq(expected);
    }

    #[test]
    fn destribution_of_2d6kh() {
        let d6: PosibilitySpace = Dice(6).into();
        let two_d6kh = d6.multiply(2).keep_highest(1);
        let dist: Distribution = two_d6kh.into();
        dist.should().eq(Distribution(
            [(1, 1), (2, 3), (3, 5), (4, 7), (5, 9), (6, 11)].into(),
        ));
    }

    #[test]
    fn destribution_of_2d6() {
        let d6: PosibilitySpace = Dice(6).into();
        let two_d6 = d6.multiply(2);
        let _dist: Distribution = two_d6.into();
        // as long as it doesnt overflow we good
    }

    #[test]
    fn destribution_of_3d6() {
        let d6: PosibilitySpace = Dice(6).into();
        let two_d6 = d6.multiply(3);
        let dist: Distribution = two_d6.into();
        dist.should().eq(Distribution(
            [
                (3, 1),
                (4, 3),
                (5, 6),
                (6, 10),
                (7, 15),
                (8, 21),
                (9, 25),
                (10, 27),
                (11, 27),
                (12, 25),
                (13, 21),
                (14, 15),
                (15, 10),
                (16, 6),
                (17, 3),
                (18, 1),
            ]
            .into(),
        ));
    }

    #[test]
    fn destribution_of_24d6() {
        let d6: PosibilitySpace = Dice(6).into();
        let manyd6 = d6.multiply(24);
        let dist: Distribution = manyd6.into();
        dist.should().eq(Distribution(
            [
                (3, 1),
                (4, 3),
                (5, 6),
                (6, 10),
                (7, 15),
                (8, 21),
                (9, 25),
                (10, 27),
                (11, 27),
                (12, 25),
                (13, 21),
                (14, 15),
                (15, 10),
                (16, 6),
                (17, 3),
                (18, 1),
            ]
            .into(),
        ));
    }

    #[test]
    fn destribution_of_3d6_plus_2d8() {
        let d6: PosibilitySpace = Dice(6).into();
        let three_d6 = d6.multiply(3);
        let d8: PosibilitySpace = Dice(8).into();
        let two_d8 = d8.multiply(2);
        let dist: Distribution = (three_d6 + two_d8).into();
        dist.should().eq(Distribution(
            [
                (5, 1),
                (6, 5),
                (7, 15),
                (8, 35),
                (9, 70),
                (10, 126),
                (11, 207),
                (12, 315),
                (13, 448),
                (14, 600),
                (15, 761),
                (16, 917),
                (17, 1053),
                (18, 1153),
                (19, 1206),
                (20, 1206),
                (21, 1153),
                (22, 1053),
                (23, 917),
                (24, 761),
                (25, 600),
                (26, 448),
                (27, 315),
                (28, 207),
                (29, 126),
                (30, 70),
                (31, 35),
                (32, 15),
                (33, 5),
                (34, 1),
            ]
            .into(),
        ));
    }
}

pub type Int = i32;
pub type BigInt = u64;
