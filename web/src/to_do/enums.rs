pub enum TaskStatus {
    PENDING,
    DONE,
}

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self{
            &Self::DONE => "Done".to_string(),
            &Self::PENDING => "Pending".to_string(),
        }
    }
}
