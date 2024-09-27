use crate::to_do::enums::TaskStatus;

use super::base::Base;

pub struct Pending{
    pub super_struct: Base
}

impl Pending {
    pub fn new(title: &str) -> Self{
        let pending = Base{
            title: title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending{super_struct: pending};
    }
}
