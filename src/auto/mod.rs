// This file was generated by gir (e95e5d8) from gir-files (db49619)
// DO NOT EDIT

mod buffer;
pub use self::buffer::Buffer;
pub use self::buffer::BufferExt;

mod completion;
pub use self::completion::Completion;
pub use self::completion::CompletionExt;

mod completion_context;
pub use self::completion_context::CompletionContext;
pub use self::completion_context::CompletionContextExt;

mod completion_info;
pub use self::completion_info::CompletionInfo;
pub use self::completion_info::CompletionInfoExt;

mod completion_item;
pub use self::completion_item::CompletionItem;
pub use self::completion_item::CompletionItemExt;

mod completion_proposal;
pub use self::completion_proposal::CompletionProposal;
pub use self::completion_proposal::CompletionProposalExt;

mod completion_provider;
pub use self::completion_provider::CompletionProvider;
pub use self::completion_provider::CompletionProviderExt;

mod completion_words;
pub use self::completion_words::CompletionWords;
pub use self::completion_words::CompletionWordsExt;

#[cfg(feature = "v3_14")]
mod file;
#[cfg(feature = "v3_14")]
pub use self::file::File;
#[cfg(feature = "v3_14")]
pub use self::file::FileExt;

#[cfg(feature = "v3_14")]
mod file_loader;
#[cfg(feature = "v3_14")]
pub use self::file_loader::FileLoader;
#[cfg(feature = "v3_14")]
pub use self::file_loader::FileLoaderExt;

#[cfg(feature = "v3_14")]
mod file_saver;
#[cfg(feature = "v3_14")]
pub use self::file_saver::FileSaver;
#[cfg(feature = "v3_14")]
pub use self::file_saver::FileSaverExt;

mod gutter;
pub use self::gutter::Gutter;
pub use self::gutter::GutterExt;

mod gutter_renderer;
pub use self::gutter_renderer::GutterRenderer;
pub use self::gutter_renderer::GutterRendererExt;

mod gutter_renderer_pixbuf;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbuf;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufExt;

mod gutter_renderer_text;
pub use self::gutter_renderer_text::GutterRendererText;
pub use self::gutter_renderer_text::GutterRendererTextExt;

mod language;
pub use self::language::Language;
pub use self::language::LanguageExt;

mod language_manager;
pub use self::language_manager::LanguageManager;
pub use self::language_manager::LanguageManagerExt;

#[cfg(feature = "v3_18")]
mod map;
#[cfg(feature = "v3_18")]
pub use self::map::Map;
#[cfg(feature = "v3_18")]
pub use self::map::MapExt;

mod mark;
pub use self::mark::Mark;
pub use self::mark::MarkExt;

mod mark_attributes;
pub use self::mark_attributes::MarkAttributes;
pub use self::mark_attributes::MarkAttributesExt;

mod print_compositor;
pub use self::print_compositor::PrintCompositor;
pub use self::print_compositor::PrintCompositorExt;

#[cfg(feature = "v3_10")]
mod search_context;
#[cfg(feature = "v3_10")]
pub use self::search_context::SearchContext;
#[cfg(feature = "v3_10")]
pub use self::search_context::SearchContextExt;

#[cfg(feature = "v3_10")]
mod search_settings;
#[cfg(feature = "v3_10")]
pub use self::search_settings::SearchSettings;
#[cfg(feature = "v3_10")]
pub use self::search_settings::SearchSettingsExt;

#[cfg(feature = "v3_24")]
mod space_drawer;
#[cfg(feature = "v3_24")]
pub use self::space_drawer::SpaceDrawer;
#[cfg(feature = "v3_24")]
pub use self::space_drawer::SpaceDrawerExt;

mod style;
pub use self::style::Style;
pub use self::style::StyleExt;

mod style_scheme;
pub use self::style_scheme::StyleScheme;
pub use self::style_scheme::StyleSchemeExt;

#[cfg(feature = "v3_16")]
mod style_scheme_chooser;
#[cfg(feature = "v3_16")]
pub use self::style_scheme_chooser::StyleSchemeChooser;
#[cfg(feature = "v3_16")]
pub use self::style_scheme_chooser::StyleSchemeChooserExt;

#[cfg(feature = "v3_16")]
mod style_scheme_chooser_button;
#[cfg(feature = "v3_16")]
pub use self::style_scheme_chooser_button::StyleSchemeChooserButton;

#[cfg(feature = "v3_16")]
mod style_scheme_chooser_widget;
#[cfg(feature = "v3_16")]
pub use self::style_scheme_chooser_widget::StyleSchemeChooserWidget;

mod style_scheme_manager;
pub use self::style_scheme_manager::StyleSchemeManager;
pub use self::style_scheme_manager::StyleSchemeManagerExt;

#[cfg(feature = "v3_20")]
mod tag;
#[cfg(feature = "v3_20")]
pub use self::tag::Tag;
#[cfg(feature = "v3_20")]
pub use self::tag::TagExt;

mod undo_manager;
pub use self::undo_manager::UndoManager;
pub use self::undo_manager::UndoManagerExt;

mod view;
pub use self::view::View;
pub use self::view::ViewExt;

#[cfg(feature = "v3_14")]
mod encoding;
#[cfg(feature = "v3_14")]
pub use self::encoding::Encoding;

mod enums;
#[cfg(feature = "v3_16")]
pub use self::enums::BackgroundPatternType;
#[cfg(feature = "v3_12")]
pub use self::enums::ChangeCaseType;
#[cfg(feature = "v3_14")]
pub use self::enums::CompressionType;
pub use self::enums::GutterRendererAlignmentMode;
#[cfg(feature = "v3_14")]
pub use self::enums::NewlineType;
pub use self::enums::SmartHomeEndType;

mod flags;
pub use self::flags::CompletionActivation;
pub use self::flags::COMPLETION_ACTIVATION_NONE;
pub use self::flags::COMPLETION_ACTIVATION_INTERACTIVE;
pub use self::flags::COMPLETION_ACTIVATION_USER_REQUESTED;
pub use self::flags::DrawSpacesFlags;
pub use self::flags::DRAW_SPACES_SPACE;
pub use self::flags::DRAW_SPACES_TAB;
pub use self::flags::DRAW_SPACES_NEWLINE;
pub use self::flags::DRAW_SPACES_NBSP;
pub use self::flags::DRAW_SPACES_LEADING;
pub use self::flags::DRAW_SPACES_TEXT;
pub use self::flags::DRAW_SPACES_TRAILING;
pub use self::flags::DRAW_SPACES_ALL;
#[cfg(feature = "v3_14")]
pub use self::flags::FileSaverFlags;
#[cfg(feature = "v3_14")]
pub use self::flags::FILE_SAVER_FLAGS_NONE;
#[cfg(feature = "v3_14")]
pub use self::flags::FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS;
#[cfg(feature = "v3_14")]
pub use self::flags::FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME;
#[cfg(feature = "v3_14")]
pub use self::flags::FILE_SAVER_FLAGS_CREATE_BACKUP;
pub use self::flags::GutterRendererState;
pub use self::flags::GUTTER_RENDERER_STATE_NORMAL;
pub use self::flags::GUTTER_RENDERER_STATE_CURSOR;
pub use self::flags::GUTTER_RENDERER_STATE_PRELIT;
pub use self::flags::GUTTER_RENDERER_STATE_SELECTED;
#[cfg(feature = "v3_18")]
pub use self::flags::SortFlags;
#[cfg(feature = "v3_18")]
pub use self::flags::SORT_FLAGS_NONE;
#[cfg(feature = "v3_18")]
pub use self::flags::SORT_FLAGS_CASE_SENSITIVE;
#[cfg(feature = "v3_18")]
pub use self::flags::SORT_FLAGS_REVERSE_ORDER;
#[cfg(feature = "v3_18")]
pub use self::flags::SORT_FLAGS_REMOVE_DUPLICATES;

#[doc(hidden)]
pub mod traits {
    pub use super::BufferExt;
    pub use super::CompletionExt;
    pub use super::CompletionContextExt;
    pub use super::CompletionInfoExt;
    pub use super::CompletionItemExt;
    pub use super::CompletionProposalExt;
    pub use super::CompletionProviderExt;
    pub use super::CompletionWordsExt;
    #[cfg(feature = "v3_14")]
    pub use super::FileExt;
    #[cfg(feature = "v3_14")]
    pub use super::FileLoaderExt;
    #[cfg(feature = "v3_14")]
    pub use super::FileSaverExt;
    pub use super::GutterExt;
    pub use super::GutterRendererExt;
    pub use super::GutterRendererPixbufExt;
    pub use super::GutterRendererTextExt;
    pub use super::LanguageExt;
    pub use super::LanguageManagerExt;
    #[cfg(feature = "v3_18")]
    pub use super::MapExt;
    pub use super::MarkExt;
    pub use super::MarkAttributesExt;
    pub use super::PrintCompositorExt;
    #[cfg(feature = "v3_10")]
    pub use super::SearchContextExt;
    #[cfg(feature = "v3_10")]
    pub use super::SearchSettingsExt;
    #[cfg(feature = "v3_24")]
    pub use super::SpaceDrawerExt;
    pub use super::StyleExt;
    pub use super::StyleSchemeExt;
    #[cfg(feature = "v3_16")]
    pub use super::StyleSchemeChooserExt;
    pub use super::StyleSchemeManagerExt;
    #[cfg(feature = "v3_20")]
    pub use super::TagExt;
    pub use super::UndoManagerExt;
    pub use super::ViewExt;
}
