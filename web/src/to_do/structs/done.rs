use super::base::Base;

pub struct Done{
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Self{
        let done =Base{
            title: title.to_string(),
            status: crate::to_do::enums::TaskStatus::DONE,
        };
        return Done{super_struct: done }
    }
}
