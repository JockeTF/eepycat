use syntect::easy::HighlightLines;
use syntect::highlighting::Color;
use syntect::highlighting::Theme;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use syntect::util::as_24_bit_terminal_escaped;

const THEME: &str = "Solarized (dark)";

fn brighten_color(color: &mut Color) {
    color.r = color.r.saturating_add(32);
    color.g = color.g.saturating_add(32);
    color.b = color.b.saturating_add(32);
}

fn brighten_theme(theme: &mut Theme) {
    theme.settings.foreground.as_mut().map(brighten_color);

    for item in &mut theme.scopes {
        item.style.foreground.as_mut().map(brighten_color);
    }
}

pub struct Syntect {
    theme: Theme,
    types: SyntaxSet,
}

impl Syntect {
    pub fn new() -> Self {
        let themes = ThemeSet::load_defaults();
        let types = SyntaxSet::load_defaults_nonewlines();

        let mut theme = themes.themes[THEME].clone();

        brighten_theme(&mut theme);

        Syntect { theme, types }
    }

    pub fn highlight(&self, ext: &str, text: &str) -> String {
        let theme = &self.theme;
        let types = &self.types;

        let syntax = types
            .find_syntax_by_extension(ext)
            .unwrap_or_else(|| types.find_syntax_plain_text());

        let lines = LinesWithEndings::from(text);
        let mut state = HighlightLines::new(syntax, theme);

        lines
            .flat_map(|line| state.highlight_line(line, types))
            .map(|vec| as_24_bit_terminal_escaped(&vec, false))
            .collect::<String>()
    }
}
