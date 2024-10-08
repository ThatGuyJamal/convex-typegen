// This schema.rs was auto generated by the convex-typegen crate. Modify at your own risk.
// For more information, visit https://github.com/ThatGuyJamal/convex-typegen

pub enum Users {
    Create,
    Find,
}

impl Users {
    pub fn to_string(&self) -> &'static str {
        match self {
            Users::Create => "users:create",
            Users::Find => "users:find",
        }
    }
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "users:create" => Some(Users::Create),
            "users:find" => Some(Users::Find),
            _ => None,
        }
    }
}
pub enum Posts {
    Create,
    Find,
}

impl Posts {
    pub fn to_string(&self) -> &'static str {
        match self {
            Posts::Create => "posts:create",
            Posts::Find => "posts:find",
        }
    }
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "posts:create" => Some(Posts::Create),
            "posts:find" => Some(Posts::Find),
            _ => None,
        }
    }
}
