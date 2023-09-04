use std::collections::HashMap;

use gitql_ast::{object::GQLObject, value::Value};

pub fn select_gql_objects(
    repo: &git2::Repository,
    table: String,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    return match table.as_str() {
        "refs" => select_references(repo, fields, alias_table),
        "commits" => select_commits(repo, fields, alias_table),
        "branches" => select_branches(repo, fields, alias_table),
        "diffs" => select_diffs(repo, fields, alias_table),
        "tags" => select_tags(repo, fields, alias_table),
        _ => vec![],
    };
}

fn select_references(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    let repo_path = repo.path().to_str().unwrap().to_string();
    let mut gql_references: Vec<GQLObject> = Vec::new();
    let git_references = repo.references();
    if git_references.is_err() {
        return gql_references;
    }

    let is_limit_fields_empty = fields.is_empty();
    let references = git_references.ok().unwrap();

    for reference_result in references {
        if reference_result.is_err() {
            break;
        }

        let reference = reference_result.ok().unwrap();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = alias_table
                .get("name")
                .unwrap_or(&"name".to_string())
                .to_string();
            let name = reference.shorthand().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(name));
        }

        if is_limit_fields_empty || fields.contains(&String::from("full_name")) {
            let key = alias_table
                .get("full_name")
                .unwrap_or(&"full_name".to_string())
                .to_string();
            let full_name = reference.name().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(full_name));
        }

        if is_limit_fields_empty || fields.contains(&String::from("type")) {
            let key = alias_table
                .get("type")
                .unwrap_or(&"type".to_string())
                .to_string();

            if reference.is_branch() {
                attributes.insert(key, Value::Text("branch".to_owned()));
            } else if reference.is_remote() {
                attributes.insert(key, Value::Text("remote".to_owned()));
            } else if reference.is_tag() {
                attributes.insert(key, Value::Text("tag".to_owned()));
            } else if reference.is_note() {
                attributes.insert(key, Value::Text("note".to_owned()));
            } else {
                attributes.insert(key, Value::Text("other".to_owned()));
            }
        }

        if is_limit_fields_empty || fields.contains(&String::from("repo")) {
            let key = alias_table
                .get("repo")
                .unwrap_or(&"repo".to_string())
                .to_string();

            attributes.insert(key, Value::Text(repo_path.to_string()));
        }

        let gql_reference = GQLObject { attributes };
        gql_references.push(gql_reference);
    }

    return gql_references;
}

fn select_commits(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut commits: Vec<GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let is_limit_fields_empty = fields.is_empty();
    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("commit_id")) {
            let key = alias_table
                .get("commit_id")
                .unwrap_or(&"commit_id".to_string())
                .to_string();
            attributes.insert(key, Value::Text(commit.id().to_string()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = alias_table
                .get("name")
                .unwrap_or(&"name".to_string())
                .to_string();
            let name = commit.author().name().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(name));
        }

        if is_limit_fields_empty || fields.contains(&String::from("email")) {
            let key = alias_table
                .get("email")
                .unwrap_or(&"email".to_string())
                .to_string();
            let email = commit.author().email().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(email));
        }

        if is_limit_fields_empty || fields.contains(&String::from("title")) {
            let key = alias_table
                .get("title")
                .unwrap_or(&"title".to_string())
                .to_string();
            attributes.insert(key, Value::Text(commit.summary().unwrap().to_string()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("message")) {
            let key = alias_table
                .get("message")
                .unwrap_or(&"message".to_string())
                .to_string();
            attributes.insert(key, Value::Text(commit.message().unwrap_or("").to_string()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("time")) {
            let key = alias_table
                .get("time")
                .unwrap_or(&"time".to_string())
                .to_string();
            attributes.insert(key, Value::Date(commit.time().seconds()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("repo")) {
            let key = alias_table
                .get("repo")
                .unwrap_or(&"repo".to_string())
                .to_string();

            attributes.insert(key, Value::Text(repo_path.to_string()));
        }

        let gql_commit = GQLObject { attributes };
        commits.push(gql_commit);
    }

    return commits;
}

fn select_diffs(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut diffs: Vec<GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let is_limit_fields_empty = fields.is_empty();
    let select_insertions = fields.contains(&String::from("insertions"));
    let select_deletions = fields.contains(&String::from("deletions"));
    let select_file_changed = fields.contains(&String::from("files_changed"));
    let repo_path = repo.path().to_str().unwrap().to_string();

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("commit_id")) {
            let key = alias_table
                .get("commit_id")
                .unwrap_or(&"commit_id".to_string())
                .to_string();
            attributes.insert(key, Value::Text(commit.id().to_string()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = alias_table
                .get("name")
                .unwrap_or(&"name".to_string())
                .to_string();
            let name = commit.author().name().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(name));
        }

        if is_limit_fields_empty || fields.contains(&String::from("email")) {
            let key = alias_table
                .get("email")
                .unwrap_or(&"email".to_string())
                .to_string();
            let email = commit.author().email().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(email));
        }

        if is_limit_fields_empty || fields.contains(&String::from("repo")) {
            let key = alias_table
                .get("repo")
                .unwrap_or(&"repo".to_string())
                .to_string();

            attributes.insert(key, Value::Text(repo_path.to_string()));
        }

        if is_limit_fields_empty || select_insertions || select_deletions || select_file_changed {
            let diff = if commit.parents().len() > 0 {
                repo.diff_tree_to_tree(
                    Some(&commit.parent(0).unwrap().tree().unwrap()),
                    Some(&commit.tree().unwrap()),
                    None,
                )
            } else {
                repo.diff_tree_to_tree(None, Some(&commit.tree().unwrap()), None)
            };

            let diff_status = diff.unwrap().stats().unwrap();

            if is_limit_fields_empty || select_insertions {
                let key = alias_table
                    .get("insertions")
                    .unwrap_or(&"insertions".to_string())
                    .to_string();
                attributes.insert(key, Value::Number(diff_status.insertions() as i64));
            }

            if is_limit_fields_empty || select_deletions {
                let key = alias_table
                    .get("deletions")
                    .unwrap_or(&"deletions".to_string())
                    .to_string();
                attributes.insert(key, Value::Number(diff_status.deletions() as i64));
            }

            if is_limit_fields_empty || select_file_changed {
                let key = alias_table
                    .get("files_changed")
                    .unwrap_or(&"files_changed".to_string())
                    .to_string();
                attributes.insert(key, Value::Number(diff_status.files_changed() as i64));
            }
        }

        let gql_diff = GQLObject { attributes };
        diffs.push(gql_diff);
    }

    return diffs;
}

fn select_branches(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut branches: Vec<GQLObject> = Vec::new();
    let local_branches = repo.branches(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();
    let repo_path = repo.path().to_str().unwrap().to_string();

    for branch in local_branches {
        let (branch, _) = branch.unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = alias_table
                .get("name")
                .unwrap_or(&"name".to_string())
                .to_string();
            let branch_name = branch.name().unwrap().unwrap_or("").to_string();
            attributes.insert(key, Value::Text(branch_name));
        }

        if is_limit_fields_empty || fields.contains(&String::from("commit_count")) {
            let key = alias_table
                .get("commit_count")
                .unwrap_or(&"commit_count".to_string())
                .to_string();
            let branch_ref = branch.get().peel_to_commit().unwrap();
            let mut revwalk = repo.revwalk().unwrap();
            let _ = revwalk.push(branch_ref.id());
            attributes.insert(key, Value::Number(revwalk.count() as i64));
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_head")) {
            let key = alias_table
                .get("is_head")
                .unwrap_or(&"is_head".to_string())
                .to_string();
            attributes.insert(key, Value::Boolean(branch.is_head()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_remote")) {
            let key = alias_table
                .get("is_remote")
                .unwrap_or(&"is_remote".to_string())
                .to_string();
            attributes.insert(key, Value::Boolean(branch.get().is_remote()));
        }

        if is_limit_fields_empty || fields.contains(&String::from("repo")) {
            let key = alias_table
                .get("repo")
                .unwrap_or(&"repo".to_string())
                .to_string();

            attributes.insert(key, Value::Text(repo_path.to_string()));
        }

        let gql_branch = GQLObject { attributes };
        branches.push(gql_branch);
    }

    return branches;
}

fn select_tags(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut tags: Vec<GQLObject> = Vec::new();
    let tag_names = repo.tag_names(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();
    let repo_path = repo.path().to_str().unwrap().to_string();

    for tag_name in tag_names.iter() {
        match tag_name {
            Some(name) => {
                let mut attributes: HashMap<String, Value> = HashMap::new();

                if is_limit_fields_empty || fields.contains(&String::from("name")) {
                    let key = alias_table
                        .get("name")
                        .unwrap_or(&"name".to_string())
                        .to_string();
                    attributes.insert(key, Value::Text(name.to_string()));
                }

                if is_limit_fields_empty || fields.contains(&String::from("repo")) {
                    let key = alias_table
                        .get("repo")
                        .unwrap_or(&"repo".to_string())
                        .to_string();

                    attributes.insert(key, Value::Text(repo_path.to_string()));
                }

                let gql_tag = GQLObject { attributes };
                tags.push(gql_tag);
            }
            None => {}
        }
    }
    return tags;
}