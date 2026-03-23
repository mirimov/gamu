#[cfg(target_pointer_width = "16")]
pub type Int = u8;

#[cfg(target_pointer_width = "64")]
pub type Int = i32;

#[macro_export]
macro_rules! numeric_struct {
  ($name:ident, $v:ident => $body:block) => {
    #[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
    pub struct $name(Int);

    impl $name {
      fn validate($v: Int) -> Self {
        $body
        $name($v)
      }
    }

    impl std::ops::Deref for $name {
      type Target = Int;
      fn deref(&self) -> &Self::Target {
        &self.0
      }
    }

    impl From<Int> for $name {
      fn from($v: Int) -> Self {
        Self::validate($v)
      }
    }

    impl From<$name> for Int {
      fn from($v: $name) -> Self {
        $v.0
      }
    }

    impl PartialEq<Int> for $name {
      fn eq(&self, other: &Int) -> bool {
        self.0 == *other
      }
    }

    impl std::ops::Add<Int> for $name {
      type Output = Self;
      fn add(self, rhs: Int) -> Self {
        Self::validate(self.0 + rhs)
      }
    }

    impl std::ops::Sub<Int> for $name {
      type Output = Self;
      fn sub(self, rhs: Int) -> Self {
        Self::validate(self.0 - rhs)
      }
    }
  };
}
