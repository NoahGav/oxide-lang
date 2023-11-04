use oxide_compiler::Compiler;

use tower_lsp::{
    jsonrpc::Result,
    lsp_types::{InitializeParams, InitializeResult, InitializedParams, MessageType},
    Client, LanguageServer, LspService, Server,
};

struct Backend {
    _compiler: Compiler,
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult::default())
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        _compiler: Compiler::default(),
        client,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

// let mut compiler = Compiler::default();

// compiler.mutate(|state| state.add(Document::new("main.ox", "fn foo(bar: i32)")));

// let snapshot = compiler.snapshot();
// let tree = snapshot.get_syntax_tree("main.ox");

// println!("{:#?}", tree);
