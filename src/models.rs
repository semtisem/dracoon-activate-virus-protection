use dco3::{nodes::NodePermissions, FilterOperator, FilterQuery};

#[derive(Debug, Clone)]
pub enum PermissionFilter {
    Permission(FilterOperator, NodePermission),
}

impl FilterQuery for PermissionFilter {
    fn to_filter_string(&self) -> String {
        match self {
            PermissionFilter::Permission(op, perm) => {
                let op = String::from(op);
                format!("perm:{}:{}", op, perm.to_string())
            }
        }
    }
}

impl PermissionFilter {
    pub fn has_permission(perm: NodePermission) -> PermissionFilter {
        PermissionFilter::Permission(FilterOperator::Eq, perm)
    }
}

impl From<PermissionFilter> for Box<dyn FilterQuery> {
    fn from(filter: PermissionFilter) -> Self {
        Box::new(filter)
    }
}

#[derive(Debug, Clone)]
pub enum NodePermission {
    Manage,
    Read,
    Create,
    Change,
    Delete,
    ManageDownloadShare,
    ManageUploadShare,
    ReadRecycleBin,
    RestoreRecycleBin,
    DeleteRecycleBin,
}

impl NodePermission {
    pub fn to_string(&self) -> String {
        match self {
            NodePermission::Manage => "manage".to_owned(),
            NodePermission::Read => "read".to_owned(),
            NodePermission::Create => "create".to_owned(),
            NodePermission::Change => "change".to_owned(),
            NodePermission::Delete => "delete".to_owned(),
            NodePermission::ManageDownloadShare => "manageDownloadShare".to_owned(),
            NodePermission::ManageUploadShare => "manageUploadShare".to_owned(),
            NodePermission::ReadRecycleBin => "readRecycleBin".to_owned(),
            NodePermission::RestoreRecycleBin => "restoreRecycleBin".to_owned(),
            NodePermission::DeleteRecycleBin => "deleteRecycleBin".to_owned(),
        }
    }
}
