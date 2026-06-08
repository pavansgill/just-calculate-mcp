use rmcp::{
    ServerHandler,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

use crate::tools::arithmetic::{ArithmeticInput, compute};

#[derive(Debug, Clone)]
pub struct Calculator {
    tool_router: ToolRouter<Self>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router]
impl Calculator {
    #[tool(description = "Perform a simple arithmetic operation (+, -, *, /, %) on two numbers")]
    pub fn simple_arithmetic(
        &self,
        Parameters(input): Parameters<ArithmeticInput>,
    ) -> String {
        compute(input)
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("A simple arithmetic calculator MCP server")
    }
}
