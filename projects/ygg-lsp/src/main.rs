#![feature(once_cell)]
#![feature(extend_one)]

use lspower::{
    jsonrpc::{Result},
    lsp::*,
    Client, LanguageServer, LspService, Server,
};
use serde_json::Value;


use crate::{
    commands::{command_provider, server_commands},
    completion::{completion_provider, COMPLETION_OPTIONS},
    hint::{code_action_provider, code_lens_provider, document_symbol_provider, hover_provider},
};

mod commands;
mod completion;
mod hint;
#[allow(dead_code)]
mod io;

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[lspower::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        let server_info = ServerInfo {
            name: format!("Yggdrasil Config LSP"),
            version: Some(format!("v{}", env!("CARGO_PKG_VERSION"))),
        };
        let ws = WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: None,
        };
        let init = InitializeResult {
            server_info: Some(server_info),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(COMPLETION_OPTIONS.to_owned()),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: None,
                    retrigger_characters: None,
                    work_done_progress_options: Default::default(),
                }),
                selection_range_provider: Some(SelectionRangeProviderCapability::Simple(true)),
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                code_lens_provider: Some(CodeLensOptions { resolve_provider: None }),
                document_highlight_provider: Some(OneOf::Left(false)),
                document_symbol_provider: Some(OneOf::Left(false)),
                document_formatting_provider: Some(OneOf::Left(false)),
                workspace_symbol_provider: Some(OneOf::Left(false)),
                execute_command_provider: Some(server_commands()),
                workspace: Some(ws),
                ..ServerCapabilities::default()
            },
            offset_encoding: None,
        };
        return Ok(init);
    }
    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::Info, "Yggdrasil server initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
    }
    async fn symbol(&self, params: WorkspaceSymbolParams) -> Result<Option<Vec<SymbolInformation>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(command_provider(params, &self.client).await)
    }
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let url = params.text_document.uri.clone();
        self.check_the_file(&url).await;
    }
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
    }
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let url = params.text_document.uri.clone();
        self.check_the_file(&url).await;
    }
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let url = params.text_document.uri.clone();
        self.check_the_file(&url).await;
    }
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        // completion_resolve was closed
        Ok(completion_provider(params).await)
    }
    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(hover_provider(params))
    }
    async fn signature_help(&self, params: SignatureHelpParams) -> Result<Option<SignatureHelp>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
    /// 当光标在位置 x 时, 哪些内容要被选中
    async fn document_highlight(
        &self,
        _: DocumentHighlightParams,
    ) -> Result<Option<Vec<DocumentHighlight>>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", hp)).await;
        Ok(None)
    }

    #[rustfmt::skip]
    async fn document_symbol(&self, params: DocumentSymbolParams) -> Result<Option<DocumentSymbolResponse>> {
        document_symbol_provider(params).await
    }

    /// Alt 键列出可执行的命令
    async fn code_action(&self, params: CodeActionParams) -> Result<Option<CodeActionResponse>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(code_action_provider(params))
    }
    /// 单独一行的特殊注释
    async fn code_lens(&self, params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        // self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(code_lens_provider(params))
    }
    async fn document_link(&self, params: DocumentLinkParams) -> Result<Option<Vec<DocumentLink>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(vec![])
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }

    async fn rename(&self, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        self.client.log_message(MessageType::Info, format!("{:#?}", params)).await;
        Ok(None)
    }
}

#[tokio::main]
async fn main() {
    let std_in = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, messages) = LspService::new(|client| Backend { client });
    Server::new(std_in, stdout).interleave(messages).serve(service).await;
}
