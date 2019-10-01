/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::cell::DomRefCell;
use crate::dom::bindings::codegen::Bindings::MediaMetadataBinding;
use crate::dom::bindings::codegen::Bindings::MediaMetadataBinding::MediaMetadataInit;
use crate::dom::bindings::codegen::Bindings::MediaMetadataBinding::MediaMetadataMethods;
use crate::dom::bindings::error::Fallible;
use crate::dom::bindings::reflector::{reflect_dom_object, Reflector};
use crate::dom::bindings::root::{DomRoot, MutNullableDom};
use crate::dom::bindings::str::DOMString;
use crate::dom::mediasession::MediaSession;
use crate::dom::window::Window;
use dom_struct::dom_struct;

#[dom_struct]
pub struct MediaMetadata {
    reflector_: Reflector,
    session: MutNullableDom<MediaSession>,
    title: DomRefCell<DOMString>,
    artist: DomRefCell<DOMString>,
    album: DomRefCell<DOMString>,
}

impl MediaMetadata {
    fn new_inherited(init: &MediaMetadataInit) -> MediaMetadata {
        MediaMetadata {
            reflector_: Reflector::new(),
            // TODO(ferjm): Set active media session?
            session: Default::default(),
            title: DomRefCell::new(init.title.clone()),
            artist: DomRefCell::new(init.artist.clone()),
            album: DomRefCell::new(init.album.clone()),
        }
    }

    pub fn new(global: &Window, init: &MediaMetadataInit) -> DomRoot<MediaMetadata> {
        reflect_dom_object(
            Box::new(MediaMetadata::new_inherited(init)),
            global,
            MediaMetadataBinding::Wrap,
        )
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-mediametadata
    pub fn Constructor(
        window: &Window,
        init: &MediaMetadataInit,
    ) -> Fallible<DomRoot<MediaMetadata>> {
        Ok(MediaMetadata::new(window, init))
    }

    fn queue_update_metadata_algorithm(&self) {
        if self.session.get().is_none() {
            return;
        }
    }
}

impl MediaMetadataMethods for MediaMetadata {
    /// https://w3c.github.io/mediasession/#dom-mediametadata-title
    fn Title(&self) -> DOMString {
        self.title.borrow().clone()
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-title
    fn SetTitle(&self, value: DOMString) -> () {
        *self.title.borrow_mut() = value;
        self.queue_update_metadata_algorithm();
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-artist
    fn Artist(&self) -> DOMString {
        self.artist.borrow().clone()
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-artist
    fn SetArtist(&self, value: DOMString) -> () {
        *self.artist.borrow_mut() = value;
        self.queue_update_metadata_algorithm();
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-album
    fn Album(&self) -> DOMString {
        self.album.borrow().clone()
    }

    /// https://w3c.github.io/mediasession/#dom-mediametadata-album
    fn SetAlbum(&self, value: DOMString) -> () {
        *self.album.borrow_mut() = value;
        self.queue_update_metadata_algorithm();
    }
}
