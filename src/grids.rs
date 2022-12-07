use std::ops::Index;

/// a fixed size 2D array (WxH) that can be indexed by [usize;2]
pub trait Grid<C: Copy + PartialEq, const W: usize, const H: usize>:
    Index<[usize; 2], Output = Option<C>>
{
}

// // allow to transform one grid implementation into another as
// // long as they contain the same elements and have the same size
// impl<C, O, I, const W: usize, const H: usize> From<I> for O
// where
//     O: Default + Grid<C, W, H>,
//     I: Grid<C, W, H>,
//     C: Copy,
// {
//     fn from(input: O) -> O {
//         let mut output = O::default();
//         for i in 0..O::W {
//             for j in 0..O::H {
//                 output[[i, j]] = input[[i, j]];
//             }
//         }
//         output
//     }
// }
