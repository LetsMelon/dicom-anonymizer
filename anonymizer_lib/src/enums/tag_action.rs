use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TagAction<T> {
    /// Change the tag with the given value, similar to `Option::Some(T)`
    Change(T),
    /// Changes nothing, similar to `Option::None`
    Keep,
    /// Remove the tag
    Remove,
}

impl<T> TagAction<T> {
    /// Maps an `TagAction::Change<T>` to `Option<U>` by applying a function to a contained value.
    /// Copied from https://doc.rust-lang.org/std/option/enum.Option.html#method.map
    #[inline]
    pub fn map<U, F>(self, f: F) -> TagAction<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TagAction::Change(v) => TagAction::Change(f(v)),
            TagAction::Keep => TagAction::Keep,
            TagAction::Remove => TagAction::Remove,
        }
    }
}

impl<T> Default for TagAction<T> {
    fn default() -> Self {
        TagAction::Keep
    }
}

impl<T> From<Option<T>> for TagAction<T> {
    fn from(opt: Option<T>) -> Self {
        match opt {
            None => TagAction::default(),
            Some(value) => TagAction::Change(value),
        }
    }
}

impl<T> From<TagAction<T>> for Option<T> {
    fn from(tag_action: TagAction<T>) -> Self {
        match tag_action {
            TagAction::Change(value) => Option::Some(value),
            TagAction::Keep => Option::None,
            TagAction::Remove => Option::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TagAction;

    #[test]
    fn has_option_map_like_function() {
        assert_eq!(
            TagAction::Change("MyString"),
            TagAction::Change(1).map(|_| "MyString")
        );
        assert_eq!(
            TagAction::Change("MyString"),
            TagAction::Change(Option::Some("MyString")).map(|value| value.unwrap())
        );
        assert_ne!(
            TagAction::Change("MyString"),
            TagAction::Keep.map(|_: String| "MyString")
        );
    }

    #[test]
    fn can_be_transformed_into_an_option_and_back() {
        type T = usize;

        assert_eq!(
            Option::Some("MyString"),
            Option::from(TagAction::Change("MyString")),
        );
        assert_eq!(Option::<T>::None, Option::from(TagAction::<T>::Keep),);
        assert_eq!(Option::<T>::None, Option::from(TagAction::<T>::Remove),);

        assert_eq!(TagAction::Change(1), TagAction::from(Option::Some(1)),);
        assert_eq!(TagAction::<T>::Keep, TagAction::from(Option::<T>::None),);
    }
}
