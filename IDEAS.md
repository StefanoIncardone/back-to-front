# Feature Ideas

>[!WARNING]
> no feature is final, modifications can happen at any moment

## 0.1.1 - Index iterator

```rust
trait IndexType {}
impl IndexType for u8 {}
impl IndexType for i8 {}
impl IndexType for ... {}

struct Indices<I: IndexType> {
    start: I,
    end: I,
}

pub struct Indices<I: IndexType> {
    pub start: I,
    pub end: I,
}

impl<I: IndexType> Iterator for Indices<I> {
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let index = self.start;
            self.start += 1;
            return Some(index);
        } else {
            return None;
        }
    }
}
```
