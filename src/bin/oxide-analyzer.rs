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

        // TODO: Use COMPILER.change_file(...);
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
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });

    Server::new(stdin, stdout, socket).serve(service).await;
}
