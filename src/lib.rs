use thiserror;

pub struct SvnCmd {}

impl SvnCmd {
    pub fn commit(&self) {}
    pub fn checkout(&self) {}
    pub fn update(&self) {}
    pub fn log(&self) {}
    // pub fn status(&self) -> SvnStatus {}
    // pub fn info(&self) -> SvnInfo {}
}

pub struct SvnInfo {}
pub struct SvnStatus {}
