use uuid::Uuid;

pub enum GuidVersion {
    V4,
    V7,
}

pub fn generate_guid(version: GuidVersion) -> String {
    match version {
        GuidVersion::V4 => Uuid::new_v4().to_string(),
        GuidVersion::V7 => Uuid::now_v7().to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guid_v4() {
        let guid = generate_guid(GuidVersion::V4);
        assert!(Uuid::parse_str(&guid).is_ok());
        let uuid = Uuid::parse_str(&guid).unwrap();
        assert_eq!(uuid.get_version(), Some(uuid::Version::Random));
    }

    #[test]
    fn test_guid_v7() {
        let guid = generate_guid(GuidVersion::V7);
        assert!(Uuid::parse_str(&guid).is_ok());
    }
}
