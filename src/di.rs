use shaku::module;

use crate::{application::sample_service::SampleService, infrastructure::{sample_repo::SampleRepo, DbProvider}};

module!{
    pub Deps{
        components = [DbProvider,SampleService,SampleRepo],
        providers = []
    }
}