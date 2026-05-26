use crate::conversion;
use crate::protocol::{Message, read_message, write_error, write_notification, write_response};
use crate::uri::uri_to_path;
use dowe_compiler::{
    LanguageDocument, analyze_document, complete_document, definition_at, document_symbols,
    find_workspace_root, format_document, hover_at,
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader};
use std::path::PathBuf;

pub fn run() -> io::Result<()> {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut server = LanguageServer::default();
    while let Some(message) = read_message(&mut reader)? {
        if server.handle(message)? {
            break;
        }
    }
    Ok(())
}

#[derive(Default)]
struct LanguageServer {
    root: PathBuf,
    documents: HashMap<String, LanguageDocument>,
}

impl LanguageServer {
    fn handle(&mut self, message: Message) -> io::Result<bool> {
        let Some(method) = message.method.as_deref() else {
            return Ok(false);
        };
        match method {
            "initialize" => self.initialize(message),
            "initialized" => Ok(false),
            "shutdown" => {
                if let Some(id) = message.id {
                    write_response(id, Value::Null)?;
                }
                Ok(false)
            }
            "exit" => Ok(true),
            "textDocument/didOpen" => {
                self.did_open(&message.params)?;
                Ok(false)
            }
            "textDocument/didChange" => {
                self.did_change(&message.params)?;
                Ok(false)
            }
            "textDocument/didSave" => {
                self.did_save(&message.params)?;
                Ok(false)
            }
            "textDocument/didClose" => {
                self.did_close(&message.params)?;
                Ok(false)
            }
            "textDocument/completion" => {
                self.respond(message, |server, params| server.completion(params))
            }
            "textDocument/hover" => self.respond(message, |server, params| server.hover(params)),
            "textDocument/definition" => {
                self.respond(message, |server, params| server.definition(params))
            }
            "textDocument/documentSymbol" => {
                self.respond(message, |server, params| server.document_symbols(params))
            }
            "textDocument/formatting" => {
                self.respond(message, |server, params| server.formatting(params))
            }
            _ => {
                if let Some(id) = message.id {
                    write_error(id, -32601, "method not found")?;
                }
                Ok(false)
            }
        }
    }

    fn initialize(&mut self, message: Message) -> io::Result<bool> {
        self.root = root_from_initialize(&message.params).unwrap_or_else(current_root);
        if let Some(id) = message.id {
            write_response(id, capabilities())?;
        }
        Ok(false)
    }

    fn respond(
        &mut self,
        message: Message,
        handler: impl FnOnce(&mut Self, &Value) -> io::Result<Value>,
    ) -> io::Result<bool> {
        let Some(id) = message.id else {
            return Ok(false);
        };
        match handler(self, &message.params) {
            Ok(value) => write_response(id, value)?,
            Err(error) => write_error(id, -32603, error.to_string())?,
        }
        Ok(false)
    }

    fn did_open(&mut self, params: &Value) -> io::Result<()> {
        let Some(uri) = params.pointer("/textDocument/uri").and_then(Value::as_str) else {
            return Ok(());
        };
        let Some(path) = uri_to_path(uri) else {
            return Ok(());
        };
        let source = params
            .pointer("/textDocument/text")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let document = LanguageDocument { path, source };
        self.documents.insert(uri.to_string(), document);
        self.publish(uri)
    }

    fn did_change(&mut self, params: &Value) -> io::Result<()> {
        let Some(uri) = params.pointer("/textDocument/uri").and_then(Value::as_str) else {
            return Ok(());
        };
        let Some(change) = params
            .get("contentChanges")
            .and_then(Value::as_array)
            .and_then(|values| values.last())
            .and_then(|value| value.get("text"))
            .and_then(Value::as_str)
        else {
            return Ok(());
        };
        if let Some(document) = self.documents.get_mut(uri) {
            document.source = change.to_string();
        }
        self.publish(uri)
    }

    fn did_save(&mut self, params: &Value) -> io::Result<()> {
        let Some(uri) = params.pointer("/textDocument/uri").and_then(Value::as_str) else {
            return Ok(());
        };
        if !self.documents.contains_key(uri)
            && let Some(path) = uri_to_path(uri)
            && let Ok(source) = fs::read_to_string(&path)
        {
            self.documents
                .insert(uri.to_string(), LanguageDocument { path, source });
        }
        self.publish(uri)
    }

    fn did_close(&mut self, params: &Value) -> io::Result<()> {
        let Some(uri) = params.pointer("/textDocument/uri").and_then(Value::as_str) else {
            return Ok(());
        };
        self.documents.remove(uri);
        write_notification(
            "textDocument/publishDiagnostics",
            json!({ "uri": uri, "diagnostics": [] }),
        )
    }

    fn completion(&mut self, params: &Value) -> io::Result<Value> {
        let Some((document, line, column)) = self.document_position(params) else {
            return Ok(Value::Null);
        };
        Ok(Value::Array(
            complete_document(&self.root, &document, line, column)
                .iter()
                .map(conversion::completion)
                .collect(),
        ))
    }

    fn hover(&mut self, params: &Value) -> io::Result<Value> {
        let Some((document, line, column)) = self.document_position(params) else {
            return Ok(Value::Null);
        };
        let Some(value) = hover_at(&self.root, &document, line, column) else {
            return Ok(Value::Null);
        };
        Ok(json!({
            "contents": {
                "kind": "plaintext",
                "value": value
            }
        }))
    }

    fn definition(&mut self, params: &Value) -> io::Result<Value> {
        let Some((document, line, column)) = self.document_position(params) else {
            return Ok(Value::Null);
        };
        Ok(definition_at(&self.root, &document, line, column)
            .map(|value| conversion::location(&value))
            .unwrap_or(Value::Null))
    }

    fn document_symbols(&mut self, params: &Value) -> io::Result<Value> {
        let Some(document) = self.document(params) else {
            return Ok(Value::Array(Vec::new()));
        };
        Ok(Value::Array(
            document_symbols(&self.root, &document)
                .iter()
                .map(conversion::symbol)
                .collect(),
        ))
    }

    fn formatting(&mut self, params: &Value) -> io::Result<Value> {
        let Some(document) = self.document(params) else {
            return Ok(Value::Null);
        };
        let formatted = match format_document(&self.root, &document.path, &document.source) {
            Ok(value) => value,
            Err(_) => return Ok(Value::Array(Vec::new())),
        };
        if formatted == document.source {
            return Ok(Value::Array(Vec::new()));
        }
        Ok(json!([{
            "range": conversion::full_document_range(&document.source),
            "newText": formatted
        }]))
    }

    fn publish(&mut self, uri: &str) -> io::Result<()> {
        let Some(document) = self.documents.get(uri) else {
            return Ok(());
        };
        let diagnostics = analyze_document(&self.root, document)
            .iter()
            .map(conversion::diagnostic)
            .collect::<Vec<_>>();
        write_notification(
            "textDocument/publishDiagnostics",
            json!({ "uri": uri, "diagnostics": diagnostics }),
        )
    }

    fn document_position(&self, params: &Value) -> Option<(LanguageDocument, usize, usize)> {
        let document = self.document(params)?;
        let line = params.pointer("/position/line")?.as_u64()? as usize + 1;
        let column = params.pointer("/position/character")?.as_u64()? as usize + 1;
        Some((document, line, column))
    }

    fn document(&self, params: &Value) -> Option<LanguageDocument> {
        let uri = params.pointer("/textDocument/uri")?.as_str()?;
        self.documents.get(uri).cloned().or_else(|| {
            let path = uri_to_path(uri)?;
            let source = fs::read_to_string(&path).ok()?;
            Some(LanguageDocument { path, source })
        })
    }
}

fn root_from_initialize(params: &Value) -> Option<PathBuf> {
    params
        .get("rootUri")
        .and_then(Value::as_str)
        .and_then(uri_to_path)
        .or_else(|| {
            params
                .get("rootPath")
                .and_then(Value::as_str)
                .map(PathBuf::from)
        })
        .and_then(|path| find_workspace_root(&path).or(Some(path)))
}

fn current_root() -> PathBuf {
    let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    find_workspace_root(&current).unwrap_or(current)
}

fn capabilities() -> Value {
    json!({
        "capabilities": {
            "textDocumentSync": {
                "openClose": true,
                "change": 1,
                "save": true
            },
            "completionProvider": {
                "triggerCharacters": [".", ":", "/", "\""]
            },
            "hoverProvider": true,
            "definitionProvider": true,
            "documentSymbolProvider": true,
            "documentFormattingProvider": true
        },
        "serverInfo": {
            "name": "dowe-language-server",
            "version": env!("CARGO_PKG_VERSION")
        }
    })
}
