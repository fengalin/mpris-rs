#![allow(unknown_lints)]
#![allow(clippy::all)]
#![allow(missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications,
        unused_imports)]
// This code was autogenerated with dbus-codegen-rust, see https://github.com/diwic/dbus-rs

use dbus as dbus;
use dbus::arg;
use dbus::ffidisp;

pub trait OrgMprisMediaPlayer2 {
    fn raise(&self) -> Result<(), dbus::Error>;
    fn quit(&self) -> Result<(), dbus::Error>;
    fn get_can_quit(&self) -> Result<bool, dbus::Error>;
    fn get_fullscreen(&self) -> Result<bool, dbus::Error>;
    fn set_fullscreen(&self, value: bool) -> Result<(), dbus::Error>;
    fn get_can_set_fullscreen(&self) -> Result<bool, dbus::Error>;
    fn get_can_raise(&self) -> Result<bool, dbus::Error>;
    fn get_has_track_list(&self) -> Result<bool, dbus::Error>;
    fn get_identity(&self) -> Result<String, dbus::Error>;
    fn get_desktop_entry(&self) -> Result<String, dbus::Error>;
    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, dbus::Error>;
    fn get_supported_mime_types(&self) -> Result<Vec<String>, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=ffidisp::Connection>> OrgMprisMediaPlayer2 for ffidisp::ConnPath<'a, C> {

    fn raise(&self) -> Result<(), dbus::Error> {
        self.method_call("org.mpris.MediaPlayer2", "Raise", ())
    }

    fn quit(&self) -> Result<(), dbus::Error> {
        self.method_call("org.mpris.MediaPlayer2", "Quit", ())
    }

    fn get_can_quit(&self) -> Result<bool, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanQuit")
    }

    fn get_fullscreen(&self) -> Result<bool, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "Fullscreen")
    }

    fn get_can_set_fullscreen(&self) -> Result<bool, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanSetFullscreen")
    }

    fn get_can_raise(&self) -> Result<bool, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "CanRaise")
    }

    fn get_has_track_list(&self) -> Result<bool, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "HasTrackList")
    }

    fn get_identity(&self) -> Result<String, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "Identity")
    }

    fn get_desktop_entry(&self) -> Result<String, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "DesktopEntry")
    }

    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "SupportedUriSchemes")
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.mpris.MediaPlayer2", "SupportedMimeTypes")
    }

    fn set_fullscreen(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as ffidisp::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.mpris.MediaPlayer2", "Fullscreen", value)
    }
}