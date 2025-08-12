use flagset::FlagSet;
use wut::{
    gx2::buffer::{BufferError, Flags, RawBuffer},
    ops::Deref,
};

pub struct Vector<T: Copy> {
    capacity: usize,
    buf: RawBuffer<T>,
}

impl<T: Copy> Vector<T> {
    const GROWTH_FACTOR: f32 = 1.5;
    const GROWTH_MIN: usize = 32;

    pub fn new(flags: impl Into<FlagSet<Flags>>) -> Result<Self, BufferError> {
        let mut s = Self {
            capacity: Self::GROWTH_MIN,
            buf: unsafe { RawBuffer::uninit(Self::GROWTH_MIN, flags.into()) }?,
        };

        s.buf.as_raw_mut().elemCount = 0;

        Ok(s)
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn default(usage: Flags) -> Result<Self, BufferError> {
        Self::new(
            usage
                | Flags::GX2RAllocated
                | Flags::UsageCPURead
                | Flags::UsageCPUWrite
                | Flags::UsageGPURead,
        )
    }

    pub fn resize(&mut self) -> Result<(), BufferError> {
        let len = self.len();

        let new_capacity = ((self.capacity() as f32 * Self::GROWTH_FACTOR) as usize)
            .max(self.capacity() + Self::GROWTH_MIN);

        let mut new_buf = unsafe { RawBuffer::uninit(new_capacity, self.buf.flags()) }?;

        if len > 0 {
            let old = self.buf.read()?;
            let mut new = new_buf.write()?;

            new[..len].copy_from_slice(&old[..len]);
        }

        self.buf = new_buf;
        self.capacity = new_capacity;
        self.buf.as_raw_mut().elemCount = len as u32;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.buf.as_raw_mut().elemCount = 0;
    }

    pub fn push(&mut self, value: impl Into<T>) -> Result<(), BufferError> {
        let len = self.len();

        if self.capacity() <= len {
            self.resize()?;
        }

        self.buf.as_raw_mut().elemCount += 1;
        match self.buf.write() {
            Ok(mut v) => v[len] = value.into(),
            Err(e) => panic!("Error: {:?}", e),
        }

        Ok(())
    }

    pub fn get(&self) -> &RawBuffer<T> {
        &self.buf
    }

    pub fn get_mut(&mut self) -> &mut RawBuffer<T> {
        &mut self.buf
    }
}

impl<T: Copy> Deref for Vector<T> {
    type Target = RawBuffer<T>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}
