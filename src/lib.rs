pub trait SafeBuilder
{
    fn build() -> PartialBuilder;
}

pub trait PartialBuilder { }