use crate::uri::path_to_uri;
use dowe_compiler::{
    LanguageCompletion, LanguageCompletionKind, LanguageDiagnostic, LanguageDiagnosticSeverity,
    LanguageDocumentSymbol, LanguageLocation, LanguagePosition, LanguageRange, LanguageSymbolKind,
};
use serde_json::{Value, json};

pub fn diagnostic(value: &LanguageDiagnostic) -> Value {
    json!({
        "range": range(&value.range),
        "severity": diagnostic_severity(value.severity),
        "code": value.code,
        "source": "dowe",
        "message": value.message
    })
}

pub fn completion(value: &LanguageCompletion) -> Value {
    json!({
        "label": value.label,
        "kind": completion_kind(value.kind),
        "detail": value.detail
    })
}

pub fn location(value: &LanguageLocation) -> Value {
    json!({
        "uri": path_to_uri(&value.path),
        "range": range(&value.range)
    })
}

pub fn symbol(value: &LanguageDocumentSymbol) -> Value {
    json!({
        "name": value.name,
        "kind": symbol_kind(&value.kind),
        "range": range(&value.range),
        "selectionRange": range(&value.selection_range),
        "children": value.children.iter().map(symbol).collect::<Vec<_>>()
    })
}

pub fn range(value: &LanguageRange) -> Value {
    json!({
        "start": position(value.start),
        "end": position(value.end)
    })
}

pub fn full_document_range(source: &str) -> Value {
    let line_count = source.lines().count();
    json!({
        "start": { "line": 0, "character": 0 },
        "end": { "line": line_count, "character": 0 }
    })
}

fn position(value: LanguagePosition) -> Value {
    json!({
        "line": value.line.saturating_sub(1),
        "character": value.column.saturating_sub(1)
    })
}

fn diagnostic_severity(value: LanguageDiagnosticSeverity) -> u8 {
    match value {
        LanguageDiagnosticSeverity::Error => 1,
        LanguageDiagnosticSeverity::Warning => 2,
        LanguageDiagnosticSeverity::Info => 3,
        LanguageDiagnosticSeverity::Hint => 4,
    }
}

fn completion_kind(value: LanguageCompletionKind) -> u8 {
    match value {
        LanguageCompletionKind::Keyword => 14,
        LanguageCompletionKind::Component => 7,
        LanguageCompletionKind::Property => 10,
        LanguageCompletionKind::Value => 12,
        LanguageCompletionKind::Function => 3,
        LanguageCompletionKind::Variable => 6,
        LanguageCompletionKind::File => 17,
    }
}

fn symbol_kind(value: &LanguageSymbolKind) -> u8 {
    match value {
        LanguageSymbolKind::File => 1,
        LanguageSymbolKind::Module => 2,
        LanguageSymbolKind::Class => 5,
        LanguageSymbolKind::Function => 12,
        LanguageSymbolKind::Method => 6,
        LanguageSymbolKind::Property => 7,
        LanguageSymbolKind::Variable => 13,
    }
}
