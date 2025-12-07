use std::iter;

use rusqlite::Connection;

use crate::db::{self, Project};

// A struct for caching project data to minimize DB access
pub struct Projects {
    projects: Vec<Project>,
    active: Option<usize>,
}

impl Projects {
    pub fn new(conn: &Connection) -> Self {
        let mut p = Projects {
            projects: vec![],
            active: None,
        };
        p.fetch(conn);
        p
    }
    fn fetch(&mut self, conn: &Connection) {
        self.projects.truncate(0);
        self.projects
            .append(&mut db::get_projects(conn).expect("Failed to fetch projects"));
    }
    pub fn get_all(&self) -> &[Project] {
        &self.projects
    }
    pub fn get_all_tree_style(&self) -> Vec<(usize, &Project)> {
        fn recurse<'a>(
            project: &'a Project,
            all_projects: &'a [Project],
            depth: usize,
        ) -> Vec<(usize, &'a Project)> {
            iter::once((depth, project))
                .chain(
                    project
                        .children
                        .iter()
                        .map(|id| all_projects.iter().find(|p| p.id == *id).unwrap())
                        .flat_map(|p| recurse(p, all_projects, depth + 1)),
                )
                .collect()
        }
        let all_projects = self.get_all();
        all_projects
            .iter()
            .filter(|p| p.parent.is_none())
            .flat_map(|p| recurse(p, all_projects, 0))
            .collect()
    }
    pub fn get(&self, id: i32) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }
    pub fn add(&mut self, project: Project, conn: &Connection) {
        db::add_project(conn, &project).expect("Failed to add project");
        self.fetch(conn);
    }
    pub fn set_active(&mut self, id: Option<i32>) {
        self.active = id.and_then(|id| self.projects.iter().position(|x| x.id == id));
    }
    pub fn get_active(&self) -> Option<&Project> {
        self.active.and_then(|x| self.projects.get(x))
    }
}
