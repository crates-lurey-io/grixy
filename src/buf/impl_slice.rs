use crate::{buf::GridBuf, ops::layout};

impl<T, B, L> AsRef<[T]> for GridBuf<T, B, L>
where
    B: AsRef<[T]>,
    L: layout::Linear,
{
    fn as_ref(&self) -> &[T] {
        self.buffer.as_ref()
    }
}

impl<T, B, L> AsMut<[T]> for GridBuf<T, B, L>
where
    B: AsMut<[T]>,
    L: layout::Linear,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.buffer.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::{buf::GridBuf, core::Pos, ops::GridRead as _};

    #[test]
    fn vec_u8_as_ref() {
        let grid = GridBuf::<u8, _, _>::new(10, 10);
        let slice: &[u8] = grid.as_ref();
        assert_eq!(slice.len(), 100);
    }

    #[test]
    fn vec_u8_as_mut() {
        let mut grid = GridBuf::<u8, _, _>::new(10, 10);
        let slice: &mut [u8] = grid.as_mut();
        assert_eq!(slice.len(), 100);
        slice[0] = 42;
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&42));
    }
}
