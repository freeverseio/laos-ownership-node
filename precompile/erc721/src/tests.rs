use super::*;

#[test]
fn check_selectors() {
	assert_eq!(Action::OwnerOf as u32, 0x6352211E);
	assert_eq!(Action::TockenURI as u32, 0xC87B56DD);
}
