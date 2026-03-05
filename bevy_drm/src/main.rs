use bevy_drm::card::Card;
use drm::control::Device as _;

fn main() {
    let card = Card::open_default().unwrap();
    let resources = card.resource_handles().unwrap();
    for &handle in resources.connectors() {
        for mode in card.get_modes(handle).unwrap() {
            println!("{mode:?}");
        }
    }
}
