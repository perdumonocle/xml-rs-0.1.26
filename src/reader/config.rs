//! Contains parser configuration structure.

/// Parser configuration structure.
///
/// This structure contains various configuration options which affect
/// behavior of the parser.
pub struct ParserConfig {
    /// Whether or not should whitespace be removed. Default is false.
    ///
    /// When true, all standalone whitespace will be removed (this means no
    /// `Whitespace` events will ve emitted), and leading and trailing whitespace 
    /// from `Character` events will be deleted. If after trimming `Characters`
    /// event will be empty, it will also be omitted from output stream. This is
    /// possible, however, only if `whitespace_to_characters` or 
    /// `cdata_to_characters` options are set.
    ///
    /// This option does not affect CDATA events, unless `cdata_to_characters`
    /// option is also set. In that case CDATA content will also be trimmed.
    pub trim_whitespace: bool,

    /// Whether or not should whitespace be converted to characters.
    /// Default is false.
    ///
    /// If true, instead of `Whitespace` events `Characters` events with the
    /// same content will be emitted. If `trim_whitespace` is also true, these
    /// events will be trimmed to nothing and, consequently, not emitted.
    pub whitespace_to_characters: bool,

    /// Whether or not should CDATA be converted to characters.
    /// Default is false.
    ///
    /// If true, instead of `CData` events `Characters` events with the same
    /// content will be emitted. If `trim_whitespace` is also true, these events
    /// will be trimmed. If corresponding CDATA contained nothing but whitespace,
    /// this event will be omitted from the stream.
    pub cdata_to_characters: bool,

    /// Whether or not should comments be omitted. Default is true.
    ///
    /// If true, `Comment` events will not be emitted at all.
    pub ignore_comments: bool,

    /// Whether or not should sequential `Characters` events be merged.
    /// Default is true.
    ///
    /// If true, multiple sequential `Characters` events will be merged into
    /// a single event, that is, their data will be concatenated.
    ///
    /// Multiple sequential `Characters` events are only possible if either
    /// `cdata_to_characters` or `ignore_comments` are set. Otherwise character
    /// events will always be separated by other events.
    pub coalesce_characters: bool
}

impl ParserConfig {
    /// Returns a new config with default values.
    ///
    /// You can tweak default values using builder-like pattern:
    ///
    /// ```rust
    /// use xml::reader::ParserConfig;
    ///
    /// let config = ParserConfig::new()
    ///     .trim_whitespace(true)
    ///     .ignore_comments(true)
    ///     .coalesce_characters(false);
    /// ```
    pub fn new() -> ParserConfig {
        ParserConfig {
            trim_whitespace: false,
            whitespace_to_characters: false,
            cdata_to_characters: false,
            ignore_comments: true,
            coalesce_characters: true
        }
    }
}

gen_setters!(ParserConfig,
    trim_whitespace: bool,
    whitespace_to_characters: bool,
    cdata_to_characters: bool,
    ignore_comments: bool,
    coalesce_characters: bool
);
