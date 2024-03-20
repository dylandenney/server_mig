use users::{Groups, UsersCache};


use std::collections::HashSet;

pub struct Group {
    pub name: String,
    pub gid: u32,
}

pub fn gather_relevant_groups(user_gids: HashSet<u32>) -> Vec<Group> {
    let mut relevant_groups = Vec::new();
    let cache = UsersCache::new();

    for gid in user_gids {
        if let Some(group) = cache.get_group_by_gid(gid) {
            // Exclude system-level groups, focusing on those with GIDs >= 1000
            if gid >= 1000 {
                relevant_groups.push(Group {
                    name: group.name().to_string_lossy().into_owned(),
                    gid,
                });
            }
        }
    }
    relevant_groups
}

pub fn generate_group_create_script(groups_to_create: &[Group]) -> String {
    let mut script = String::new();
    script.push_str("#!/bin/bash\n");

    for group in groups_to_create {
        script.push_str(&format!("groupadd {} -g {}\n", group.name, group.gid));
    }

    script
}
