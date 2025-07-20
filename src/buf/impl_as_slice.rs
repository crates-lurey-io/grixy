use crate::buf::GridBuf;

impl<T, B> AsRef<[T]> for GridBuf<T, B>
where
    B: AsRef<[T]>,
{
    fn as_ref(&self) -> &[T] {
        self.buffer.as_ref()
    }
}

impl<T, B> AsMut<[T]> for GridBuf<T, B>
where
    B: AsMut<[T]>,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{buf::VecGrid, core::Pos};

    #[test]
    fn vec_u8_as_ref() {
        let grid = VecGrid::new(10, 10);
        let slice: &[u8] = grid.as_ref();
        assert_eq!(slice.len(), 100);
    }

    #[test]
    fn vec_u8_as_mut() {
        let mut grid = VecGrid::new(10, 10);
        let slice: &mut [u8] = grid.as_mut();
        assert_eq!(slice.len(), 100);
        slice[0] = 42;
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
    }
}
