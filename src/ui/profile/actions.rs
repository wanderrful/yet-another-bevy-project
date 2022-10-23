/// Request IP address from HttpBinService.
pub struct ProfileActionGetIp;

/// Request User Details from ReqRes.
pub struct ProfileActionGetUserById {
    pub id: u8
}

/// Set the visibility of the entire Menu widget itself.
pub struct ProfileActionMenuSetVisibility {
    // Whether the menu should now be hidden
    pub visible: bool
}
