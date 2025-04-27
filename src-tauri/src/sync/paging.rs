use serde::{Deserialize, Serialize};
use jira_openapi_client::models::{IssueBean, SearchResultsBean};
use crate::sync::Error;

pub trait PagedBean<'de> : Clone + Serialize + Deserialize<'de> + Default {
    type Value: Clone + Serialize + Deserialize<'de> + Default;

    fn total(&self) -> usize;
    fn add_to(&mut self, v: &mut Vec<Self::Value>);
}

impl<'de> PagedBean<'de> for SearchResultsBean {
    type Value = IssueBean;

    fn total(&self) -> usize {
        self.total.unwrap_or_default() as usize
    }

    fn add_to(&mut self, v: &mut Vec<Self::Value>) {
        if let None = self.issues {
            return
        }

        v.append(self.issues.as_mut().unwrap())    }
}

// TODO: this should probably be in src/jira/*
pub async fn paged_for_all<'de, T: PagedBean<'de>>(
    page_size: usize,
    call: impl AsyncFn(usize, usize) -> Result<T, Error>,
) -> Result<Vec<T::Value>, Error> {
    let mut result = vec![];
    let mut start_at: usize = 0;

    loop {
        let mut page = call(start_at, page_size).await?;
        let total = page.total();

        // TODO: debug assert that results is same as page size

        page.add_to(&mut result);
        start_at += page_size;

        if start_at >= total {
            break;
        }
    }

    Ok(result)
}
