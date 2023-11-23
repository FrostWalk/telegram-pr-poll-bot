use octocrab::{Error, Octocrab, params};
use octocrab::models::issues::Issue;
use octocrab::models::Label;
use octocrab::models::pulls::PullRequest;

use crate::config;

fn connect() -> Octocrab {
    match Octocrab::builder().personal_token(config::get_github_token()).build() {
        Ok(r) => { r }
        Err(err) => { panic!("unable to instantiate github api client {}", err) }
    }
}

pub async fn get_issues() -> Result<Vec<Issue>, Error> {
    let github = connect();

    let mut page = match github
        .issues(config::get_github_organization(), config::get_github_repository())
        .list().labels(&[config::get_github_filter_label()]).state(params::State::Open)
        .per_page(config::get_github_elements_per_page()).send()
        .await {
        Ok(r) => { r }
        Err(e) => { return Err(e); }
    };

    let p_num: usize = match page.number_of_pages() {
        None => { 1 }
        Some(n) => { n as usize }
    };

    let mut issues: Vec<Issue> = Vec::with_capacity(config::get_github_elements_per_page() as usize * p_num);
    loop {
        for issue in page.items {
            if issue.labels.iter().any(|x| x.name == config::get_github_reject_label() || x.name == config::get_github_approve_label()) {
                continue;
            } else {
                issues.push(issue);
            }
        }

        match github.get_page::<Issue>(&page.next).await {
            Ok(o) => {
                match o {
                    None => { break; }
                    Some(p) => { page = p }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(issues)
}

pub async fn get_pull_requests() -> Result<Vec<PullRequest>, Error> {
    let github = connect();

    let mut page = match github
        .pulls(config::get_github_organization(), config::get_github_repository())
        .list().state(params::State::Open).per_page(config::get_github_elements_per_page())
        .send().await
    {
        Ok(r) => { r }
        Err(e) => { return Err(e); }
    };

    let p_num: usize = match page.number_of_pages() {
        None => { 1 }
        Some(n) => { n as usize }
    };

    let mut pull_requests: Vec<PullRequest> = Vec::with_capacity(config::get_github_elements_per_page() as usize * p_num);
    loop {
        for pr in page.items {
            let labels = match &pr.labels {
                None => { continue; }
                Some(l) => { l }
            };

            if labels.iter().any(|x| x.name == config::get_github_reject_label() || x.name == config::get_github_approve_label()) {
                continue;
            } else {
                pull_requests.push(pr);
            }
        }

        match github.get_page::<PullRequest>(&page.next).await {
            Ok(o) => {
                match o {
                    None => { break; }
                    Some(p) => { page = p }
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(pull_requests)
}