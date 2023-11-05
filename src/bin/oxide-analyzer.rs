use compiler::COMPILER;
use tower_lsp::{jsonrpc::Result, lsp_types::*, Client, LanguageServer, LspService, Server};

struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let workspace = params.root_uri.unwrap().to_file_path().unwrap();

        COMPILER.initialize(
            workspace,
            compiler::InitializeOptions {
                watch: compiler::Watch::Yes,
                block: compiler::Block::No,
            },
        );

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensOptions(
                        SemanticTokensOptions {
                            work_done_progress_options: WorkDoneProgressOptions {
                                work_done_progress: Some(false),
                            },
                            legend: SemanticTokensLegend {
                                token_types: vec![SemanticTokenType::KEYWORD],
                                token_modifiers: vec![],
                            },
                            range: Some(false),
                            full: Some(SemanticTokensFullOptions::Bool(true)),
                        },
                    ),
                ),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("{} opened!", params.text_document.uri),
            )
            .await;

        COMPILER.open_file(
            params.text_document.uri.to_file_path().unwrap(),
            &params.text_document.text,
        );
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("{} changed!", params.text_document.uri),
            )
            .await;

        let change = &params.content_changes[0];
        let range = change.range.unwrap();

        let start = compiler::Position {
            line: range.start.line as usize,
            character: range.start.character as usize,
        };

        let end = compiler::Position {
            line: range.end.line as usize,
            character: range.end.character as usize,
        };

        COMPILER.change_file(
            params.text_document.uri.to_file_path().unwrap(),
            start..end,
            &change.text,
        );
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("{} closed!", params.text_document.uri),
            )
            .await;

        COMPILER.close_file(params.text_document.uri.to_file_path().unwrap());
    }

    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        self.client
            .log_message(MessageType::INFO, format!("{:#?}", params))
            .await;

        // TODO: Need a SemanticTokensBuilder as they are pretty complicated.
        // TODO: Basically, the delta line and start is how the token is relative
        // TODO: to the last one (line is line and start is character). Start is
        // TODO: relative to 0 if there was no previous token on the same line.
        // TODO: Length is just the length of the token. Token type is the index
        // TODO: of the token type in the legend. Token modifiers bitset is a flag
        // TODO: containing all of the modifiers for the token (based on legend).
        // TODO: For example, 0 means no modifiers, 1 means the first modifier
        // TODO: 3 (0b11) means the first two modifiers, ...

        // TODO: Figure out how to use text mate scopes and stuff for more control.

        let tokens = vec![SemanticToken {
            delta_line: 0,
            delta_start: 0,
            length: 2,
            token_type: 0,
            token_modifiers_bitset: 0,
        }];

        let result = SemanticTokens {
            result_id: None,
            data: tokens,
        };

        Ok(Some(SemanticTokensResult::Tokens(result)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(stdin, stdout, socket).serve(service).await;
}
